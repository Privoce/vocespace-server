use live_end::{db, router, s3::Conf};
use salvo::{conn::TcpListener, Listener, Server};

// #[tokio::main]
// async fn main() {
//     tracing_subscriber::fmt::init();

//     // init database
//     let cn = db::init().await;
//     let router = router::build().hoop(salvo::affix_state::inject(cn));
//     let acceptor = TcpListener::new("127.0.0.1:3060").bind().await;
//     Server::new(acceptor).serve(router).await;
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = live_end::s3::S3Manager::new().await?;

    dbg!(client.test_connection().await.is_ok());
    Ok(())
}
