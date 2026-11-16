# facts Specification

## Purpose
`weir-policy`'s facts: a fact's names, values and sorts, in flight
and at rest; the people list, read and checked whole; and the join of
attendees and speakers from it (117 to 121; C.9; record 0010; core.55).

## Requirements

### Requirement: A fact is a name with one value or several

A fact SHALL be a name with one value or several, each a text, a number, true or false, or a date, a date being a string that reads as an RFC 3339 full-date. A fact of no value, or a value that is null, an object or a list in a list, SHALL be refused naming the fact. Two numbers SHALL be one value when they are the same number (117; C.9; record 0010; core.55).

#### Scenario: A fact of several values
- **WHEN** an offer's `facts` gives `attendee` as `["dana@acme.com", "kim@globex.com"]`
- **THEN** the fact `attendee` holds both values, in that order

#### Scenario: A fact of no value
- **WHEN** an offer's `facts` gives `level` as `[]`
- **THEN** it is refused, naming `facts.level`

#### Scenario: A value of an object
- **WHEN** an offer's `facts` gives `level` as `{"major": 2}`
- **THEN** it is refused, naming `facts.level`

#### Scenario: A date in a string
- **WHEN** an offer's `facts` gives `released` as `"2026-11-14"`
- **THEN** its value is a date, equal to the date `"2026-11-14"` a policy spells

#### Scenario: One number two spellings
- **WHEN** one fact's value is `900` and another's is `900.0`
- **THEN** the two values are equal

### Requirement: Each fact's name has one sort

A fact's name SHALL be read, `says`, `speaker`, `about`, `owner` or `due`; joined, `attendee.` or `speaker.` then `name`, `role`, `teams`, `org` or `client`; or stated, lowercase letters, digits, `_` and `-` but `source`, `title`, `event_at` and a read name. An offer's `facts` SHALL hold stated names alone, and any other SHALL be refused naming it (117, 118; C.1 step 1, C.9; record 0010; core.6).

#### Scenario: A fact named in capitals
- **WHEN** an offer's `facts` holds `Level`
- **THEN** it is refused, naming `facts.Level`

#### Scenario: A stated fact under a read fact's name
- **WHEN** an offer's `facts` holds `says`
- **THEN** it is refused, naming `facts.says`

#### Scenario: A stated fact under a joined fact's name
- **WHEN** an offer's `facts` holds `attendee.client`
- **THEN** it is refused, naming `facts.attendee.client`

#### Scenario: A stated fact under an offer's own field
- **WHEN** an offer's `facts` holds `title`
- **THEN** it is refused, naming `facts.title`, since the offer's own `title` is that fact

### Requirement: Facts at rest are one field per value

Facts SHALL be written at rest one `fact` field per value, `<name> <value as JSON>`, in their names' order, each fact's values in the order given, and SHALL read back as written; a fact of one value and a list of one SHALL read back alike. A field not of that shape SHALL be refused naming `fact` (C.2, C.9; record 0010; core.55).

#### Scenario: Facts at rest in their names' order
- **WHEN** an offer states `level` as `"patch"` and `attendee` as `["b@acme.com", "a@acme.com"]`
- **THEN** they are written `fact: attendee "b@acme.com"`, `fact: attendee "a@acme.com"` and `fact: level "patch"`, and read back as stated

#### Scenario: A fact field out of shape
- **WHEN** a field reads `fact: level patch`
- **THEN** it is refused, naming `fact`

### Requirement: The people list is read whole and refused naming the field

The people list SHALL be read from `[about]`, `[[person]]` and `[[org]]` of its settings document, ignoring the document's other tables. A person or an org with no `name`, a field of the wrong shape, or one the profile does not name SHALL be refused naming it, as `person[0].name`; every other field SHALL take its default (119; C.9; record 0010; core.54).

#### Scenario: A people list read from the settings
- **WHEN** the settings document of C.9's example, which holds a table of the host's own besides, is read
- **THEN** the people list holds Dana Reyes with the address, names, role and team C.9 gives, Globex as a client, and `[about]`'s teams and repositories

#### Scenario: A person with no name
- **WHEN** a `[[person]]` gives addresses and no `name`
- **THEN** the list is refused, naming `person[0].name`

#### Scenario: A field the people list does not name
- **WHEN** a `[[person]]` holds `adresses`
- **THEN** the list is refused, naming `person[0].adresses`

### Requirement: An attendee joins by its address and a speaker by its name

An attendee's address SHALL join the person whose addresses hold it and the org whose domains hold its domain; a speaker's name SHALL join the person whose names hold it and the orgs of that person's addresses. Each SHALL be compared with its case evened. An address or a name two persons hold SHALL join neither, and a domain two orgs hold neither (119; record 0010; core.55).

#### Scenario: An attendee joins its person and org
- **WHEN** a capture states the attendees `dana@acme.com` and `kim@globex.com`, and the people list is C.9's
- **THEN** the joined facts give `attendee.name` Dana Reyes, `attendee.role` executive, `attendee.teams` web, `attendee.org` Globex and `attendee.client` true

#### Scenario: A speaker joins by the name a transcript gives
- **WHEN** a reader gives the speaker `dana`
- **THEN** the joined facts give `speaker.name` Dana Reyes and `speaker.role` executive

#### Scenario: A name two persons hold joins neither
- **WHEN** a reader gives the speaker `Dana`, and two persons of the people list hold that name
- **THEN** no `speaker.` fact is joined (S29)

#### Scenario: An attendee with no address joins nothing
- **WHEN** a capture states the attendee `Dana Reyes`, which holds no `@`
- **THEN** no `attendee.` fact is joined

#### Scenario: A domain two orgs hold joins neither
- **WHEN** two orgs of the people list hold the domain `acme.com`, and a capture states the attendee `lee@acme.com`
- **THEN** neither `attendee.org` nor `attendee.client` is joined
