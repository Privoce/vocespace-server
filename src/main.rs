use live_end::{db, router, s3};
use salvo::{conn::TcpListener, Listener, Server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // init database
    let cn = db::init().await;
    let s3 = s3::S3Manager::new().await;

    let mut router = router::build().hoop(salvo::affix_state::inject(cn));

    match s3 {
        Ok(s3) => {
            router = router.hoop(salvo::affix_state::inject(s3));
        },
        Err(e) => {
            eprintln!("S3 Failed: {}", e);
        }
    }

    let acceptor = TcpListener::new("127.0.0.1:3060").bind().await;
    Server::new(acceptor).serve(router).await;
}
