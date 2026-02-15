#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::rc::Rc;

    use trs::{AttrValue, TemplateNode};
    use trs_macro::trs;
    #[test]
    fn test_trs_macro() {
        let document = trs! {
            block {
                class: "",
                id: if true { 1 } else { 2 },
                width: 100,
                text {}
                text {}
                ""
            }
        };

        let mut attrs: HashMap<String, AttrValue> = HashMap::new();
        attrs.insert("class".to_string(), AttrValue::Text("".to_string()));
        attrs.insert(
            "id".to_string(),
            AttrValue::Reactive(Rc::new(move || AttrValue::Int(if true { 1 } else { 2 }))),
        );
        attrs.insert("width".to_string(), AttrValue::Int(100));

        let template = document.template.unwrap();
        match template {
            TemplateNode::Element {
                tag,
                attrs: got_attrs,
                children,
            } => {
                assert_eq!(tag, "block");
                assert_eq!(
                    got_attrs.get("class"),
                    Some(&AttrValue::Text("".to_string()))
                );
                assert_eq!(got_attrs.get("width"), Some(&AttrValue::Int(100)));
                if let Some(AttrValue::Reactive(_)) = got_attrs.get("id") {
                    // Expected - id is reactive
                } else {
                    panic!("id should be reactive");
                }
                assert_eq!(children.len(), 3);
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_expr_property() {
        let document = trs! {
            block {
                width: {let a = 3; a},
            }
        };
        let template = document.template.unwrap();
        match template {
            TemplateNode::Element {
                tag,
                attrs,
                children: _,
            } => {
                assert_eq!(tag, "block");
                let width = attrs.get("width").expect("width should exist");
                match width {
                    AttrValue::Reactive(f) => {
                        let result = f();
                        assert_eq!(result, AttrValue::Int(3));
                    }
                    _ => panic!("width should be reactive"),
                }
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_expr_child() {
        let document = trs! {
            block {
                {"hello"}
            }
        };
        let template = document.template.unwrap();
        match template {
            TemplateNode::Element {
                tag,
                attrs: _,
                children,
            } => {
                assert_eq!(tag, "block");
                assert_eq!(children.len(), 1);
                match &children[0] {
                    TemplateNode::ReactiveChild(f) => {
                        let result = f();
                        assert_eq!(result, TemplateNode::Literal("hello".to_string()));
                    }
                    _ => panic!("child should be ReactiveChild"),
                }
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_expr_child_string() {
        let document = trs! {
            block {
                {let a = "aa".to_string(); a.split_at(1).0.to_string()}
            }
        };
        let template = document.template.unwrap();
        match template {
            TemplateNode::Element {
                tag,
                attrs: _,
                children,
            } => {
                assert_eq!(tag, "block");
                assert_eq!(children.len(), 1);
                match &children[0] {
                    TemplateNode::ReactiveChild(f) => {
                        let result = f();
                        assert_eq!(result, TemplateNode::Literal("a".to_string()));
                    }
                    _ => panic!("child should be ReactiveChild"),
                }
            }
            _ => panic!("expected Element"),
        }
    }
}
