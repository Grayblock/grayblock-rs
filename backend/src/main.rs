use grayblock_frontend::home;
use mogwai::prelude::*;
use warp::Filter;

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
    pretty_env_logger::init();

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

    let static_dir = warp::any().and(warp::fs::dir("frontend/dist"));

    let app = stylesheet.or(home).or(static_dir);

    println!("Running backend server on port 8080");

    warp::serve(app).run(([127, 0, 0, 1], 8080)).await;
}
