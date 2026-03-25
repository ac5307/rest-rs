//! The resource methods for all of the resources are implemented
//! in this file.
//! These methods will then be called and utilized within
//! the 'api' directory for the HTTP methods.

use crate::{Arr, Rslt, args};
use super::{resources::*, utils::Utils};

impl User {
  // Get a list of [User].
  pub(crate) async fn list(utils: &Utils) -> Rslt<Arr<Self>> {
    utils.exec_get_all(Self::SEL_ALL, []).await
  }

  /// Create a [User].
  pub(crate) async fn create(
    utils: &Utils,
    name: &str,
    email: &str,
    phone: &str,
  ) -> Rslt<Self> {
    utils
      .exec_get_one(Self::INSRT_ONE, args![name, email, phone])
      .await
  }

  /// Get a [User].
  pub(crate) async fn fetch(utils: &Utils, id: i32) -> Rslt<Self> {
    utils.exec_get_one(Self::SEL_ONE, args![id]).await
  }

  /// Update a [User].
  pub(crate) async fn update(
    utils: &Utils,
    id: i32,
    name: &str,
    email: &str,
    phone: &str,
  ) -> Rslt<Self> {
    utils
      .exec_get_one(Self::UPD_ONE, args![id, name, email, phone])
      .await
  }

  /// Remove a [User].
  pub(crate) async fn remove(utils: &Utils, id: i32) -> Rslt<usize> {
    utils.exec_commit(Self::DEL_ONE, args![id]).await
  }
}
