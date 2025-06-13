use crate::error::{ApiError, ApiResult, ErrCode};
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

/// POST /api/s3/download?key={key}
#[handler]
pub async fn generate_download_url(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> ApiResult<()> {
    let key = req.query::<String>("key").ok_or_else(|| {
        ApiError::new(
            ErrCode::InvalidRequest,
            "Query parameter 'key' is required".to_string(),
        )
    })?;
    if key.is_empty() {
        return Err(ApiError::new(
            ErrCode::InvalidRequest,
            "Query parameter 'key' cannot be empty".to_string(),
        ));
    }

    let s3 = depot.obtain::<crate::s3::S3Manager>().map_err(|_| {
        ApiError::new(
            ErrCode::S3EnvUnSet,
            "Failed to obtain S3 manager".to_string(),
        )
    })?;

    let url = s3
        .generate_download_url(&key, 60 * 60 * 24 * 3)
        .await
        .map_err(|e| {
            ApiError::new(
                ErrCode::S3ConnectionFailed,
                format!("Failed to generate download URL: {}", e),
            )
        })?;

    res.render(Json(serde_json::json!({
        "success": true,
        "url": url
    })));

    Ok(())
}

/// DELETE /api/s3/delete?key={key}
#[handler]
pub async fn delete_object(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> ApiResult<()> {
    let key = req.query::<String>("key").ok_or_else(|| {
        ApiError::new(
            ErrCode::InvalidRequest,
            "Query parameter 'key' is required".to_string(),
        )
    })?;
    if key.is_empty() {
        return Err(ApiError::new(
            ErrCode::InvalidRequest,
            "Query parameter 'key' cannot be empty".to_string(),
        ));
    }
    let s3 = depot.obtain::<crate::s3::S3Manager>().map_err(|_| {
        ApiError::new(
            ErrCode::S3EnvUnSet,
            "Failed to obtain S3 manager".to_string(),
        )
    })?;
    s3.delete_object(&key).await.map_err(|e| {
        ApiError::new(
            ErrCode::S3ConnectionFailed,
            format!("Failed to delete object from S3: {}", e),
        )
    })?;
    res.render(Json(serde_json::json!({
        "success": true,
        "message": "Object deleted successfully"
    })));
    Ok(())
}
