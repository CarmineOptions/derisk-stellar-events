use crate::core::{construct_derisk_event, DeriskEvent, StellarNetwork};
use ingest::{BoundedRange, CaptiveCore, IngestionConfig, Range, SupportedNetwork};
use stellar_xdr::next::{
    LedgerCloseMeta, LedgerCloseMetaV1, LedgerCloseMetaV2, TransactionMeta, TransactionMetaV3,
};

trait SorobanCapableLedger {
    fn get_transaction_meta(self) -> Option<TransactionMetaV3>;
}

impl SorobanCapableLedger for LedgerCloseMetaV1 {
    fn get_transaction_meta(self) -> Option<TransactionMetaV3> {
        if let Some(data) = self.tx_processing.first() {
            match &data.tx_apply_processing {
                // Soroban events only in TransactionMetaV3 ???
                TransactionMeta::V3(meta_v3) => Some(meta_v3.clone()),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl SorobanCapableLedger for LedgerCloseMetaV2 {
    fn get_transaction_meta(self) -> Option<TransactionMetaV3> {
        if let Some(data) = self.tx_processing.first() {
            match &data.tx_apply_processing {
                // Soroban events only in TransactionMetaV3 ???
                TransactionMeta::V3(meta_v3) => Some(meta_v3.clone()),
                _ => None,
            }
        } else {
            None
        }
    }
}

const _INTERESTING_LEDGER_SEQUENCE: u32 = 1100676;
const _FIRST_SOROBAN_EVENT_SEQUENCE: u32 = 5584;

fn extract_events<T: SorobanCapableLedger>(ledger: T, seq: u32) -> Option<Vec<DeriskEvent>> {
    match ledger.get_transaction_meta() {
        Some(meta_v3) => parse_soroban_events(&meta_v3, seq),
        None => None,
    }
}

fn parse_soroban_events(
    transaction_meta: &TransactionMetaV3,
    sequence: u32,
) -> Option<Vec<DeriskEvent>> {
    match &transaction_meta.soroban_meta {
        Some(soroban_meta) if soroban_meta.events.len() > 0 => {
            let derisk_events: Vec<DeriskEvent> = soroban_meta
                .events
                .iter()
                // match against desired contract ids
                .filter(|event| event.contract_id.is_some())
                .enumerate()
                .filter_map(|(index, event)| construct_derisk_event(event, sequence, index))
                .collect();
            Some(derisk_events)
        }
        _ => None,
    }
}

pub fn get_historic_events(network: StellarNetwork, start: u32, end: u32) -> Vec<DeriskEvent> {
    let config_network = match network {
        StellarNetwork::Futurenet => SupportedNetwork::Futurenet,
        StellarNetwork::Testnet => SupportedNetwork::Testnet,
        StellarNetwork::Mainnet => SupportedNetwork::Pubnet,
    };

    let config = IngestionConfig {
        executable_path: "/usr/local/bin/stellar-core".to_string(),
        context_path: Default::default(),
        network: config_network,
        bounded_buffer_size: None,
        staggered: None,
    };

    let mut captive_core = CaptiveCore::new(config);

    let range = Range::Bounded(BoundedRange(start, end));
    captive_core.prepare_ledgers_single_thread(&range).unwrap();

    let mut events_accumulator: Vec<DeriskEvent> = vec![];

    for n in start..end {
        let ledger = captive_core.get_ledger(n);

        if let Err(e) = ledger {
            println!("Failed getting ledger {}, {:#?}", n, e);
            continue;
        }

        let ledger_seq = match ledger.as_ref().unwrap() {
            LedgerCloseMeta::V1(v1) => v1.ledger_header.header.ledger_seq,
            LedgerCloseMeta::V0(v0) => v0.ledger_header.header.ledger_seq,
            LedgerCloseMeta::V2(v2) => v2.ledger_header.header.ledger_seq,
        };

        let events = match ledger.unwrap() {
            LedgerCloseMeta::V0(_) => None,
            LedgerCloseMeta::V1(v1) => extract_events(v1, ledger_seq),
            LedgerCloseMeta::V2(v2) => extract_events(v2, ledger_seq),
        };

        if let Some(events_vec) = events {
            if !events_vec.is_empty() {
                events_accumulator.extend(events_vec);
            }
        }

        if n % 5000 == 0 {
            println!("{}", n);
        }
    }

    events_accumulator
}
