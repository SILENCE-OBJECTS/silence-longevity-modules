use governance_contracts::{GovernanceRecord, RecordKind, SourceClass, Status};
use governance_validator::publication_check;

fn rec() -> GovernanceRecord {
    GovernanceRecord {
        id: "WN-2026-0001".into(),
        kind: RecordKind::Submission,
        created: "2026-09-10".into(),
        updated: "2026-09-10".into(),
        correlation_id: "COR-2026-0001".into(),
        status: Status::Decided,
        owner: "OWNER".into(),
        parties: vec!["reporter".into()],
        thesis: "Publish the no-raw-stream contract as PUBLIC CANDIDATE.".into(),
        evidence: vec!["EV-2026-0001".into()],
        decision: Some("DEC-2026-0001".into()),
        source_class: SourceClass::Technical,
        lineage_prev: None,
        body: "SPDX-License-Identifier: MIT\nlegal basis: public documentation of an interface boundary.\nStatus: PUBLIC CANDIDATE.".into(),
    }
}

#[test]
fn qualified_candidate_passes() {
    let r = rec();
    publication_check(&r, &r.body).unwrap();
}

#[test]
fn production_api_claim_fails() {
    let mut r = rec();
    r.body.push_str("\nThis is the production API.\n");
    assert!(publication_check(&r, &r.body).is_err());
}
