use governance_validator::{parse_record, validate_record};
use std::fs;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn valid_public_report_intakes() {
    let p = root().join("fixtures/valid-public-report.md");
    let text = fs::read_to_string(&p).expect("fixture");
    let rec = parse_record(&text).expect("parse");
    validate_record(&rec, None).expect("validate");
}

#[test]
fn missing_permission_fails_publication() {
    let p = root().join("fixtures/missing-permission-report.md");
    let text = fs::read_to_string(&p).expect("fixture");
    let rec = parse_record(&text).expect("parse");
    let err = governance_validator::publication_check(&rec, &rec.body).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("evidence")
            || msg.contains("OWNER")
            || msg.contains("license")
            || msg.contains("decision"),
        "{msg}"
    );
}

#[test]
fn restricted_report_is_rejected() {
    let p = root().join("fixtures/restricted-report.md");
    let text = fs::read_to_string(&p).expect("fixture");
    let rec = parse_record(&text).expect("parse");
    let err = validate_record(&rec, None).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("restricted")
            || msg.contains("RESTRICTED")
            || msg.contains("S11")
            || msg.contains("PII")
            || msg.contains("banned"),
        "{msg}"
    );
}
