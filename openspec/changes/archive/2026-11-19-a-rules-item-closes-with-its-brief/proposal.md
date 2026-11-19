# Proposal

## Why

The statement gives a rule's item a close: when a time its window names
has come since the item was made, its host closes it, records
`closed_at` on its `weir-item/1`, and writes its brief of its signals'
excerpts as they stand, in the order of their event times, adding no
word (36, 128, 129; C.2; S26). After it, the item takes no more of its
rule's joins, and the rule's next join makes its next item.

weir reads `closed_at` and finds a rule's current item by it, but no
operation writes it: the check refuses every write that replaces a
tip's `item.rec`. The change that brought the policy left the close to
a change of its own. millpond's tick closes a rule's item and sends it
(millpond 130, 131), and without the close a rule's second window would
join a sent item, which the check refuses, so every later capture the
rule matches would be refused.

## What Changes

- `InboxAt::close_rule_item(item, closed_at)` forms the close: the
  item's `item.rec` again with `closed_at` added, and, where it holds
  signals, its `brief.md`, their excerpts as they stand, in the order
  of their captures' event times, then of their names, each after a
  blank line from the one before. Refused when the item is not held, no
  rule made it, it is closed already, or `closed_at` is not a time.
- `InboxAt::check` takes a write that replaces a tip's `item.rec` when
  the new record is the tip's with `closed_at` added and nothing else
  changed, of a rule's item not closed yet. Every other replacement of
  an `item.rec` is refused as before.
- Whether a window has come stays its caller's to read, by cadence
  (129): the crate reads no clock (core.1).

## Capabilities

### Modified Capabilities

- `inbox-items`: a rule's item closes once, at its window, with its
  brief.

## Impact

- `crates/weir-signals/src/write.rs`: `close_rule_item`, and the check's
  one allowed replacement of an `item.rec`.
- `crates/weir-signals/tests/closes.rs`: the scenarios.
