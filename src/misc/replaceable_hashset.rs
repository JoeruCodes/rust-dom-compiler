use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct ReplaceableHashSet<T>
where
    T: PartialEq + Eq + Hash,
{
    map: HashMap<T, ()>,
}

impl<T> ReplaceableHashSet<T>
where
    T: PartialEq + Eq + Hash,
{
    pub fn new() -> Self {
        ReplaceableHashSet {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.map.remove(&value);
        self.map.insert(value, ());
    }

    pub fn contains(&self, value: &T) -> bool {
        self.map.contains_key(value)
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.map.remove(value).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.map.keys()
    }
}

// Implementing PartialEq for ReplaceableHashSet
impl<T> PartialEq for ReplaceableHashSet<T>
where
    T: PartialEq + Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.map.keys().collect::<std::collections::HashSet<_>>() == other.map.keys().collect::<std::collections::HashSet<_>>()
    }
}

// Implementing Eq for ReplaceableHashSet
impl<T> Eq for ReplaceableHashSet<T> where T: PartialEq + Eq + Hash {}

#[macro_export]
macro_rules! replaceable_hash_set {
    ( $( $x:expr ),* ) => {
        {
            let mut set = crate::misc::replaceable_hashset::ReplaceableHashSet::new();
            $(
                set.insert($x);
            )*
            set
        }
    };
}