use governance_contracts::{Clock, FixedClock, GovernanceRecord, RecordKind, SourceClass, Status};
use governance_effectlog::{record_hash, verify_chain};

fn rec(id: &str, stamp: &str) -> GovernanceRecord {
    let clock = FixedClock(stamp.into());
    GovernanceRecord {
        id: id.into(),
        kind: RecordKind::Evidence,
        created: clock.now_iso(),
        updated: clock.now_iso(),
        correlation_id: "COR-2026-0001".into(),
        status: Status::Open,
        owner: "owner".into(),
        parties: vec!["cli".into()],
        thesis: "synthetic evidence".into(),
        evidence: vec![],
        decision: None,
        source_class: SourceClass::Machine,
        lineage_prev: None,
        body: "fixture".into(),
    }
}

#[test]
fn same_fixture_same_hash_across_wall_clocks() {
    let a = rec("EV-2026-0001", "2026-09-10T08:00:00Z");
    let b = rec("EV-2026-0001", "2026-09-10T08:00:00Z");
    assert_eq!(record_hash(&a), record_hash(&b));
    verify_chain(&[a]).unwrap();
}
