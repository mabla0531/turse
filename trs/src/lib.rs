use std::collections::HashMap;
use std::rc::Rc;

pub trait TurseElement {
    const TAG_NAME: &'static str;
}

pub enum TemplateNode {
    Element {
        tag: String,
        attrs: HashMap<String, AttrValue>,
        children: Vec<TemplateNode>,
    },
    Literal(String),
    ReactiveChild(Rc<dyn Fn() -> TemplateNode>),
}

pub trait IntoTemplateNode {
    fn into_template_node(self) -> TemplateNode;
}

impl IntoTemplateNode for String {
    fn into_template_node(self) -> TemplateNode {
        TemplateNode::Literal(self)
    }
}

impl IntoTemplateNode for &str {
    fn into_template_node(self) -> TemplateNode {
        TemplateNode::Literal(self.to_string())
    }
}

impl IntoTemplateNode for TemplateNode {
    fn into_template_node(self) -> TemplateNode {
        self
    }
}

impl From<String> for TemplateNode {
    fn from(s: String) -> Self {
        TemplateNode::Literal(s)
    }
}

impl From<&str> for TemplateNode {
    fn from(s: &str) -> Self {
        TemplateNode::Literal(s.to_string())
    }
}

impl<T> From<Vec<T>> for TemplateNode
where
    T: IntoTemplateNode,
{
    fn from(vec: Vec<T>) -> Self {
        TemplateNode::Element {
            tag: "fragment".to_string(),
            attrs: HashMap::new(),
            children: vec.into_iter().map(|t| t.into_template_node()).collect(),
        }
    }
}

impl std::fmt::Debug for TemplateNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateNode::Element {
                tag,
                attrs,
                children,
            } => f
                .debug_struct("Element")
                .field("tag", tag)
                .field("attrs", attrs)
                .field("children", children)
                .finish(),
            TemplateNode::Literal(s) => f.debug_tuple("Literal").field(s).finish(),
            TemplateNode::ReactiveChild(_) => f.debug_tuple("ReactiveChild").finish(),
        }
    }
}

impl Clone for TemplateNode {
    fn clone(&self) -> Self {
        match self {
            TemplateNode::Element {
                tag,
                attrs,
                children,
            } => TemplateNode::Element {
                tag: tag.clone(),
                attrs: attrs.clone(),
                children: children.clone(),
            },
            TemplateNode::Literal(s) => TemplateNode::Literal(s.clone()),
            TemplateNode::ReactiveChild(_) => {
                unreachable!("Cannot clone ReactiveChild closures")
            }
        }
    }
}

impl PartialEq for TemplateNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                TemplateNode::Element {
                    tag: t1,
                    attrs: a1,
                    children: c1,
                },
                TemplateNode::Element {
                    tag: t2,
                    attrs: a2,
                    children: c2,
                },
            ) => t1 == t2 && a1 == a2 && c1 == c2,
            (TemplateNode::Literal(s1), TemplateNode::Literal(s2)) => s1 == s2,
            _ => false,
        }
    }
}

pub enum AttrValue {
    Text(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    Reactive(Rc<dyn Fn() -> AttrValue>),
}

impl std::fmt::Debug for AttrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttrValue::Text(s) => f.debug_tuple("Text").field(s).finish(),
            AttrValue::Float(fl) => f.debug_tuple("Float").field(fl).finish(),
            AttrValue::Int(i) => f.debug_tuple("Int").field(i).finish(),
            AttrValue::Bool(b) => f.debug_tuple("Bool").field(b).finish(),
            AttrValue::Reactive(_) => f.debug_tuple("Reactive").finish(),
        }
    }
}

impl Clone for AttrValue {
    fn clone(&self) -> Self {
        match self {
            AttrValue::Text(s) => AttrValue::Text(s.clone()),
            AttrValue::Float(fl) => AttrValue::Float(*fl),
            AttrValue::Int(i) => AttrValue::Int(*i),
            AttrValue::Bool(b) => AttrValue::Bool(*b),
            AttrValue::Reactive(_) => {
                unreachable!("Cannot clone Reactive closures")
            }
        }
    }
}

