//! All of the resources and their data within the RESTful API will be
//! defined here, along with the necessary SQL statements for CRUD operations.
//!
//! This allows for a better layered systems because the concerns for how
//! the resources are structured and how they're reflected by their SQL
//! statements are all here, while the functions that utilized these SQL
//! statements can be somewhere else.

use crate::{Serialize, Str, Utils, Rslt};
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct User {
  id: i32,
  name: Str,
  email: Str,
  phone: Str,
}

impl User {
  /// SELECT statement for all [User].
  pub(super) const SEL_ALL: &str = r#"
    SELECT * FROM users
    "#;

  /// INSERT statement for [User].
  pub(super) const INSRT_ONE: &str = r#"
    INSERT INTO users
        (name, email, phone)
    VALUES
        ($1, $2, $3)
    RETURNING *
    "#;

  /// SELECT statement for one [User].
  pub(super) const SEL_ONE: &str = r#"
    SELECT * FROM users
    WHERE id = $1
    "#;

  /// UPDATE statement for [User].
  pub(super) const UPD_ONE: &str = r#"
    UPDATE users
    SET
        name = $2,
        email = $3,
        phone = $4
    WHERE id = $1
    AND (name, email, phone) IS DISTINCT FROM ($2, $3, $4)
    RETURNING *
    "#;

  /// DELETE statement for [User].
  pub(super) const DEL_ONE: &str = r#"
    DELETE FROM users
    WHERE id = $1
    "#;
}

pub(crate) async fn init_tables(utils: &Utils) -> Rslt<()> {
  utils.exec_sql_file("schema.sql").await
}

pub(crate) async fn drop_tables(utils: &Utils) -> Rslt<()> {
  utils.exec_sql_file("drop.sql").await
}
