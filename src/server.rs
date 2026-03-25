#[tokio::main]
async fn main() {
  use rest_rs::init;

  // Initialize the server.
  init().await.unwrap()
}
