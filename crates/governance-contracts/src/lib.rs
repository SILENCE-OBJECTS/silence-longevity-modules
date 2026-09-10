//! Public candidate contracts.
//!
//! Status: PUBLIC CANDIDATE
//! Scope: open-core documentation and synthetic fixtures only
//! Not included: scoring, individual profiling, raw telemetry, enterprise logic
//! Evidence status: design contract — not a clinical, predictive, or performance claim

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Record lifecycle. Illegal transitions are rejected by the validator FSM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Draft,
    Open,
    Waiting,
    Decided,
    Closed,
    Archived,
}

impl Status {
    pub fn parse(s: &str) -> Result<Self, ContractError> {
        match s.trim().to_ascii_uppercase().as_str() {
            "DRAFT" => Ok(Self::Draft),
            "OPEN" => Ok(Self::Open),
            "WAITING" => Ok(Self::Waiting),
            "DECIDED" => Ok(Self::Decided),
            "CLOSED" => Ok(Self::Closed),
            "ARCHIVED" => Ok(Self::Archived),
            other => Err(ContractError::UnknownStatus(other.to_string())),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "DRAFT",
            Self::Open => "OPEN",
            Self::Waiting => "WAITING",
            Self::Decided => "DECIDED",
            Self::Closed => "CLOSED",
            Self::Archived => "ARCHIVED",
        }
    }
}

/// Source classification: Human, Technical, External, Machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceClass {
    #[serde(rename = "H")]
    Human,
    #[serde(rename = "T")]
    Technical,
    #[serde(rename = "E")]
    External,
    #[serde(rename = "M")]
    Machine,
}

impl SourceClass {
    pub fn parse(s: &str) -> Result<Self, ContractError> {
        match s
            .trim()
            .trim_matches(['[', ']'])
            .to_ascii_uppercase()
            .as_str()
        {
            "H" => Ok(Self::Human),
            "T" => Ok(Self::Technical),
            "E" => Ok(Self::External),
            "M" => Ok(Self::Machine),
            other => Err(ContractError::UnknownSourceClass(other.to_string())),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Human => "H",
            Self::Technical => "T",
            Self::External => "E",
            Self::Machine => "M",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordKind {
    Inbox,
    Submission,
    Correspondence,
    Decision,
    Evidence,
}

impl RecordKind {
    pub fn prefix(self) -> &'static str {
        match self {
            Self::Inbox => "IN",
            Self::Submission => "WN",
            Self::Correspondence => "KOR",
            Self::Decision => "DEC",
            Self::Evidence => "EV",
        }
    }

    pub fn from_prefix(p: &str) -> Result<Self, ContractError> {
        match p {
            "IN" => Ok(Self::Inbox),
            "WN" => Ok(Self::Submission),
            "KOR" => Ok(Self::Correspondence),
            "DEC" => Ok(Self::Decision),
            "EV" => Ok(Self::Evidence),
            other => Err(ContractError::UnknownKind(other.to_string())),
        }
    }

    pub fn dir(self) -> &'static str {
        match self {
            Self::Inbox => "00_inbox",
            Self::Submission => "01_wnioski",
            Self::Correspondence => "02_korespondencja",
            Self::Decision => "03_decyzje",
            Self::Evidence => "04_evidence",
        }
    }
}

/// Tenant / actor context as a product boundary — not a user model.
/// No PII fields are permitted on this type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorContext {
    pub role: String,
    pub tenant_slot: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceRecord {
    pub id: String,
    pub kind: RecordKind,
    pub created: String,
    pub updated: String,
    pub correlation_id: String,
    pub status: Status,
    pub owner: String,
    pub parties: Vec<String>,
    pub thesis: String,
    pub evidence: Vec<String>,
    pub decision: Option<String>,
    pub source_class: SourceClass,
    pub lineage_prev: Option<String>,
    pub body: String,
}

impl GovernanceRecord {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        // Deterministic encoding: field order fixed, no ambient clock.
        let mut s = String::new();
        s.push_str(&self.id);
        s.push('\n');
        s.push_str(self.kind.prefix());
        s.push('\n');
        s.push_str(&self.created);
        s.push('\n');
        s.push_str(&self.updated);
        s.push('\n');
        s.push_str(&self.correlation_id);
        s.push('\n');
        s.push_str(self.status.as_str());
        s.push('\n');
        s.push_str(&self.owner);
        s.push('\n');
        s.push_str(&self.parties.join(","));
        s.push('\n');
        s.push_str(&self.thesis);
        s.push('\n');
        s.push_str(&self.evidence.join(","));
        s.push('\n');
        s.push_str(self.decision.as_deref().unwrap_or(""));
        s.push('\n');
        s.push_str(self.source_class.as_str());
        s.push('\n');
        s.push_str(self.lineage_prev.as_deref().unwrap_or(""));
        s.push('\n');
        s.push_str(&self.body);
        s.into_bytes()
    }
}

/// Structure-only telemetry envelope. Raw streams are a type error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructureVector {
    pub schema: String,
    pub horizon: String,
    pub residual_band: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncomingPayload {
    Structure(StructureVector),
    RawStream { bytes_hint: usize },
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum IngestError {
    #[error("RAW_STREAM_REFUSED: telemetry interface does not accept raw streams")]
    RawStreamRefused,
    #[error("USER_MODEL_REFUSED: public contracts carry no user model")]
    UserModelRefused,
}

/// A telemetry interface that refuses raw streams makes privacy testable
/// before data reaches a server.
pub fn ingest(payload: IncomingPayload) -> Result<StructureVector, IngestError> {
    match payload {
        IncomingPayload::Structure(v) => Ok(v),
        IncomingPayload::RawStream { .. } => Err(IngestError::RawStreamRefused),
    }
}

/// Clock injection. Ambient time is not part of the hash domain.
pub trait Clock {
    fn now_iso(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct FixedClock(pub String);

impl Clock for FixedClock {
    fn now_iso(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ContractError {
    #[error("unknown status: {0}")]
    UnknownStatus(String),
    #[error("unknown source class: {0}")]
    UnknownSourceClass(String),
    #[error("unknown record kind: {0}")]
    UnknownKind(String),
    #[error("parse error: {0}")]
    Parse(String),
}

impl fmt::Display for RecordKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.prefix())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingest_refuses_raw() {
        let err = ingest(IncomingPayload::RawStream { bytes_hint: 12 }).unwrap_err();
        assert_eq!(err, IngestError::RawStreamRefused);
    }

    #[test]
    fn ingest_accepts_structure() {
        let v = ingest(IncomingPayload::Structure(StructureVector {
            schema: "continuity.v0".into(),
            horizon: "30d".into(),
            residual_band: None,
        }))
        .unwrap();
        assert_eq!(v.horizon, "30d");
    }
}
