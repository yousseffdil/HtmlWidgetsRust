use iced::widget::{Column, Container, Image, Text};
use iced::widget::image::Handle;
use iced::{Element, Length};

use crate::parser::html_parser::DomNode;

#[derive(Debug, Clone)]
pub enum Message {
    Empty,
}

fn load_image(src: &str) -> Element<'static, Message> {
    // Handle::from_path funciona tanto para URLs como para archivos locales
    let handle = Handle::from_path(src);

    Image::new(handle)
        .content_fit(iced::ContentFit::Contain)
        .width(Length::Fixed(400.0)) // Tamaño fijo para mejor control
        .into()
}

pub fn render_dom_to_iced<'a>(node: &DomNode) -> Element<'a, Message> {
    // Si el nodo tiene contenido de texto, renderizarlo
    if let Some(content) = &node.text_content {
        return Text::new(content.clone()).into();
    }

    // Manejo especial para imágenes
    if node.tag_name == "img" {
        if let Some(src) = node.attributes.get("src") {
            return load_image(src);
        } else {
            return Text::new("⚠️ Imagen sin atributo 'src'").into();
        }
    }

    // Renderizar hijos recursivamente
    let children = node
        .children
        .iter()
        .map(render_dom_to_iced)
        .collect::<Vec<_>>();

    // Crear el elemento según el tag
    let element: Element<'a, Message> = match node.tag_name.as_str() {
        "body" => Column::with_children(children)
            .spacing(20)
            .padding(20)
            .into(),

        "div" | "id" => Column::with_children(children)
            .spacing(10)
            .into(),

        "h1" => Column::with_children(children)
            .padding(10)
            .into(),

        "p" => Column::with_children(children)
            .padding([0, 0, 10, 0])
            .into(),

        _ => Column::with_children(children).into(),
    };

    Container::new(element)
        .width(Length::Fill)
        .into()
}