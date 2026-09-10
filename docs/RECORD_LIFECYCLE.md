# Record lifecycle

Every record carries:

- unique id (`WN-2026-0001`, `KOR-2026-0001`, `DEC-2026-0001`, `EV-2026-0001`, `IN-2026-0001`)
- created / updated
- correlation_id
- status: DRAFT, OPEN, WAITING, DECIDED, CLOSED, ARCHIVED
- owner and parties
- thesis, evidence refs, optional decision ref
- source class `[H] [T] [E] [M]`
- optional lineage_prev (EffectLog)

Inbox is unprocessed. Submissions are formal. Correspondence is threaded by correlation_id. Decisions are OWNER-only. Archive is terminal.
