use salvo::writing::Json;
use salvo::{async_trait, Depot, Request, Response, Writer};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::{error::Error, fmt::Display};

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: u16,
    pub msg: String,
}

impl Error for ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // use serde_json to serialize self to a string
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[async_trait]
impl Writer for ApiError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(self));
    }
}

impl From<(u16, String)> for ApiError {
    fn from(value: (u16, String)) -> Self {
        Self {
            code: value.0,
            msg: value.1,
        }
    }
}

impl From<(ErrCode, String)> for ApiError {
    fn from(value: (ErrCode, String)) -> Self {
        Self {
            code: value.0.into(),
            msg: value.1,
        }
    }
}

pub enum ErrCode {
    CreateToken,
}

impl From<ErrCode> for u16 {
    fn from(value: ErrCode) -> Self {
        match value {
            ErrCode::CreateToken => 1001,
        }
    }
}
