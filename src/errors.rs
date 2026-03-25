//!  This is just some quick and easy error handling.
use std::{error, fmt, io};

use axum::{
  response::{IntoResponse, Response},
};

use crate::StatusCode;

/// A Result type that either returns a successful Ok(T) or
/// an error from the [Error] enum.
pub type Rslt<T> = Result<T, Error>;

/// An enum for general errors
/// that could occur on the server.
#[derive(Debug)]
pub enum Error {
  Db(sqlx::Error),
  Io(io::Error),
  Http(StatusCode),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::Db(err) => write!(f, "Database error: {}", err),
      Error::Io(err) => write!(f, "I/O error: {}", err),
      _ => Ok(()),
    }
  }
}

impl error::Error for Error {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      Error::Db(err) => Some(err),
      Error::Io(err) => Some(err),
      _ => None,
    }
  }
}

/// Implement how the server sends errors
/// as a response.
impl IntoResponse for Error {
  fn into_response(self) -> Response {
    // print to the server terminal.
    println!("{:?}", self);
    let status = match self {
      Error::Http(code) => code,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    status.into_response()
  }
}

/// Implement how a SQLx error converts
/// into this server's error type.
impl From<sqlx::Error> for Error {
  fn from(err: sqlx::Error) -> Self {
    Error::Db(err)
  }
}

/// Implement how an I/O error converts
/// into this server's error type.
impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Error::Io(err)
  }
}

/// Implement how a Http statuscode converts
/// into this server's error type.
impl From<StatusCode> for Error {
  fn from(err: StatusCode) -> Self {
    Error::Http(err)
  }
}
