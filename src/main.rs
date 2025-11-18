use iced::{executor, Application, Command, Element, Settings, Theme};

mod parser;
mod renderer;

use parser::html_parser::{parse_html, DomNode};
use renderer::{render_dom_to_iced, Message};

const HTML_SOURCE: &str = include_str!("html_source.html");

struct HtmlApp {
    root_dom_node: DomNode,
}

impl Application for HtmlApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let root_node = parse_html(HTML_SOURCE).unwrap_or_else(|| DomNode {
            tag_name: "div".to_string(),
            attributes: std::collections::HashMap::new(),
            children: vec![],
            text_content: Some("Error al parsear el HTML.".to_string()),
        });

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
    HtmlApp::run(Settings::default())
}
