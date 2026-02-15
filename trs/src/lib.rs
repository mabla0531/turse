use std::collections::HashMap;

pub trait TurseElement {
    const TAG_NAME: &'static str;
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateNode {
    Element {
        tag: String,
        attrs: HashMap<String, AttrValue>,
        children: Vec<TemplateNode>,
    },
    Literal(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttrValue {
    Text(String),
    Float(f64),
    Int(i64),
    Bool(bool),
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
