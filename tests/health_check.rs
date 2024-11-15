use std::net::TcpListener;

#[tokio::test]
async fn health_check() {
    let address = spawn_app();
    let client  = reqwest::Client::new();
    let response = client.get(&format!("{}/health", &address))
        .send().await.expect("Could not send request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


fn spawn_app() -> String  {
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = dubbel::run(listener).expect("Error configuring server");

    let _ = tokio::spawn(server);

    format!("http://localhost:{}", port)
}