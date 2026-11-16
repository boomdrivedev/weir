# Verify

The change weir-reads-the-policy-in-a-captures-write, on
`change/weir-reads-the-policy-in-a-captures-write` at 5bb471c, holds:
the whole checks pass, every task is ticked, and every requirement and
scenario of its seven delta specs is met by the code and a test of the
scenario's name, each of the 88 run and passed in the whole checks.
`openspec validate weir-reads-the-policy-in-a-captures-write --strict`
reads the change as valid.

## The whole checks

`devenv shell -- check-fast` (`moon run :#check-fast`), run once at
5bb471c: 50 tasks completed, none failed, one from moon's cache.
Ran and passed: `root:format`, `root:toolchain`, `root:hooks`,
`root:openspec-claude`, `root:specs` (27 specs, `openspec validate
--strict`), and `format`, `lint`, `typecheck` and `test` of
`adapters`, `bounded`, `branch`, `cadence`, `capture`, `delivery`,
`kinds`, `pack`, `policy`, `signals` and `weir`. Taken from the cache,
its inputs unchanged since a pass: `root:prose`. Every test ran passed;
the one test not run, cadence's `report`, is ignored by design and run
by the cases after it with `TZ` set.

## Tasks

All 22 tasks of `tasks.md` are ticked, 1.1, 2.1 to 2.9, 3.1 to 3.5,
4.1 to 4.5, 5.1 and 6.1, and each test a task names is in the tree
under that name. `crates/weir-policy` is a moon project of its own,
`policy`, listed in `.moon/workspace.yml` with its four check-fast tasks
(core.39); `capture` depends on it; and `capture`, `signals`,
`adapters`, `pack`, `weir`, `delivery` and `kinds` read
`/crates/weir-policy/**/*` and `/crates/weir-cadence/**/*` among their
inputs. The README's table and `Cargo.toml`'s members and list name the
crate.

## facts

| Requirement | Scenarios | Where it holds |
|---|---|---|
| A fact is a name with one value or several | A fact of several values; A fact of no value; A value of an object; A date in a string; One number two spellings | `fact.rs`'s `Facts::stated` and `Value`; `weir-policy/tests/facts.rs` |
| Each fact's name has one sort | A fact named in capitals; A stated fact under a read fact's name; …under a joined fact's name; …under an offer's own field | `fact.rs`'s `sort`; `tests/facts.rs` |
| Facts at rest are one field per value | Facts at rest in their names' order; A fact field out of shape | `Facts::to_fields` and `from_fields`; `tests/at_rest.rs` |
| The people list is read whole and refused naming the field | A people list read from the settings; A person with no name; A field the people list does not name | `people.rs`'s `People::read`, over `tests/c9.toml` with its `[host]` table; `tests/people.rs` |
| An attendee joins by its address and a speaker by its name | An attendee joins its person and org; A speaker joins by the name a transcript gives; A name two persons hold joins neither; An attendee with no address joins nothing; A domain two orgs hold joins neither | `People::join`; `tests/join.rs` |

## policy

| Requirement | Scenarios | Where it holds |
|---|---|---|
| A policy is read whole and refused naming the field | C.9's policy read; A rule with two outcomes; A rule named twice; A rule name with an underscore; A drop with a priority; A launch with no repository; A launch with no priority; A window that is not five-field cron; A field the profile does not name; A match of a form 123 does not name; A value spelled as a TOML date | `policy.rs`'s `Policy::read`; `tests/read_policy.rs` |
| A rule's match reads facts and the capture's age alone | Any value of a fact holds a comparison; Contains evens case; Exists false on a fact not stated; Older than a span; An empty all holds and an empty any does not | `Match::holds`, over `Facts` and an age alone; `tests/matching.rs` |
| Reading a policy gives the first rule whose match holds | The first rule in order; No rule matches; A fact's own priority is matched like any other; A speaker two persons name matches no rule | `reading.rs`'s `Policy::for_signal`; `tests/reading.rs` |
| A capture as it lands is read by rules on stated facts alone | A rule on a read fact is passed over as a capture lands; A rule on a speaker's joined fact is passed over | `Policy::for_capture` and `Rule::names_a_read_fact`; `tests/reading.rs` |
| A rule's item is named for its rule and its window's close | An item named for its window's next time; An item of a rule with no window is named for its making; A window read in the machine's zone; A window that names no later time | `Rule::item_name` through `Schedule::first_after`; `tests/item_name.rs` |

## cadence

| Requirement | Scenarios | Where it holds |
|---|---|---|
| A schedule gives the first time it names after an instant | The first time after an instant; An instant on a named time; A schedule that names no later time | `Schedule::first_after`, public; `weir-cadence/tests/schedule.rs` |

