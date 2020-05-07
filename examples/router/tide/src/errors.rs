use thiserror::Error;
use tide::{http::StatusCode, Response};

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    #[error("{0}")]
    Any(#[from] anyhow::Error),
}

impl Error {
    pub fn respond(&self) -> Response {
        match self {
            Self::Serde(_) => Response::new(StatusCode::UnprocessableEntity),
            _ => Response::new(StatusCode::InternalServerError),
        }
    }
}
