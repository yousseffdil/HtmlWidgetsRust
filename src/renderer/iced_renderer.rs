use iced::widget::{Column, Container, Image, Text};
use iced::widget::image;
use iced::widget::image::Handle;
use iced::{Element, Length, ContentFit};

use crate::parser::html_parser::DomNode;

#[derive(Debug, Clone)]
pub enum Message {
    Empty,
}

fn load_image(src: &str) -> Element<'static, Message> {
    let handle = if src.starts_with("http") {
        match reqwest::blocking::get(src) {
            Ok(response) => match response.bytes() {
                Ok(bytes) => Handle::from_memory(bytes.to_vec()),
                Err(_) => return Text::new("Error cargando bytes de imagen").into(),
            },
            Err(_) => return Text::new("Error descargando imagen").into(),
        }
    } else {
        Handle::from_path(src)
    };

    Image::new(handle)
        .content_fit(iced::ContentFit::Contain)
        .width(Length::Fill)
        .into()
}
pub fn render_dom_to_iced<'a>(node: &DomNode) -> Element<'a, Message> {
    if let Some(content) = &node.text_content {
        return Text::new(content.clone()).into();
    }

    if node.tag_name == "img" {
        if let Some(src) = node.attributes.get("src") {
            return load_image(src);
        } else {
            return Text::new("Imagen sin src").into();
        }
    }

    let children = node
        .children
        .iter()
        .map(render_dom_to_iced)
        .collect::<Vec<_>>();

    
    let element: Element<'a, Message> = match node.tag_name.as_str() {
        "body" | "div" => Column::with_children(children)
            .spacing(10)
            .into(),

        "h1" => Column::with_children(children)
            .padding(10)
            .into(),

        "p" => Column::with_children(children)
            .padding([0, 0, 5, 0])
            .into(),
        "img" => {
            if let Some(src) = node.attributes.get("src") {
                load_image(src)
            } else {
                Text::new("[img sin src]").into()
            }
        }
        _ => Column::with_children(children).into(),
    };

    Container::new(element)
        .width(Length::Fill)
        .into()
}
