mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

const TRACKED_CONTRACT: [u8; 20] = hex!("B1026b8e7276e7AC75410F1fcbbe21796e8f7526");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    Ok(contract::Events {
        swaps: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Swap::match_and_decode(log) {
                            return Some(contract::Swap {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                evt_address: Hex(&log.address).to_string(),
                                amount0: event.amount0.to_string(),
                                amount1: event.amount1.to_string(),
                                liquidity: event.liquidity.to_string(),
                                recipient: event.recipient,
                                sender: event.sender,
                                price: event.price.to_string(),
                                tick: Into::<num_bigint::BigInt>::into(event.tick).to_i64().unwrap(),
                            });
                        }

                        None
                })
            })
            .collect(),
        fees: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Fee::match_and_decode(log) {
                            return Some(contract::Fee {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                evt_address: Hex(&log.address).to_string(),
                                fee_zt_o: Into::<num_bigint::BigInt>::into(event.fee_otz).to_u32().unwrap(),
                                fee_ot_z: Into::<num_bigint::BigInt>::into(event.fee_zto).to_u32().unwrap(),
                                
                            });
                        }

                        None
                })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = DatabaseChangeTables::new();
    events.swaps.into_iter().for_each(|evt| {
        tables
            .create_row("swaps", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", evt.evt_address)
            .set("amount0", BigDecimal::from_str(&evt.amount0).unwrap())
            .set("amount1", BigDecimal::from_str(&evt.amount1).unwrap())
            .set("liquidity", BigDecimal::from_str(&evt.liquidity).unwrap())
            .set("recipient", Hex(&evt.recipient).to_string())
            .set("sender", Hex(&evt.sender).to_string())
            .set("price", BigDecimal::from_str(&evt.price).unwrap())
            .set("tick", evt.tick);
    });
    events.fees.into_iter().for_each(|evt| {
        tables
            .create_row("fees", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", evt.evt_address)
            .set("feeOtZ", evt.fee_ot_z)
            .set("feeZtO", evt.fee_zt_o);   
    });

    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = EntityChangesTables::new();

    // Loop over all the abis events to create changes
    events.swaps.into_iter().for_each(|evt| {
        tables
            .create_row("swaps", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", evt.evt_address)
            .set("amount_0", BigDecimal::from_str(&evt.amount0).unwrap())
            .set("amount_1", BigDecimal::from_str(&evt.amount1).unwrap())
            .set("liquidity", BigDecimal::from_str(&evt.liquidity).unwrap())
            .set("recipient", Hex(&evt.recipient).to_string())
            .set("sender", Hex(&evt.sender).to_string())
            .set("sqrt_price_x_96", BigDecimal::from_str(&evt.price).unwrap())
            .set("tick", evt.tick);
    });
    events.fees.into_iter().for_each(|evt| {
        tables
            .create_row("fees", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("evt_address", evt.evt_address)
            .set("feeOtZ", evt.fee_ot_z)
            .set("feeZtO", evt.fee_zt_o);   
    });

    Ok(tables.to_entity_changes())
}
