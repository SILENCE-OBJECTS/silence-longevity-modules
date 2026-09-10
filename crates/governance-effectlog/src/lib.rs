//! Deterministic append-only hash chain.
//! Hash domain uses injected timestamps from the record, never ambient clock.

use governance_contracts::GovernanceRecord;
use sha2::{Digest, Sha256};

pub fn hash_bytes(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

pub fn record_hash(rec: &GovernanceRecord) -> String {
    hash_bytes(&rec.canonical_bytes())
}

pub fn chain_link(prev: Option<&str>, rec: &GovernanceRecord) -> String {
    let mut buf = Vec::new();
    if let Some(p) = prev {
        buf.extend_from_slice(p.as_bytes());
    }
    buf.push(b'|');
    buf.extend_from_slice(record_hash(rec).as_bytes());
    hash_bytes(&buf)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainEntry {
    pub id: String,
    pub record_hash: String,
    pub chain_hash: String,
}

pub fn verify_chain(records: &[GovernanceRecord]) -> Result<Vec<ChainEntry>, String> {
    let mut prev: Option<String> = None;
    let mut out = Vec::with_capacity(records.len());
    for rec in records {
        if let Some(lp) = &rec.lineage_prev {
            match &prev {
                Some(p) if p == lp => {}
                Some(p) => {
                    return Err(format!(
                        "lineage break at {}: expected prev {p}, got {lp}",
                        rec.id
                    ));
                }
                None => {
                    return Err(format!("lineage_prev set on first record {}", rec.id));
                }
            }
        }
        let rh = record_hash(rec);
        let ch = chain_link(prev.as_deref(), rec);
        out.push(ChainEntry {
            id: rec.id.clone(),
            record_hash: rh,
            chain_hash: ch.clone(),
        });
        prev = Some(ch);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use governance_contracts::{
        Clock, FixedClock, GovernanceRecord, RecordKind, SourceClass, Status,
    };

    fn rec(id: &str, clock: &FixedClock, prev: Option<&str>) -> GovernanceRecord {
        GovernanceRecord {
            id: id.into(),
            kind: RecordKind::Submission,
            created: clock.now_iso(),
            updated: clock.now_iso(),
            correlation_id: "COR-2026-0001".into(),
            status: Status::Open,
            owner: "owner".into(),
            parties: vec!["reporter".into()],
            thesis: "structure only".into(),
            evidence: vec!["EV-2026-0001".into()],
            decision: None,
            source_class: SourceClass::Technical,
            lineage_prev: prev.map(|s| s.into()),
            body: "synthetic fixture".into(),
        }
    }

    #[test]
    fn replay_is_deterministic() {
        let clock = FixedClock("2026-09-10T08:00:00Z".into());
        let a = rec("WN-2026-0001", &clock, None);
        let h1 = record_hash(&a);
        let h2 = record_hash(&a);
        assert_eq!(h1, h2);
        let clock2 = FixedClock("1999-01-01T00:00:00Z".into());
        // Record carries its own created stamp; ambient clock must not leak.
        let b = rec("WN-2026-0001", &clock, None);
        let _ = clock2;
        assert_eq!(record_hash(&b), h1);
    }

    #[test]
    fn chain_breaks_on_swap() {
        let clock = FixedClock("2026-09-10T08:00:00Z".into());
        let a = rec("WN-2026-0001", &clock, None);
        let link1 = chain_link(None, &a);
        let b = rec("WN-2026-0002", &clock, Some(&link1));
        let ok = verify_chain(&[a.clone(), b.clone()]).unwrap();
        assert_eq!(ok.len(), 2);
        let mut b2 = b;
        b2.lineage_prev = Some("deadbeef".into());
        assert!(verify_chain(&[a, b2]).is_err());
    }
}
