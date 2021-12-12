use std::env;

use warp::{path::Peek, Filter};

mod object_storage;

fn render_page(path: &str) -> Result<String, String> {
    let index_html = include_str!("../../frontend/dist/index.html");

    let mut page_src = grayblock_frontend::view(format!("/{}", path))?;
    page_src.push_str("</noscript><link href=\"styles.css\" rel=\"stylesheet\">");
    let src = index_html.replace("Please enable JavaScript</noscript>", &page_src);

    Ok(src)
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "grayblock_backend");
    }

    pretty_env_logger::init();
    zenv::zenv!();

    let stylesheet_src = grayblock_frontend::styles();

    let stylesheet = warp::get()
        .and(warp::path!("styles.css"))
        .map(move || warp::reply::with_header(stylesheet_src.clone(), "Content-Type", "text/css"));

    let index = warp::get()
        .and(warp::path::end())
        .map(move || match render_page("/") {
            Ok(page_src) => warp::reply::html(page_src),
            Err(_) => warp::reply::html("500 Internal Server Error".to_owned()),
        });

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

    let page = warp::get().and(warp::path::peek()).map(move |path: Peek| {
        match render_page(path.as_str()) {
            Ok(page_src) => warp::reply::html(page_src),
            Err(_) => warp::reply::html("500 Internal Server Error".to_owned()),
        }
    });

    let app = stylesheet.or(index).or(files).or(static_dir).or(page);

    println!("Running backend server on port 8080");

    warp::serve(app).run(([127, 0, 0, 1], 8080)).await;
}
