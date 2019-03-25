use juniper::RootNode;
use rocket::{get, post};
use rocket::response::content;
use rocket::State;

use crate::database::WanderAPIDbConn;
use crate::graphql::schema::{Context, Mutation, Query};

pub type Schema = RootNode<'static, Query, Mutation>;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?query")]
pub fn get_graphql_handler(
    context: WanderAPIDbConn,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context { connection: context })
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: WanderAPIDbConn,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context { connection: context })
}