use chrono::{DateTime, Utc};
use serde::{de::Error, Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T: Serialize> {
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        return Response {
            data: Some(data),
            error: None,
        };
    }

    pub fn err(err: Error) -> Response<()> {
        return Response {
            data: None,
            error: Some(err.into()),
        };
    }
}
