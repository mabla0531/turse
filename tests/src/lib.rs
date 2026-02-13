#[cfg(test)]
mod tests {
    use trs::{TemplateNode, VNode, elements::ElementTag};
    use trs_macro::trs;
    #[test]
    fn test_trs_macro() {
        let document = trs! {
            block {
                text {
                    "Hello,"
                }
                text {
                    "World!"
                }
            }
        };

        assert_eq!(
            document,
            VNode::new(TemplateNode::Element {
                tag: ElementTag::block,
                children: vec![
                    TemplateNode::Element {
                        tag: ElementTag::text,
                        children: vec![TemplateNode::Literal("Hello,".to_string())]
                    },
                    TemplateNode::Element {
                        tag: ElementTag::text,
                        children: vec![TemplateNode::Literal("World!".to_string())]
                    }
                ]
            })
        );
    }
}
