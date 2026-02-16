#[cfg(test)]
mod tests {
    use trs::{AttrValue, Node};
    use trs_macro::trs;

    #[test]
    fn test_trs_macro() {
        let _document = trs! {
            block {
                class: "",
                id: if true { 1 } else { 2 },
                width: 100,
                text {}
                text {}
                ""
            }
        };
    }

    #[test]
    fn test_empty_inner() {
        let document = trs! {};
        assert!(document.inner.is_none());
    }

    #[test]
    fn test_text_node() {
        let document = trs! { "hello world" };
        let inner = document.inner.unwrap();
        match inner {
            Node::Body(text) => assert_eq!(text, "hello world"),
            _ => panic!("expected Body node"),
        }
    }

    #[test]
    fn test_block_element() {
        let document = trs! {
            block {
                id: "my-block",
                class: "container"
            }
        };
        let inner = document.inner.unwrap();
        match inner {
            Node::Element {
                tag,
                attrs,
                children,
            } => {
                assert_eq!(tag, "block");
                assert_eq!(
                    attrs.get("id"),
                    Some(&AttrValue::Text("my-block".to_string()))
                );
                assert_eq!(
                    attrs.get("class"),
                    Some(&AttrValue::Text("container".to_string()))
                );
                assert!(children.is_empty());
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_nested_elements() {
        let document = trs! {
            block {
                text {
                    "inner text"
                }
            }
        };
        let inner = document.inner.unwrap();
        match inner {
            Node::Element { tag, children, .. } => {
                assert_eq!(tag, "block");
                assert_eq!(children.len(), 1);
                match &children[0] {
                    Node::Element { tag: inner_tag, .. } => {
                        assert_eq!(inner_tag, "text");
                    }
                    _ => panic!("expected nested text element"),
                }
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_all_valid_elements() {
        let _ = trs! { block { } };
        let _ = trs! { text { } };
        let _ = trs! { input { } };
        let _ = trs! { dropdown { } };
    }

    #[test]
    fn test_integer_attribute() {
        let document = trs! {
            input {
                value: 42,
                max: 100
            }
        };
        let inner = document.inner.unwrap();
        match inner {
            Node::Element { attrs, .. } => {
                assert_eq!(attrs.get("value"), Some(&AttrValue::Int(42)));
                assert_eq!(attrs.get("max"), Some(&AttrValue::Int(100)));
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_float_attribute() {
        let document = trs! {
            input {
                price: 19.99,
                discount: 0.5
            }
        };
        let inner = document.inner.unwrap();
        match inner {
            Node::Element { attrs, .. } => {
                assert_eq!(attrs.get("price"), Some(&AttrValue::Float(19.99)));
                assert_eq!(attrs.get("discount"), Some(&AttrValue::Float(0.5)));
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_boolean_attribute() {
        let document = trs! {
            input {
                disabled: true,
                readonly: false
            }
        };
        let inner = document.inner.unwrap();
        match inner {
            Node::Element { attrs, .. } => {
                assert_eq!(attrs.get("disabled"), Some(&AttrValue::Bool(true)));
                assert_eq!(attrs.get("readonly"), Some(&AttrValue::Bool(false)));
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_if_expression_attribute() {
        let document = trs! {
            block {
                id: if true { "visible" } else { "hidden" }
            }
        };
        let inner = document.inner.unwrap();
        match inner {
            Node::Element { attrs, .. } => {
                let id = attrs.get("id").unwrap();
                match id {
                    AttrValue::Expr(_) => {}
                    _ => panic!("expected Expr attribute"),
                }
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_match_expression_attribute() {
        let document = trs! {
            block {
                class: match 2 {
                    1 => "one",
                    2 => "two",
                    _ => "other"
                }
            }
        };
        let inner = document.inner.unwrap();
        match inner {
            Node::Element { attrs, .. } => {
                assert!(matches!(attrs.get("class"), Some(AttrValue::Expr(_))));
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_expression_child() {
        let x = 10;
        let document = trs! {
            block {
                { x * 2 }
            }
        };
        let inner = document.inner.unwrap();
        match inner {
            Node::Element { children, .. } => {
                assert_eq!(children.len(), 1);
                match &children[0] {
                    Node::Body(s) => {
                        assert_eq!(s, "20");
                    }
                    _ => panic!("expected Body node"),
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
        let inner = document.inner.unwrap();
        match inner {
            Node::Element {
                tag,
                attrs: _,
                children,
            } => {
                assert_eq!(tag, "block");
                assert_eq!(children.len(), 1);
                match &children[0] {
                    Node::Body(d) => {
                        let result = d.to_string();
                        assert_eq!(result, "a");
                    }
                    _ => panic!("child should be Child"),
                }
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_multiple_children() {
        let document = trs! {
            block {
                "first"
                text { }
                "second"
                block { }
                "third"
            }
        };
        let inner = document.inner.unwrap();
        match inner {
            Node::Element { children, .. } => {
                assert_eq!(children.len(), 5);
            }
            _ => panic!("expected Element"),
        }
    }

    #[test]
    fn test_mixed_children() {
        let count = 5;
        let document = trs! {
            block {
                "static text"
                { count.to_string() }
                text {
                    name: "child"
                }
            }
        };
        let inner = document.inner.unwrap();
        match inner {
            Node::Element { children, .. } => {
                assert_eq!(children.len(), 3);
            }
            _ => panic!("expected Element"),
        }
    }
}
