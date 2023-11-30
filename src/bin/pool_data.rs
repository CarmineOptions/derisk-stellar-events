use derisk_stellar_events::{core::StellarNetwork, db::DatabaseConnector, pools::get_pool_data};

fn main() {
    if let Ok(pools) = get_pool_data() {
        let mut db_connector = DatabaseConnector::from_env(StellarNetwork::Mainnet);
        db_connector.create_stellarx_pools(&pools);
        println!("Stored StellarX pool data");
    } else {
        println!("Failed storing StellarX pool data");
    }
}
