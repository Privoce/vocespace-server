use crate::{
    entry::{self},
    error::{ApiError, ApiResult, ErrCode},
};
use salvo::{handler, writing::Json, Depot, Request, Response};

/// GET /api/s3/connect
#[handler]
pub async fn s3_connect(
    _req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> ApiResult<()> {
    let s3 = depot.obtain::<crate::s3::S3Manager>().map_err(|_| {
        ApiError::new(
            ErrCode::S3EnvUnSet,
            "Failed to obtain S3 manager".to_string(),
        )
    })?;

    s3.test_connection().await.map_err(|e| {
        ApiError::new(
            ErrCode::S3ConnectionFailed,
            format!("Failed to connect to S3: {}", e),
        )
    })?;

    res.render(Json(serde_json::json!({
        "success": true,
        "message": "S3 connection successful"
    })));

    Ok(())
}

/// GET /api/s3/{room}
#[handler]
pub async fn get_room_records(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> ApiResult<()> {
    let room = req.param::<String>("room").ok_or_else(|| {
        ApiError::new(
            ErrCode::InvalidRequest,
            "Room parameter is required".to_string(),
        )
    })?;

    let s3 = depot.obtain::<crate::s3::S3Manager>().map_err(|_| {
        ApiError::new(
            ErrCode::S3EnvUnSet,
            "Failed to obtain S3 manager".to_string(),
        )
    })?;
    let objects = s3.list_all_objects(Some(room)).await.map_err(|e| {
        ApiError::new(
            ErrCode::S3ConnectionFailed,
            format!("Failed to list objects in S3: {}", e),
        )
    })?;
    res.render(Json(serde_json::json!({
        "success": true,
        "records": objects
    })));

    Ok(())
}
