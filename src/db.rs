use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

use crate::core::{DeriskEvent, StellarNetwork};

const BATCH_SIZE: usize = 500;
pub struct DatabaseConnector {
    pub network: StellarNetwork,
    bastch_size: usize,
    connection: PgConnection,
}

impl DatabaseConnector {
    pub fn from_env(network: StellarNetwork) -> Self {
        let username = env::var("DB_USER").expect("Could not read \"DB_USER\"");
        let password = env::var("DB_PASSWORD").expect("Could not read \"DB_PASSWORD\"");
        let host = env::var("DB_HOST").expect("Could not read \"DB_HOST\"");
        let port = env::var("DB_PORT").expect("Could not read \"DB_PORT\"");

        DatabaseConnector::new(username, password, host, port, network)
    }
    pub fn new(
        username: String,
        password: String,
        host: String,
        port: String,
        network: StellarNetwork,
    ) -> Self {
        let db_name = match network {
            StellarNetwork::Futurenet => "stellar-futurenet",
            StellarNetwork::Testnet => {
                panic!("Testnet not implemented yet")
            }
            StellarNetwork::Mainnet => {
                panic!("Mainnet not implemented yet")
            }
        };
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            username, password, host, port, db_name
        );
        let connection = PgConnection::establish(&connection_string)
            .unwrap_or_else(|_| panic!("Error connecting to {}", connection_string));

        Self {
            network,
            bastch_size: BATCH_SIZE,
            connection,
        }
    }

    pub fn create_batch_of_events(&mut self, new_events: &Vec<DeriskEvent>) {
        use crate::schema::events::dsl::*;

        let chunks = new_events.chunks(self.bastch_size);

        for chunk in chunks {
            diesel::insert_into(events)
                .values(chunk)
                .on_conflict_do_nothing()
                .execute(&mut self.connection)
                .expect("Error saving batch of events");
        }
    }
}

fn get_db_url() -> String {
    let username = env::var("DB_USER").expect("Could not read \"DB_USER\"");
    let password = env::var("DB_PASSWORD").expect("Could not read \"DB_PASSWORD\"");
    let host = env::var("DB_HOST").expect("Could not read \"DB_HOST\"");

    format!(
        "postgres://{}:{}@{}/stellar-futurenet",
        username, password, host
    )
}

fn establish_connection() -> PgConnection {
    let database_url = get_db_url();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_batch_of_events(new_events: &Vec<DeriskEvent>) {
    use crate::schema::events::dsl::*;

    let mut connection = establish_connection();

    let chunks = new_events.chunks(BATCH_SIZE);

    for chunk in chunks {
        diesel::insert_into(events)
            .values(chunk)
            .on_conflict_do_nothing()
            .execute(&mut connection)
            .expect("Error saving batch of events");
    }
}
