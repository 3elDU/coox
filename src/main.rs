mod app;
mod config;
mod context;
mod dom;
mod format;
mod tabs;
/// Text measuring function
mod text;
mod widgets;

use std::{env, path::Path};

use crate::{app::App, context::BrowserContext};

fn main() {
    let mut context = BrowserContext::new().expect("Unable to create browser context");

    let path: String = env::args()
        .into_iter()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Please provide the path to an HTML file as the first argument")
        .clone();

    if path.starts_with("http") {
        context
            .load_document_from_network(path)
            .expect("Unable to load the HTML file");
    } else {
        context
            .load_document_from_file(Path::new(&path))
            .expect("Unable to load the HTML file");
    }

    let mut app = App::new(context);

    ratatui::run(|terminal| app.run(terminal)).unwrap();
}
