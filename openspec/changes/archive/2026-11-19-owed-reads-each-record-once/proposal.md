# owed reads each record once

## Why

flume's settle on deneb asks `owed` over the host's deliveries, 5,088
records holding 971 deliveries, 2,372 hand-overs and 1,727
confirmations. `owed` kept each kind of record in a list and searched
it for every delivery and reader: a delivery's key against every
delivery before it, each reader's confirmation against every
confirmation, its hand-overs against every hand-over, and its place
among the readers owed against every one. Its cost grew with the square
of the records, and a deliverer whose records grow as its deliveries do
pays more for each one it keeps.

## What Changes

- **`owed` reads each record once (111).** It indexes the keys it has
  read, the confirmations by key and reader, the hand-overs by key and
  reader in the order added, the names known and each reader's place in
  its answer, and answers each lookup from them. Its answer is what it
  was, record for record and in the same order (104 to 107, 111; I9).
- **A test holds each delivery's hand-overs in the order added** where a
  deliverer's hand-overs of two deliveries to two readers interleave.

## Impact

- `crates/weir-delivery/src/owed.rs`: `owed`.
- `crates/weir-delivery/tests/owed.rs`:
  `each_deliverys_hand_overs_in_the_order_added`.
