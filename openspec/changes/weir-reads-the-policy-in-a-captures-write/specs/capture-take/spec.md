## MODIFIED Requirements

### Requirement: The capture record is weir-capture/1 in recutils

A taken capture SHALL be written as one recutils record naming `format: weir-capture/1`, with its key, source, event time, title and link as offered, its pointer, how it came in (`run:<adapter>`, `intake` or `import:<body>`), the machine, the taken-at time, whether it is unread, and its stated facts as `fact` fields, never a joined fact; a field with no value is left out, and the record SHALL pass `recfix` (16, 22, 40, 121; C.2; core.22, core.24, core.57).

#### Scenario: A capture from a run
- **WHEN** an offer from a run of the adapter `releases` is taken on `deneb`
- **THEN** its record names `format: weir-capture/1`, `came_in: run:releases` and `machine: deneb`, and passes `recfix`

#### Scenario: A multi-line value stays one field
- **WHEN** an offer's title holds a newline
- **THEN** the record holds it as one field across lines, it passes `recfix`, and it reads back as the title offered

#### Scenario: A capture's stated facts are written with it
- **WHEN** a meeting's offer states `length` as `900` and `attendee` as `["dana@acme.com"]`, and the take is given C.9's people list and policy
- **THEN** the record holds `fact: attendee "dana@acme.com"` and `fact: length 900` after `unread`, holds no `attendee.` fact, passes `recfix`, and reads back with those facts

### Requirement: An own excerpt becomes the capture's one signal

A capture whose offer carried an own excerpt, or that a rule matched as it landed, SHALL be handed to the inbox together with exactly one signal, `<key>#1`, in the same write, and SHALL NOT be unread; every other capture SHALL be written unread with no signal (19; core.13). An own excerpt's signal's fields but its excerpt text SHALL be as offered.

#### Scenario: A release's notes as its own excerpt
- **WHEN** an offer with an own excerpt of nature `ask` is taken
- **THEN** the inbox receives in one write the capture, not unread, and the signal `<key>#1` of nature `ask`

#### Scenario: A meeting with no excerpt
- **WHEN** an offer with a transcript as raw material and no excerpt is taken
- **THEN** the inbox receives the capture, unread, and no signal

### Requirement: Two hosts given the same lines write the same records

Given the same lines, raw material and limit, and the same policy, people list, taken-at time and time zone or none, taking SHALL write byte-identical capture records and identical own signals and rule's moves, apart from the taken-at time, the machine and how the capture came in; nothing else a host holds SHALL reach a record (24; I8, I10; core.14).

#### Scenario: Two hosts, one set of lines
- **WHEN** two separate hosts, with the same machine name, taken-at time and bank, take the same lines
- **THEN** every capture record they write is the same bytes, and every own signal the same

#### Scenario: Only the machine differs
- **WHEN** two hosts that differ only in machine name take the same lines
- **THEN** their records differ only in the `machine` field and the machine part of the pointer

#### Scenario: Two hosts, one policy
- **WHEN** two separate hosts, given C.9's policy and people list, one taken-at time and one time zone, take the same lines of a stand-up and two patch releases
- **THEN** every capture they hand their inboxes is the same, with the same own signal and the same rule's move

## ADDED Requirements

### Requirement: Take given a policy reads it in the capture's write

A take given a policy, a people list and the machine's time zone SHALL read the policy for an own excerpt's signal over its capture's stated and joined facts, and for a capture with no own excerpt as it lands, at the taken-at time, and SHALL hand the inbox the first matching rule's move with the capture: a join, with the rule's priority and the item name for the taken-at time, or a drop with its reason (125, 126, 128; core.60).

#### Scenario: A patch release joins its rule's item
- **WHEN** a take given C.9's policy at 2026-11-12T10:05:00Z, in UTC, takes a release's offer with an own excerpt stating `level` as `"patch"`
- **THEN** the inbox is handed the capture, its own signal and `point-releases`' move, a join at low priority naming `point-releases-20261112T103000Z`

#### Scenario: A signal no rule matches stays unmoved
- **WHEN** a take given C.9's policy takes a release's offer with an own excerpt stating `level` as `"major"`
- **THEN** the inbox is handed the capture and its own signal, and no rule's move

#### Scenario: A take given no policy
- **WHEN** a take given no policy takes a meeting's offer titled `Daily stand-up` with no excerpt
- **THEN** the inbox is handed the capture, unread, with no signal and no rule's move

### Requirement: A capture a rule matches as it lands is its own signal

A capture with no own excerpt that a rule matches as it lands SHALL be handed with its own signal, `<key>#1`: its excerpt and assertion its title as offered, or its key where the offer gave none, uncut; asserted by its source; at `title`; with no nature, subjects or argument. It SHALL NOT be unread. A capture no rule matches as it lands SHALL be unread (19, 127; core.13, core.16).

#### Scenario: A stand-up dropped as it lands
- **WHEN** a take given C.9's policy takes a meeting's offer titled `Daily stand-up`, with no excerpt, stating its length and three attendees
- **THEN** the inbox is handed the capture, not unread, the own signal whose excerpt and assertion are `Daily stand-up`, asserted by `meeting`, with no nature, and `stand-ups`' drop with the reason `a stand-up` (S27)

#### Scenario: A capture with no title is its own signal by its key
- **WHEN** a policy's rule drops every capture of source `deploy`, and a deploy's offer with no title and no excerpt is taken under the key `deploy/web/42`
- **THEN** its own signal's excerpt and assertion are `deploy/web/42`

#### Scenario: A rule on a read fact charges a reader
- **WHEN** a take given a policy whose one rule matches `says` is `request` takes a meeting's offer with no excerpt
- **THEN** the inbox is handed the capture, unread, with no signal and no rule's move
