CREATE TABLE IF NOT EXISTS fees (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "feeOtZ" INT,
    "feeZtO" INT
    -- PRIMARY KEY (evt_tx_hash, evt_index)
);
CREATE TABLE IF NOT EXISTS swaps (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "evt_address" VARCHAR(40),
    "amount0" DECIMAL,
    "amount1" DECIMAL,
    "liquidity" DECIMAL,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    "price" DECIMAL,
    "tick" INT
    -- PRIMARY KEY (evt_tx_hash, evt_index)
);
