use rocket_contrib::databases::diesel;

#[database("wanderAPI")]
pub struct WanderAPIDbConn(diesel::PgConnection);