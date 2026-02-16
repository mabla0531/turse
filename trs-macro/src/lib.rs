use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, token, Ident, LitBool, LitFloat, LitInt, LitStr, Result, Token,
};
use trs::AttrValue;

#[proc_macro]
pub fn trs(input: TokenStream) -> TokenStream {
    let call = parse_macro_input!(input as TrsCall);
    let output = call.render();
    TokenStream::from(output)
}

struct TrsCall {
    root: Option<Node>,
}

impl Parse for TrsCall {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(TrsCall { root: None });
        }

        Ok(TrsCall {
            root: Some(input.parse()?),
        })
    }
}

struct Node {
    tag: String,
    attrs: HashMap<String, AttrValueExpr>,
    children: Vec<Node>,
    text: Option<String>,
    expr_children: Vec<proc_macro2::TokenStream>,
}

const VALID_ELEMENTS: [&str; 4] = ["block", "text", "input", "dropdown"];

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            return Ok(Node {
                tag: String::new(),
                attrs: HashMap::new(),
                children: Vec::new(),
                text: Some(input.parse::<LitStr>()?.value()),
                expr_children: Vec::new(),
            });
        }

        let name: Ident = input.parse()?;

        let name_str = name.to_string();
        if !VALID_ELEMENTS.contains(&name_str.as_str()) {
            panic!("{} is not a valid tag", name_str);
        }

        let content;
        syn::braced!(content in input);

        let mut attrs = HashMap::new();
        let mut children = Vec::new();
        let mut expr_children = Vec::new();

        while !content.is_empty() {
            if content.peek(Ident) && content.peek2(Token![:]) {
                let attr_name: Ident = content.parse()?;
                content.parse::<Token![:]>()?;
                let attr_value: AttrValueExpr = content.parse()?;
                attrs.insert(attr_name.to_string(), attr_value);
                let _ = content.parse::<Token![,]>();
            } else if content.peek(token::Brace) {
                let expr: syn::Expr = content.parse()?;
                expr_children.push(quote! { #expr });
            } else {
                children.push(content.parse()?);
            }
        }

        Ok(Node {
            tag: name_str,
            attrs,
            children,
            text: None,
            expr_children,
        })
    }
}

enum AttrValueExpr {
    Literal(AttrValue),
    Expr(proc_macro2::TokenStream),
}

impl Parse for AttrValueExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            input
                .parse::<LitStr>()
                .map(|lit| AttrValueExpr::Literal(AttrValue::Text(lit.value())))
        } else if input.peek(LitInt) {
            let int: LitInt = input.parse()?;
            int.base10_parse::<i64>()
                .map(|i| AttrValueExpr::Literal(AttrValue::Int(i)))
        } else if input.peek(LitFloat) {
            let f: LitFloat = input.parse()?;
            f.base10_parse::<f64>()
                .map(|f| AttrValueExpr::Literal(AttrValue::Float(f)))
        } else if input.peek(LitBool) {
            let b: LitBool = input.parse()?;
            Ok(AttrValueExpr::Literal(AttrValue::Bool(b.value())))
        } else if input.peek(token::Brace) || input.peek(Token![if]) || input.peek(Token![match]) {
            let expr: syn::Expr = input.parse()?;
            let expr_tokens = quote! { #expr };
            Ok(AttrValueExpr::Expr(expr_tokens))
        } else {
            Err(input.error("expected attribute value"))
        }
    }
}

impl From<AttrValueExpr> for AttrValue {
    fn from(_parser: AttrValueExpr) -> Self {
        unreachable!()
    }
}

impl TrsCall {
    fn render(&self) -> proc_macro2::TokenStream {
        match &self.root {
            Some(node) => {
                let template = node.render();
                quote! {
                    ::trs::VNode::new(#template)
                }
            }
            None => {
                quote! {
                    ::trs::VNode::empty()
                }
            }
        }
    }
}

impl AttrValueExpr {
    fn render(&self) -> proc_macro2::TokenStream {
        match self {
            AttrValueExpr::Literal(lit) => quote! { #lit },
            AttrValueExpr::Expr(expr) => {
                quote! { ::trs::AttrValue::Reactive(std::rc::Rc::new(move || ::trs::IntoAttrValue::into_attr_value(#expr))) }
            }
        }
    }
}

impl Node {
    fn render(&self) -> proc_macro2::TokenStream {
        if self.text.is_some() {
            let text = self.text.as_ref().unwrap();
            return quote! {
                ::trs::TemplateNode::Literal(#text.to_string())
            };
        }

        let tag = &self.tag;
        let children: Vec<_> = self.children.iter().map(|c| c.render()).collect();
        let expr_children: Vec<_> = self
            .expr_children
            .iter()
            .map(|e| quote! { ::trs::TemplateNode::Child(std::boxed::Box::new(#e) as std::boxed::Box<dyn std::fmt::Display>) })
            .collect();

        let all_children = [children, expr_children].concat();

        let mut attrs_expr = quote! {
            std::collections::HashMap::new()
        };
        for (k, v) in &self.attrs {
            let v = v.render();
            attrs_expr = quote! {
                {
                    let mut m = #attrs_expr;
                    m.insert(#k.to_string(), #v);
                    m
                }
            };
        }

        quote! {
            ::trs::TemplateNode::Element {
                tag: #tag.to_string(),
                attrs: #attrs_expr,
                children: vec![#(#all_children),*],
            }
        }
    }
}
