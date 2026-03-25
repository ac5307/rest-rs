use httpc_test::{Response, Result, new_client};
use serde_json::{Value, json};

#[tokio::test]
#[allow(clippy::unit_arg)]
async fn client() -> Result<()> {
  let hc = new_client("http://localhost:5000")?;
  let mut res: Response;
  let mut payload: Value;

  //#### Management endpoints ####//
  println!("Getting Postgres version.");
  res = hc.do_get("/manage/version").await?;
  assert!(res.status().is_success());
  res.print().await?;

  println!("Resetting the database.");
  res = hc.do_post("/manage/reset", json!({})).await?;
  assert!(res.status().is_success());
  res.print().await?;

  //### User endpoints ###//

  // get all
  println!("Getting the list of users. (Empty)");
  res = hc.do_get("/users").await?;
  assert!(res.status().is_success());
  res.print().await?;

  // post
  println!("Creating a new user.");
  payload = json!({
      "name": "TEST_NAME",
      "email": "test@email.edu",
      "phone": "0123456789"
  });
  res = hc.do_post("/users", payload).await?;
  assert!(res.status().is_success());
  res.print().await?;

  // get one
  println!("Getting the user by id.");
  res = hc.do_get("/users/1").await?;
  assert!(res.status().is_success());
  res.print().await?;

  // put
  println!("Updating the user's info.");
  payload = json!({
      "name": "RESTful",
      "email": "axum@rest.api",
      "phone": "9876543210"
  });
  res = hc.do_put("/users/1", payload).await?;
  assert!(res.status().is_success());
  res.print().await?;

  // get all again.
  println!("Getting all of the users again. (Not Empty)");
  res = hc.do_get("/users").await?;
  assert!(res.status().is_success());
  res.print().await?;

  // delete
  println!("Deleting a user.");
  res = hc.do_delete("/users/1").await?;
  assert!(res.status().is_success());
  res.print().await?;

  // Everything's okay.
  Ok(println!("Client tests have concluded."))
}
