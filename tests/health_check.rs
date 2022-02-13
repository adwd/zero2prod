use std::net::{IpAddr, SocketAddr, TcpListener};

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let (ip_addr, port) = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("http://{}:{}/health_check", ip_addr, port))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> (IpAddr, u16) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let addr = &listener.local_addr().unwrap();
    let server = zero2prod::run(listener).expect("Failed to run server");
    let _ = tokio::spawn(server);
    (addr.ip(), addr.port())
}
