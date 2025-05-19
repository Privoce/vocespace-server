use std::fmt::Display;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::Utc;
use rand::{rng, RngCore};
use sea_orm::{prelude::StringLen, *};
use sha2::{Digest, Sha256};
/// # Table for license
/// | key        | type        |
/// | ---------- | ----------- |
/// | id         | i32         |
/// | email      | String      |
/// | domains    | String      |
/// | created_at | Timestamp   |
/// | expires_at | Timestamp   |
/// | value      | String      |
/// | limit      | Limit       | (String in database)
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "license")]
pub struct Model {
    /// primary key (use uuid)
    #[sea_orm(primary_key)]
    pub id: String,
    pub email: String,
    // /// signed is true if the license is signed
    // /// if is false, means the user may use the free test version
    // pub signed: bool,
    /// real is Vec<String> use `,` to split
    /// for example: `"hello.world.com, hello2.world.com"`
    pub domains: String,
    /// created_at is the time when the license is created (timestamp)
    pub created_at: i64,
    /// expires_at is the time when the license is expired (timestamp)
    pub expires_at: i64,
    /// license value (token)
    pub value: String,
    pub ilimit: Limit,
}

impl Model {
    pub fn new(email: String, domains: String, created_at: i64) -> Self {
        let expires_at = created_at + 60 * 60 * 24 * 365; // 1 year
        let id = uuid::Uuid::new_v4().to_string();
        let value = Self::generate_license_value(&email, &domains, created_at);
        Self {
            id,
            email,
            domains,
            created_at,
            expires_at,
            value,
            ilimit: Limit::Professional,
        }
    }
    pub fn domains(&self) -> Vec<String> {
        self.domains.split(',').map(|s| s.to_string()).collect()
    }
    /// ## check if the license is expired
    /// check expores_at < now
    pub fn is_valid(&self) -> bool {
        let now = Utc::now().timestamp();
        return self.expires_at > now;
    }
    /// check the license table is exists, if not, create it (sqlite)
    pub fn check_table_or_create() -> Statement {
        let sql = "CREATE TABLE IF NOT EXISTS license (
            id TEXT PRIMARY KEY,
            email TEXT NOT NULL,
            domains TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            expires_at INTEGER NOT NULL,
            value TEXT NOT NULL,
            ilimit TEXT NOT NULL
        )";

        Statement::from_string(DbBackend::Sqlite, sql.to_string())
    }

    fn generate_license_value(email: &str, domains: &str, timestamp: i64) -> String {
        // 创建一个包含用户信息和随机数据的混合字节数组
        let mut rng = rng();
        let mut random_bytes = [0u8; 32];
        rng.fill_bytes(&mut random_bytes);

        // 创建一个哈希器，混合用户数据和随机数据
        let mut hasher = Sha256::new();
        hasher.update(email.as_bytes());
        hasher.update(&timestamp.to_be_bytes());
        hasher.update(domains.as_bytes());
        hasher.update(&random_bytes);

        let result = hasher.finalize();

        // 编码为base64并添加前缀
        let prefix = "vocespace_pro_";

        format!("{}_{}", prefix, URL_SAFE_NO_PAD.encode(result))
    }

    pub fn to_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id),
            email: Set(self.email),
            domains: Set(self.domains),
            created_at: Set(self.created_at),
            expires_at: Set(self.expires_at),
            value: Set(self.value),
            ilimit: Set(self.ilimit),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// Limit for license
/// different license types have different limits which cost different
/// - free: free license
/// - pro: professional license
/// - enterprise: enterprise license
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(12))")]
pub enum Limit {
    #[sea_orm(string_value = "free")]
    Free,
    #[sea_orm(string_value = "pro")]
    Professional,
    #[sea_orm(string_value = "enterprise")]
    Enterprise,
}

impl Display for Limit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Limit::Free => "free",
            Limit::Professional => "pro",
            Limit::Enterprise => "enterprise",
        })
    }
}

impl serde::Serialize for Limit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
