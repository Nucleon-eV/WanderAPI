use rocket_contrib::databases::diesel;
use rocket_contrib::databases::postgres;

#[database("wanderAPI")]
pub struct WanderAPIDbConn(diesel::PgConnection);