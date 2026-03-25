use axum::{
  Json, Router,
  extract::State,
  routing::{get, post},
  http::StatusCode,
};
use serde::{Deserialize, Serialize};

mod errors;
mod api {
  pub mod management;
  pub mod users;
}
mod db {
  pub mod utils;
  pub mod resources;
  mod rest;
  mod types;
}
use errors::Rslt;
use db::utils::Utils;

/// A type alias for an owned fixed-size immutable string.
///
/// This allocates about 2/3 of the memory that the [String] type
/// normally uses. Capacity is not tracked and no wasted space.
type Str = Box<str>;

/// A type alias for an owned fixed-size immutable array/list.
///
/// This allocates about 2/3 of the memory the  the [Vec] type
/// normally uses. Capacity is not tracked and no wasted space.
type Arr<T> = Box<[T]>;

/// Unify all of the URIs.
fn routes() -> Router<Utils> {
  // use everything in the api directory
  use api::*;
  Router::new()
    .route("/", get(|| async { "hello, world!" }))
    .merge(management::routes())
    .merge(users::routes())
}

/// The startup process for the server.
pub async fn init() -> Rslt<()> {
  use db::resources::init_tables;
  use tokio::net::TcpListener;

  // Create an instance of Utils.
  let utils = Utils::connect()?;
  println!("Connection pool created.");
  init_tables(&utils).await?;

  // Initialize the routes and with Utils as the state.
  let app: Router<()> = routes().with_state(utils);

  let addr = "0.0.0.0:5000";
  let listener = TcpListener::bind(addr).await?;

  println!("Running on http://{addr}/");
  axum::serve(listener, app).await?;
  Ok(())
}