## adapter-contract

| Requirement | Scenarios | Where it holds |
|---|---|---|
| An offer carries a key, a source and an event time (modified) | A missing required field refuses the offer; An event time without its offset refuses the offer; The event time is kept as offered; An offer's facts are read; An offer stating a fact C.9 does not spell | `line.rs`'s `offer_fields` reads `facts` through `Facts::stated`, a refusal naming `facts.<name>` and the key; `line.rs`'s tests |

The built-in adapters' offers state no fact (task 5.1, commit af7071d).

## capture-take

| Requirement | Scenarios | Where it holds |
|---|---|---|
| The capture record is weir-capture/1 in recutils (modified) | A capture from a run; A multi-line value stays one field; A capture's stated facts are written with it | `capture.rs` writes `fact` fields after `unread`, never a joined fact; `capture.rs`'s tests, and `tests/policy.rs` through `recfix --check` |
| An own excerpt becomes the capture's one signal (modified) | A release's notes as its own excerpt; A meeting with no excerpt | `tests/take.rs`, the own signal's nature `Some(Nature::Ask)` |
| Two hosts given the same lines write the same records (modified) | Two hosts, one set of lines; Only the machine differs; Two hosts, one policy | `tests/take.rs`; `tests/policy.rs`'s `two_hosts_one_policy` over two fresh hosts |
| Take given a policy reads it in the capture's write | A patch release joins its rule's item; A signal no rule matches stays unmoved; A take given no policy | `Take::with_policy`; `take.rs` reads `for_signal` for an own excerpt and `for_capture` as a capture lands, at the taken-at time; `tests/policy.rs` |
| A capture a rule matches as it lands is its own signal | A stand-up dropped as it lands; A capture with no title is its own signal by its key; A rule on a read fact charges a reader | `take.rs`'s own signal of the title or key, no nature, at `title`; `tests/policy.rs` |

## inbox-signals

| Requirement | Scenarios | Where it holds |
|---|---|---|
| A signal holds every field, and its nature is one of five (modified) | A signal of an unknown nature; Empty lists; An own signal a rule made has no nature; A signal of no nature that no rule made | `Signal::kind` an option; `InboxAt::read` refuses a signal of no nature but a rule's own, naming the path and the field; `weir-signals/tests/signals.rs` |
| A capture's own signal is recorded in the same write as the capture (modified) | A cut own signal; An unread capture; A held key; A rule's drop in the capture's write; A rule's join makes its item in the capture's write; A daily stand-up dropped as it lands | `InboxAt::take`; `tests/signals.rs` and `tests/rules.rs`, the last through `weir_capture::take` over a tip |
| An own signal a rule made is its capture's title | A rule's signal whose excerpt is not the title; A signal of no nature with no rule's move | `tests/rules.rs` |
| A rule's move stands once written, in no batch | A rule's move names its rule; A rule's move with no signal | `tests/rules.rs`: `by: rule:point-releases`, the taken-at time, no `batch`, the standing move |

## inbox-items

| Requirement | Scenarios | Where it holds |
|---|---|---|
| A rule's item names its rule and its priority | A rule's item record; A rule's item with a rule and no priority; A closed_at on an item no rule made | `Item`'s `rule`, `priority` and `closed_at` after `named_at`; `tests/rules.rs` and `tests/items.rs` |
| A rule's join goes into its current item | A patch release then another ten minutes later; A closed item takes no more joins; Two writers of one rule meet one item; A rule's item under a used name | `InboxAt::rule_item`, read by `write.rs` on the tip the write is formed against, so a replay finds it again; `tests/rules.rs` |

Each modified requirement keeps every scenario the main spec holds, so
archiving drops none.

## The rules

- core.1, core.14: `weir-policy` reads no clock, starts no thread and
  holds no static; the time reaches a rule only as the taken-at time
  `take` hands it, and the zone only as `with_policy`'s.
- core.2: `weir-policy/tests/names.rs` passes; the people and the rules
  are C.9's example, kept once as `crates/weir-policy/tests/c9.toml`.
- core.55: no crate but `weir-policy` reads a `rules`, `person`, `org`
  or `about` table.
- core.57: a match reads `Facts` and an age alone; no code of
  `weir-policy` reads an excerpt, an assertion or raw material.
- core.58: a rule's move is formed in `take` and written by
  `InboxAt::take` in the capture's write, in no batch, its item found at
  the tip the write lands on.
- `weir conform`'s seam gives a host no policy (C.7): neither the `weir`
  crate nor the pack calls `with_policy`, and both pass unchanged.

## Findings

None.
