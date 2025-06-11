use crate::{
    entry::{self},
    error::{ApiError, ApiResult, ErrCode},
};
use salvo::{handler, writing::Json, Depot, Request, Response};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RoomReq {
    pub room: String,
}

/// GET /api/rooms
#[handler]
pub async fn get_rooms(_req: &mut Request, res: &mut Response, depot: &mut Depot) -> ApiResult<()> {
    let db = depot.obtain::<DatabaseConnection>().or_else(|_| {
        Err(ApiError::new(
            ErrCode::DatabaseError,
            "Failed to obtain database connection".to_string(),
        ))
    })?;

    let rooms = entry::rooms::Entity::find().all(db).await.map_err(|err| {
        ApiError::new(
            ErrCode::DatabaseError,
            format!("Failed to fetch rooms: {}", err),
        )
    })?;

    res.render(Json(serde_json::json!({
        "success": true,
        "rooms": rooms
    })));
    Ok(())
}

/// POST /api/rooms
///
/// if room is not exists, create it or awake it
#[handler]
pub async fn awake_or_create_room(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> ApiResult<()> {
    let room_req: RoomReq = req
        .parse_json()
        .await
        .map_err(|_| ApiError::new(ErrCode::InvalidRequest, "Invalid request".to_string()))?;

    let db = depot.obtain::<DatabaseConnection>().or_else(|_| {
        Err(ApiError::new(
            ErrCode::DatabaseError,
            "Failed to obtain database connection".to_string(),
        ))
    })?;

    // check if room exists
    let mut room = entry::rooms::Entity::find()
        .filter(entry::rooms::Column::Name.eq(room_req.room.clone()))
        .one(db)
        .await
        .map_err(|err| {
            ApiError::new(
                ErrCode::DatabaseError,
                format!("Failed to fetch room: {}", err),
            )
        })?;

    let room = if let Some(room) = room.as_mut() {
        // room exist, if room is not awake, awake it
        room.update_today();
        room.update_during();
        room.to_active_model()
    } else {
        // create a room model
        let room = entry::rooms::Model::new(room_req.room);
        let room = room.to_active_model();
        room
    };

    match room.insert(db).await {
        Ok(model) => {
            res.render(Json(serde_json::json!({
                "success": true,
                "room": model.name,
                "msg": "Room created successfully"
            })));
        }
        Err(err) => {
            if err.to_string().contains("RecordNotFound") {
                // 这种情况下实际上记录可能已经被插入，只是无法检索
                res.render(Json(serde_json::json!({
                    "success": true,
                    "msg": "Room likely created but couldn't be retrieved",
                    "warning": err.to_string()
                })));
            } else {
                res.render(Json(serde_json::json!({
                    "success": false,
                    "msg": err.to_string()
                })));
            }
        }
    }

    Ok(())
}

/// PUT /api/rooms
/// 这个接口会对数据库中所有的房间进行更新，一般在使用看板时会调用，看板每分钟调用一次/点击刷新数据
/// 返回所有房间的最新状态
#[handler]
pub async fn update_room(
    _req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> ApiResult<()> {
    let db = depot.obtain::<DatabaseConnection>().or_else(|_| {
        Err(ApiError::new(
            ErrCode::DatabaseError,
            "Failed to obtain database connection".to_string(),
        ))
    })?;
    
    let mut rooms = entry::rooms::Entity::find().all(db).await.map_err(|err| {
        ApiError::new(
            ErrCode::DatabaseError,
            format!("Failed to fetch rooms: {}", err),
        )
    })?;

    for room in &mut rooms {
        room.update_today();
        room.update_during();
        let active_model = room.to_active_model();
        active_model.update(db).await.map_err(|err| {
            ApiError::new(
                ErrCode::DatabaseError,
                format!("Failed to update room: {}", err),
            )
        })?;
    }
    res.render(Json(serde_json::json!({
        "success": true,
        "rooms": rooms,
        "msg": "Rooms updated successfully"
    })));

    Ok(())
}
