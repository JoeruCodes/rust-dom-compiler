use crate::html_ops::html::Html;

pub enum Diff<'a> {
    Added { html: &'a Html, index: Vec<usize> },
    Removed { html: &'a Html, index: Vec<usize> },
    Modified { old: &'a Html, new: &'a Html, index: Vec<usize> },
    Unchanged { html: &'a Html, index: Vec<usize> },
}

pub fn diff_trees<'a>(old: &'a Html, new: &'a Html, index: Vec<usize>) -> Vec<Diff<'a>> {
    let mut diffs = Vec::new();

    // Check if the root nodes are different
    if old.tag() != new.tag() || old.get_atrs() != new.get_atrs() {
        diffs.push(Diff::Modified {
            old,
            new,
            index: index.clone(),
        });
    } else {
        diffs.push(Diff::Unchanged {
            html: old,
            index: index.clone(),
        });
    }

    let old_children = old.get_children();
    let new_children = new.get_children();
    let min_length = std::cmp::min(old_children.len(), new_children.len());

    // Compare children
    for i in 0..min_length {
        let mut new_index = index.clone();
        new_index.push(i);
        diffs.extend(diff_trees(&old_children[i], &new_children[i], new_index));
    }

    // Handle added nodes
    for i in min_length..new_children.len() {
        let mut new_index = index.clone();
        new_index.push(i);
        diffs.push(Diff::Added {
            html: &new_children[i],
            index: new_index,
        });
    }

    // Handle removed nodes
    for i in min_length..old_children.len() {
        let mut new_index = index.clone();
        new_index.push(i);
        diffs.push(Diff::Removed {
            html: &old_children[i],
            index: new_index,
        });
    }

    diffs
}

// Function to traverse the tree based on the given index
pub fn traverse_tree<'a>(root: &'a Html, index: &[usize]) -> Option<&'a Html> {
    let mut current_node = root;
    for &i in index {
        if let Some(child) = current_node.get_children().get(i) {
            current_node = child;
        } else {
            return None; // Index path is invalid
        }
    }
    Some(current_node)
}
