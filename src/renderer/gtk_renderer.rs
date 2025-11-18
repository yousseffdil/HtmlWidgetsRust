use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Image, Label, Orientation, Widget};
use crate::parser::html_parser::DomNode;

pub fn render_dom_to_gtk(node: &DomNode) -> Widget {
    // Texto
    if let Some(text) = &node.text_content {
        let lbl = Label::new(Some(text));
        lbl.set_wrap(true);
        return lbl.upcast();
    }

    // Imagen
    if node.tag_name == "img" {
        if let Some(src) = node.attributes.get("src") {
            let img = Image::from_file(src);
            img.set_pixel_size(350); // tamaño controlado
            return img.upcast();
        }
    }

    // Contenedor vertical para hijos
    let container = GtkBox::new(Orientation::Vertical, 6);

    for child in &node.children {
        container.append(&render_dom_to_gtk(child));
    }

    container.upcast()
}
