extern crate alloc;
use fuel_indexer_macros::indexer;

#[indexer(manifest = "/home/atul/Projects/fuel-project/counter-indexer/manifest.yaml")]
pub mod my_counter_index_module {
    fn increment_event_handler(event: IncrementEvent) {
        let IncrementEvent { id, count } = event;
        let increment_entity = IncrementEntity { id, count: count.try_into().unwrap() };
        increment_entity.save();
    }

    fn decrement_event_handler(event: DecrementEvent) {
        let DecrementEvent { id, count } = event;
        let decrement_entity = DecrementEntity { id, count: count.try_into().unwrap() };
        decrement_entity.save();
    }
}
