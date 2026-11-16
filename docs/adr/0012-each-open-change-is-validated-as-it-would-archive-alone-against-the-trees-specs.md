---
number: 12
title: Each open change is validated as it would archive, alone against the tree's specs
status: proposed
date: 2026-11-16
decision-makers:
- the user, by boomdrivedev/blueprints principle 16 as amended on 2026-11-15
- the code stage of the chore weirs-checks-catch-changes-that-wont-archive
---

# Each open change is validated as it would archive, alone against the tree's specs

Status: proposed, until the user's yes on it as a record. Point 1 is the user's, boomdrivedev/blueprints principle 16 as amended on 2026-11-15, which the chore's brief carries to this repository. Points 2 to 5 are boomdrivedev/boomdrive's record 0050, the family's one design for this check, which weir takes as Boomdrive wrote it, as sluice's record 0128 and flume's record 0073 do. Points 6 to 9 are the code stage's own, made on the rules written and each cheap to change.

Deciders: the user. Their answer of 2026-11-15 on their card `req_278df7ab2cc34988a3f5`: "OpenSpec's own validation stays; the rule is about checks of what pages say". Principle 16 as amended keeps "OpenSpec's `openspec validate` of a kit's specs and its open changes, each change validated as it would archive". The brief of the chore `weirs-checks-catch-changes-that-wont-archive`, on the bolt `bolt/weirs-changes-are-checked-as-archived`: weir's checks fail a change OpenSpec would refuse to archive, such as one with a requirement over 500 characters, so it is caught while the code is written instead of at the end, as Boomdrive's and Flume's checks do. The chore's code stage, on points 6 to 9.

Sources: boomdrivedev/blueprints `design/principles.md` 16, on its main; boomdrivedev/boomdrive's record 0050 and core.61, and its `check-changes.sh` at `d9414da6`; boomdrivedev/flume's record 0073 and core.59, and its `check-changes-cases.sh` at `cdc3ddf3`; boomdrivedev/sluice's record 0128 and core.61; OpenSpec 1.14.3, the release `devenv.nix` pins, its `openspec archive` and `openspec validate`; record 0007 and its amendment of 2026-11-15; rules core.39 and core.40.

## Context and problem statement

`root:specs` runs `openspec validate --specs --strict`. Nothing in the gate read the open changes: a change was first validated whole at its archive, after the gate had passed its branch. `openspec validate --changes --strict` alone would not catch it either:

- `openspec archive` refuses a delta that does not apply, an ADDED requirement its spec already holds or a MODIFIED one its spec does not hold. `openspec validate --changes --strict` reports it only as information and passes.
- `openspec archive` checks the specs it rebuilds without `--strict`. A new capability whose delta gives no `## Purpose` gets archive's placeholder, and a MODIFIED requirement past OpenSpec's 500 characters is let through; `--strict` fails both once the change is archived, and `openspec validate --changes` holds only an ADDED requirement to that length.

Record 0007's amendment of 2026-11-15 left this open: `root:specs` validates the specs alone, and each open change validated as it would archive "is a check of its own for a change to add".

## Considered options

Boomdrive's record 0050 weighed these, and weir takes its choices.

For what a change is applied to:

1. **Each change alone, applied to a scratch copy of the tree's specs.**
2. Every open change applied in turn to one copy.
3. Each change applied to the tree's own specs, which the check then puts back.

For which specs are held to `--strict` after the archive:

1. **Each spec the archive writes, a new one or one whose bytes differ from the tree's.**
2. Every spec in the copy, as `openspec validate --specs --strict` reads them.

For where the script lives, how it is proven, where the check is written down and the numbers it takes, the code stage weighed the options in points 6 to 9.

## Decision outcome

1. **`root:changes` holds each open change to OpenSpec's validation as it would archive (the user's).** It runs `openspec validate --changes --strict`. Then, for each open change, it runs `openspec archive` in a scratch copy and holds the specs that result to `openspec validate --strict`. A change that fails is named with OpenSpec's words. `check-changes.sh` does it, and core.59 says so.
2. **Each change is applied alone to a scratch copy of the tree's specs (record 0050, its first option).** The copy holds the tree's specs, `openspec/config.yaml` and that change, under `mktemp -d`. Each change archives at its own merge onto its base as it stands then.
   - Not taken: every open change applied in turn. It judges a change by the other open changes and the order they are applied in, and neither is the change's own. Two open changes that conflict are found by the gate on the branch that lands second.
   - Not taken: the tree's own specs, put back after. A check writes nothing into the tree.
