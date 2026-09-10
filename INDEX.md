# Index

| Path | Role |
| --- | --- |
| records/00_inbox | Unprocessed intake |
| records/01_wnioski | Formal submissions (`WN-YYYY-NNNN`) |
| records/02_korespondencja | Correspondence (`KOR-YYYY-NNNN`) |
| records/03_decyzje | OWNER decisions (`DEC-YYYY-NNNN`) |
| records/04_evidence | Evidence (`EV-YYYY-NNNN`) |
| records/05_templates | Record templates |
| records/99_archive | ARCHIVED |
| publications/ | Publication funnel (gated) |
| crates/ | Rust gates |
| docs/ | Public candidate documentation |
| fixtures/ | Synthetic pass/fail cases |
| llms.txt | Machine retrieval map |

correlation_id binds WN + KOR + EV + DEC.
Source class: `[H]` human, `[T]` technical, `[E]` external, `[M]` machine.
