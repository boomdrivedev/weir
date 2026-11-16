---
number: 11
title: weir-policy holds the facts and the policy, and a rule's move is formed at the tip its write lands on
status: proposed
date: 2026-11-15
decision-makers:
- the construct stage of change/weir-reads-the-policy-in-a-captures-write
---

# weir-policy holds the facts and the policy, and a rule's move is formed at the tip its write lands on

Status: proposed on 2026-11-15 by the construct stage of the change `weir-reads-the-policy-in-a-captures-write`. It is that stage's own choice, made on the statement and the rules written, and cheap to change: it holds no choice of the user's. What the statement fixes, it follows as written; this record decides only where the code lies and what C.9 leaves open.

Sources: the brief of the change `weir-reads-the-policy-in-a-captures-write`; the statement in boomdrivedev/blueprints, `design/weir/requirements.md`, 2, 13, 16, 19, 24 to 26, 31, 36, 70, 84, 117 to 129, C.1, C.2, C.5, C.9, section 9 and I10 as amended at blueprints `63a14c5` and `104f847`, read at `30dce35`; its research `design/research/2026-11-11-update-ladder.html`, whose meeting example states its attendees as addresses; millpond's statement, 125 to 132, and its rule core.1 on boomdrivedev/millpond's `design/millponds-constitution` at `75b0cea`, which has millpond write a rule's own signal and its move as `take` hands them; `crates/weir-capture/src/take.rs`, `crates/weir-signals/src/write.rs` and `crates/weir-cadence/src/schedule.rs` at `5e3f128`; rules core.1, core.4, core.6, core.13, core.14, core.16, core.20, core.26, core.33 and core.55 to core.58.

## Context and problem statement

The statement gives an inbox a policy: its keeper's rules, in order, each a match on facts with a kind to launch, a person's turn or a drop (122). A host reads it for each own signal and each capture with no own excerpt in the capture's write, and writes the move of the rule it gives in that write (126). A capture with no own excerpt that a rule matches as it lands becomes its own signal (127). A rule's join goes into the rule's current item, or makes one named for the rule and its window's close (128). Section 9 names `weir-policy` as the crate of the facts, the people list's join and the policy. millpond's core.1 has millpond write what `take` hands it, so every host takes a line the same way.

Three things are left to weir. Which crate depends on which, so `take` can read a policy. Where a rule's current item is found, so two writers at once never make two. And what C.9 leaves open: the facts' details, the join's, and the match's.

## Considered options

For where the code lies:

1. **`weir-policy` holds the facts, the people list and the policy, and depends on `weir-cadence` alone; `weir-capture` depends on it, and `take` calls it** (chosen).
2. **The facts in `weir-capture`, and `weir-policy` over them, `take` reading the policy through a trait its host hands in.** A host could hand a reader of its own, which millpond's core.1 and core.55 rule out.
3. **All of it in `weir-capture`.** Section 9 names `weir-policy`, and a host that reads its policy at its status, outside a take, would take the capture crate for it.

For where a rule's current item is found:

1. **At the tip the write lands on, by `weir-signals`' write of what `take` hands over** (chosen).
2. **By `take`, through the host's `Inbox` answering a rule's current item.** The answer comes from the tip read before the take, and a replay on a new tip (core.26) would keep it.

## Decision outcome

