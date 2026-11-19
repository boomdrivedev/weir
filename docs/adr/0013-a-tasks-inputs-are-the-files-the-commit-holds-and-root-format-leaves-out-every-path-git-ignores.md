---
number: 13
title: A task's inputs are the files the commit holds, and root:format leaves out every path git ignores
status: proposed
date: 2026-11-19
decision-makers:
- boomdrive-design
- the code stage of fix/main--format-hashes-only-tracked-files
---

# A task's inputs are the files the commit holds, and root:format leaves out every path git ignores

Status: proposed on 2026-11-19, until the operator's yes. Point 1 is boomdrive-design's ruling of 2026-11-19 on the signal `a-key-check-refuses-files-git-ignores`, which the fix's brief carries to this repository. Points 2 and 3 are boomdrivedev/boomdrive's record 0054, the family's one design for root:format's inputs, which weir takes as Boomdrive wrote it. Points 4 and 5 are the code stage's own, made on the rules written and each cheap to change. None is the user's.

Deciders: boomdrive-design, whose ruling the brief quotes: "A check's inputs are the files the commit holds: no task's inputs take in a file git ignores." The code stage of `fix/main--format-hashes-only-tracked-files`, 2026-11-19, from the brief: "Make moon hash only tracked files for the format task: by moon's own setting for that if the pinned moon has one, read from moon's documentation, or by inputs that leave every ignored path out. treefmt reads only what git tracks, so the check itself is unchanged."

Sources: boomdrivedev/boomdrive's record 0054 and core.7, at `3a127cc7`, and its `moon.yml` and `.gitignore` at `d4c5386e`; boomdrivedev/sluice's C.6 and its key check, which refuses a key whose manifest names an input the tree lacks, `<task>: <path> is not in the tree`; moonrepo/moon at `v2.6.1`, `crates/task-hasher/src/task_hasher.rs`, `aggregate_inputs` and `process_inputs`, and moonrepo/starbase's `starbase_utils` 0.14.6, which moon 2.6.1 locks, `GlobSet::is_included`; moon's documentation of `hasher.walkStrategy` and `hasher.ignorePatterns`; git's `gitignore(5)`; commit `ebd0e0e`, which first negated `.gitignore`'s entries among weir's root:format inputs; record 0007; rule core.39.

## Context and problem statement

sluice signs a run's key for a task only where every input the key's hash manifest names is a file the combination's tree holds. A task whose inputs take in a file the commit does not hold has its key refused. boom's submission #1 was refused on `.devenv/bootstrap/bootstrapLib.nix` and timber's #1 on `.claude/settings.json`, each written by the devenv shell, ignored by git and taken in by their root:format's `/**/*`.

weir's root:format has negated each entry of `.gitignore` among its inputs since `ebd0e0e`. With a file planted at every path `.gitignore` names, at the root and under `crates/weir/` and `openspec/specs/`, moon hashed the 988 files git tracks and no other. Two gaps were left:

- `.claude/settings.local.json`, which Claude Code writes in a checkout for one person's own settings, was neither ignored nor left out. With it planted, moon hashed 989 files, a key sluice would refuse naming it, and treefmt failed, `no formatter for path: .claude/settings.local.json`.
- `.moon/cache/` holds a slash before its last, so git matches it at the root alone, but it was negated at any depth, `!/**/.moon/cache/**`. A file a commit held under a nested `.moon/cache/` would be left out of the key while treefmt reads it.

moon 2.6.1 has no setting that limits a task's inputs to what git tracks. For a project at the workspace's root, `aggregate_inputs` walks the file system for the task's globs whatever `hasher.walkStrategy` says, and the files `git status` lists as changed, untracked ones among them, are added for every project. For a crate's project, moon resolves the globs under the crate's own folder from git's listing, so they hash no file git ignores; a glob from the workspace's root naming another folder, another crate's or `pack/`'s, is walked over the file system.

## Considered options

Boomdrive's record 0054 weighed these, and weir takes its choice:

1. **`/**/*`, with each entry of `.gitignore` negated where git matches it** (chosen).
2. `hasher.walkStrategy: vcs` in `.moon/workspace.yml`.
3. Inputs that name the tree's top-level folders and files, each as a glob.
4. A file group of every tracked file, written from `git ls-files` and held to it by a check.
5. root:format uncached, `cache: false`.

weir weighed one more, as the brief asks of moon's own settings:

6. `hasher.ignorePatterns` in `.moon/workspace.yml`, holding `.gitignore`'s entries for every task.

## Decision outcome

1. **No task's inputs take in a file git ignores, so a task's key names only files the commit holds (boomdrive-design's).** core.39 says so.
2. **root:format's inputs are `/**/*`, with each entry of `.gitignore` negated where git matches it (option 1; Boomdrive's record 0054).** An entry with no slash but a last one is negated at any depth, `!/**/<entry>`, and one with a slash before its last is negated at the root, `!/<entry>`, so `.moon/cache/` is negated as `!/.moon/cache/**`. An entry added to `.gitignore` is added among the negations, and each file says so. With a file planted at each ignored path, `.claude/settings.local.json` among them, moon hashed the 988 files git tracks, and treefmt passed.
   - Not taken: option 2. It is moon's default, and moon reads it only for a project below the workspace's root.
   - Not taken: option 3. A folder or a file added at the top of the tree and named in no glob would change nothing of the key, so a replay could pass a file treefmt was never run on.
   - Not taken: option 4. Every file added or removed would change `moon.yml`, and the check would be one more task for what the negations already do.
   - Not taken: option 5. treefmt would run over the whole tree at every pass, where a key that matches now replays it.
3. **`.claude/settings.local.json` is ignored and left out of root:format's inputs (Boomdrive's record 0054).** Claude Code writes it in a checkout, a place among them, for one person's own settings, which no commit holds. Until it is ignored, treefmt reads it too, as it reads every file git does not ignore.
4. **Not taken: option 6 (the code stage's).** moon matches `hasher.ignorePatterns` against a file's absolute path, so an entry `.gitignore` anchors at the root, `/target`, `/result`, `/.mcp.json` or `/.claude/settings.json`, could be written there only as a pattern at any depth, which would leave out a file of the same name the commit holds deeper in the tree. And the family's other trees negate their entries among root:format's inputs, so weir's would be the one copy kept another way.
5. **The check itself is unchanged (the code stage's).** treefmt reads only what git does not ignore, so leaving the same paths out of moon's hash changes nothing it formats.

### Consequences

- Good: a key for root:format names only files the commit holds, wherever a checkout's shell, tools or person wrote files `.gitignore` names.
- Bad: the negations are kept to `.gitignore` by hand, and a missed one is found when sluice refuses a key, naming the file.
- Bad: a file ignored only by a host's own excludes, `core.excludesFile` or `info/exclude`, is outside the tree's reach and still hashed where a checkout the gate runs in holds it, and refused the same way. So is an untracked file git does not ignore.
- Bad: a crate's task that names another crate's folder or a folder of `pack/` by a glob from the root would hash a file `.gitignore` names at any depth, `.devenv/` or `devenv.local.nix`, written into that folder. None is written there while the devenv shell is entered at the root alone.
- core.39's rule and details say so, in the commit that writes this record.
