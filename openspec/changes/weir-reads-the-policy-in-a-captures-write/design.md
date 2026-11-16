# Design

## Context

`weir-capture`'s `take` reads a run's or an intake's lines in C.1's
seven steps. For each offer it hands the host's `Inbox` a `Taken`: the
capture, and the own signal of an own excerpt, cut to the limit. The
host's `Inbox::write` forms the write with `weir-signals`'
`InboxAt::take` on the tip it reads, through `weir-branch`, which
replays it on a new tip when the push is refused (core.26). An offer's
`facts` is ignored today as a field the contract does not name
(core.6). No crate holds a fact, a people list or a policy.

The statement, read at blueprints `30dce35`, gives an inbox a policy.
117 to 121 define the facts and the people list's join. 122 to 129 define
the policy, its two readings, a rule's own signal, its move and its
item. 2, 13, 16, 19, 25, 26, 31, 36, 70, 84, C.1, C.2, C.5, C.9 and I10
were amended to fit them at blueprints `63a14c5` and `104f847`.
millpond's 127 and 128 have millpond read the policy in the write that
brings a signal, and its core.1, at boomdrivedev/millpond `75b0cea`, has
`take` do it and millpond write what `take` hands it.

Record 0011, proposed on this branch, decides what the statement leaves
to weir. `weir-policy` holds the facts, the people list and the policy,
and `weir-capture` depends on it. A rule's current item is found at the
tip the write lands on. It also settles the details C.9 leaves open,
each cited below as record 0011.

Rules this design relies on, all in `docs/architecture/core.md`, the one
page whose area the change touches:

- core.1: `weir-policy` keeps no state and reads no clock: the time and
  the time zone are its caller's.
- core.2: no source or test names a tool of the family. The fixtures
  are C.9's people and rules.
- core.4 and core.60: no move is chosen but the first matching rule's.
- core.6: an offer's `facts` is read, each fact spelled as C.9 spells
  it, and a field the contract does not name is still ignored.
- core.8: a capture's age reads its event time.
- core.13: the capture, its own signal, the rule's move and the item it
  makes are one write.
- core.14: what reaches a record is the lines, the policy, the people
  list, the taken-at time, the machine and its time zone.
- core.16: an own signal a rule made is its capture's title or key.
- core.18 and core.19: a rule's move is a join or a drop, stands, and is
  in no batch.
- core.20 and core.21: an item holds no signals, and no record the tip
  holds is rewritten. Closing a rule's item is not in this change.
- core.22: `weir-capture/1`, `weir-signal/1` and `weir-item/1` keep their
  majors, as C.2 spells them.
- core.24 and core.26: every write passes the checks on the tip it lands
  on, and a replay forms the rule's move again there.
- core.33: a rule's window is a schedule `weir-cadence` reads.
- core.39 and core.40: `weir-policy` is a moon project of its own, with
  its four checks.
- core.56 to core.58: one reader of the facts, the people list and the
  policy, reading facts alone.

The design departs from no rule.

## Goals / Non-Goals

**Goals:**

- One crate reads a fact, a people list and a policy, and every host
  reads a policy through it.
- `take`, given a policy, reads it in the capture's write, and hands the
  inbox a rule's own signal and its move with the capture.
- `weir-signals` writes the rule's move, and the item it makes, in the
  capture's write, its current item found on the tip it lands on.
- A host given no policy takes every line as it does today.

**Non-Goals:**

- A reader's read facts checked against their lists (120), and a rule
  read in a reading's write (126): the reading's change.
- A rule's item closed when its window is due, with its brief (36, 129):
  the close's change, with how it adds `closed_at`.
- The meeting checks of a meeting adapter's stated facts (66).
- A link (114 to 116).
- A policy given to a host through `weir conform`, whose seam C.7 fixes
  with no policy.
- Facts on an import line's capture: an import reads no rule (126), and
  crew's bodies hold no facts.

## Decisions

### 1. The crate

`crates/weir-policy` is a library crate depending on `weir-cadence`,
`jiff`, `serde`, `serde_json` and `toml`, and on no other crate of
weir's. Its tests take jiff's `tzdb-bundle-always`, as `weir-cadence`'s
do. Its modules are `fact`, `people`, `policy` and `reading`, with
`#![deny(missing_docs)]`. `weir-capture` depends on it, and so does
`weir-signals` for `Priority` (record 0011). `weir-cadence`'s
`Schedule::first_after` becomes public, documented as it stands.

