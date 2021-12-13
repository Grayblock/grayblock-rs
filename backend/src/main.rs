use std::env;

use warp::{path::Peek, Filter};

mod object_storage;

fn wrap_page(src: String) -> String {
    let index_html = include_str!("../../frontend/dist/index.html");
    index_html
        .replace(
            "<!-- styles.css -->",
            "<link href=\"styles.css\" rel=\"stylesheet\">",
        )
        .replace("Please enable JavaScript", &src)
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "grayblock_backend");
    }

    pretty_env_logger::init();
    zenv::zenv!();

    let home_page_src = wrap_page(grayblock_frontend::view("/").unwrap());
    let dashboard_page_src = wrap_page(grayblock_frontend::view("/dashboard").unwrap());
    let projects_page_src = wrap_page(grayblock_frontend::view("/projects").unwrap());
    let staking_page_src = wrap_page(grayblock_frontend::view("/staking").unwrap());
    let learn_page_src = wrap_page(grayblock_frontend::view("/learn").unwrap());

    let home_route = warp::get()
        .and(warp::path::end())
        .map(move || warp::reply::html(home_page_src.clone()));

    let dashboard_route = warp::get()
        .and(warp::path("dashboard"))
        .and(warp::path::end())
        .map(move || warp::reply::html(dashboard_page_src.clone()));

    let projects_route = warp::get()
        .and(warp::path("projects"))
        .and(warp::path::end())
        .map(move || warp::reply::html(projects_page_src.clone()));

    let staking_route = warp::get()
        .and(warp::path("staking"))
        .and(warp::path::end())
        .map(move || warp::reply::html(staking_page_src.clone()));

    let learn_route = warp::get()
        .and(warp::path("learn"))
        .and(warp::path::end())
        .map(move || warp::reply::html(learn_page_src.clone()));

    let pages = home_route
        .or(dashboard_route)
        .or(projects_route)
        .or(staking_route)
        .or(learn_route);

    let stylesheet_src = grayblock_frontend::styles();
    let stylesheet = warp::get()
        .and(warp::path!("styles.css"))
        .map(move || warp::reply::with_header(stylesheet_src.clone(), "Content-Type", "text/css"));

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

    let static_dir = warp::get().and(warp::fs::dir("frontend/dist"));

    let app = pages.or(stylesheet).or(files).or(static_dir);

    println!("Running backend server on port 8080");

    warp::serve(app).run(([127, 0, 0, 1], 8080)).await;
}
