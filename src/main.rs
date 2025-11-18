use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

mod parser;
mod renderer;

use parser::html_parser::{parse_html, DomNode, WindowConfig};
use renderer::gtk_renderer::render_dom_to_gtk;

const HTML_SOURCE: &str = include_str!("html_source.html");

fn build_ui(app: &Application) {
    let parse_result = parse_html(HTML_SOURCE);
    let root_dom: DomNode;
    let config: WindowConfig;

    if let Some(result) = parse_result {
        root_dom = result.body;
        config = result.config;
    } else {
        root_dom = DomNode {
            tag_name: "div".into(),
            attributes: Default::default(),
            children: vec![],
            text_content: Some("Error al parsear HTML".into()),
        };
        config = WindowConfig::default();
    }


    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(config.width as i32)
        .default_height(config.height as i32)
        .decorated(false)
        .resizable(config.resizable)
        .build();

    let root_widget = render_dom_to_gtk(&root_dom);

    window.set_child(Some(&root_widget));
    window.show();
}

fn main() {
    let app = Application::builder()
        .application_id("htmlwidgets.rust.gtk")
        .build();

    app.connect_activate(build_ui);
    app.run();
}
