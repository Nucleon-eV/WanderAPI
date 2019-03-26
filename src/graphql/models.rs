use diesel::prelude::*;
use juniper::{FieldError, FieldResult};
use rocket_contrib::databases::diesel;

use crate::models::{HikingTrailDB, NewHikingTrailDB, NewPOIDB, PoiDB};

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A hiking trail")]
pub struct HikingTrail {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub pois: Vec<POI>,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A Point of Interest")]
pub struct POI {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub location: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A hiking trail")]
pub struct NewHikingTrail {
    pub name: String,
    pub location: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Point of Interest")]
pub struct NewPOI {
    pub name: String,
    pub hiking_trail: i32,
    pub description: String,
    pub location: String,
}

impl HikingTrail {
    pub fn hiking_trail(conn: &diesel::PgConnection, id_l: i32) -> FieldResult<HikingTrail> {
        use crate::schema::hiking_trails::dsl::*;
        use crate::schema::pois::dsl::pois;
        use crate::schema::pois::columns::hiking_trail;

        let hiking_trail_db = hiking_trails
            .filter(id.eq(id_l))
            .limit(1)
            .first::<HikingTrailDB>(conn)
            .expect("Error loading data");

        let mut pois_list = Vec::new();
        let poi_row_db = pois
            .filter(hiking_trail.eq(id_l))
            .load::<PoiDB>(conn)
            .expect("Error loading data");

        for poi_row in poi_row_db {
            let poi = POI {
                id: poi_row.id,
                name: poi_row.name,
                description: poi_row.description,
                location: poi_row.location,
            };
            pois_list.push(poi);
        }
        let hiking_trail_graphql = HikingTrail { id: hiking_trail_db.id, name: hiking_trail_db.name, location: hiking_trail_db.location, pois: pois_list };
        // Return the result.
        Ok(hiking_trail_graphql)
    }


    pub fn hiking_trails(conn: &diesel::PgConnection) -> FieldResult<Vec<HikingTrail>> {
        use crate::schema::hiking_trails::dsl::*;
        use crate::schema::pois::dsl::pois;
        use crate::schema::pois::columns::hiking_trail;

        let mut hiking_trails_list = Vec::new();

        let hiking_trail_db = hiking_trails
            .load::<HikingTrailDB>(conn)
            .expect("Error loading data");
        if hiking_trail_db.len() == 0 {
            Err(FieldError::new("No data found", graphql_value!({ "internal_warning": "No data found" })))
        } else {
            for hiking_trail_row in hiking_trail_db {
                let mut pois_list = Vec::new();
                let poi_row_db = pois
                    .filter(hiking_trail.eq(hiking_trail_row.id))
                    .load::<PoiDB>(conn)
                    .expect("Error loading data");

                for poi_row in poi_row_db {
                    let poi = POI {
                        id: poi_row.id,
                        name: poi_row.name,
                        description: poi_row.description,
                        location: poi_row.location,
                    };
                    pois_list.push(poi);
                }
                let hiking_trail_graphql = HikingTrail { id: hiking_trail_row.id, name: hiking_trail_row.name, location: hiking_trail_row.location, pois: pois_list };
                hiking_trails_list.push(hiking_trail_graphql);
            }
            // Return the result.
            Ok(hiking_trails_list)
        }
    }
}

impl NewHikingTrail {
    pub fn create_hiking_trail(conn: &diesel::PgConnection, new_hiking_trail: NewHikingTrail) -> FieldResult<HikingTrail> {
        use crate::schema::hiking_trails;
        use crate::schema::pois::dsl::pois;
        use crate::schema::pois::columns::hiking_trail;

        let new_hiking_trail_db = NewHikingTrailDB {
            name: new_hiking_trail.name.as_str(),
            location: new_hiking_trail.location.as_str(),
        };

        let added_hiking_trail_db: HikingTrailDB = diesel::insert_into(hiking_trails::table)
            .values(&new_hiking_trail_db)
            .get_result(conn)
            .expect("Error saving new hiking trail");

        let mut pois_list = Vec::new();
        let poi_row_db = pois
            .filter(hiking_trail.eq(added_hiking_trail_db.id))
            .load::<PoiDB>(conn)
            .expect("Error loading data");

        for poi_row in poi_row_db {
            let poi = POI {
                id: poi_row.id,
                name: poi_row.name,
                description: poi_row.description,
                location: poi_row.location,
            };
            pois_list.push(poi);
        }

        let added_hiking_trail = HikingTrail {
            id: added_hiking_trail_db.id,
            name: added_hiking_trail_db.name,
            location: added_hiking_trail_db.location,
            pois: pois_list,
        };

        Ok(added_hiking_trail)
    }
}

impl NewPOI {
    pub fn create_poi(conn: &diesel::PgConnection, new_poi: NewPOI) -> FieldResult<POI> {
        use crate::schema::pois;

        let new_poi_db = NewPOIDB {
            hiking_trail: &new_poi.hiking_trail,
            name: new_poi.name.as_str(),
            description: new_poi.description.as_str(),
            location: new_poi.location.as_str(),
        };

        let added_poi_db: PoiDB = diesel::insert_into(pois::table)
            .values(&new_poi_db)
            .get_result(conn)
            .expect("Error saving new hiking trail");

        let added_poi = POI {
            id: added_poi_db.id,
            name: added_poi_db.name,
            description: added_poi_db.description,
            location: added_poi_db.location,
        };

        Ok(added_poi)
    }
}