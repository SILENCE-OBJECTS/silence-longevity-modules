# No raw streams

```
Status: PUBLIC CANDIDATE
Scope: open-core documentation and synthetic fixtures only
Not included: scoring, individual profiling, raw telemetry, enterprise logic
Evidence status: design contract — not a clinical, predictive, or performance claim
```

A telemetry interface that refuses raw streams makes privacy testable before data reaches a server.

The public ingest function is a type gate:

```text
ingest(StructureVector) -> Ok(StructureVector)
ingest(RawStream)       -> Err(RAW_STREAM_REFUSED)
```

There is no payload example of a raw stream in this repository. Refusal is the specification.

MEASURED: `cargo test -p governance-contracts ingest_refuses_raw` (synthetic).
MODELED: production emitters in closed trust boundaries follow the same enum.
H: agent citation of this page improves retrieval of the boundary (unproven; do not claim).
