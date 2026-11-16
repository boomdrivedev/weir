## MODIFIED Requirements

### Requirement: An offer carries a key, a source and an event time

An offer SHALL be read only when it holds `key`, `source` and `event_at`, the event time in RFC 3339 with its offset; `title`, `url`, `raw`, `excerpt` and `facts` are optional, `facts` an object of the facts its source states, each spelled as C.9 spells it, and a field the contract does not name SHALL be ignored (2, 5, 13; C.1, C.9; core.6). The event time SHALL be kept as offered.

#### Scenario: A missing required field refuses the offer
- **WHEN** an offer holds a key and a source but no `event_at`
- **THEN** it is refused, naming `event_at` as missing

#### Scenario: An event time without its offset refuses the offer
- **WHEN** an offer's `event_at` is `2026-10-10T14:02:11`
- **THEN** it is refused, naming `event_at` as not an RFC 3339 time with an offset

#### Scenario: The event time is kept as offered
- **WHEN** an offer's `event_at` is `2026-10-10T16:02:11+02:00`
- **THEN** the offer read holds `2026-10-10T16:02:11+02:00`, unchanged

#### Scenario: An offer's facts are read
- **WHEN** an offer's `facts` is `{"level": "patch", "attendee": ["dana@acme.com"]}`
- **THEN** the offer read holds the stated facts `attendee` and `level` with those values

#### Scenario: An offer stating a fact C.9 does not spell
- **WHEN** an offer's `facts` is `{"says": "request"}`
- **THEN** it is refused, naming `facts.says` and the offer's key
