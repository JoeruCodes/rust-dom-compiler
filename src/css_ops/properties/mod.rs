pub mod animation;
pub mod background;
pub mod helpers{
    use std::{f64, hash::Hash};

    #[derive(Hash, PartialEq, Eq, Debug)]
    pub enum Position{
        Pixels(F64),
        Percent(F64),
        Rem(F64),
        Vh(F64)
    }
    #[derive(Hash, PartialEq, Eq, Debug)]
    pub enum PositionLiterals{
        Left,
        Top,
        Bottom,
        Right
    }
    #[derive(PartialEq, Debug)]
    pub struct F64(f64);
    impl Eq for F64{}
    impl Hash for F64{
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            format!("{:?}", self.0).hash(state);
        }
    }
}