- **The crates.** `crates/weir-policy` holds the facts' names, forms and sorts, the people list and its join, the policy's form and check, the two readings of a policy, and a rule's item's name. It depends on `weir-cadence` for a window, and on no other crate of weir's. `weir-capture` depends on it: an offer's facts are its `Facts`, and `take` is handed a `Policy`, a `People` and the machine's time zone beside the taken-at time. Not taken: option 2, since a reader a host hands in is a second reader. Not taken: option 3, which puts a host's settings into the capture contract's crate.
- **The tip.** `take` reads which rule matches and hands the inbox the rule's move: its rule, its word, its priority and the name an item it makes takes, or the drop's reason. `InboxAt::take` forms the move on the tip it is given. The rule's current item is the item naming the rule with no `closed_at`. Where there is none, the move makes an item under the name `take` handed. Each replay forms it again on its new tip, so writers of one rule at once meet one item. Not taken: option 2.
- **One sort per name.** A stated fact named `source`, `title` or `event_at`, a read fact's name, or a name holding a `.` is refused as one C.9 does not spell (C.1 step 1). An offer's own fields give the first three (117), a reader gives the read facts (120), and a host gives the joined (119). Not taken: taking such a fact and letting the other sort's value stand over it, which leaves a rule's reading to the order of a merge.
- **Values.** A fact's value is a string, a number, `true` or `false`, or a list of them holding at least one (117). `null`, an object or a list in a list is refused naming the fact. A string that is an RFC 3339 full-date is a date. Two numbers are one value when they are the same number, so `900` and `900.0` match. A policy spells a value the same way. A TOML date or time is refused, since C.1's JSON spells none. Not taken: TOML's dates taken as dates, which no offer can state.
- **At rest.** A capture's facts are written in their names' order, each fact's values in the order offered, one `fact` field per value (C.9). A fact of one value and a list of one read back alike. Not taken: the order of the offer's object, which JSON does not keep.
- **The join.** An attendee's value is its address where it is one: one `@`, text on each side, and no blank. Any other value joins nothing. The research's example states attendees so. Addresses, names and domains are compared with their case evened, so a transcript's `dana` joins Dana. A domain two orgs hold joins neither org, as an address or a name two persons hold joins neither person (119). Joined values are listed in the order of their attendees or speakers, and an org's `client` is `false` where the org does not say. Not taken: a name and an address read out of `Name <address>`, a form no source is known to give. Not taken: an exact comparison, by which a name a transcript gives in lower case joins no one. Not taken: a people list refused for a domain two orgs hold.
- **The people list.** A person needs a `name`; `addresses`, `names` and `teams` default to empty and `role` to none. An org needs a `name`; `domains` defaults to empty and `client` to `false`. `[about]`'s `teams` and `repositories` default to empty. A field the profile does not name is refused naming it, as a policy's is (122). The settings' other tables are the host's and are not read. Not taken: a field the profile does not name ignored, which hides a misspelt `adresses`.
- **The match.** `older` holds when the capture's age at the time given is past its span, strictly. A capture whose event time is after that time is past no span. An `all` of none holds and an `any` of none does not. Not taken: an empty list refused, which leaves a keeper no rule that matches every capture.
- **A window.** A rule's item is named `<rule>-<time>` (C.9). A window that names no time after the item's making names it by its making, as a rule with no window does. Not taken: such a window refused, which needs a time to read a policy against. A rule with no window that makes two items in one second makes the second under a used name. The check at the tip refuses that capture's write as any used name's (35), and the adapter's next run offers it again.
- **Refusals.** A policy or a people list refused names its field by its path, counting from 0: `rules[1].when.all[0].fact`. A field the profile does not name is refused before any other field of its table, so a misspelt `prority` is named as written. A rule holding none of `kind`, `person` and `drop` is refused naming the rule, `rules[0]`, and a document that is not TOML is refused naming no field. A rule's item holding `priority` and no `rule` is refused naming `priority`, as one holding `rule` and no `priority` is. Not taken: the fields checked in the profile's order alone, by which a misspelt `prority` is reported as `priority` missing.

### Consequences

- A host reads its policy and people list through `weir-policy`, and hands them to `take` with its machine's time zone, or hands none. Given none, `take` writes what it wrote before.
- `weir-capture` and every crate that depends on it build on `weir-policy` and `weir-cadence`, and their moon projects take both as inputs.
- An adapter states an attendee by its address, for the attendee to join.
- A window's close in the machine's local time names a rule's item. Two hosts in different zones may name one rule's item differently, as C.5 reads every schedule locally.
- Checking a reader's read facts against their lists, a rule read in a reading's write, a rule's item closed with its brief, and a link are not in this record. Each is a change of its own.
- This record bears on core.6, core.13, core.14, core.16 and core.55 to core.58, which are drawn from the statement. It adds no rule of its own. The change `weir-reads-the-policy-in-a-captures-write` builds it.
