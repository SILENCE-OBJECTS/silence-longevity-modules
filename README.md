# silence-longevity-modules

Public canonical surface for humans, maintainers and machine retrieval. Canonical SSOT lives in versioned repository files.

Open-core publication funnel for PatternLens longevity **modules**. Typed contracts. No user model. No raw streams. Injected clock. No scoring engine.

> A telemetry interface that refuses raw streams makes privacy testable before data reaches a server.

```
Status: PUBLIC CANDIDATE
Scope: open-core documentation and synthetic fixtures only
Not included: scoring, individual profiling, raw telemetry, enterprise logic
Evidence status: design contract — not a clinical, predictive, or performance claim
```

## Proof of boundary (run it)

```bash
cargo test --workspace --locked
cargo run -p governance-cli -- validate records/
cargo run -p governance-cli -- intake-check records/00_inbox/
cargo run -p governance-cli -- publication-check publications/
cargo run -p governance-cli -- verify-chain records/
```

Illegal FSM transitions, S11 clinical vocabulary, RESTRICTED/PII, and missing OWNER decisions fail the gate. That failure is the product.

## What this is / is not

| Is | Is not |
| --- | --- |
| Candidate public contracts | Released production API |
| Structure-only envelopes | Raw telemetry warehouse |
| Deterministic replay with injected time | Ambient `SystemTime` in the hash |
| MIT open-core validator | Scoring / fusion / enterprise logic |
| Taxonomy + FSM language | A rank, a health score, a diagnosis |

Proper nouns in this tree: PatternLens, PatternsLab, Portal. Scoring stays closed.

## Tree

- `crates/` — Rust workspace: contracts, validator, EffectLog, CLI
- `records/00_inbox` → `01_wnioski` → `02_korespondencja` → `03_decyzje` → `04_evidence` → `99_archive`
- `publications/` — drafts / review / published; blocked until `publication-check`
- `docs/` — architecture, publication policy, public contracts, no-raw-streams, injected-clock
- `llms.txt` — machine map (20–40 lines)

License: MIT. Enterprise and RESTRICTED records live behind a closed trust boundary (`silence-enterprise`). Dependency may point enterprise → open-core, never the reverse.
