pub mod replaceable_hashset;

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;
    use crate::{html_ops::html::GlobalAttributes, replaceable_hash_set};
    use log::info;
    use wasm_logger;

    #[wasm_bindgen_test]
    fn test_replaceable_hashset() {
        wasm_logger::init(wasm_logger::Config::default());

        let r_hs = replaceable_hash_set!(
            GlobalAttributes::Id("hi".to_string()),
            GlobalAttributes::Accesskey("hiiii".to_string()),
            GlobalAttributes::Id("hello".to_string())
        );

        info!("{:?}", r_hs);

        assert_eq!(
            r_hs,
            replaceable_hash_set!(
                GlobalAttributes::Id("hello".to_string()),
                GlobalAttributes::Accesskey("hiiii".to_string())
            )
        );
    }
}