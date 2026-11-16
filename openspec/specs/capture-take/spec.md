# capture-take Specification

## Purpose
What a host does with an adapter's lines, the one way the statement fixes it for every host: which offers it refuses or holds, how it banks raw material and writes each capture, how an own excerpt becomes the capture's one signal, cut to the limit, and what it reports of the run (statement 13 to 24, 75 to 77, C.1).

## Requirements

### Requirement: A refused offer is named and stops no other

Taking lines SHALL refuse a malformed line and an offer that fails the contract (13), name each in the report with its line number, its key where it has one, and its reason, and go on to take every other line (15; core.5).

#### Scenario: One bad key among fourteen (S3)
- **WHEN** a run's lines offer fourteen captures and the fifth has the key `moves/x`
- **THEN** that offer is refused and named with its reason, and the other thirteen are taken

#### Scenario: A line that is not JSON is named and passed
- **WHEN** the second of three lines is `not json`
- **THEN** the report names line 2 as malformed, and lines 1 and 3 are taken

### Requirement: A held key writes nothing and banks nothing

An offer whose key the inbox holds, or whose key an earlier line of the same lines was taken under, SHALL be counted held, and nothing SHALL be banked or written for it, whatever it carries (14, 16; core.10).

#### Scenario: The same release the next day (S2)
- **WHEN** a release adapter offers a capture whose key the inbox already holds, with raw material and an own excerpt
- **THEN** nothing is banked, nothing is written, and the report counts one held

#### Scenario: One key twice in one run
- **WHEN** one run's lines offer the key `deploy/a/1` twice
- **THEN** the first is taken and the second counted held

### Requirement: Raw material is banked whole before its capture is written

Each raw file an offer names SHALL be handed to the host's bank whole and byte for byte, and only once the bank has taken it SHALL the capture be written (17, 18; core.11). An offer whose raw file cannot be read or banked SHALL be refused, and nothing written for it.

#### Scenario: The bank is filled before the inbox
- **WHEN** an offer naming two raw files is taken
- **THEN** both files reach the bank with their exact bytes before the capture reaches the inbox

#### Scenario: A raw file that is not there
- **WHEN** an offer names `notes.md` and the raw place holds no such file
- **THEN** the offer is refused naming `notes.md`, and nothing is banked or written for it

#### Scenario: Intake with no raw place
- **WHEN** lines are taken with no raw place and an offer names raw material
- **THEN** that offer is refused, naming the raw material it names

### Requirement: A pointer names the machine that banked

A capture SHALL hold a pointer `<machine>:<path>`, the machine as the host names it and the path where its bank keeps the capture's material, only when something was banked for it, and no pointer otherwise (17; record 0002; core.11).

#### Scenario: A banked capture points at its machine
- **WHEN** the host `deneb` takes an offer with raw material and its bank keeps it at `/data/inbox/release/mise/2026.10.10`
- **THEN** the capture's pointer is `deneb:/data/inbox/release/mise/2026.10.10`

#### Scenario: Nothing banked, no pointer
- **WHEN** an offer with no raw material and no excerpt is taken
- **THEN** the capture holds no pointer

### Requirement: The capture record is weir-capture/1 in recutils

A taken capture SHALL be written as one recutils record naming `format: weir-capture/1`, with its key, source, event time, title and link as offered, its pointer, how it came in (`run:<adapter>`, `intake` or `import:<body>`), the machine, the taken-at time, whether it is unread, and its stated facts as `fact` fields, never a joined fact; a field with no value is left out, and the record SHALL pass `recfix` (16, 22, 40, 121; C.2; core.22, core.24, core.55).

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

### Requirement: The cut keeps whole lines and says what it left out

An own excerpt's text within the limit SHALL be kept whole with no line added. A longer text SHALL be kept from its start through the last whole line that fits together with the closing line `[cut: <n> lines and <b> bytes left out, banked whole with the capture]`, C.1's one spelling, or as the closing line alone when no line fits. A limit too short to hold the closing line alone SHALL be refused (20, 24; C.1; core.12).

#### Scenario: A text within the limit
- **WHEN** a 300-byte text is cut at a limit of 8192 bytes
- **THEN** the excerpt is the text, unchanged, and nothing is recorded as cut

#### Scenario: A forty-kibibyte text (S1)
- **WHEN** a 40 KiB text of short lines is cut at a limit of 8192 bytes
- **THEN** the excerpt is a prefix of the text ending at a line's end, followed by the closing line naming the lines and bytes left out, and it is at most 8192 bytes

