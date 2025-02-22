use zero2prod::run;
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("Server running on {}", format!("http://127.0.0.1:{}", port));
    run(listener).await
}
