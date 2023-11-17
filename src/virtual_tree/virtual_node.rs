use dioxus::prelude::{TemplateAttribute, TemplateNode};

#[derive(Clone)]
pub enum Attribute {
    Dynamic { id: usize },
}

#[derive(Clone)]
pub enum VirtualNode {
    Text(String),
    Element {
        tag: String,
        attrs: Vec<Attribute>,
        children: Vec<Self>,
    },
}

impl VirtualNode {
    pub fn from_template(template_node: &TemplateNode) -> Self {
        match template_node {
            TemplateNode::Text { text } => VirtualNode::Text(text.to_string()),
            TemplateNode::Element {
                tag,
                namespace: _,
                attrs,
                children,
            } => {
                let children = children.iter().map(Self::from_template).collect();
                let attrs = attrs
                    .into_iter()
                    .map(|attr| match attr {
                        TemplateAttribute::Dynamic { id } => Attribute::Dynamic { id: *id },
                        _ => todo!(),
                    })
                    .collect();
                VirtualNode::Element {
                    tag: tag.to_string(),
                    attrs,
                    children,
                }
            }
            TemplateNode::DynamicText { id: _ } => VirtualNode::Text(String::new()),
            _ => todo!(),
        }
    }
}
