#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;
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
use rocket_contrib::templates::Template;

use database::WanderAPIDbConn;

use crate::graphql::schema::{Mutation, Query};
use crate::routes::Schema;

mod graphql;
mod database;
mod routes;
mod schema;
mod models;

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
    let _args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    rocket::ignite()
        .attach(WanderAPIDbConn::fairing())
        .attach(Template::fairing())
        .manage(Schema::new(
            Query,
            Mutation,
        ))
        .mount("/", routes![
            routes::index,
            routes::get_graphql_handler,
            routes::post_graphql_handler
        ])
        .register(catchers![routes::method_not_allowed])
        .mount("/graphiql", routes![routes::graphiql])
        .launch();
}