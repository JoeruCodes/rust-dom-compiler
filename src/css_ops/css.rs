use std::{hash::Hash, time::Instant};

use wasm_bindgen::JsValue;
use web_sys::{Element};
use oxideRS_css_macros::css_property;

use super::{color_ops::Color, properties::{animation::{AnimationDirection, AnimationFillMode, AnimationInherited, AnimationPlayState, AnimationTimingFunction, DelayInherited, DurationInherited, IterCountInherited, NameInherited}, background::{Background, BackgroundAttachment, BackgroundInherited}}};
use crate::css_ops::properties::helpers::F64;


#[derive(Hash, Debug)]
pub enum CssProperties {
    AccentColor(AccentColor),
    AlignContent(AlignContent),
    AlignItems(AlignItems),
    AlignSelf(AlignSelf),
    All(All),
    Animation(AnimationInherited),
    AnimationDelay(DelayInherited),
    AnimationDirection(AnimationDirection),
    AnimationDuration(DurationInherited),
    AnimationFillMode(AnimationFillMode),
    AnimationIterCount(IterCountInherited),
    AnimationName(NameInherited),
    AnimationPlayState(AnimationPlayState),
    AnimationTimingFunction(AnimationTimingFunction),
    AspectRatio(AspectRatioInherited),
    BackdropFilter(BackdropFilter),
    BackfaceVisibility(BackfaceVisibility),
    Background(BackgroundInherited),
    BackgroundAttachment(BackgroundAttachment),
    BackgroundBlenderMode()
}
#[derive(Debug, Hash)]
#[css_property(inherit : true, initial_inherit_wrap : [])]
struct AspectRatio(F64, F64);
#[css_property(sister : [], inherit : true)]
enum All{
    Unset
}
#[css_property(sister: [], inherit: true)]
enum BackdropFilter{
    None,
    Blur,
    Brightness,
    Contrast,
    DropShadow,
    Grayscale,
    HueRotated,
    Invert,
    Opacity,
    Sepia,
    Saturate,
    Url(String)
}
#[css_property(sister: [], inherit: true)]
enum BackfaceVisibility{
    Visible,
    Hidden
}
#[css_property(sister : [], inherit : true)]
enum AlignSelf{
    Auto,
    Stretch,
    Center,
    FlexStart,
    FlexEnd,
    Baseline,
}
#[css_property(sister : [], inherit : true)]
enum AlignItems{
    Normal,
    Stretch,
    Center,
    FlexStart,
    FlexEnd,
    Start,
    Baseline
}
#[css_property(sister : [], inherit : true)]
enum AlignContent{
    Stretch,
    Center,
    FlexStart,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly
}
#[css_property(sister : [], inherit : true)]
enum AccentColor {
    Auto,
    Color(Color),
}
pub trait RenderCss{
    fn hydrate(&self, element: &Element) -> Result<(), JsValue>;
}
impl RenderCss for &CssProperties {
    fn hydrate(&self, element: &Element) ->Result<(), JsValue> {
        todo!()
    }
}
