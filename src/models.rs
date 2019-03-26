use crate::schema::{hiking_trails, pois};

#[derive(Queryable)]
pub struct HikingTrailDB {
    pub id: i32,
    pub name: String,
    pub location: String,
}

#[derive(Queryable)]
pub struct PoiDB {
    pub id: i32,
    pub hiking_trail: i32,
    pub name: String,
    pub description: String,
    pub location: String,
}

#[derive(Insertable)]
#[table_name = "hiking_trails"]
pub struct NewHikingTrailDB<'a> {
    pub name: &'a str,
    pub location: &'a str,
}

#[derive(Insertable)]
#[table_name = "pois"]
pub struct NewPOIDB<'a> {
    pub hiking_trail: &'a i32,
    pub name: &'a str,
    pub description: &'a str,
    pub location: &'a str,
}