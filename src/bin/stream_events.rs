use derisk_stellar_events::{
    core::StellarNetwork, db::DatabaseConnector, event_streaming::stream_events,
};

fn main() {
    stream_events(DatabaseConnector::from_env(StellarNetwork::Futurenet));
}
