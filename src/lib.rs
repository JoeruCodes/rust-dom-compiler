use html_ops::html::GlobalAttributes;

mod html_ops;
mod css_ops;
mod wasm_gen;
mod misc;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod update_wasm;
#[cfg(test)]
mod tests {
    use super::*;
    use update_wasm::diffing::{diff_trees, Diff};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;
    use web_sys::{Document, Element};
    use crate::html_ops::html::{Html, GlobalAttributes};
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    #[wasm_bindgen_test]
    fn test_generate_page() {
        // Create a simple HTML structure
        let child = Html::P{g_atr_tok: replaceable_hash_set![GlobalAttributes::Id("hi".to_string())], children: vec![]};
        let root = Html::Div{g_atr_tok: replaceable_hash_set![], children: vec![child]};

        // Call the generate_page method
        root.generate_page().unwrap();

        // Get the document and body elements
        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().expect("Body element not found");

        // Check if the generated elements exist in the DOM
        let div_element = body.query_selector("div").expect("Failed to query selector").expect("Div element not found");
        let p_element = div_element.query_selector("p").expect("Failed to query selector").expect("P element not found");

        // Verify that elements are appended correctly
        assert!(div_element.is_instance_of::<Element>());
        assert!(p_element.is_instance_of::<Element>());
    }
    #[wasm_bindgen_test]
    fn setup() {
        console_error_panic_hook::set_once();
    }

#[test]
pub fn diffing_test(){
    let child = Html::P{g_atr_tok: replaceable_hash_set![GlobalAttributes::Id("hi".to_string())], children: vec![Html::P{g_atr_tok: replaceable_hash_set![GlobalAttributes::Id("hi".to_string())], children: vec![]}]};
    let root = Html::Div{g_atr_tok: replaceable_hash_set![], children: vec![child]};
    let root_old =  Html::Div{g_atr_tok: replaceable_hash_set![], children: vec![Html::P{g_atr_tok: replaceable_hash_set![GlobalAttributes::Id("hello".to_string())], children: vec![]}]};

    let differences = diff_trees(&root_old, &root, vec![]);

    let changes = Html::P{g_atr_tok: replaceable_hash_set![GlobalAttributes::Id("hi".to_string())], children: vec![]};
    let actual_difs = vec![Diff::Added { html: &changes, index: vec![0] }, Diff::Modified { old: &root_old , new: &changes, index: vec![0] }];
    assert_eq!(actual_difs, differences)
}
}