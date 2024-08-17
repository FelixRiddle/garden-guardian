use sea_orm::DatabaseConnection;

pub async fn establish_connection() -> DatabaseConnection {
    let db_url = "mysql://username:password@localhost/database_name";
    sea_orm::Database::connect(db_url).await.expect("Failed to connect to database")
}
