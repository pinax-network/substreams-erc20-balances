use erc20_balances::pb::erc20::types::v1::Events;
use substreams::errors::Error;
use substreams_entity_change::pb::entity::EntityChanges;

#[substreams::handlers::map]
pub fn graph_out(events: Events) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    for balance_change in events.balance_changes {
        tables
            .create_row(
                "BalanceChange",
                format!("{}:{}", &balance_change.block_num, &balance_change.storage_ordinal),
            )
            // -- block --
            .set_bigint("block_num", &balance_change.block_num.to_string())
            .set("block_hash", &balance_change.block_hash)
            // Timestamp support has been added in v0.36.0
            // https://github.com/graphprotocol/graph-node/releases/tag/v0.36.0
            .set("timestamp", balance_change.timestamp.expect("missing timestamp"))
            .set("date", &balance_change.date)

            // -- transaction --
            .set("transaction_id", &balance_change.transaction_id)

            // -- balance change --
            .set("contract", &balance_change.contract)
            .set("owner", &balance_change.owner)
            .set_bigint("old_balance", &balance_change.old_balance)
            .set_bigint("new_balance", &balance_change.new_balance);

        tables
            .create_row("Balance", format!("{}:{}", balance_change.contract, balance_change.owner))
            // -- block --
            .set_bigint("block_num", &balance_change.block_num.to_string())
            .set("block_hash", &balance_change.block_hash)
            .set("timestamp", balance_change.timestamp.expect("missing timestamp"))
            .set("date", balance_change.date)
            // -- current balance --
            .set("contract", balance_change.contract)
            .set("owner", balance_change.owner)
            .set_bigint("balance", &balance_change.new_balance);
    }

    for transfer in events.transfers {
        tables
            .create_row("Transfer", format!("{}:{}", transfer.block_num, transfer.log_block_index))
            // -- block --
            .set_bigint("block_num", &transfer.block_num.to_string())
            .set("block_hash", transfer.block_hash)
            .set("timestamp", transfer.timestamp.expect("missing timestamp"))
            .set("date", transfer.date)
            // -- transaction --
            .set("transaction_id", transfer.transaction_id)
            // -- transfer --
            .set("contract", transfer.contract)
            .set("from", transfer.from)
            .set("to", transfer.to)
            .set_bigint("value", &transfer.value);
    }

    Ok(tables.to_entity_changes())
}
