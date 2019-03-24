extern crate env_logger;
extern crate juniper_warp;
#[macro_use]
extern crate log as irrelevant_log;
#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
extern crate warp;

use std::env;

use juniper::FieldResult;
use postgres::{Connection, TlsMode};
use warp::{Filter, http::Response, log};

#[derive(juniper::GraphQLObject, FromSql, Debug)]
#[graphql(description = "A hiking trail")]
struct HikingTrail {
    id: String,
    name: String,
    location: String,
}

#[derive(juniper::GraphQLInputObject, ToSql, Debug)]
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

        // Arguments to resolvers can either be simple types or input objects.
        // The executor is a special (optional) argument that allows accessing the context.
        field hiking_trail(&executor, id: String) -> FieldResult<HikingTrail> {
            // Get the context from the executor.
            let context = executor.context();
            // Get a db connection.
            let connection = &context.db;
            // Execute a db query.
            // Note the use of `?` to propagate errors.
            let hiking_trail_db = &connection.query("SELECT id, name, location FROM hiking_trails WHERE id = $1", &[&id])?;
            let first_result = &hiking_trail_db.get(0);
            let hiking_trail = HikingTrail {id: first_result.get(0), name: first_result.get(1), location: first_result.get(2)};
            // Return the result.
            Ok(hiking_trail)
        }
    });

struct Mutation;

juniper::graphql_object!(Mutation: Context |&self| {

        field createHikingTrail(&executor, new_hiking_trail: NewHikingTrail) -> FieldResult<HikingTrail> {
            let rows_updated = executor.context().db.execute("INSERT INTO hiking_trails (name, location) VALUES ($1, $2)", &[&new_hiking_trail.name, &new_hiking_trail.location])?;
            let hiking_trail_db = &executor.context().db.query("SELECT id, name, location FROM hiking_trails WHERE ROWNUM = $1", &[&rows_updated.to_string()])?;
            let first_result = &hiking_trail_db.get(0);
            let hiking_trail = HikingTrail {id: first_result.get(0), name: new_hiking_trail.name, location: new_hiking_trail.location};
            Ok(hiking_trail)
        }
    });

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, Mutation>;

fn schema() -> Schema {
    return juniper::RootNode::new(Query, Mutation);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    ::std::env::set_var("RUST_LOG", "WanderAPI");
    env_logger::init();

    let conn = Connection::connect(&*args[1], TlsMode::None).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS hiking_trails (
                    id              VARCHAR PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    location        VARCHAR NOT NULL
                  )", &[]).unwrap();

    //let log = log("WanderAPI");
    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    info!("Listening on 127.0.0.1:[YOUR_PORT]");

    let state = warp::any().map(move || Context { db: Connection::connect("postgres://postgres@localhost:5433", TlsMode::None).unwrap() });
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());


    warp::serve(
        warp::get2()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql"))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            //.with(log),
    )
        .run(([127, 0, 0, 1], args[2].parse::<u16>().unwrap()));
}