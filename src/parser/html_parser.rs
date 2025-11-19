use kuchiki::traits::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DomNode {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<DomNode>,
    pub text_content: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub decorations: bool,
    pub transparent: bool,
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig {
            width: 800.0,
            height: 600.0,
            x: None,
            y: None,
            decorations: true,
            transparent: false,
            resizable: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WidgetDefinition {
    pub id: String,
    pub config: WindowConfig,
    pub body: DomNode,
}

pub fn parse_html(source: &str) -> Option<Vec<WidgetDefinition>> {
    let dom = kuchiki::parse_html().from_utf8().one(source.as_bytes());
    let mut widgets = Vec::new();

    // Intentar parsear formato nuevo (múltiples widgets)
    let widget_nodes: Vec<_> = dom.select("widget").unwrap().collect();
    
    if !widget_nodes.is_empty() {
        // Formato nuevo: múltiples widgets
        for widget_node in widget_nodes {
            let widget_element = widget_node.as_node().as_element().unwrap();
            let widget_id = widget_element.attributes.borrow()
                .get("id")
                .unwrap_or("unnamed")
                .to_string();

            let mut config = WindowConfig::default();

            // Parsear config dentro de este widget
            if let Ok(config_node) = widget_node.as_node().select_first("config") {
                if let Ok(window_node) = config_node.as_node().select_first("window") {
                    let element = window_node.as_node().as_element().unwrap();
                    let attrs = element.attributes.borrow();
                    
                    if let Some(width) = attrs.get("width") {
                        config.width = width.parse().unwrap_or(800.0);
                    }
                    if let Some(height) = attrs.get("height") {
                        config.height = height.parse().unwrap_or(600.0);
                    }
                    if let Some(x) = attrs.get("x") {
                        config.x = x.parse().ok();
                    }
                    if let Some(y) = attrs.get("y") {
                        config.y = y.parse().ok();
                    }
                }

                if let Ok(decorations_node) = config_node.as_node().select_first("decorations") {
                    let element = decorations_node.as_node().as_element().unwrap();
                    let attrs = element.attributes.borrow();
                    
                    if let Some(enabled) = attrs.get("enabled") {
                        config.decorations = enabled == "true";
                    }
                }

                if let Ok(transparent_node) = config_node.as_node().select_first("transparent") {
                    let element = transparent_node.as_node().as_element().unwrap();
                    let attrs = element.attributes.borrow();
                    
                    if let Some(enabled) = attrs.get("enabled") {
                        config.transparent = enabled == "true";
                    }
                }

                if let Ok(resizable_node) = config_node.as_node().select_first("resizable") {
                    let element = resizable_node.as_node().as_element().unwrap();
                    let attrs = element.attributes.borrow();
                    
                    if let Some(enabled) = attrs.get("enabled") {
                        config.resizable = enabled == "true";
                    }
                }
            }

            // Parsear body
            if let Ok(body) = widget_node.as_node().select_first("body") {
                let body_node = build_dom_node(body.as_node());
                
                widgets.push(WidgetDefinition {
                    id: widget_id,
                    config,
                    body: body_node,
                });
            }
        }
    } else {
        // Formato antiguo: un solo widget con <body> y <config> en la raíz
        if let Ok(body) = dom.select_first("body") {
            let mut config = WindowConfig::default();
            
            // Parsear configuración
            if let Ok(config_node) = dom.select_first("config") {
                if let Ok(window_node) = config_node.as_node().select_first("window") {
                    let element = window_node.as_node().as_element().unwrap();
                    let attrs = element.attributes.borrow();
                    
                    if let Some(width) = attrs.get("width") {
                        config.width = width.parse().unwrap_or(800.0);
                    }
                    if let Some(height) = attrs.get("height") {
                        config.height = height.parse().unwrap_or(600.0);
                    }
                    if let Some(x) = attrs.get("x") {
                        config.x = x.parse().ok();
                    }
                    if let Some(y) = attrs.get("y") {
                        config.y = y.parse().ok();
                    }
                }

                if let Ok(decorations_node) = config_node.as_node().select_first("decorations") {
                    let element = decorations_node.as_node().as_element().unwrap();
                    let attrs = element.attributes.borrow();
                    
                    if let Some(enabled) = attrs.get("enabled") {
                        config.decorations = enabled == "true";
                    }
                }

                if let Ok(transparent_node) = config_node.as_node().select_first("transparent") {
                    let element = transparent_node.as_node().as_element().unwrap();
                    let attrs = element.attributes.borrow();
                    
                    if let Some(enabled) = attrs.get("enabled") {
                        config.transparent = enabled == "true";
                    }
                }

                if let Ok(resizable_node) = config_node.as_node().select_first("resizable") {
                    let element = resizable_node.as_node().as_element().unwrap();
                    let attrs = element.attributes.borrow();
                    
                    if let Some(enabled) = attrs.get("enabled") {
                        config.resizable = enabled == "true";
                    }
                }
            }
            
            let body_node = build_dom_node(body.as_node());
            
            // Filtrar cualquier nodo <config> que haya quedado dentro del body
            let filtered_body = DomNode {
                tag_name: body_node.tag_name,
                attributes: body_node.attributes,
                children: body_node.children.into_iter()
                    .filter(|child| child.tag_name != "config")
                    .collect(),
                text_content: body_node.text_content,
            };
            
            widgets.push(WidgetDefinition {
                id: "main".to_string(),
                config,
                body: filtered_body,
            });
        }
    }

    if widgets.is_empty() {
        None
    } else {
        Some(widgets)
    }
}

fn build_dom_node(kuchiki_node: &kuchiki::NodeRef) -> DomNode {
    let mut attributes = HashMap::new();

    // Obtener el nombre de la etiqueta
    let tag_name = if let Some(element) = kuchiki_node.as_element() {
        for (key, value) in element.attributes.borrow().map.iter() {
            attributes.insert(key.local.to_string(), value.value.clone());
        }
        element.name.local.to_string()
    } else {
        "text".to_string()
    };

    let mut node = DomNode {
        tag_name,
        attributes,
        children: Vec::new(),
        text_content: None,
    };

    // Recorrer nodos hijos
    for child in kuchiki_node.children() {
        match child.data() {
            kuchiki::NodeData::Element(_) => {
                node.children.push(build_dom_node(&child));
            }
            kuchiki::NodeData::Text(text) => {
                let content = text.borrow().trim().to_string();
                if !content.is_empty() {
                    node.children.push(DomNode {
                        tag_name: "text".to_string(),
                        attributes: HashMap::new(),
                        children: vec![],
                        text_content: Some(content),
                    });
                }
            }
            _ => {}
        }
    }

    node
}