use ingest::{CaptiveCore, IngestionConfig, LedgerCloseMetaReader, SupportedNetwork};

use crate::{
    core::{construct_derisk_event, DeriskEvent},
    db::DatabaseConnector,
};

pub fn stream_events(mut db_connector: DatabaseConnector) {
    let config = IngestionConfig {
        executable_path: "/usr/local/bin/stellar-core".to_string(),
        context_path: Default::default(),
        network: SupportedNetwork::Futurenet,
        bounded_buffer_size: None,
        staggered: None,
    };

    let mut captive_core = CaptiveCore::new(config);

    let receiver = captive_core.start_online_no_range().unwrap();

    let mut event_count = 0;
    let mut loop_count = 0;
    let loop_count_reset = 100;

    println!("Capturing events...");
    for result in receiver.iter() {
        let ledger_sequence = LedgerCloseMetaReader::ledegr_sequence(&result).unwrap();
        let events: Vec<DeriskEvent> = LedgerCloseMetaReader::soroban_events(&result)
            .expect("Failed getting events from meta")
            .iter()
            .filter(|event| event.contract_id.is_some())
            .enumerate()
            .filter_map(|(index, event)| construct_derisk_event(event, ledger_sequence, index))
            .collect();

        event_count += events.len();
        loop_count += 1;

        db_connector.create_batch_of_events(&events);

        // report found events
        if loop_count >= loop_count_reset {
            println!("{} events in the last {} ledgers", event_count, loop_count);
            event_count = 0;
            loop_count = 0;
        }
    }
}
