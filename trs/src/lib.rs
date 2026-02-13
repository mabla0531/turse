pub trait TurseElement {
    const TAG_NAME: &'static str;
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateNode {
    Element {
        tag: &'static str,
        children: Vec<TemplateNode>,
    },
    Text {
        text: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct VNode {
    pub template: Option<TemplateNode>,
}

impl VNode {
    pub fn new(template: TemplateNode) -> Self {
        Self {
            template: Some(template),
        }
    }

    pub fn empty() -> Self {
        Self { template: None }
    }
}

pub mod elements {
    use super::TurseElement;
    pub struct block;
    impl TurseElement for block {
        const TAG_NAME: &'static str = "block";
    }

    pub struct text;
    impl TurseElement for text {
        const TAG_NAME: &'static str = "text";
    }
}
