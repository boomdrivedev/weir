# Proposal

## Why

The statement gives an inbox a policy: its keeper's rules, in order,
each a match on a signal's facts with a kind to launch, a person's turn
or a drop, a priority and a window. A rule moves what it recognises as
it lands, and curation takes what no rule names. The statement at
blueprints `63a14c5` and `104f847`, from the update-ladder talk of
2026-11-11 and 2026-11-12, writes it into weir:

- A.16, 117 to 121: a fact is a name with one value or several. A
  signal carries facts of three sorts. Stated facts come from its
  capture's offer. Joined facts a host joins from its people list,
  attendees by address and speakers by name. Read facts its reader
  gives, each from a closed list. A host writes no joined fact.
- A.17, 122 to 129: a policy is checked whole. A rule reads facts
  alone (123), and no fact sets its outcome (124). Reading a policy
  gives the first rule that matches, and a capture as it lands passes
  over every rule on read facts (125). A host reads the policy in the
  capture's write (126). A capture a rule matches as it lands is its
  own signal, its excerpt its title (127). A rule's move is a join into
  the rule's current item or a drop. It is by its rule, in no batch,
  and its item is named for the rule and its window's close (128, 129).
- 2, 13, 16, 19, 25, 26, 31, 36, 70, 84, C.1, C.2, C.5, C.9 and I10
  follow: an offer carries its source's facts, a capture records them,
  and the same policy, people list, facts and time give the same rule
  under every host.

millpond's core.1, at boomdrivedev/millpond `75b0cea`, has millpond
write a rule's own signal and its move as weir's `take` hands them, so
every host takes a line the same way (millpond 127, 128). weir has no
facts, no people list and no policy yet, and `take` hands an inbox a
capture and its own excerpt's signal alone.

Record 0011, proposed on this branch, is this change's own choice of
where the code lies, where a rule's item is found, and what C.9 leaves
open. core.56 to core.58, core.60 and the amended core.4, core.6, core.13,
core.14, core.16, core.20 and core.33 say it as rules, on this branch.

## What Changes

- A new crate, `weir-policy`, with its moon project `policy`, depending
  on `weir-cadence`. It holds a fact's names, values and sorts, in
  flight as JSON and at rest as `fact` fields. It reads the people list
  and joins attendees and speakers from it. It reads a policy and
  checks it whole, matches a rule's `when` over facts and a capture's
  age, and reads a policy for a signal and for a capture as it lands.
  It names a rule's item for its window's close.
- `weir-cadence`'s `Schedule::first_after` is public, so a rule's item
  is named for the first time its window names after its making.
- `weir-capture` depends on `weir-policy`. An offer's `facts` are read
  and checked at step 1, and a capture writes its stated facts as `fact`
  fields. `Take::with_policy` hands `take` a policy, a people list and
  the machine's time zone. `take` then reads the policy for an own
  excerpt's signal, or for a capture with no own excerpt as it lands. A
  capture a rule matches as it lands gets its own signal, of its title
  or its key, with no nature. `Taken` carries the rule's move: its rule,
  and a join with its priority and the name an item it makes takes, or
  a drop with its reason. **BREAKING** for a builder of `Capture`,
  `Taken` or `OwnSignal`: `Capture` gains `facts`, `Taken` gains
  `rule_move`, and `OwnSignal`'s `kind` is an option.
- `weir-signals` depends on `weir-policy`. A signal's nature is an
  option, and a tip's signal with none is refused unless it is an own
  signal a rule made. An item records its rule, its priority and when
  its window closed. `InboxAt::take` writes the capture, its own signal,
  the rule's move and the item the move makes in one write. A join goes
  into the rule's current item at the tip, or makes the item under the
  name handed. **BREAKING** for a builder of `Signal` or `Item`.
- The workspace, the moon projects whose crates depend on
  `weir-capture`, and the README name the new crate.

## Capabilities

### New Capabilities

- `facts`: a fact's names, values and sorts, in flight and at rest; the
  people list, read and checked; and the join of attendees and speakers
  from it.
- `policy`: a policy read and checked whole; a rule's match over facts
  and a capture's age; the policy read for a signal and for a capture as
  it lands; and a rule's item's name.

### Modified Capabilities

- `adapter-contract`: an offer may carry the facts its source states.
- `capture-take`: a capture records its stated facts; `take`, given a
  policy, reads it in the capture's write; a capture a rule matches as it
  lands is its own signal; two hosts given one policy write the same
  records.
- `inbox-signals`: an own signal a rule made has no nature and is its
  capture's title; a rule's move is written in its capture's write, by
  its rule, in no batch.
- `inbox-items`: a rule's item names its rule and its priority, and a
  rule's join goes into its current item.
- `cadence`: a schedule gives the first time it names after an instant.

## Impact

- New: `crates/weir-policy/` with its `moon.yml`.
- `crates/weir-cadence/src/schedule.rs`: `first_after` public.
- `crates/weir-capture`: `line.rs`, `capture.rs`, `take.rs` and
  `lib.rs`, and a dependency on `weir-policy`.
- `crates/weir-signals`: `signal.rs`, `item.rs`, `inbox.rs` and
  `write.rs`, and a dependency on `weir-policy`.
- `crates/weir-adapters` and `crates/weir`: their builders of `Taken`
  and `Capture` give the new fields. `weir conform`'s seam gives a host
  no policy (C.7), so the pack-host example takes as before.
- `Cargo.toml`, `.moon/workspace.yml`, the moon projects `capture`,
  `signals`, `adapters`, `pack`, `weir`, `delivery` and `kinds`, which
  build on `weir-capture`, and the README's table of crates.
- Not in this change, each a change of its own: a reader's read facts
  checked against their lists, and a rule read in a reading's write
  (120, 126); a rule's item closed at its window with its brief (36,
  129); a meeting adapter's stated facts held by the meeting checks
  (66); a link (114 to 116). millpond moves onto `take`'s policy in a
  change of its own.
