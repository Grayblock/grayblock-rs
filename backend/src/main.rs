#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    println!("Running backend server on port 8080");

    warp::serve(warp::fs::dir("frontend/dist"))
        .run(([127, 0, 0, 1], 8080))
        .await;
}