impl PartialEq for AttrValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AttrValue::Text(s1), AttrValue::Text(s2)) => s1 == s2,
            (AttrValue::Float(f1), AttrValue::Float(f2)) => f1 == f2,
            (AttrValue::Int(i1), AttrValue::Int(i2)) => i1 == i2,
            (AttrValue::Bool(b1), AttrValue::Bool(b2)) => b1 == b2,
            _ => false,
        }
    }
}

impl quote::ToTokens for AttrValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            AttrValue::Text(s) => {
                quote::quote!(::trs::AttrValue::Text(#s.to_string())).to_tokens(tokens);
            }
            AttrValue::Float(f) => {
                quote::quote!(::trs::AttrValue::Float(#f)).to_tokens(tokens);
            }
            AttrValue::Int(i) => {
                quote::quote!(::trs::AttrValue::Int(#i)).to_tokens(tokens);
            }
            AttrValue::Bool(b) => {
                quote::quote!(::trs::AttrValue::Bool(#b)).to_tokens(tokens);
            }
            AttrValue::Reactive(_) => {
                unreachable!("Reactive AttrValue should not use ToTokens - handled in macro")
            }
        }
    }
}

pub trait IntoAttrValue {
    fn into_attr_value(self) -> AttrValue;
}

impl<T> IntoAttrValue for T
where
    AttrValue: From<T>,
{
    fn into_attr_value(self) -> AttrValue {
        AttrValue::from(self)
    }
}

impl From<i64> for AttrValue {
    fn from(v: i64) -> Self {
        AttrValue::Int(v)
    }
}

impl From<i32> for AttrValue {
    fn from(v: i32) -> Self {
        AttrValue::Int(v as i64)
    }
}

impl From<i16> for AttrValue {
    fn from(v: i16) -> Self {
        AttrValue::Int(v as i64)
    }
}

impl From<i8> for AttrValue {
    fn from(v: i8) -> Self {
        AttrValue::Int(v as i64)
    }
}

impl From<u64> for AttrValue {
    fn from(v: u64) -> Self {
        AttrValue::Int(v as i64)
    }
}

impl From<u32> for AttrValue {
    fn from(v: u32) -> Self {
        AttrValue::Int(v as i64)
    }
}

impl From<u16> for AttrValue {
    fn from(v: u16) -> Self {
        AttrValue::Int(v as i64)
    }
}

impl From<u8> for AttrValue {
    fn from(v: u8) -> Self {
        AttrValue::Int(v as i64)
    }
}

impl From<f64> for AttrValue {
    fn from(v: f64) -> Self {
        AttrValue::Float(v)
    }
}

impl From<f32> for AttrValue {
    fn from(v: f32) -> Self {
        AttrValue::Float(v as f64)
    }
}

impl From<String> for AttrValue {
    fn from(v: String) -> Self {
        AttrValue::Text(v)
    }
}

impl From<bool> for AttrValue {
    fn from(v: bool) -> Self {
        AttrValue::Bool(v)
    }
}

impl From<char> for AttrValue {
    fn from(v: char) -> Self {
        AttrValue::Text(v.to_string())
    }
}

impl From<&str> for AttrValue {
    fn from(v: &str) -> Self {
        AttrValue::Text(v.to_string())
    }
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
    impl block {
        const width: &'static str = "width";
    }

    pub struct text;
    impl TurseElement for text {
        const TAG_NAME: &'static str = "text";
    }

    pub struct input;
    impl TurseElement for input {
        const TAG_NAME: &'static str = "input";
    }

    pub struct dropdown;
    impl TurseElement for dropdown {
        const TAG_NAME: &'static str = "dropdown";
    }
}
