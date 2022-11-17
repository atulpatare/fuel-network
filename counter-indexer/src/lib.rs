extern crate alloc;
use fuel_indexer_macros::indexer;

#[indexer(manifest = "./manifest.yaml")]
pub mod my_counter_index_module {

    fn counter_module_handler_one(event: ValueUpdated) {
        Logger::info("In the indexer function");
        // let ValueUpdated { counter } = event;
        println!("running indexer for event with counter");

        // let count_entity = match CountEntity::load(counter) {
        //     Some(o) => o,
        //     None => CountEntity { id: counter, count: counter },
        // };

        // count_entity.save();

        // let CountEntity { id, count } = count_entity;

        // let adjusted_count_entity = AdjustedCountEntity { 
        //     id, 
        //     count: count_entity.id, 
        //     adjusted_count: count + 1
        // };

        // adjusted_count_entity.save();
    }
}
