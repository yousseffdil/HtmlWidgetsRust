use kuchiki::traits::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DomNode {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<DomNode>,
    pub text_content: Option<String>,
}

pub fn parse_html(source: &str) -> Option<DomNode> {
    let dom = kuchiki::parse_html().from_utf8().one(source.as_bytes());

    if let Ok(body) = dom.select_first("body") {
        return Some(build_dom_node(body.as_node()));
    }
    None
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
