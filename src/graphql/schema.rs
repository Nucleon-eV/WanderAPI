use juniper::{FieldError, FieldResult};

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
            // Get the context from the executor.
            let context = executor.context();
            // Get a db connection.
            let connection = &context.connection;
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
            let connection = &context.connection;

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

pub struct Mutation;

juniper::graphql_object!(Mutation: Context |&self| {

        field createHikingTrail(&executor, new_hiking_trail: NewHikingTrail) -> FieldResult<HikingTrail> {
            let hiking_trail_db = executor.context().connection.query("INSERT INTO hiking_trails (name, location) VALUES ($1, $2) RETURNING id, name, location", &[&new_hiking_trail.name, &new_hiking_trail.location])?;
            if hiking_trail_db.len() == 0 {
                Err(FieldError::new("No data found", graphql_value!({ "internal_warning": "No data found" })))
            } else {
                let first_result = &hiking_trail_db.get(0);
                let id: i32 = first_result.get(0);
                let mut pois = Vec::new();
                for poi_row in &executor.context().connection.query("SELECT id, name, description, location FROM pois WHERE hiking_trail = $1", &[&id]).unwrap() {
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

        field createPOI(&executor, new_poi: NewPOI) -> FieldResult<POI> {
            let poi_db = executor.context().connection.query("INSERT INTO pois (hiking_trail, name, description, location) VALUES ($1, $2, $3, $4) RETURNING id, name, description, location", &[&new_poi.hiking_trail, &new_poi.name, &new_poi.description, &new_poi.location])?;
            if poi_db.len() == 0 {
                Err(FieldError::new("No data found", graphql_value!({ "internal_warning": "No data found" })))
            } else {
                let first_result = &poi_db.get(0);
                let poi = POI {
                    id: first_result.get(0),
                    name: first_result.get(1),
                    description: first_result.get(2),
                    location: first_result.get(3),
                };
                Ok(poi)
            }
        }
    });