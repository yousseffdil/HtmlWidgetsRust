// src/renderer/iced_renderer.rs
use iced::widget::{column, container, text};
use iced::{Element, Length};

use crate::parser::html_parser::DomNode;   // <-- ruta correcta

#[derive(Debug, Clone)]
pub enum Message {
    Empty,
}

pub fn render_dom_to_iced<'a>(node: &DomNode) -> Element<'a, Message> {
    if let Some(content) = &node.text_content {
        return text(content).into();
    }

    let children_elements: Vec<Element<'a, Message>> = node
        .children
        .iter()
        .map(render_dom_to_iced)
        .collect();

    let element: Element<'a, Message> = match node.tag_name.as_str() {
        "body" | "div" | "main-container" => column(children_elements).spacing(5).into(),
        "h1" => column(children_elements).padding(10).into(),
        "p" => column(children_elements).padding([0, 0, 5, 0]).into(),
        _ => column(children_elements).into(),
    };

    container(element).width(Length::Fill).into()
}