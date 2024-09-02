use diffing::{diff_trees, Diff};

use crate::{html_ops::html::{GlobalAttributes, Html}, replaceable_hash_set};

pub mod diffing;

#[test]
pub fn diffing_test(){
    let child = Html::P{g_atr_tok: replaceable_hash_set![GlobalAttributes::Id("hi".to_string())], children: vec![]};
    let root = Html::Div{g_atr_tok: replaceable_hash_set![], children: vec![child]};
    let root_old =  Html::Div{g_atr_tok: replaceable_hash_set![], children: vec![]};

    let differences = diff_trees(&root_old, &root, vec![]);

    for diff in differences {
        match diff {
            Diff::Added { html, index } => println!("Added at {:?}: {:?}", index, html),
            Diff::Removed { html, index } => println!("Removed at {:?}: {:?}", index, html),
            Diff::Modified { old, new, index } => println!("Modified at {:?}: {:?} -> {:?}", index, old, new),
            Diff::Unchanged { html, index } => println!("Unchanged at {:?}: {:?}", index, html),
        }
    }
}