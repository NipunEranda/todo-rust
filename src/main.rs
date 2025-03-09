#[path = "./routes/router.rs"]
mod router;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:5000";
    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server running on {addr:?}");

    axum::serve(listner, router::router()).await.unwrap();
}