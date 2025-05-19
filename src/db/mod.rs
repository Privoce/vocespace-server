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
    let db = Database::connect(
        "sqlite:///Users/shengyifei/projects/livekit/live_end/vocespace.db?mode=rwc",
    )
    .await
    .expect("Failed to connect to database");
    db
}

/// check if license table is exists, if not, create it
pub async fn check_table_or(db: &DatabaseConnection) -> () {
    db.execute(entry::Model::check_table_or_create())
        .await
        .expect("Failed to create table");
}