3. **The specs the archive writes pass `openspec validate --strict`, though `openspec archive` validates them without it (record 0050).** `--strict` is how `root:specs` reads them once the change is archived.
4. **Only the specs the archive writes are validated (record 0050, its first option).** The script knows which by comparing each with the tree's byte for byte, by `cmp`, which reads no word and no form of either page and is part of OpenSpec's validation as archived, which principle 16 as amended keeps. A spec the change leaves as it was is `root:specs`' to check.
   - Not taken: every spec in the copy. A spec on the base that fails would fail every open change, each named for what it did not write.
5. **A task left unchecked fails nothing (record 0050).** The archive runs with `--yes`, and a change is open while its tasks are worked.
6. **The script is weir's own, Boomdrive's `check-changes.sh` at `d9414da6` copied whole but for its opening words (the code stage's).** weir's gate runs files its tree holds, at versions it pins (core.40). The opening words cite this record and core.59, and name Boomdrive's script as the design a change makes first.
   - Not taken: a script one tool owns that weir's gate calls, which weir's tree neither holds nor pins.
   - The cost: the copies in the family can drift. A change to the design is made in Boomdrive's record 0050 first and carried here (CLAUDE.md, "The family").
7. **The script is proven by a task of its own, `root:changes-cases`, flume's `check-changes-cases.sh` at `cdc3ddf3` copied whole but for its opening words (the code stage's).** core.39 rules out a change whose only verification is a command run by hand. The cases run the script over five scratch trees: a change that does not validate, one archive refuses, a new capability with no Purpose, a MODIFIED requirement past OpenSpec's length, and two changes that each archive alone with their tasks open, which the check passes and leaves the tree as it found it. Each case writes its spec and its changes into its scratch tree as data, which core.39 does not rule out: they are no Markdown file of the tree's own.
   - Not taken: no test, as Boomdrive's record 0050 and sluice's record 0128 have it, the refusals seen once by hand. weir's core.39 rules that out.
   - Not taken: the cases inside `check-changes.sh`, which would run them over every tree it checks.
8. **The check is written in core.59, core.39 and the gate's own words, and in no spec of weir's (the code stage's).** weir's specs say what its crates do. Its gate is written in `docs/architecture/core.md`, `moon.yml`, `.moon/workspace.yml`, `CLAUDE.md` and `openspec/config.yaml`'s context, as `root:specs` and record 0007 are. core.39 is amended so that each open change validated as it would archive stands beside the specs as OpenSpec's own validation, and so that the cases' scratch trees are named as data.
   - Not taken: a gate capability in `openspec/specs` through an OpenSpec change, as flume's `the-gate` and sluice's `repository-gate` hold theirs. weir's specs hold no gate requirement for one to sit beside.
9. **The record is 0012 and its rule core.59 (the code stage's).** main holds records to 0009 and rules to core.53. Records 0010 and 0011 and rules core.54 to core.58 are taken on `bolt/weir-carries-the-release-chains-parts` and `change/weir-reads-the-policy-in-a-captures-write`, so each number here is the next one free across them.

### Consequences

- Good: a change that would not archive, or would archive into specs `root:specs` fails, fails its own branch's gate, named with OpenSpec's words, before it lands.
- Good: a delta that adds a capability has to carry its own `## Purpose`, since archive reads a delta's Purpose only when it creates the capability and the placeholder fails `--strict`.
- Bad: `root:changes` runs `openspec` once more for each open change and once for each spec its archive writes, and `root:changes-cases` runs it for five trees, about three seconds on deneb.
- Bad: weir's `check-changes.sh` and the family's other copies can drift; the design is Boomdrive's record 0050's to change first.
- On 2026-11-16 the tree holds no open change, and the one open change on another branch, `weir-reads-the-policy-in-a-captures-write` on `change/weir-reads-the-policy-in-a-captures-write`, passes the check.
- OpenSpec 1.14.3 reads an ADDED requirement that its spec holds word for word as already in sync, and archives it with no change. The check passes it, as archive does.
- Rule core.59 holds the decision, and core.39 is amended in the same commit.
