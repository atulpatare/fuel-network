
#![allow(unused)]
fn main() {
extern crate alloc;
use fuel_indexer_macros::indexer;

#[indexer(manifest = "/home/atul/Projects/fuel-project/indexer/counter-index.manifest.yml")]
pub mod my_counter_index_module {

    fn counter_module_handler_one(event: Count) {
        let Count { count, id } = event;

        let count_entity = match CountEntity::load(id) {
            Some(o) => o,
            None => CountEntity { id, count },
        };

        count_entity.save();

        let CountEntity { id, count } = count_entity;

        let adjusted_count_entity = AdjustedCountEntity{ 
            id, 
            count: count_entity.id, 
            adjusted_count: count + 1
        };

        adjusted_count_entity.save();
    }
}

}
