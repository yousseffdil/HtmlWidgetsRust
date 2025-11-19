use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Image, Label, Button, Orientation, Widget};
use crate::parser::html_parser::DomNode;

pub fn render_dom_to_gtk(node: &DomNode) -> Widget {
    match node.tag_name.as_str() {
        "text" => {
            if let Some(text) = &node.text_content {
                let lbl = Label::new(Some(text));
                lbl.set_wrap(true);
                return lbl.upcast();
            }
            return Label::new(None).upcast();
        }

        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
            let container = GtkBox::new(Orientation::Vertical, 0);
            
            for child in &node.children {
                let widget = render_dom_to_gtk(child);
                
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

        "p" => {
            let container = GtkBox::new(Orientation::Vertical, 0);
            
            for child in &node.children {
                container.append(&render_dom_to_gtk(child));
            }
            
            return container.upcast();
        }

        "img" => {
            if let Some(src) = node.attributes.get("src") {
                let exe_dir = std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_path_buf();

                let widget_dir = exe_dir.join("widget");

                let img_path = widget_dir.join(src);

                let img = Image::from_file(&img_path);

                let size = node.attributes
                    .get("width")
                    .and_then(|w| w.parse::<i32>().ok())
                    .unwrap_or(350);

                img.set_pixel_size(size);

                return img.upcast();
            }

            let fallback = Label::new(Some("⚠️ missing src"));
            return fallback.upcast();
        }


        "div" | "body" | "id" | "span" => {
            let container = GtkBox::new(Orientation::Vertical, 10);
            container.set_margin_start(10);
            container.set_margin_end(10);
            container.set_margin_top(10);
            container.set_margin_bottom(10);
            
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
        
        "button"=>{
           let button_text = if let Some(text_child) = node.children.first() {
            text_child.text_content.clone().unwrap_or_else(|| "Button".to_string())
            } else {
                node.attributes.get("value")
                    .cloned()
                    .or_else(|| node.attributes.get("label").cloned())
                    .unwrap_or_else(|| "Button".to_string())
            };

            let button = Button::with_label(&button_text);
            
            if let Some(width) = node.attributes.get("width") {
                if let Ok(w) = width.parse::<i32>() {
                    button.set_width_request(w);
                }
            }
            
            if let Some(height) = node.attributes.get("height") {
                if let Ok(h) = height.parse::<i32>() {
                    button.set_height_request(h);
                }
            }

            if let Some(id) = node.attributes.get("id") {
                button.set_widget_name(id);
            }

            return button.upcast();
      
        }
        _ => {
            let container = GtkBox::new(Orientation::Vertical, 6);
            
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