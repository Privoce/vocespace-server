use livekit_api::access_token::{self, VideoGrants};
use salvo::{handler, writing::Json, Response};

use crate::error::{ApiResult, ErrCode};

use super::util::{api_key, api_secret};

#[handler]
pub async fn create_token(res: &mut Response) -> ApiResult<()>{
    let api_key = api_key();
    let api_secret = api_secret();

    let token = access_token::AccessToken::with_api_key(&api_key, &api_secret)
    .with_identity("hello")
    .with_name("syf")
    .with_grants(VideoGrants{
        room_join: true,
        room: "my-room".to_string(),
        ..Default::default()
    })
    .to_jwt().map_err(|e| (ErrCode::CreateToken, e.to_string()))?;

    res.render(Json(token));
    Ok(())
}