pub fn get_database(client: &mongodb::Client) -> mongodb::Database {
    let db_name: String = dotenv::var("DB_NAME").expect("Missing DB_NAME environment variable.");
    client.database(&db_name)
}

pub fn get_collection<T: Send + Sync>(client: &mongodb::Client) -> mongodb::Collection<T> {
    let database: mongodb::Database = get_database(client);
    let coll_name: String =
        dotenv::var("COLL_NAME").expect("Missing COLL_NAME environment variable.");
    database.collection(&coll_name)
}
