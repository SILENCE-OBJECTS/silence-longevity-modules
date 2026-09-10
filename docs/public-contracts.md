# Public contracts

```
Status: PUBLIC CANDIDATE
Scope: open-core documentation and synthetic fixtures only
Not included: scoring, individual profiling, raw telemetry, enterprise logic
Evidence status: design contract — not a clinical, predictive, or performance claim
```

Open-core contracts without a scoring engine. Public DTO names describe a **boundary**, not a person.

## Candidate names (schema ideas, not a released API)

| Name | Role |
| --- | --- |
| `ActorContext` | role + tenant slot. No PII fields. |
| `StructureVector` | schema, horizon, optional residual band. |
| `GovernanceRecord` | id, correlation_id, status, source class, thesis, evidence refs. |
| `IncomingPayload` | `Structure` or `RawStream` — the latter is unrepresentable as success. |

A typed event is append-only. Idempotency is an engineering property of the writer, not a public correlation UI.

Do not import enterprise modules. Do not place a user model on this side of the boundary.
