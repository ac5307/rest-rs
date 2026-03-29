use crate::{
  Router, State, Json, Rslt, Str, Utils,
  db::resources::{init_tables, drop_tables},
  get, post,
};

/// All of the URIs for management.
pub(crate) fn routes() -> Router<Utils> {
  Router::new()
    .route("/manage/reset", post(rebuild_tables))
    .route("/manage/version", get(version))
}

async fn rebuild_tables(State(u): State<Utils>) -> Rslt<()> {
  drop_tables(&u).await?;
  init_tables(&u).await?;
  Ok(())
}

async fn version(State(u): State<Utils>) -> Rslt<Json<Str>> {
  use sqlx::query_scalar;
  let ver = query_scalar("SELECT VERSION()").fetch_one(u.pool()).await?;
  Ok(Json(ver))
}
