use rocket_contrib::databases::postgres;

#[database("wanderAPI")]
pub struct WanderAPIDbConn(postgres::Connection);