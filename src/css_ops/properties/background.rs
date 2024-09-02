use oxideRS_css_macros::css_property;

use crate::css_ops::color_ops::Color;

use super::helpers::{Position, PositionLiterals};
#[derive(Hash, Debug)]
#[css_property(inherit: true, initial_inherit_wrap: [])]
pub struct Background{
    color: BackgroundColor,
    image: BackgroundImage,
    position: BackgroundPosition,
    size: BackgroundSize,
    repeat: BackgroundRepeat,
    origin: BackgroundOrigin,
    clip: BackgroundOrigin,
    attachement: BackgroundAttachment
}
#[css_property(sister: [], inherit: true)]
pub enum BackgroundColor{
    Color(Color),
    Transparent
}
#[css_property(sister: [], inherit: true)]
pub enum BackgroundImage{
    Url(String),
    None
}

#[css_property(sister: [], inherit: true)]
pub enum BackgroundPosition{
    Literals(PositionLiterals),
    Position(Position),
}

#[css_property(sister: [], inherit: true)]
pub enum BackgroundSize{
    Auto,
    Length(Position),
    Length2((Position, Position)),
    Cover,
    Contain
}
#[css_property(sister: [], inherit: true)]
pub enum BackgroundRepeat{
    Repeat,
    RepeatX,
    RepeatY,
    NoRepeat,
    Space,
    Round
}

#[css_property(sister: [], inherit: true)]
pub enum BackgroundOrigin{
    PaddingBox,
    BorderBox,
    ContentBox
}

#[css_property(sister: [], inherit: true)]
pub enum BackgroundAttachment{
    Scroll,
    Fixed,
    Local
}