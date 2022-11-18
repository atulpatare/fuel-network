extern crate alloc;
use fuel_indexer_macros::indexer;

#[indexer(manifest = "./manifest.yaml")]
pub mod my_counter_index_module {
    fn increment_event_handler(event: BlockData) {
        // let IncrementEvent { id, count } = event;
        let increment_entity = IncrementEntity { id: 1, count: 2 };
        increment_entity.save();
    }

    // fn decrement_event_handler(event: DecrementEvent) {
    //     // let DecrementEvent { id, count } = event;
    //     let decrement_entity = DecrementEntity { id: 2, count: 2 };
    //     decrement_entity.save();
    // }
}
