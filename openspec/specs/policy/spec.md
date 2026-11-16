# policy Specification

## Purpose
`weir-policy`'s policy: a keeper's rules read and checked whole; a
rule's match over facts and a capture's age; the policy read for a
signal and for a capture as it lands; and a rule's item's name (122 to
129; C.9; record 0010; core.56, core.57).

## Requirements

### Requirement: A policy is read whole and refused naming the field

A policy SHALL be read from its settings document's `[[rules]]`, in order, each rule with its name, its `when`, exactly one of `kind` with `repository`, `person` and `drop`, a `priority` for a launch or a person's turn and none for a drop, and a `window` or none, not with a drop. A rule that breaks this or 123, names a rule twice, or holds a field C.9 does not name SHALL refuse the policy naming the field (122; C.9; core.56).

#### Scenario: C.9's policy read
- **WHEN** the settings document of C.9's example is read
- **THEN** the policy holds `stand-ups`, a drop, `point-releases`, a launch of `update` at low priority with a thirty-minute window, and `a-clients-ask`, a person's turn at urgent priority, in that order

#### Scenario: A rule with two outcomes
- **WHEN** a rule holds both `kind` and `drop`
- **THEN** the policy is refused, naming `rules[0].drop`

#### Scenario: A rule named twice
- **WHEN** the second rule takes the first rule's name
- **THEN** the policy is refused, naming `rules[1].name`

#### Scenario: A rule name with an underscore
- **WHEN** a rule is named `point_releases`
- **THEN** the policy is refused, naming `rules[0].name`

#### Scenario: A drop with a priority
- **WHEN** a drop gives `priority = "low"`
- **THEN** the policy is refused, naming `rules[0].priority`

#### Scenario: A launch with no repository
- **WHEN** a rule gives `kind` and no `repository`
- **THEN** the policy is refused, naming `rules[0].repository`

#### Scenario: A launch with no priority
- **WHEN** a rule gives `kind` and `repository` and no `priority`
- **THEN** the policy is refused, naming `rules[0].priority`

#### Scenario: A window that is not five-field cron
- **WHEN** a rule's `window` is `@hourly`
- **THEN** the policy is refused, naming `rules[0].window`

#### Scenario: A field the profile does not name
- **WHEN** a rule holds `level = "patch"` beside its `when`
- **THEN** the policy is refused, naming `rules[0].level`

#### Scenario: A match of a form 123 does not name
- **WHEN** a rule's `when` is `{ fact = "level", above = 3 }`
- **THEN** the policy is refused, naming `rules[0].when`

#### Scenario: A value spelled as a TOML date
- **WHEN** a rule's `when` is `{ fact = "due", is = 2026-11-14 }`, a TOML date
- **THEN** the policy is refused, naming `rules[0].when.is`

### Requirement: A rule's match reads facts and the capture's age alone

A match SHALL combine, by `all`, `any` and `not`, a fact compared with `is`, `in`, `exists` or `contains`, and the capture's age with `older`. A fact of several values SHALL hold a comparison when any value does; `contains` SHALL compare a text value with its case evened; `older` SHALL hold when the age is past its span, strictly. A match SHALL read no signal's words and no raw material (123; record 0010; core.56).

#### Scenario: Any value of a fact holds a comparison
- **WHEN** a rule matches `attendee.client` is true, and of two attendees one joins a client and one an org that is not
- **THEN** the match holds

#### Scenario: Contains evens case
- **WHEN** a rule matches `title` contains `stand-up`, and the capture's title is `Daily Stand-up`
- **THEN** the match holds

#### Scenario: Exists false on a fact not stated
- **WHEN** a rule matches `level` exists false, and the capture states no `level`
- **THEN** the match holds

#### Scenario: Older than a span
- **WHEN** a capture's event time is two hours before the time given
- **THEN** `older = "1h"` holds and `older = "2h"` does not

#### Scenario: An empty all holds and an empty any does not
- **WHEN** one rule's `when` is `{ all = [] }` and another's `{ any = [] }`
- **THEN** the first holds over any facts, and the second over none

### Requirement: Reading a policy gives the first rule whose match holds

Reading a policy for a signal SHALL give the first rule, in the policy's order, whose match holds over the signal's stated, joined and read facts at the time given, or none. A rule's outcome, priority and window SHALL be its own, whatever the facts say. The same policy, people list, facts and time SHALL give the same rule, and reading SHALL change nothing (124, 125; I10; core.56).

#### Scenario: The first rule in order
- **WHEN** two rules of a policy match a signal
- **THEN** reading the policy gives the first of them

#### Scenario: No rule matches
- **WHEN** no rule of a policy matches a signal
- **THEN** reading the policy gives none

#### Scenario: A fact's own priority is matched like any other
- **WHEN** an offer states `priority` as `"urgent"`, and the rule it matches is `{ fact = "priority", is = "urgent" }` at low priority
- **THEN** reading the policy gives that rule, whose priority is low

#### Scenario: A speaker two persons name matches no rule
- **WHEN** a rule matches `speaker.role` is `executive`, and a signal's speaker is a name two persons of the people list hold
- **THEN** reading the policy for the signal gives none (S29)

### Requirement: A capture as it lands is read by rules on stated facts alone

Reading a policy for a capture as it lands SHALL pass over every rule whose match names a read fact, or a fact joined from one, at any depth, and SHALL give the first of the others whose match holds over the capture's stated facts and those joined from them, or none (125; core.57).

#### Scenario: A rule on a read fact is passed over as a capture lands
- **WHEN** a policy's first rule is `{ not = { fact = "says", exists = true } }`, and its second matches the capture's title
- **THEN** reading it for the capture as it lands gives the second, and reading it for the capture's own signal gives the first

#### Scenario: A rule on a speaker's joined fact is passed over
- **WHEN** a policy's one rule is `{ any = [{ fact = "speaker.client", is = true }, { fact = "source", is = "meeting" }] }`, and a meeting lands
- **THEN** reading the policy for the capture as it lands gives none

### Requirement: A rule's item is named for its rule and its window's close

A rule's item SHALL be named `<rule>-<time>`, the time the first its window names after the item's making, read in the time zone given, or its making where the rule has no window or its window names no later time, written in UTC as `YYYYMMDDTHHMMSSZ` (128, 129; C.5, C.9; record 0010; core.57).

#### Scenario: An item named for its window's next time
- **WHEN** `point-releases`, its window `*/30 * * * *`, makes an item at 2026-11-12T10:05:00Z, read in UTC
- **THEN** the item's name is `point-releases-20261112T103000Z`

#### Scenario: An item of a rule with no window is named for its making
- **WHEN** `a-clients-ask`, which has no window, makes an item at 2026-11-12T10:05:07Z
- **THEN** the item's name is `a-clients-ask-20261112T100507Z`

#### Scenario: A window read in the machine's zone
- **WHEN** a rule whose window is `0 9 * * *` makes an item at 2026-11-12T15:00:00Z, read in America/New_York
- **THEN** the item's name ends `-20261113T140000Z`, 09:00 there the next morning

#### Scenario: A window that names no later time
- **WHEN** a rule whose window is `0 0 30 2 *` makes an item at 2026-11-12T10:05:07Z
- **THEN** the item's name ends `-20261112T100507Z`, its making
