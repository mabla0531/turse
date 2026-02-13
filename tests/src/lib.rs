#[cfg(test)]
mod tests {
    use trs_macro::trs;
    #[test]
    fn test_trs_macro() {
        let my_stuff = trs! {
            goober {}
        };
    }
}