### 2. A fact's names, sorts and values

`fact::sort(name)` gives the sort C.9 gives a name, or none where C.9
spells none:

| sort | names |
|---|---|
| read | `says`, `speaker`, `about`, `owner`, `due` |
| joined | `attendee.` or `speaker.`, then `name`, `role`, `teams`, `org` or `client` |
| stated | lowercase letters, digits, `_` and `-`, but `source`, `title`, `event_at` and the read names |

`source`, `title` and `event_at` are stated under their own names from
the offer's own fields (117), never from `facts` (record 0011).

`Value` is `Text(String)`, `Number(serde_json::Number)`, `Bool(bool)` or
`Date(jiff::civil::Date)`. A JSON string that reads as an RFC 3339
full-date is a `Date`. Two numbers are equal when they are one number,
compared as `f64`. `Facts` maps each name to its values, in name order,
each fact's values in the order given, and is never a name with none.
`Facts::stated(&serde_json::Value)` reads an offer's `facts` object, and
`Facts::read` reads an object of read facts, with names of that sort
alone and values of 117's forms. Their lists are the reading's change,
so `Facts::read` checks no value against one. Each refuses with a
`FactError` naming the field as `facts.<name>`, or `facts.<name>[<n>]`
for one value of a list:

- a name of another sort, or none;
- a value that is `null`, an object or a list in a list;
- a list of no values.

`Facts::to_json` writes the object back, a fact of one value as the value
and one of several as a list, and `Facts` serializes as it writes, so an
offer printed as a line carries its facts (record 0011).

### 3. Facts at rest

`Facts::to_fields` gives one text per value, `<name> <value as JSON>`, in
the order of `Facts`. `Facts::from_fields` reads them back, a name's
values gathered in the order written, and refuses a text that is not a
stated name, one blank and a JSON value of 117's forms, naming `fact`
(C.9). A fact of one value and a list of one read back alike (record
0011).

### 4. The people list

`People::read(settings: &str)` reads `[about]`, `[[person]]` and
`[[org]]` from a settings document and ignores its other tables, which
are its host's. A person needs `name`. `addresses`, `names` and `teams`
default to empty and `role` to none. An org needs `name`. `domains`
defaults to empty and `client` to `false`. `[about]`'s `teams` and
`repositories` default to empty. A field of the wrong shape, or one the
profile does not name, is refused with a `SettingsError` naming it:
`person[0].adresses`, counting from 0 (record 0011).

### 5. The join

`People::join(&self, stated: &Facts, read: &Facts) -> Facts` gives the
joined facts, written in no record (121):

- Each value of the stated `attendee` that is an address, one `@` with
  text on each side and no blank, joins the person whose `addresses`
  hold it. It joins the org whose `domains` hold the text after its `@`
  (record 0011).
- Each value of the read `speaker` joins the person whose `names` hold
  it, and the orgs whose `domains` hold the domain of one of that
  person's addresses.
- An address or a name two persons hold joins neither person, and a
  domain two orgs hold joins neither org (119; record 0011).
- Addresses, names and domains are compared with their case evened
  (record 0011).

A joined person gives `<whom>.name`, `<whom>.role` where it has one and
`<whom>.teams`, each team a value. A joined org gives `<whom>.org`, its
name, and `<whom>.client`, true or false. `<whom>` is `attendee` or
`speaker`. Values are listed in the order of the attendees or speakers
they come from, so a fact of several values holds a comparison when any
of them does (123).

### 6. The policy's form

`Policy::read(settings: &str)` reads the settings document's `rules`, an
array of tables, `[[rules]]`, into `Rule`s in order. No `rules` is a
policy of none. A `Rule` holds:

- `name`: lowercase letters, digits and `-`, no other rule's.
- `when`: a `Match`.
- `outcome`: `Outcome::Launch { kind, repository }`,
  `Outcome::Person(what it waits on them for)` or
  `Outcome::Drop(reason)`, each text non-empty.
- `priority`: `Priority`, one of `low`, `normal`, `elevated`, `high` and
  `urgent`.
- `window`: a `weir_cadence::Schedule`.

It is refused with a `SettingsError` naming the field, `rules[<n>].<field>`
counting from 0, in the field order above (122; C.9):

- a field of the wrong shape, or one the profile does not name;
- a name used twice, or not spelled as above;
- no `kind`, `person` or `drop`, or more than one;
- `repository` without `kind`, or `kind` without `repository`;
- `priority` missing from a launch or a person's turn, or given with a
  drop;
