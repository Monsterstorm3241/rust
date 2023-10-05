use std::env;
use simple_pdf_generator::{Asset, AssetType, PrintOptions};
use simple_pdf_generator_derive::PdfTemplate;

#[derive(PdfTemplate)]
struct Example {
    id: i64,
    name: Option<String>,
    opt_value: Option<String>,
    surname: String,
    is_true: bool,
}

#[tokio::main]
async fn main() {
    // fill the struct
    let example = Example {
        id: 1,
        name: Some("Foo".to_string()),
        opt_value: None,
        surname: "Bar".to_string(),
        is_true: true,
    };

    let html_path = env::current_dir()
        .unwrap()
        .join("src/template/index.html");

    let assets = [Asset {
        path: env::current_dir()
            .unwrap()
            .join("src/template/css/bootstrap.min.css"),
        r#type: AssetType::Style,
    }];

    let print_options = PrintOptions {
        paper_width: Some(210.0),
        paper_height: Some(297.0),
        margin_top: Some(10.0),
        margin_bottom: Some(10.0),
        margin_left: Some(10.0),
        margin_right: Some(10.0),
        ..PrintOptions::default()
    };

    let pdf_buf = example.generate_pdf(html_path, &assets, &print_options).await;
    let pdf = pdf_buf.as_ref().unwrap();
    tokio::fs::write("example.pdf", pdf).await.expect("Failed to write file");
}