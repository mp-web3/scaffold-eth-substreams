mod abi;
mod pb;
mod rpc;
use crate::abi::contract::events::Transfer as TransferEvent;
use crate::rpc::TokenMeta;
use pb::contract::v1::{Transfer, Transfers};
use substreams::store::{StoreAdd, StoreAddInt64, StoreNew}; // TODO! 1. (Checkpoint 3.3) import the correct store type and trait
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

substreams_ethereum::init!(); // Macro that initializes Substreams for Ethereum.


// This Rust code is a Substreams module that processes Ethereum blocks to extract specific token transfer events.
// Specifically, it looks for token transfers where the token's name contains the word "Ape". 
// It collects these transfers into a custom protobuf message called Transfers, 
// which contains a list of Transfer messages.
#[substreams::handlers::map]
// Defines a function named map_apes that takes an Ethereum block as input.
fn map_apes(blk: eth::Block) -> 
    // The function returns a Result type that, on success, contains a Transfers message, or an error on failure.
    Result<Transfers, substreams::errors::Error> {
    // Iterate through the block logs, filter and map them to our `Transfer` protobuf
    let transfers = blk.logs().filter_map(|log| {
        // Check if the log matches the `TransferEvent`
        if TransferEvent::match_log(&log.log) { // Checks if the current log matches the ERC-20
            // Create token metadata using the log address
            let token_meta = TokenMeta::new(&log.log.address);
            
            // If the token name contains "Ape", map it to a `Transfer` message
            if token_meta.name.contains("Ape") {
                Some(Transfer {
                    address: Hex::encode(log.log.address.clone()), // Encode address as hex
                    name: token_meta.name.clone(), // Copy token name
                    symbol:token_meta.symbol.clone(), // Copy token symbol
                })
            } else {
                None
            }
        } else {
            None
        }
    }).collect::<Vec<Transfer>>(); // Collect the results into a vector

    // Return the `Transfers` message
    Ok(Transfers {transfers})

}

// #[substreams::handlers::store]
// fn store_transfer_volume(transfers: Transfers) {
//     todo!("1. add the correct store as the second function argument");

//     todo!("2. iterate over the transfers");

//     todo!("3. use the `.add()` method on your store to increment the value by 1");
// }

// #[substreams::handlers::map]
// fn graph_out() -> Result<EntityChanges, substreams::errors::Error> {
//     // Initializing EntityChanges container
//     let mut tables = EntityChangesTables::new();

//     todo!("1. scroll to top of file to add the imports");

//     todo!("2. pass in the store as the first function argument");

//     todo!("3. pass in the second function argument");

//     todo!("4. iterate over the transfers");

//     todo!("5. get the volume from the store");

//     todo!("6. create EntityChanges");

//     // returning EntityChanges
//     Ok(tables.to_entity_changes())
// }
