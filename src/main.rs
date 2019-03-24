extern crate docopt;
extern crate env_logger;
#[macro_use]
extern crate juniper;
extern crate juniper_warp;
#[macro_use]
extern crate log as irrelevant_log;
extern crate postgres;
#[macro_use]
extern crate serde_derive;
extern crate warp;

use docopt::Docopt;
use juniper::{FieldError, FieldResult};
use postgres::{Connection, TlsMode};
use warp::{Filter, http::Response, log};

const USAGE: &'static str = "
Wander API

Usage:
  WanderAPI <postgresurl> <port>
  WanderAPI (-h | --help)

Options:
  -h --help     Show this screen.
";


#[derive(Debug, Deserialize)]
struct Args {
    arg_postgresurl: String,
    arg_port: u16,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A hiking trail")]
struct HikingTrail {
    id: i32,
    name: String,
    location: String,
    pois: Vec<POI>,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A Point of Interest")]
struct POI {
    id: i32,
    name: String,
    description: String,
    location: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A hiking trail")]
struct NewHikingTrail {
    name: String,
    location: String,
}

struct Context {
    db: Connection, // TODO wrap with helper
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

struct Query;

juniper::graphql_object!(Query: Context |&self| {

        field apiVersion() -> &str {
            "1.0"
        }

        field hiking_trail(&executor, id: i32) -> FieldResult<HikingTrail> {
            // Get the context from the executor.
            let context = executor.context();
            // Get a db connection.
            let connection = &context.db;
            // Execute a db query.
            // Note the use of `?` to propagate errors.
            let hiking_trail_db = &connection.query("SELECT id, name, location FROM hiking_trails WHERE id = $1", &[&id])?;
            if hiking_trail_db.len() == 0 {
                Err(FieldError::new("No data found", graphql_value!({ "internal_warning": "No data found" })))
            } else {
                let first_result = &hiking_trail_db.get(0);
                let mut pois = Vec::new();
                for poi_row in &connection.query("SELECT id, name, description, location FROM pois WHERE hiking_trail = $1", &[&id]).unwrap() {
                    let poi = POI {
                        id: poi_row.get(0),
                        name: poi_row.get(1),
                        description: poi_row.get(2),
                        location: poi_row.get(3),
                    };
                    pois.push(poi);
                }
                let hiking_trail = HikingTrail {id: first_result.get(0), name: first_result.get(1), location: first_result.get(2), pois: pois};
                // Return the result.
                Ok(hiking_trail)
            }
        }

        field hiking_trails(&executor) -> FieldResult<Vec<HikingTrail>> {
            // Get the context from the executor.
            let context = executor.context();
            // Get a db connection.
            let connection = &context.db;

            let mut hiking_trails = Vec::new();
            for trail in &connection.query("SELECT id, name, location FROM hiking_trails", &[]).unwrap() {
                let mut pois = Vec::new();
                let id: i32 = trail.get(0);
                for poi_row in &connection.query("SELECT id, name, description, location FROM pois WHERE hiking_trail = $1", &[&id]).unwrap() {
                    let poi = POI {
                        id: poi_row.get(0),
                        name: poi_row.get(1),
                        description: poi_row.get(2),
                        location: poi_row.get(3),
                    };
                    pois.push(poi);
                }
                let hiking_trail = HikingTrail {
                    id: id,
                    name: trail.get(1),
                    location: trail.get(2),
                    pois: pois,
                };
                hiking_trails.push(hiking_trail);
            }
            if hiking_trails.len() == 0 {
                Err(FieldError::new("No data found", graphql_value!({ "internal_warning": "No data found" })))
            } else {
                // Return the result.
                Ok(hiking_trails)
            }
        }
    });

struct Mutation;

juniper::graphql_object!(Mutation: Context |&self| {

        field createHikingTrail(&executor, new_hiking_trail: NewHikingTrail) -> FieldResult<HikingTrail> {
            let hiking_trail_db = executor.context().db.query("INSERT INTO hiking_trails (name, location) VALUES ($1, $2) RETURNING id, name, location", &[&new_hiking_trail.name, &new_hiking_trail.location])?;
            if hiking_trail_db.len() == 0 {
                Err(FieldError::new("No data found", graphql_value!({ "internal_warning": "No data found" })))
            } else {
                let first_result = &hiking_trail_db.get(0);
                let id: i32 = first_result.get(0);
                let mut pois = Vec::new();
                for poi_row in &executor.context().db.query("SELECT id, name, description, location FROM pois WHERE hiking_trail = $1", &[&id]).unwrap() {
                    let poi = POI {
                        id: poi_row.get(0),
                        name: poi_row.get(1),
                        description: poi_row.get(2),
                        location: poi_row.get(3),
                    };
                    pois.push(poi);
                }
                let hiking_trail = HikingTrail {id: id, name: first_result.get(1), location: first_result.get(2), pois: pois};
                Ok(hiking_trail)
            }
        }
    });

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, Mutation>;

fn schema() -> Schema {
    return juniper::RootNode::new(Query, Mutation);
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    let postgres_url = args.arg_postgresurl;
    let port = args.arg_port;

    ::std::env::set_var("RUST_LOG", "WanderAPI");
    env_logger::init();

    let conn = Connection::connect(postgres_url.clone(), TlsMode::None).unwrap();
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
                ").unwrap();

    let log = log("WanderAPI");
    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    info!("Listening on 127.0.0.1:[YOUR_PORT]");

    let state = warp::any().map(move || Context { db: Connection::connect(postgres_url.clone(), TlsMode::None).unwrap() });
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());


    warp::serve(
        warp::get2()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql"))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
        .run(([0, 0, 0, 0], port));
}