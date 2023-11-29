use std::env;

use derisk_stellar_events::{
    core::StellarNetwork, db::DatabaseConnector, historic_events::get_historic_events,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <start_ledger> <end_ledger>", args[0]);
        std::process::exit(1);
    }

    let start: u32 = args[1].parse().expect("Failed to parse argument 1");
    let end: u32 = args[2].parse().expect("Failed to parse argument 2");

    if start > end {
        eprintln!(
            "\"start_ledger\" cannot be greater than \"end_ledger\": start: {}, end: {}",
            start, end
        );
        std::process::exit(1);
    }

    let events = get_historic_events(start, end);

    let mut db_connector = DatabaseConnector::from_env(StellarNetwork::Futurenet);

    db_connector.create_batch_of_events(&events);

    println!("Stored historic events from ledger {} to {}", start, end);
}
