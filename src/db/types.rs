//! This file is mainly used in conjunction with ['super::utils'].
//! The main purpose is to make binding arguments to queries easier.

use sqlx::{
  Postgres,
  postgres::PgArguments,
  query::{Query, QueryAs},
};

/// An enum representing the
/// accepted input arguments.
pub(super) enum DbType<'t> {
  Int(i32),
  Float(f32),
  Bool(bool),
  Text(&'t str),
}

/// A type alias for a Postgres query.
type RsQ<'q> = Query<'q, Postgres, PgArguments>;

/// A type alias for a Postgres query that maps
/// the fetched result(s) into the generic type T.
type RsQs<'q, T> = QueryAs<'q, Postgres, T, PgArguments>;

/// Given a [RsQ] and an iterator containing elements
/// of [DbType], bind all of the elements as arguments.
pub(super) fn bind_qry<'t, I>(mut qry: RsQ<'t>, args: I) -> RsQ<'t>
where
  I: IntoIterator<Item = DbType<'t>>,
{
  // for each argument,
  for arg in args {
    // replace the current query with the
    // one after binding the argument.
    qry = match arg {
      DbType::Int(v) => qry.bind(v),
      DbType::Float(v) => qry.bind(v),
      DbType::Bool(v) => qry.bind(v),
      DbType::Text(v) => qry.bind(v),
    };
  }
  qry // return the final query
}

/// Given a [RsQs] and an iterator containing elements
/// of [DbType], bind all of the the elements arguments.
pub(super) fn bind_qry_as<'t, I, T>(
  mut qry: RsQs<'t, T>,
  args: I,
) -> RsQs<'t, T>
where
  I: IntoIterator<Item = DbType<'t>>,
{
  // for each argument,
  for arg in args {
    // replace the current query with the
    // one after binding the argument.
    qry = match arg {
      DbType::Int(v) => qry.bind(v),
      DbType::Float(v) => qry.bind(v),
      DbType::Bool(v) => qry.bind(v),
      DbType::Text(v) => qry.bind(v),
    };
  }
  qry // return the final query
}

impl<'t> From<i32> for DbType<'t> {
  fn from(val: i32) -> Self {
    DbType::Int(val)
  }
}

impl<'t> From<f32> for DbType<'t> {
  fn from(val: f32) -> Self {
    DbType::Float(val)
  }
}

impl<'t> From<bool> for DbType<'t> {
  fn from(val: bool) -> Self {
    DbType::Bool(val)
  }
}

impl<'t> From<&'t str> for DbType<'t> {
  fn from(val: &'t str) -> Self {
    DbType::Text(val)
  }
}

/// Given N elements, produces a fixed array containing all of the elements.
///
/// This macro is intended to be used to hold data types defined in [DbType].
#[macro_export]
macro_rules! args {
    ($($arg:expr),* $(,)?) => {
        [$($arg.into()),*]
    };
}
