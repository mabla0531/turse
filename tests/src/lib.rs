#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use trs::{AttrValue, TemplateNode, VNode};
    use trs_macro::trs;
    #[test]
    fn test_trs_macro() {
        let a = 1;
        let document = trs! {
            block {
                class: "",
                id: "",
                width: if true { a } else { 31 },
                text {}
                text {}
                ""
            }
        };

        let mut attrs: HashMap<String, AttrValue> = HashMap::new();
        attrs.insert("class".to_string(), AttrValue::Text("".to_string()));
        attrs.insert("id".to_string(), AttrValue::Text("".to_string()));
        attrs.insert("width".to_string(), AttrValue::Int(1));

        assert_eq!(
            document,
            VNode::new(TemplateNode::Element {
                tag: "block".to_string(),
                attrs,
                children: vec![
                    TemplateNode::Element {
                        tag: "text".to_string(),
                        attrs: HashMap::new(),
                        children: vec![]
                    },
                    TemplateNode::Element {
                        tag: "text".to_string(),
                        attrs: HashMap::new(),
                        children: vec![]
                    },
                    TemplateNode::Literal("".to_string())
                ]
            })
        );
    }
}
