use grayblock_frontend::home;
use mogwai::prelude::*;
use once_cell::sync::Lazy;
use tide::{http::mime, Response};

fn build_view(view_builder: ViewBuilder<Dom>) -> String {
    let index_html = include_str!("../../frontend/dist/index.html");
    let view = Component::from(view_builder).build().unwrap();
    let html = String::from(view);

    let mut src = index_html.replace("Please enable JavaScript", &html);
    src.push_str("<link href=\"styles.css\" rel=\"stylesheet\">");

    src
}

static HOME_SRC: Lazy<String> = Lazy::new(|| build_view(home::view()));
static STYLES_SRC: Lazy<String> = Lazy::new(|| {
    let mut styles: Vec<String> = vec![];
    home::styles(&mut styles);
    styles.join("\n\n")
});

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();

    let mut app = tide::new();

    app.at("/styles.css").get(|_| async {
        Ok(Response::builder(200)
            .body(STYLES_SRC.clone())
            .content_type(mime::CSS)
            .build())
    });

    app.at("/").get(|_| async {
        Ok(Response::builder(200)
            .body(HOME_SRC.clone())
            .content_type(mime::HTML)
            .build())
    });

    app.at("/*").serve_dir("frontend/dist/")?;

    println!("Running backend server on port 8080");

    app.listen("localhost:8080").await?;

    Ok(())
}
