use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitStr, Result,
};

#[proc_macro]
pub fn trs(input: TokenStream) -> TokenStream {
    let call = parse_macro_input!(input as TrsCall);
    let output = call.render();
    TokenStream::from(output)
}

struct TrsCall {
    root: Option<Element>,
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

struct Element {
    name: Ident,
    children: Vec<Node>,
}

const VALID_ELEMENTS: [&str; 2] = ["block", "text"];

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        let name_str = name.to_string();
        if !VALID_ELEMENTS.contains(&name_str.as_str()) {
            panic!("{} is not a valid tag", name_str);
        }

        let content;
        syn::braced!(content in input);

        let mut children = Vec::new();
        while !content.is_empty() {
            children.push(content.parse()?);
        }

        Ok(Element { name, children })
    }
}

enum Node {
    Element(Element),
    Text(LitStr),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            return Ok(Node::Text(input.parse()?));
        }

        Ok(Node::Element(input.parse()?))
    }
}

impl TrsCall {
    fn render(&self) -> proc_macro2::TokenStream {
        match &self.root {
            Some(element) => {
                let template = element.render();
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

impl Element {
    fn render(&self) -> proc_macro2::TokenStream {
        let tag_ident = Ident::new(&self.name.to_string(), self.name.span());
        let children: Vec<_> = self.children.iter().map(|c| c.render()).collect();

        quote! {
            ::trs::TemplateNode::Element {
                tag: ::trs::elements::ElementTag::#tag_ident,
                children: vec![#(#children),*],
            }
        }
    }
}

impl Node {
    fn render(&self) -> proc_macro2::TokenStream {
        match self {
            Node::Element(el) => el.render(),
            Node::Text(lit) => {
                quote! {
                    ::trs::TemplateNode::Literal(#lit.to_string())
                }
            }
        }
    }
}
