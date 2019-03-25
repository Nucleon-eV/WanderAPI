#![feature(decl_macro, proc_macro_hygiene)]

extern crate docopt;
extern crate env_logger;
#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;

use database::WanderAPIDbConn;

use crate::graphql::schema::{Mutation, Query};
use crate::routes::Schema;

mod graphql;
mod database;
mod routes;

const USAGE: &'static str = "
Wander API

Usage:
  WanderAPI
  WanderAPI (-h | --help)

Options:
  -h --help     Show this screen.
";

#[derive(Debug, Deserialize)]
struct Args {}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    /*let conn = Connection::connect(postgres_url.clone(), TlsMode::None).unwrap();
    conn.batch_execute("
                CREATE TABLE IF NOT EXISTS hiking_trails (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    location        VARCHAR NOT NULL
                );

                CREATE TABLE IF NOT EXISTS pois (
                    id              SERIAL PRIMARY KEY,
                    hiking_trail    SERIAL,
                    name            VARCHAR NOT NULL,
                    description     TEXT,
                    location        VARCHAR NOT NULL
                );
                ").unwrap();*/

    rocket::ignite()
        .attach(WanderAPIDbConn::fairing())
        .manage(Schema::new(
            Query,
            Mutation,
        ))
        .mount("/", routes![
            routes::index,
            routes::get_graphql_handler,
            routes::post_graphql_handler
        ])
        .mount("/graphiql", routes![routes::graphiql])
        .launch();
}