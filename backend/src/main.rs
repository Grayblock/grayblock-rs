#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    warp::serve(warp::fs::dir("frontend/dist"))
        .run(([127, 0, 0, 1], 8000))
        .await;
}
