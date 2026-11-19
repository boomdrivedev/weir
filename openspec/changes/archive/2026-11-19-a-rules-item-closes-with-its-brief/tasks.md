# Tasks

## 1. The close

- [x] 1.1 Add `InboxAt::close_rule_item` to `crates/weir-signals/src/write.rs`, and let `InboxAt::check` take a replacement of an `item.rec` that adds `closed_at` alone to a rule's item not closed yet (36, 128, 129). Add `crates/weir-signals/tests/closes.rs` with `a_rules_item_closes_with_its_brief`, `a_closed_items_rule_makes_its_next_item`, `an_item_closes_once`, `an_item_no_rule_made_does_not_close`, `a_close_at_no_time` and `an_item_record_changed_otherwise_is_refused`. True when `devenv shell -- moon run signals:test -- --test closes` passes them, holding the inbox-items scenarios of those names.
