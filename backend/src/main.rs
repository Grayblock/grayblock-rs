use std::env;

use grayblock_frontend::home;
use mogwai::prelude::*;
use warp::{path::Peek, Filter};

mod object_storage;

fn build_view(view_builder: ViewBuilder<Dom>) -> String {
    let index_html = include_str!("../../frontend/dist/index.html");
    let view = Component::from(view_builder).build().unwrap();
    let html = String::from(view);

    let mut src = index_html.replace("Please enable JavaScript", &html);
    src.push_str("<link href=\"styles.css\" rel=\"stylesheet\">");

    src
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "grayblock_backend");
    }

    pretty_env_logger::init();
    zenv::zenv!();

    let mut styles: Vec<String> = vec![];
    home::styles(&mut styles);

    let stylesheet_src = styles.join("\n\n");
    let stylesheet = warp::get()
        .and(warp::path!("styles.css"))
        .map(move || warp::reply::with_header(stylesheet_src.clone(), "Content-Type", "text/css"));

    let home_src = build_view(home::view());
    let home = warp::get()
        .and(warp::path::end())
        .map(move || warp::reply::html(home_src.clone()));

    let files = warp::get()
        .and(warp::path::path("files"))
        .and(warp::path::peek())
        .then(|path: Peek| async move {
            let (data, content_type) = object_storage::get(path.as_str())
                .await
                .expect("retrieved from object store");

            if let Some(content_type) = content_type {
                warp::reply::with_header(data, "Content-Type", content_type)
            } else {
                warp::reply::with_header(data, "Content-Type", "application/octet-stream")
            }
        });

    let static_dir = warp::any().and(warp::fs::dir("frontend/dist"));

    let app = stylesheet.or(home).or(files).or(static_dir);

    println!("Running backend server on port 8080");

    warp::serve(app).run(([127, 0, 0, 1], 8080)).await;
}
