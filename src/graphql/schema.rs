use juniper::FieldResult;

use crate::database::WanderAPIDbConn;
use crate::graphql::models::*;

pub struct Context {
    pub connection: WanderAPIDbConn
}

impl juniper::Context for Context {}

pub struct Query;

juniper::graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    field hiking_trail(&executor, id: i32) -> FieldResult<HikingTrail> {
        let context = executor.context();
        HikingTrail::hiking_trail(&context.connection, id)
    }

    field hiking_trails(&executor) -> FieldResult<Vec<HikingTrail>> {
        let context = executor.context();
        HikingTrail::hiking_trails(&context.connection)
    }
});

pub struct Mutation;

juniper::graphql_object!(Mutation: Context |&self| {

    field createHikingTrail(&executor, new_hiking_trail: NewHikingTrail) -> FieldResult<HikingTrail> {
        let context = executor.context();
        NewHikingTrail::create_hiking_trail(&context.connection, new_hiking_trail)
    }

    field createPOI(&executor, new_poi: NewPOI) -> FieldResult<POI> {
        let context = executor.context();
        NewPOI::create_poi(&context.connection, new_poi)
    }
});