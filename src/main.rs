use zero2prod::configuration::get_configuration;
use zero2prod::run;
#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("Server running on {}", format!("http://127.0.0.1:{}", port));
    run(listener).await
}
