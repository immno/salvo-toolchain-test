use salvo::prelude::*;
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

#[handler]
async fn hello(res: &mut Response) {
    res.render(Text::Plain("Hello World"));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer().with_filter(LevelFilter::from_level(Level::INFO))).init();
    let mut router = Router::new().get(hello);
    let listener = TcpListener::new("0.0.0.0:443")
        .acme()
        .add_domain("test.salvo.rs") // Replace this domain name with your own.
        .http01_challenge(&mut router).quinn("0.0.0.0:443");
    let acceptor = listener.join(TcpListener::new("0.0.0.0:8880")).bind().await;
    Server::new(acceptor).serve(router).await;
}