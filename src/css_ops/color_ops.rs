use std::{hash::{Hash, Hasher}, ops::{Deref, DerefMut}};

use palette::Srgba;

// Define a newtype that wraps `Srgba`
#[derive(PartialEq, Debug)]
pub struct Color(Srgba);

// Implement `Deref` to allow easy access to the inner `Srgba`
impl Deref for Color {
    type Target = Srgba;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implement `DerefMut` to allow mutable access to the inner `Srgba`
impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Implement the `Hash` trait for the newtype
impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{:?}", self.0).hash(state);
    }
}

impl Eq for Color{}