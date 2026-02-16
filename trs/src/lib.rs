use std::{collections::HashMap, fmt::Display};

pub trait TurseElement {
    const TAG_NAME: &'static str;
}

pub enum Node {
    Element {
        tag: String,
        attrs: HashMap<String, AttrValue>,
        children: Vec<Node>,
    },
    Body(String),
}

pub trait IntoNode {
    fn into_template_node(self) -> Node;
}

impl IntoNode for String {
    fn into_template_node(self) -> Node {
        Node::Body(self)
    }
}

impl IntoNode for &str {
    fn into_template_node(self) -> Node {
        Node::Body(self.to_string())
    }
}

impl IntoNode for Node {
    fn into_template_node(self) -> Node {
        self
    }
}

impl From<String> for Node {
    fn from(s: String) -> Self {
        Node::Body(s)
    }
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        Node::Body(s.to_string())
    }
}

impl<T> From<Vec<T>> for Node
where
    T: IntoNode,
{
    fn from(vec: Vec<T>) -> Self {
        Node::Element {
            tag: "fragment".to_string(),
            attrs: HashMap::new(),
            children: vec.into_iter().map(|t| t.into_template_node()).collect(),
        }
    }
}

#[cfg(debug_assertions)]
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Element {
                tag,
                attrs,
                children,
            } => f
                .debug_struct("Element")
                .field("tag", tag)
                .field("attrs", attrs)
                .field("children", children)
                .finish(),
            Node::Body(s) => f.debug_tuple("Body").field(&s.to_string()).finish(),
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        match self {
            Node::Element {
                tag,
                attrs,
                children,
            } => Node::Element {
                tag: tag.clone(),
                attrs: attrs.clone(),
                children: children.clone(),
            },
            Node::Body(s) => Node::Body(s.clone()),
        }
    }
}

#[cfg(debug_assertions)]
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Node::Element {
                    tag: t1,
                    attrs: a1,
                    children: c1,
                },
                Node::Element {
                    tag: t2,
                    attrs: a2,
                    children: c2,
                },
            ) => t1 == t2 && a1 == a2 && c1 == c2,
            (Node::Body(s1), Node::Body(s2)) => s1 == s2,
            _ => false,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub enum AttrValue {
    Text(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    Expr(fn() -> Box<dyn Display>),
}

#[cfg(debug_assertions)]
impl PartialEq for AttrValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Text(lv), Self::Text(rv)) => lv == rv,
            (Self::Float(lv), Self::Float(rv)) => lv == rv,
            (Self::Int(lv), Self::Int(rv)) => lv == rv,
            (Self::Bool(lv), Self::Bool(rv)) => lv == rv,
            _ => false,
        }
    }
}

impl quote::ToTokens for AttrValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            AttrValue::Text(s) => {
                quote::quote!(::trs::AttrValue::Text(#s.to_string())).to_tokens(tokens)
            }
            AttrValue::Float(f) => quote::quote!(::trs::AttrValue::Float(#f)).to_tokens(tokens),
            AttrValue::Int(i) => quote::quote!(::trs::AttrValue::Int(#i)).to_tokens(tokens),
            AttrValue::Bool(b) => quote::quote!(::trs::AttrValue::Bool(#b)).to_tokens(tokens),
            AttrValue::Expr(_) => {
                quote::quote!(::trs::AttrValue::Expr(fn() -> Box<dyn Display>)).to_tokens(tokens)
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

#[cfg_attr(debug_assertions, derive(Debug, PartialEq))]
#[derive(Clone)]
pub struct VNode {
    pub template: Option<Node>,
}

impl VNode {
    pub fn new(template: Node) -> Self {
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
