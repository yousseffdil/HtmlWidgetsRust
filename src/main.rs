use iced::{executor, window, Application, Command, Element, Settings, Size, Theme};

mod parser;
mod renderer;

use parser::html_parser::{parse_html, DomNode, WindowConfig};
use renderer::{render_dom_to_iced, Message};

const HTML_SOURCE: &str = include_str!("html_source.html");

struct HtmlApp {
    root_dom_node: DomNode,
}

impl Application for HtmlApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = WindowConfig;

    fn new(config: WindowConfig) -> (Self, Command<Message>) {
        let parse_result = parse_html(HTML_SOURCE);
        
        let root_node = if let Some(result) = parse_result {
            result.body
        } else {
            DomNode {
                tag_name: "div".to_string(),
                attributes: std::collections::HashMap::new(),
                children: vec![],
                text_content: Some("Error al parsear el HTML.".to_string()),
            }
        };

        (HtmlApp { root_dom_node: root_node }, Command::none())
    }

    fn title(&self) -> String {
        "HTML → Iced Renderer".into()
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        iced::widget::Container::new(render_dom_to_iced(&self.root_dom_node))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    // Parsear el HTML para obtener la configuración
    let parse_result = parse_html(HTML_SOURCE);
    let config = if let Some(result) = parse_result {
        result.config
    } else {
        WindowConfig::default()
    };

    println!("🪟 Configuración de ventana:");
    println!("   Tamaño: {}x{}", config.width, config.height);
    println!("   Decoraciones: {}", config.decorations);
    println!("   Transparente: {}", config.transparent);
    println!("   Redimensionable: {}", config.resizable);

    HtmlApp::run(Settings {
        window: window::Settings {
            size: Size::new(config.width, config.height),
            decorations: config.decorations,
            transparent: config.transparent,
            resizable: config.resizable,
            ..Default::default()
        },
        flags: config,
        ..Default::default()
    })
}