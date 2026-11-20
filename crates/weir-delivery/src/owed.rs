//! `owed` (111): what each reader is owed, read from a store's records and
//! a time alone (104 to 107; I9; core.49, core.50, core.51).

use crate::calls::Refusal;
use crate::record::{Delivery, Handover, Record};
use crate::text::{Key, Reader, Time};
use std::collections::{HashMap, HashSet};

/// What `owed` gives: each reader with something it has not confirmed, and
/// every delivery done (111).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Owed {
    /// Each reader owed a delivery, in the order a delivery was first owed
    /// to it as the records hold them.
    pub readers: Vec<ReaderOwed>,
    /// The key of each delivery every one of whose readers has confirmed
    /// it, in the order recorded (107).
    pub done: Vec<Key>,
}

/// One reader and every delivery it has not confirmed (105).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReaderOwed {
    /// The reader, as its deliverer names it.
    pub reader: Reader,
    /// Whether a `weir-known/1` record names it: whether a reader has ever
    /// confirmed under this name (106; core.50).
    pub known: bool,
    /// Every delivery naming it that it has not confirmed, in the order
    /// recorded.
    pub deliveries: Vec<OwedDelivery>,
}

/// One delivery a reader has not confirmed, owed or expired (104, 105).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwedDelivery {
    /// The delivery, as recorded.
    pub delivery: Delivery,
    /// Whether the time asked at is at or past its `expires_at` (104).
    pub expired: bool,
    /// Each hand-over of it to this reader, in the order added.
    pub handovers: Vec<Handover>,
}

/// `owed` (111): every delivery each reader has not confirmed, owed or
/// expired at `now`, with its hand-overs to that reader, whether the
/// reader's name is known, and every delivery done; or a refusal naming
/// `now` when it is not a time. It reads the deliveries in the order the
/// records hold them, the first under each key alone (101), and leaves out
/// nothing a reader has not confirmed (105; I9). A hand-over or a
/// confirmation naming no delivery of the records is passed over. It reads
/// each record once and looks each delivery's readers, confirmations and
/// hand-overs up by key, so its cost grows with the records, not with
/// their square.
pub fn owed(records: &[Record], now: &str) -> Result<Owed, Refusal> {
    let now = Time::parse(now, "now")?;
    let mut deliveries: Vec<&Delivery> = Vec::new();
    let mut keys: HashSet<&Key> = HashSet::new();
    let mut handovers: HashMap<(&Key, &Reader), Vec<&Handover>> = HashMap::new();
    let mut confirmed: HashSet<(&Key, &Reader)> = HashSet::new();
    let mut known: HashSet<&Reader> = HashSet::new();
    for record in records {
        match record {
            Record::Delivery(delivery) => {
                if keys.insert(delivery.key()) {
                    deliveries.push(delivery);
                }
            }
            Record::Handover(handover) => handovers
                .entry((&handover.key, &handover.reader))
                .or_default()
                .push(handover),
            Record::Confirmation(confirmation) => {
                confirmed.insert((&confirmation.key, &confirmation.reader));
            }
            Record::Known(name) => {
                known.insert(&name.reader);
            }
        }
    }
    let mut readers: Vec<ReaderOwed> = Vec::new();
    let mut place: HashMap<&Reader, usize> = HashMap::new();
    let mut done = Vec::new();
    for delivery in deliveries {
        let mut every_reader_confirmed = true;
        for reader in delivery.readers() {
            if confirmed.contains(&(delivery.key(), reader)) {
                continue;
            }
            every_reader_confirmed = false;
            let owed = OwedDelivery {
                delivery: delivery.clone(),
                expired: now.instant() >= delivery.expires_at().instant(),
                handovers: handovers
                    .get(&(delivery.key(), reader))
                    .map(|handed| handed.iter().map(|handover| (*handover).clone()).collect())
                    .unwrap_or_default(),
            };
            match place.get(reader) {
                Some(&at) => readers[at].deliveries.push(owed),
                None => {
                    place.insert(reader, readers.len());
                    readers.push(ReaderOwed {
                        reader: reader.clone(),
                        known: known.contains(reader),
                        deliveries: vec![owed],
                    });
                }
            }
        }
        if every_reader_confirmed {
            done.push(delivery.key().clone());
        }
    }
    Ok(Owed { readers, done })
}
