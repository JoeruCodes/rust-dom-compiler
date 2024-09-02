use std::time::Instant;

use oxideRS_css_macros::{css_property, ExportField};
use super::helpers::F64;

#[css_property(inherit : true, initial_inherit_wrap : ["name", "duration", "iter_count", "delay"])]
#[derive(Hash, Debug)]
pub struct Animation{
    name: String,
    duration: Instant,
    timing_function: AnimationTimingFunction,
    delay: Instant,
    iter_count: usize,
    direction: AnimationDirection,
    fill_mode: AnimationFillMode,
    play_state: AnimationPlayState
}

#[css_property(sister : [], inherit : true)]
pub enum AnimationTimingFunction{
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    StepStart,
    StepEnd,
    Steps(usize, StartEnd),
    CubicBezier(F64, F64, F64, F64)
}


#[derive(Hash, PartialEq, Debug, Eq)]
pub enum StartEnd{
    Start,
    End
}

#[css_property(sister : [], inherit : true)]
pub enum AnimationDirection{
    Normal,
    Reverse,
    Alternate,
    AlternateReverse
}

#[css_property(sister : [], inherit : true)]
pub enum AnimationFillMode{
    None,
    Forwards,
    Backwards,
    Both
}

#[css_property(sister : [], inherit : true)]
pub enum AnimationPlayState{
    Paused,
    Running
}