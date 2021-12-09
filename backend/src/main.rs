use grayblock_frontend::home;
use mogwai::prelude::*;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let index_html = include_str!("../../frontend/dist/index.html");

    let home = warp::get().and(warp::path::end()).map(|| {
        let view = Component::from(home::view()).build().unwrap();
        let html = String::from(view);
        let src = index_html.replace("Please enable JavaScript", &html);
        warp::reply::html(src)
    });

    let static_dir = warp::any().and(warp::fs::dir("frontend/dist"));

    let app = home.or(static_dir);

    println!("Running backend server on port 8080");

    warp::serve(app).run(([127, 0, 0, 1], 8080)).await;
}
