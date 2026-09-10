use governance_contracts::Status;
use governance_validator::assert_transition;

#[test]
fn allowed_path_draft_to_archive() {
    assert_transition(Status::Draft, Status::Open).unwrap();
    assert_transition(Status::Open, Status::Waiting).unwrap();
    assert_transition(Status::Waiting, Status::Decided).unwrap();
    assert_transition(Status::Decided, Status::Closed).unwrap();
    assert_transition(Status::Closed, Status::Archived).unwrap();
}

#[test]
fn archived_is_terminal() {
    assert!(assert_transition(Status::Archived, Status::Open).is_err());
    assert!(assert_transition(Status::Archived, Status::Draft).is_err());
}
