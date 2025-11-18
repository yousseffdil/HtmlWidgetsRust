// src/parser/html_parser.rs
use kuchiki::traits::*;

// 1. Estructura del Árbol DOM Intermedio (simplificada)
#[derive(Debug, Clone)]
pub struct DomNode {
    pub tag_name: String,
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
    let mut node = DomNode {
        tag_name: kuchiki_node.as_element().map_or("text".to_string(), |e| e.name.local.to_string()),
        children: Vec::new(),
        text_content: None,
    };

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