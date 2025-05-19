use live_end::{db, router};
use salvo::{conn::TcpListener, Listener, Server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // init database
    let _ = db::init().await;

    let router = router::build();
    let acceptor = TcpListener::new("127.0.0.1:3060").bind().await;
    Server::new(acceptor).serve(router).await;
}
