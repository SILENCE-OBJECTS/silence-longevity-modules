# Injected clock

```
Status: PUBLIC CANDIDATE
Scope: open-core documentation and synthetic fixtures only
Not included: scoring, individual profiling, raw telemetry, enterprise logic
Evidence status: design contract — not a clinical, predictive, or performance claim
```

Deterministic primitives need an injected clock. Hash and replay belong to the record's stamped fields, not to the wall clock of the machine that runs the test.

`Clock` is a trait. `FixedClock` is the fixture. `GovernanceRecord::canonical_bytes` concatenates fields in a fixed order. `EffectLog` hashes those bytes with SHA-256. Ambient `SystemTime` is not in the domain.

Replay: two runs with the same injected stamp produce the same record hash. That is a layer property, not a latency claim, not a CVE write-up, not a benchmark.

Longevity language in this repo is taxonomy + FSM (presence, interruption, recovery) — a structure of observation — not a score.

MEASURED: `cargo test -p governance-effectlog replay_is_deterministic`.
