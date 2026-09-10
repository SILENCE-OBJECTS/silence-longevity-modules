# External report intake

Use the GitHub issue template `external-report.yml`.

Do not paste raw telemetry, identifiers, or health information. Describe the **boundary** that failed (contract name, FSM edge, hash mismatch) on synthetic data.

Intake lands in `records/00_inbox` as `IN-YYYY-NNNN`. Promotion to `01_wnioski` is a human OWNER action, then `cargo run -p governance-cli -- intake-check records/00_inbox`.
