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