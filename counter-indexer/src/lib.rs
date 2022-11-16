extern crate alloc;
use fuel_indexer_macros::indexer;

#[indexer(manifest = "./manifest.yaml")]
pub mod my_counter_index_module {

    fn counter_module_handler_one(_event: ValueUpdated) {
        // todo: add entity manipulations
    }
}
