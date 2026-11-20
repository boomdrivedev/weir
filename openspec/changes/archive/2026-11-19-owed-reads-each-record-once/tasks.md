# Tasks

## 1. owed reads each record once

- [x] 1.1 In `crates/weir-delivery/src/owed.rs`, have `owed` index the keys read, the confirmations and the hand-overs by key and reader, the names known and each reader's place in its answer, and answer from them, its answer unchanged (104 to 107, 111; I9). Check: `devenv shell -- cargo test -p weir-delivery --test owed`.
- [x] 1.2 In `crates/weir-delivery/tests/owed.rs`, add `each_deliverys_hand_overs_in_the_order_added`, the delivery-calls spec's "Hand-overs of two deliveries interleaved". Check: `devenv shell -- cargo test -p weir-delivery --test owed each_deliverys_hand_overs_in_the_order_added`.
