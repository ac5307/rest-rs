use axum::extract::Path;

use crate::{
  Arr, Str, Json, Router, Utils, State, get, post, db::resources::User,
  Rslt, Deserialize, StatusCode,
};

/// All of the URIs for users.
pub(crate) fn routes() -> Router<Utils> {
  Router::new()
    .route("/users", post(create).get(list))
    .route("/users/{id}", get(fetch).put(update).delete(remove))
}

#[derive(Deserialize)]
struct UserPayload {
  name: Str,
  email: Str,
  phone: Str,
}

/// GET request for all users.
async fn list(State(u): State<Utils>) -> Rslt<Json<Arr<User>>> {
  User::list(&u).await.map(Json)
}

/// POST request for creating a user.
async fn create(
  State(u): State<Utils>,
  Json(pl): Json<UserPayload>,
) -> Rslt<(StatusCode, Json<User>)> {
  User::create(&u, &pl.name, &pl.email, &pl.phone)
    .await
    .map(|user| (StatusCode::CREATED, Json(user)))
}

/// GET request for a single user.
async fn fetch(
  State(u): State<Utils>,
  Path(id): Path<i32>,
) -> Rslt<Json<User>> {
  User::fetch(&u, id)
    .await
    .map(Json)
    .map_err(|_| StatusCode::NOT_FOUND.into())
}

/// PUT request for a single user.
async fn update(
  State(u): State<Utils>,
  Path(id): Path<i32>,
  Json(pl): Json<UserPayload>,
) -> Rslt<Json<User>> {
  User::update(&u, id, &pl.name, &pl.email, &pl.phone)
    .await
    .map(Json)
}

/// DELETE request for a single user.
async fn remove(
  State(u): State<Utils>,
  Path(id): Path<i32>,
) -> Rslt<StatusCode> {
  // number of rows affected.
  let num = User::remove(&u, id).await?;
  match num {
    0 => Err(StatusCode::NOT_FOUND.into()),
    _ => Ok(StatusCode::NO_CONTENT),
  }
}
