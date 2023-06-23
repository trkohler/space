use graphql_example_service::sea_orm::DatabaseConnection;

pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new(connection_string: &String) -> Self {
        let connection = graphql_example_service::sea_orm::Database::connect(connection_string)
            .await
            .expect("Could not connect to database");

        Database { connection }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}
