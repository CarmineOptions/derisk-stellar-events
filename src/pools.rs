use reqwest::blocking;

use crate::core::{DatabaseStellarXPool, PoolResponse, StellarXPool};

fn map_pool_to_database_pool(pool: StellarXPool) -> DatabaseStellarXPool {
    let StellarXPool {
        id,
        type_field,
        fee_bp,
        reserves,
        paging_token,
        total_shares,
        total_trustlines,
        last_modified_time,
        last_modified_ledger,
        liquidity,
        volume_24h,
        volume_7d,
        fee_24h,
        fee_1y,
        fee_aqua,
        assets_order,
        asset_1_domain,
        asset_2_domain,
    } = pool;

    let asset_1 = match reserves.get(0) {
        Some(v) => serde_json::to_value(v).ok(),
        None => None,
    };

    let asset_2 = match reserves.get(1) {
        Some(v) => serde_json::to_value(v).ok(),
        None => None,
    };

    DatabaseStellarXPool {
        id,
        type_field,
        fee_bp,
        paging_token,
        total_shares,
        total_trustlines,
        last_modified_time,
        last_modified_ledger,
        liquidity,
        volume_24h,
        volume_7d,
        fee_24h,
        fee_1y,
        fee_aqua,
        assets_order,
        asset_1_domain,
        asset_2_domain,
        asset_1,
        asset_2,
    }
}

pub fn get_pool_data() -> Result<Vec<DatabaseStellarXPool>, reqwest::Error> {
    // TODO: better URL handling
    let response = blocking::get("https://amm-api.stellarx.com/api/pools/?format=json&limit=50&order=desc&orderField=liquidity")?;

    let parsed: PoolResponse = response.json()?;

    let database_pools: Vec<DatabaseStellarXPool> = parsed
        .pools
        .into_iter()
        .map(map_pool_to_database_pool)
        .collect();

    Ok(database_pools)
}
