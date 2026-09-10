//! Filename, field, FSM, S11, and publication gates.

use governance_contracts::{ContractError, GovernanceRecord, RecordKind, SourceClass, Status};
use std::path::Path;
use thiserror::Error;

/// Illegal S11 vocabulary. Public text must not read as clinical or diagnostic.
pub const S11_BANNED: &[&str] = &[
    "diagnosis",
    "diagnostic",
    "diagnoses",
    "treatment",
    "therapy",
    "therapeutic",
    "patient",
    "clinical",
    "disease",
    "disorder",
    "symptom",
    "cure",
    "icd-",
    "medical claim",
    "biological age",
    "health score",
    "predicts lifespan",
    "extends life",
];

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum GateError {
    #[error("filename does not match KIND-YEAR-SEQ.md: {0}")]
    Filename(String),
    #[error("id mismatch with filename: {0}")]
    IdMismatch(String),
    #[error("missing required field: {0}")]
    Missing(&'static str),
    #[error("illegal FSM transition: {from:?} -> {to:?}")]
    IllegalTransition { from: Status, to: Status },
    #[error("S11 vocabulary lock: banned token `{0}`")]
    S11(String),
    #[error("publication blocked: {0}")]
    Publication(String),
    #[error("restricted material must not enter open-core: {0}")]
    Restricted(String),
    #[error("parse: {0}")]
    Parse(String),
    #[error(transparent)]
    Contract(#[from] ContractError),
}

pub fn id_pattern_ok(id: &str) -> bool {
    let parts: Vec<&str> = id.split('-').collect();
    if parts.len() != 3 {
        return false;
    }
    RecordKind::from_prefix(parts[0]).is_ok()
        && parts[1].len() == 4
        && parts[1].chars().all(|c| c.is_ascii_digit())
        && parts[2].len() == 4
        && parts[2].chars().all(|c| c.is_ascii_digit())
}

pub fn filename_id(path: &Path) -> Result<String, GateError> {
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| GateError::Filename(path.display().to_string()))?;
    let stem = name
        .strip_suffix(".md")
        .ok_or_else(|| GateError::Filename(name.to_string()))?;
    if !id_pattern_ok(stem) {
        return Err(GateError::Filename(name.to_string()));
    }
    Ok(stem.to_string())
}

pub fn can_transition(from: Status, to: Status) -> bool {
    use Status::*;
    matches!(
        (from, to),
        (Draft, Open)
            | (Draft, Archived)
            | (Open, Waiting)
            | (Open, Decided)
            | (Open, Closed)
            | (Open, Archived)
            | (Waiting, Open)
            | (Waiting, Decided)
            | (Waiting, Archived)
            | (Decided, Closed)
            | (Decided, Archived)
            | (Closed, Archived)
    )
}

pub fn assert_transition(from: Status, to: Status) -> Result<(), GateError> {
    if from == to || can_transition(from, to) {
        Ok(())
    } else {
        Err(GateError::IllegalTransition { from, to })
    }
}

pub fn scan_s11(text: &str) -> Result<(), GateError> {
    // Required PUBLIC CANDIDATE disclaimer may mention the banned class in negation.
    let mut lower = text.to_ascii_lowercase();
    lower = lower.replace("not a clinical", "");
    lower = lower.replace("not clinical", "");
    for token in S11_BANNED {
        if lower.contains(token) {
            return Err(GateError::S11((*token).to_string()));
        }
    }
    Ok(())
}

pub fn validate_record(rec: &GovernanceRecord, path: Option<&Path>) -> Result<(), GateError> {
    if !id_pattern_ok(&rec.id) {
        return Err(GateError::Filename(rec.id.clone()));
    }
    if let Some(p) = path {
        let fid = filename_id(p)?;
        if fid != rec.id {
            return Err(GateError::IdMismatch(format!("{} vs {}", fid, rec.id)));
        }
    }
    if rec.correlation_id.is_empty() {
        return Err(GateError::Missing("correlation_id"));
    }
    if rec.owner.is_empty() {
        return Err(GateError::Missing("owner"));
    }
    if rec.thesis.trim().is_empty() {
        return Err(GateError::Missing("thesis"));
    }
    let blob = format!("{}\n{}\n{}", rec.thesis, rec.body, rec.owner);
    scan_s11(&blob)?;
    if rec.body.to_ascii_uppercase().contains("RESTRICTED")
        || rec.body.to_ascii_uppercase().contains("PII")
    {
        return Err(GateError::Restricted(rec.id.clone()));
    }
    Ok(())
}

/// Publication requires evidence, source class, legal/license basis, OWNER decision.
pub fn publication_check(rec: &GovernanceRecord, body: &str) -> Result<(), GateError> {
    validate_record(rec, None)?;
    if rec.evidence.is_empty() {
        return Err(GateError::Publication("evidence required".into()));
    }
    if rec.status != Status::Decided && rec.status != Status::Closed {
        return Err(GateError::Publication(
            "OWNER decision required (status DECIDED or CLOSED)".into(),
        ));
    }
    if rec.decision.is_none() {
        return Err(GateError::Publication("decision id required".into()));
    }
    let lower = body.to_ascii_lowercase();
    let has_license = lower.contains("mit") || lower.contains("license") || lower.contains("spdx");
    let has_basis = lower.contains("legal") || lower.contains("basis") || has_license;
    if !has_basis {
        return Err(GateError::Publication(
            "legal/license basis required in body".into(),
        ));
    }
    if lower.contains("released mit product") || lower.contains("production api") {
        return Err(GateError::Publication(
            "release theatre: candidate must not claim production API".into(),
        ));
    }
    let _ = SourceClass::as_str(rec.source_class);
    Ok(())
}

/// Minimal frontmatter parser (key: value, no nested YAML).
pub fn parse_record(text: &str) -> Result<GovernanceRecord, GateError> {
    let text = text.trim_start_matches('\u{feff}');
    if !text.starts_with("---") {
        return Err(GateError::Parse("missing frontmatter".into()));
    }
    let rest = &text[3..];
    let end = rest
        .find("\n---")
        .ok_or_else(|| GateError::Parse("unclosed frontmatter".into()))?;
    let fm = &rest[..end];
    let body = rest[end + 4..].trim_start_matches('\n').to_string();

    let mut map = std::collections::BTreeMap::<String, String>::new();
    for line in fm.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (k, v) = line
            .split_once(':')
            .ok_or_else(|| GateError::Parse(format!("bad line: {line}")))?;
        map.insert(k.trim().to_string(), v.trim().trim_matches('"').to_string());
    }

    let get = |k: &'static str| -> Result<String, GateError> {
        map.get(k)
            .cloned()
            .filter(|s| !s.is_empty())
            .ok_or(GateError::Missing(k))
    };

    let id = get("id")?;
    let prefix = id.split('-').next().unwrap_or("");
    let kind = RecordKind::from_prefix(prefix)?;
    let parties = map
        .get("parties")
        .map(|s| {
            s.trim_matches(['[', ']'])
                .split(',')
                .map(|p| p.trim().trim_matches('"').to_string())
                .filter(|p| !p.is_empty())
                .collect()
        })
        .unwrap_or_default();
    let evidence = map
        .get("evidence")
        .map(|s| {
            s.trim_matches(['[', ']'])
                .split(',')
                .map(|p| p.trim().trim_matches('"').to_string())
                .filter(|p| !p.is_empty())
                .collect()
        })
        .unwrap_or_default();
    let decision = map
        .get("decision")
        .cloned()
        .filter(|s| !s.is_empty() && s != "null");
    let lineage_prev = map
        .get("lineage_prev")
        .cloned()
        .filter(|s| !s.is_empty() && s != "null");

    Ok(GovernanceRecord {
        id,
        kind,
        created: get("created")?,
        updated: get("updated")?,
        correlation_id: get("correlation_id")?,
        status: Status::parse(&get("status")?)?,
        owner: get("owner")?,
        parties,
        thesis: get("thesis")?,
        evidence,
        decision,
        source_class: SourceClass::parse(&get("source_class")?)?,
        lineage_prev,
        body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fsm_blocks_closed_to_open() {
        let err = assert_transition(Status::Closed, Status::Open).unwrap_err();
        match err {
            GateError::IllegalTransition { from, to } => {
                assert_eq!(from, Status::Closed);
                assert_eq!(to, Status::Open);
            }
            _ => panic!("wrong error"),
        }
    }

    #[test]
    fn s11_blocks_clinical() {
        let err = scan_s11("this is a diagnostic note").unwrap_err();
        match err {
            GateError::S11(t) => assert_eq!(t, "diagnostic"),
            _ => panic!("wrong error"),
        }
    }

    #[test]
    fn s11_allows_candidate_disclaimer() {
        scan_s11(
            "Evidence status: design contract — not a clinical, predictive, or performance claim",
        )
        .unwrap();
    }
}
