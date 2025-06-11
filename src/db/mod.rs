use std::env::current_exe;

use sea_orm::{ConnectionTrait, Database, DatabaseConnection};

use crate::entry;

/// connect to sqlite database
pub async fn init() -> DatabaseConnection {
    let db = connect().await;
    // do check
    check_table_or(&db).await;
    db
}

pub async fn connect() -> DatabaseConnection {
    let exe_path = current_exe().expect("Failed to get current executable path");
    let path = exe_path.parent().expect("Failed to get parent directory");
    let db = Database::connect(format!("sqlite://{}/vocespace.db?mode=rwc", path.display()))
        .await
        .expect("Failed to connect to database");
    db
}

/// check if license table is exists, if not, create it
pub async fn check_table_or(db: &DatabaseConnection) -> () {
    db.execute(entry::license::Model::check_table_or_create())
        .await
        .expect("Failed to create table");

    db.execute(entry::rooms::Model::check_table_or_create())
        .await
        .expect("Failed to create table");
}
