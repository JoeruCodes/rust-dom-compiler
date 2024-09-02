use std::{ops::Deref, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement};
use crate::{html_ops::html::{DirAttribute, GlobalAttributes, Html}, misc::replaceable_hashset::ReplaceableHashSet};
use wasm_bindgen::JsCast;
impl Html {
    pub fn generate_page(&self) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let generated_element_tree = self._generate_page(&document)?;
        document.body().expect("Body element not found").append_child(&generated_element_tree)?;
        Ok(())
    }

    fn _generate_page(&self, document: &Document) -> Result<Element, JsValue> {
        match self {
            Self::Div{g_atr_tok, children} => {
                Self::create_element_with_children(document, "div", g_atr_tok, children)
            },
            Self::H1{g_atr_tok, children} => {
                Self::create_element_with_children(document, "h1", g_atr_tok, children)
            },
            Self::H2{g_atr_tok, children} => {
                Self::create_element_with_children(document, "h2", g_atr_tok, children)
            },
            Self::H3{g_atr_tok, children} => {
                Self::create_element_with_children(document, "h3",g_atr_tok, children)
            },
            Self::H4{g_atr_tok, children} => {
                Self::create_element_with_children(document, "h4", g_atr_tok, children)
            },
            Self::H5{g_atr_tok, children} => {
                Self::create_element_with_children(document, "h5", g_atr_tok, children)
            },
            Self::H6{g_atr_tok, children} => {
                Self::create_element_with_children(document, "h6", g_atr_tok, children)
            },
            Self::P{g_atr_tok, children} => {
                Self::create_element_with_children(document, "p", g_atr_tok, children)
            },
        }
    }

    fn create_element_with_children(document: &Document, tag_name: &str, attributes: &ReplaceableHashSet<GlobalAttributes>, children: &Vec<Html>) -> Result<Element, JsValue> {
        let parent_element = document.create_element(tag_name)?;
        for atr in attributes.iter(){
            atr.set_atr(&parent_element.clone().dyn_into::<HtmlElement>()?)?;
        }
        for child in children {
            let child_element = child._generate_page(document)?;
            parent_element.append_child(&child_element)?;
        }
        Ok(parent_element)
    }
}

