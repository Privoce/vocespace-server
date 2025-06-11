use chrono::Utc;
use sea_orm::*;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "rooms")]
pub struct Model {
    /// name of the room
    #[sea_orm(primary_key)]
    pub name: String,
    /// total time used (timestamp)
    pub during: i64,
    /// total time used today (timestamp)
    pub today: i64,
    /// the first time the room was created (timestamp)
    pub created_at: i64,
    /// 这是一个临时字段，用来记录最近一次房间打开的时间戳
    /// 例如在2025-06-06 12:00:00打开的房间，tmp_open_at为1686038400，并且房间一直开启着
    /// 然后在看版进行请求房间数据时，这个字段就会被当前的时间戳减去，得到这次开启到当前的时间的时长
    /// 这个字段在房间关闭时会被清空，若最新的一次请求超过了该时间戳～凌晨0:00:00，则会将today字段清零
    /// 并且当today字段清零后该字段需要被更新为当前的时间戳
    pub tmp_open_at: i64,
}

impl Model {
    pub fn new(name: String) -> Self {
        let created_at = Utc::now().timestamp();
        Self {
            name,
            during: 0,
            today: 0,
            created_at,
            tmp_open_at: created_at,
        }
    }

    pub fn check_table_or_create() -> Statement {
        let sql = "CREATE TABLE IF NOT EXISTS rooms (
            name TEXT PRIMARY KEY,
            during INTEGER NOT NULL DEFAULT 0,
            today INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL,
            tmp_open_at INTEGER NOT NULL DEFAULT 0
        )";

        Statement::from_string(DbBackend::Sqlite, sql.to_string())
    }
    /// 更新今天的使用时间
    /// 一般服务器会每1分钟调用一次来进行更新
    pub fn update_today(&mut self) {
        // 如果当前tmp_open_at是0，则不进行更新
        if self.tmp_open_at == 0 {
            return;
        }
        // 获取当前时间戳
        let now = Utc::now().timestamp();
        // 若当前时间戳 > tmp_open_at 且 tmp_open_at 和 now需要在 同一天
        if now > self.tmp_open_at && (now / 86400) == (self.tmp_open_at / 86400) {
            // 更新today字段
            self.today += now - self.tmp_open_at;
            // 更新tmp_open_at为当前时间戳
            self.tmp_open_at = now;
        } else {
            // 如果不在同一天，则将today清零，并更新tmp_open_at为当前时间戳
            self.during += self.today; // 将today的值加到during上
            self.today = 0;
            self.tmp_open_at = now;
        }
    }
    /// 更新房间的总使用时间
    /// 总使用时长只会在一天结束时才会进行更新
    pub fn update_during(&mut self) {
        if self.today == 0 {
            return;
        }
        // 获取当前时间戳
        let now = Utc::now();
        let current_day = now.date_naive().and_hms_opt(23, 59, 59).unwrap();
        let now_timestamp = now.timestamp();
        // 检查当前时间戳是否为当天的最后一刻
        if now_timestamp < current_day.and_utc().timestamp() {
            return;
        }
        self.during += self.today;
        self.today = 0;
        self.tmp_open_at = 0; // 清空tmp_open_at
    }
    pub fn to_active_model(&self) -> ActiveModel {
        ActiveModel {
            name: Set(self.name.to_string()),
            during: Set(self.during),
            today: Set(self.today),
            created_at: Set(self.created_at),
            tmp_open_at: Set(self.tmp_open_at),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
