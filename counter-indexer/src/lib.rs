extern crate alloc;
use fuel_indexer_macros::indexer;

#[indexer(manifest = "manifest.yaml")]
pub mod my_counter_index_module {
    fn increment_event_handler(event: IncrementEvent) {
        let IncrementEvent { id, count } = event;
        let increment_entity = match Increment::load(id) {
            Some(mut e) => {
                e.count = count.try_into().unwrap();
                e
            },
            None => Increment {
                id,
                count: count.try_into().unwrap()
            }
        };
        increment_entity.save();
        save_count_entity(count);
    }

    fn decrement_event_handler(event: DecrementEvent) {
        let DecrementEvent { id, count } = event;
        let decrement_entity = match Decrement::load(id) {
            Some(mut e) => {
                e.count = count.try_into().unwrap();
                e
            },
            None => Decrement {
                id,
                count: count.try_into().unwrap()
            }
        };
        decrement_entity.save();
        save_count_entity(count);
    }

    fn save_count_entity(count: u64) {
        let count_entity = match Count::load(1) {
            Some(mut e) => {
                e.count = count.try_into().unwrap();
                e
            }
            None => Count {
                id: 1,
                count: count.try_into().unwrap()
            }
        };
        count_entity.save();
    }
}
