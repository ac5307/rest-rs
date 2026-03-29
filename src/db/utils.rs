use sqlx::{
  FromRow, PgPool,
  postgres::{PgConnectOptions, PgRow},
  query, query_as, raw_sql,
};
use std::{fs::read_to_string, path::Path};

use crate::{Str, Arr, Rslt, Serialize, Deserialize};
use super::types::{DbType, bind_qry, bind_qry_as};

/// The database utility.
#[derive(Clone)]
pub struct Utils {
  pool: PgPool,
}

/// A struct to map the
/// database configs to.
#[derive(Deserialize)]
struct DbConf {
  host: Str,
  database: Str,
  user: Str,
  password: Str,
  port: u16,
}

impl Utils {
  /// Loaded contents of the database configs
  const DB_CONF: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config/db.yml"));

  /// Create a new instance of Utils, which will
  /// make connections to the Postgresql database.
  pub(crate) fn connect() -> Rslt<Self> {
    let conn = {
      let conf: DbConf = serde_yml::from_str(Self::DB_CONF).unwrap();
      PgConnectOptions::new()
        .host(&conf.host)
        .database(&conf.database)
        .username(&conf.user)
        .password(&conf.password)
        .port(conf.port)
    };
    let pool = PgPool::connect_lazy_with(conn);
    Ok(Utils { pool })
  }

  /// Execute all of the SQL statements that are inside a .sql file
  /// from a given path.
  pub(super) async fn exec_sql_file(&self, path: &str) -> Rslt<()> {
    let sql: &str = {
      let dir_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/db");
      let full_path = Path::new(dir_path).join(path);
      &read_to_string(full_path).unwrap()
    };
    raw_sql(sql).execute(&self.pool).await?;
    Ok(())
  }

  /// Executes the given SQL statement with any given arguments and
  /// fetches the first row of the result and maps it to JSON.
  /// If there are no rows, an error will occur. The caller of this function
  /// can determine what to do with errors.
  pub(super) async fn exec_get_one<T, const N: usize>(
    &self,
    sql: &str,
    args: [DbType<'_>; N],
  ) -> Rslt<T>
  where
    for<'r> T: FromRow<'r, PgRow> + Serialize + Send + Unpin,
  {
    let res = {
      let qry = bind_qry_as(query_as::<_, T>(sql), args);
      qry.fetch_one(&self.pool).await
    }?;
    Ok(res)
  }

  /// Executes the given SQL statement with any given arguments and
  /// fetches all of the rows of the result and maps each row to JSON.
  /// If there are no rows, then an empty array will be returned.
  pub(super) async fn exec_get_all<T, const N: usize>(
    &self,
    sql: &str,
    args: [DbType<'_>; N],
  ) -> Rslt<Arr<T>>
  where
    for<'r> T: FromRow<'r, PgRow> + Serialize + Send + Unpin,
  {
    let res = {
      let qry = bind_qry_as(query_as::<_, T>(sql), args);
      qry.fetch_all(&self.pool).await
    }?;
    Ok(res.into())
  }

  /// Executes the given SQL statement with any given arguments and commits.
  /// Then, returns the number of affected rows.
  pub(super) async fn exec_commit<const N: usize>(
    &self,
    sql: &str,
    args: [DbType<'_>; N],
  ) -> Rslt<usize> {
    // start a transaction.
    let mut txn = self.pool.begin().await?;

    let qry = bind_qry(query(sql), args);
    let res = qry.execute(&mut *txn).await?;

    // commit the changes.
    txn.commit().await?;
    Ok(res.rows_affected() as usize)
  }

  /// Retrieves a reference of the connection pool.
  /// This should mainly be used when the caller
  /// wants to perform a special case that cannot
  /// be done with the other functions.
  pub const fn pool(&self) -> &PgPool {
    &self.pool
  }
}