- `window` with a drop;
- a `window` `Schedule::parse` refuses.

A `Match` is `All(Vec<Match>)`, `Any(Vec<Match>)`, `Not(Box<Match>)`,
`Fact { name, test }` or `Older(Span)`. A `Test` is `Is(Value)`,
`In(Vec<Value>)`, `Exists(bool)` or `Contains(String)`. A table holding
other than exactly one of `all`, `any`, `not`, `fact` and `older` is
refused, and so is a `fact` holding other than exactly one of `is`, `in`,
`exists` and `contains`. A fact's name C.9 gives no sort is refused, and
so is a span other than a whole number of `m`, `h` or `d`. Each is
refused naming its path, `rules[0].when.all[1].fact`. A value is spelled
as an offer's is (decision 2). A TOML date or time is refused (record
0011).

### 7. The match

`Match::holds(&self, facts: &Facts, age: Option<jiff::SignedDuration>)`:

- `Is(v)` holds when a value of the fact equals `v`.
- `In(list)` holds when a value of the fact equals one of the list's.
- `Exists(b)` holds when the fact has a value, as `b` says.
- `Contains(t)` holds when a text value holds `t`, both lowercased.
- `Older(span)` holds when the age is past the span, strictly. There is
  no age where the event time cannot be read, and none past any span
  where it is after the time given.
- `All` of none holds, and `Any` of none does not (record 0011).

A fact absent holds no `Is`, `In` or `Contains`.

### 8. The two readings

`Stated<'a>` is a capture's stated facts with its `source`, its `title`
where it has one and its `event_at`. The facts a rule reads are those
with `source`, `title` and `event_at` added as text values (117), the
read facts where there are some, and `People::join` of them all.

- `Policy::for_signal(&self, capture: &Stated, read: &Facts, people:
  &People, now: jiff::Timestamp) -> Option<&Rule>` gives the first rule
  in order whose `when` holds over the stated, joined and read facts.
- `Policy::for_capture(&self, capture: &Stated, people: &People, now:
  jiff::Timestamp) -> Option<&Rule>` passes over every rule for which
  `Rule::names_a_read_fact` holds. That is a rule whose `when`, at any
  depth, names a read fact or a fact joined from one, `speaker.*`. Of
  the rest it gives the first whose `when` holds over the stated facts
  and those joined from them (125).

The age is `now` less the event time. Neither reading reads a clock or
changes anything (core.1, core.58).

### 9. A rule's item's name

`Rule::item_name(&self, made_at: jiff::Timestamp, zone: &TimeZone) ->
String` is `<rule>-<time>`. `<time>` is the first time the window names
after `made_at`, read in `zone`, or `made_at` itself for a rule with no
window or a window that names no later time. It is written in UTC as
`YYYYMMDDTHHMMSSZ` (C.9; record 0011).

### 10. The offer and the capture

`Offer` gains `facts: Facts`, read by `offer_fields` from `facts` through
`Facts::stated`. A refusal names `facts.<name>` and the offer's key, as
any malformed field does (13; C.1 step 1). `Capture` gains `facts:
Facts`, its stated facts. `to_record` adds them last, one `fact` field
each, through `Facts::to_fields`. `from_record` reads every `fact` field
through `Facts::from_fields`, refusing naming `fact` (16, 121; C.2). No
joined fact is written.

### 11. `take` given a policy

`Take::with_policy(self, policy: Policy, people: People, zone: TimeZone)
-> Take` keeps them beside the taken-at time, kept too as a
`jiff::Timestamp`. A `Take` given none reads no rule. In `take_offer`,
after the own excerpt is cut:

1. Where the offer carried an own excerpt, `for_signal` is read with no
   read facts. Otherwise `for_capture` is read (126).
2. A capture with no own excerpt that a rule matched gets the own signal
   of decision 12.
3. `capture.unread` is true only where there is no own signal (19).
4. `Taken` carries `rule_move: Option<RuleMove>`.

`RuleMove { rule, outcome }` holds the rule's name and a `RuleOutcome`:

- `Join { priority, item }`, for a launch or a person's turn, `item`
  being `Rule::item_name(taken_at, zone)`;
- `Drop { reason }`, for a drop (128).

A held key reads no rule, and a refused offer none.

### 12. A rule's own signal

