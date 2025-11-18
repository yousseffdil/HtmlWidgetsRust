use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Image, Label, Orientation, Widget};
use crate::parser::html_parser::DomNode;

pub fn render_dom_to_gtk(node: &DomNode) -> Widget {
    match node.tag_name.as_str() {
        // Nodos de texto puro
        "text" => {
            if let Some(text) = &node.text_content {
                let lbl = Label::new(Some(text));
                lbl.set_wrap(true);
                return lbl.upcast();
            }
            // Si no hay texto, crear un label vacío
            return Label::new(None).upcast();
        }

        // Encabezados
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
            let container = GtkBox::new(Orientation::Vertical, 0);
            
            for child in &node.children {
                let widget = render_dom_to_gtk(child);
                
                // Si es un Label (texto), hacerlo más grande
                if let Some(label) = widget.downcast_ref::<Label>() {
                    let size = match node.tag_name.as_str() {
                        "h1" => 32,
                        "h2" => 28,
                        "h3" => 24,
                        "h4" => 20,
                        "h5" => 18,
                        _ => 16,
                    };
                    
                    let markup = format!("<span size='{}000'><b>{}</b></span>", size, label.text());
                    label.set_markup(&markup);
                }
                
                container.append(&widget);
            }
            
            return container.upcast();
        }

        // Párrafo
        "p" => {
            let container = GtkBox::new(Orientation::Vertical, 0);
            
            for child in &node.children {
                container.append(&render_dom_to_gtk(child));
            }
            
            return container.upcast();
        }

        // Imagen
        "img" => {
            if let Some(src) = node.attributes.get("src") {
                let img = Image::from_file(src);
                
                // Tamaño por defecto o desde atributos
                let size = node.attributes.get("width")
                    .and_then(|w| w.parse::<i32>().ok())
                    .unwrap_or(350);
                
                img.set_pixel_size(size);
                return img.upcast();
            }
            // Si no hay src, retornar un label de error
            return Label::new(Some("❌ Imagen sin src")).upcast();
        }

        // Contenedores (div, body, id, etc.)
        "div" | "body" | "id" | "span" => {
            let container = GtkBox::new(Orientation::Vertical, 10);
            container.set_margin_start(10);
            container.set_margin_end(10);
            container.set_margin_top(10);
            container.set_margin_bottom(10);
            
            // Si no tiene hijos pero tiene ID, mostrar info de debug
            if node.children.is_empty() {
                if let Some(id) = node.attributes.get("id") {
                    let debug_label = Label::new(Some(&format!("Contenedor: {}", id)));
                    container.append(&debug_label);
                }
            }
            
            for child in &node.children {
                container.append(&render_dom_to_gtk(child));
            }
            
            return container.upcast();
        }

        // Para cualquier otro tag desconocido
        _ => {
            let container = GtkBox::new(Orientation::Vertical, 6);
            
            // Agregar debug label para tags no reconocidos
            let debug_label = Label::new(Some(&format!("⚠️ Tag no soportado: <{}>", node.tag_name)));
            debug_label.set_opacity(0.5);
            container.append(&debug_label);
            
            for child in &node.children {
                container.append(&render_dom_to_gtk(child));
            }
            
            return container.upcast();
        }
    }
}