#### Scenario: One line and one byte left out
- **WHEN** a cut leaves out one line of one byte
- **THEN** the closing line is `[cut: 1 lines and 1 bytes left out, banked whole with the capture]`

#### Scenario: A limit too short for the closing line
- **WHEN** a host sets a limit shorter than the closing line with both numbers at their widest
- **THEN** taking is refused before any line is read, naming the limit and the smallest it may be

#### Scenario: A first line longer than the limit
- **WHEN** a text whose first line is 10000 bytes is cut at a limit of 8192 bytes
- **THEN** the excerpt is the closing line alone

#### Scenario: The cut is recorded on the signal
- **WHEN** a text is cut
- **THEN** the own signal records the lines and bytes left out, the same numbers the closing line says

### Requirement: A cut text is banked whole

Whenever an own excerpt is cut, its whole text SHALL be handed to the bank with the capture's raw material before the capture is written, so the capture has a pointer (21; core.12).

#### Scenario: Notes and excerpt both banked (S1)
- **WHEN** a release offer with its notes as raw material and a 40 KiB own excerpt is taken at a limit of 8192 bytes
- **THEN** the bank holds the notes and the whole excerpt text, the inbox holds one capture with a pointer and one cut signal, and the run is reported complete

### Requirement: The run is reported whole

Taking a run's lines SHALL report what was taken, the held count, each refusal with its reason, each failure the completion named, the cursor kept, and whether the run was complete: complete only when a completion was read and the adapter exited reporting success, and the cursor kept only then, the last one read (9, 23; core.15). Intake lines SHALL be reported the same way with no cursor.

#### Scenario: An adapter that exits without its completion (S4)
- **WHEN** a run offers five captures and ends with no completion
- **THEN** the five are taken, the report says incomplete, and no cursor is kept

#### Scenario: The next run offers the five again (S4)
- **WHEN** the next run, given the old cursor, offers the same five and its completion, and exits reporting success
- **THEN** the five are counted held, nothing is written, and the run's last cursor is kept

#### Scenario: A completion with failures and success
- **WHEN** a run's completion lists one failure and the adapter exits reporting success
- **THEN** the run is complete, its cursor kept, and the failure named in the report

#### Scenario: A completion then a failing exit
- **WHEN** a run's lines end with a completion but the adapter exits reporting failure
- **THEN** the run is incomplete and no cursor is kept

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

### Requirement: A capture the inbox refuses leaves nothing banked

When the inbox refuses a capture's write, the offer SHALL be reported refused with the inbox's reason and the bank told to discard what it banked for that capture, so nothing a taken capture does not name outlives the run (15, 18).

#### Scenario: The inbox refuses a write
- **WHEN** the inbox refuses the write of a capture whose raw material was banked
- **THEN** the offer is reported refused with the inbox's reason, and the bank no longer holds that material

### Requirement: The report is written and read as weir-report/1

A report of a run, an intake or an import SHALL be written as one JSON object on one line naming `"format": "weir-report/1"` (23, 76; C.7; core.22). The same object SHALL read back into the report it was written from, and an object naming another format, or lacking a field, SHALL be refused naming why.

#### Scenario: A run's report written and read back
- **WHEN** a run's report holds two keys taken, one held, a refusal of line 3 for key `moves/x`, a listing of fourteen with one failure, the cursor `c2` kept, and complete
- **THEN** it is written as one line naming `weir-report/1`, and reading that line gives the same report

#### Scenario: Another format
- **WHEN** the object `{"format": "weir-report/2", ...}` is read
- **THEN** it is refused, naming the format

### Requirement: A report holds what came of its run, intake or import

A `weir-report/1` object SHALL hold the keys taken in the order of their lines, the count held, each refusal with its line, its key where it had one and its reason, the listing's count where a completion was read, each failure the completion named with its `what` and `why`, the cursor kept where one was, and whether the run was complete (23, 76; C.7).

#### Scenario: A run's report written and read back
- **WHEN** a run's report holds two keys taken, one held, a refusal of line 3 for key `moves/x`, a listing of fourteen with one failure, the cursor `c2` kept, and complete
- **THEN** it is written as one line naming `weir-report/1`, and reading that line gives the same report

### Requirement: Take given a policy reads it in the capture's write

A take given a policy, a people list and the machine's time zone SHALL read the policy for an own excerpt's signal over its capture's stated and joined facts, and for a capture with no own excerpt as it lands, at the taken-at time, and SHALL hand the inbox the first matching rule's move with the capture: a join, with the rule's priority and the item name for the taken-at time, or a drop with its reason (125, 126, 128; core.57).

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