`OwnSignal::kind` becomes `Option<Nature>`. A rule's own signal is
`<key>#1` (127):

- its `excerpt` and `assertion` are the capture's title as offered, or
  its key where the offer gave none, uncut;
- its `asserted_by` is the capture's source;
- its `position` is `title`;
- its `kind` is none, and its subjects and `argues_with` are empty.

### 13. A signal of no nature

`Signal::kind` becomes `Option<Nature>`. `to_record` writes `kind` only
where there is one, and `from_record` reads a record with none.
`ReadSignal::to_signal` still refuses a reader's signal with no nature
(25). `InboxAt::read` refuses a signal with no `kind`, naming its path
and `kind`, unless it is a rule's own signal: `<key>#1`, its capture's
one signal, of a capture not unread and not imported, holding decision
12's fields. One that differs is refused naming the field that differs
(26, 127; core.16).

### 14. A rule's item

`Item` gains `rule: Option<String>`, `priority: Option<Priority>` and
`closed_at: Option<String>`. `to_record` writes them after `named_at`, in
C.2's order. `from_record` refuses each, naming the field:

- `rule` without `priority`, or `priority` without `rule`;
- `rule` not spelled as a rule's name;
- `priority` not one of the five;
- `rule` on an item a conclusion made;
- `closed_at` without `rule`, or not an RFC 3339 time.

`InboxAt::rule_item(rule)` gives the rule's current item: the item
naming the rule with no `closed_at`. Where a write formed elsewhere left
several, it gives the earliest by `named_at`, then by name (128).

### 15. The rule's move in the capture's write

`InboxAt::take` forms the capture's write as it does, and where `Taken`
carries a rule's move:

1. It refuses a rule's move with no own signal, and an own signal of no
   nature with no rule's move, naming `signal`.
2. It reads the inbox the capture's write would leave, through
   `InboxAt::check`.
3. On that inbox it forms the move:
   - signal `<key>#1`, by `rule:<rule>`, at the capture's `taken_at`,
     in no batch, replacing nothing (31, 32);
   - a drop's reason its target and its note (C.2);
   - a join's target the rule's current item there, or, where it has
     none, the name handed. A name an item of the inbox has had is
     refused (35).
4. It checks the move as `make_move` does.
5. It appends the move to `moves.rec`. Where the join makes its item, it
   writes the item's `item.rec`: made by a join, named by `rule:<rule>`
   at `taken_at`, with its rule and priority.
6. It checks the whole write on the tip. Any refusal refuses it whole.

Since `Inbox::write` forms the write on each tip it reads, a replay finds
the current item another writer made meanwhile and joins it (core.26;
record 0011).

### 16. The moon projects

`.moon/workspace.yml` lists `policy: 'crates/weir-policy'`. Its moon
project has `format`, `lint`, `typecheck` and `test`, each tagged
`check-fast`, `dependsOn: [cadence]`, and inputs naming its own crate,
`weir-cadence`'s and the toolchain's, as `weir-delivery`'s do.
`capture` gains `dependsOn: [policy]`. `capture`, `signals`, `adapters`,
`pack`, `weir`, `delivery` and `kinds` each build on `weir-capture`, and
each gains `/crates/weir-policy/**/*` and `/crates/weir-cadence/**/*` as
inputs (kit:testing.3).

### 17. The tests

A test named for a scenario holds it, under the scenario's name in snake
case. `weir-policy`'s tests read C.9's example document as a fixture,
its people and rules. weir-capture's tests take lines through its
fakes. `weir-signals`' end-to-end tests take lines through
`weir_capture::take` into an inbox over `InboxAt`, as its import tests
do. No test reaches a network or names a tool of the family (core.2).

## Risks / Trade-offs

- [A window's close names a rule's item in the machine's time zone, so
  two hosts in different zones may name one rule's next item
  differently] → Only the first item's maker names it. Every other
  writer joins it at the tip.
- [A rule with no window that makes two items in one second refuses the
  second capture's write as a used name] → The adapter's next run
  offers it again, since its key is not held.
- [`Capture`, `Taken`, `OwnSignal`, `Signal` and `Item` gain fields, which
  every builder of them gives] → Each crate's builders are given them in
  the task that changes the type, or in the task for that crate.
- [A reader of `weir-signal/1` from before this change refuses an own
  signal a rule made, which has no `kind`] → C.2 keeps the major at 1.
  Every host takes weir at one revision (millpond core.1).
