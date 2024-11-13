
#[tokio::test]
async fn health_check() {
    spawn_app();
    let client  = reqwest::Client::new();
    let response = client.get("http://localhost:8000/health")
        .send().await.expect("Could not send request");

    assert!(response.status().is_success());
    // assert_eq!(Some(0), response.content_length());
}


async fn spawn_app()  {
   let server = dubbel::run().expect("Error configuring server");

    let _ = tokio::spawn(server);

}