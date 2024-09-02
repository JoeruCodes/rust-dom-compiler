use std::hash::Hash;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{Document, Element, HtmlElement};
use crate::css_ops::css::CssProperties;
use crate::css_ops::css::RenderCss;
use crate::misc::replaceable_hashset::ReplaceableHashSet;

#[derive(PartialEq, Debug)]
pub enum Html{
    Div{g_atr_tok: ReplaceableHashSet<GlobalAttributes>, children: Vec<Html>},
    H6{g_atr_tok: ReplaceableHashSet<GlobalAttributes>, children: Vec<Html>},
    H1{g_atr_tok: ReplaceableHashSet<GlobalAttributes>, children: Vec<Html>},
    H3{g_atr_tok: ReplaceableHashSet<GlobalAttributes>, children: Vec<Html>},
    H4{g_atr_tok: ReplaceableHashSet<GlobalAttributes>, children: Vec<Html>},
    H5{g_atr_tok: ReplaceableHashSet<GlobalAttributes>, children: Vec<Html>},
    H2{g_atr_tok: ReplaceableHashSet<GlobalAttributes>, children: Vec<Html>},
    P{g_atr_tok: ReplaceableHashSet<GlobalAttributes>, children: Vec<Html>},
    
}

impl Html{
    pub fn get_atrs<'a>(&'a self) -> &'a ReplaceableHashSet<GlobalAttributes>{
        match self{
            Html::Div { g_atr_tok, ..} => return g_atr_tok,
            Html::H6 { g_atr_tok, .. } => return g_atr_tok,
            Html::H5 { g_atr_tok, .. } => return g_atr_tok,
            Html::H4 { g_atr_tok, .. } => return g_atr_tok,
            Html::H3 { g_atr_tok, .. } => return g_atr_tok,
            Html::H2 { g_atr_tok, .. } => return g_atr_tok,
            Html::H1 { g_atr_tok, .. } => return g_atr_tok,
            Html::P { g_atr_tok, .. } => return g_atr_tok,
        }
    }

    pub fn get_children<'a>(&'a self) -> &'a Vec<Html>{
        match self{
            Html::Div { children, ..} => return children,
            Html::H6 { children, .. } => return children,
            Html::H5 { children, .. } => return children,
            Html::H4 { children, .. } => return children,
            Html::H3 { children, .. } => return children,
            Html::H2 { children, .. } => return children,
            Html::H1 { children, .. } => return children,
            Html::P { children, .. } => return children,
        }
    }

    pub fn tag<'a>(&'a self) ->&'a str{
        match self{
            Html::Div { ..} => return "DIV",
            Html::H6 { ..} => return "H6",
            Html::H5 { .. } => return "H5",
            Html::H4 { .. } => return "H4",
            Html::H3 { .. } => return "H3",
            Html::H2 { .. } => return "H2",
            Html::H1 { .. } => return "H1",
            Html::P { .. } => return "P",
        }
    } 
}
#[derive(Debug)]
pub enum GlobalAttributes{
    Accesskey(Html_ID),
    Class(Vec<CssProperties>),
    ContentEditable(bool),
    _Data(HashMap<String, String>),
    Dir(DirAttribute),
    Dragable(bool), 
    EnterKeyHint(String),
    Hidden(bool),
    Id(Html_ID),
    Inert(bool),
    InputMode(InputMode),
    Lang(String), //change this to enum lang code references
    PopOver(Html_ID), //construct here
    PopOverTarget(Html_ID), // consume here may require RC
    SpellCheck(bool),
    TabIndex(i32),
    Title(String),
}

type Html_ID = String;

impl GlobalAttributes{
    pub fn set_atr(&self, element: &HtmlElement) -> Result<(), JsValue> {
        match self{
            Self::Accesskey(id) => element.set_access_key(&id),
            Self::Dragable(is_draggable) => element.set_draggable(*is_draggable),
            Self::Hidden(is_hidden) => element.set_hidden(*is_hidden),
            Self::ContentEditable(is_content_editable) => element.set_content_editable(&format!("{}", is_content_editable)),
            Self::_Data(data) => {
                for (k, v) in data{
                    element.set_attribute(k, v)?;
                }
            },
            Self::Dir(dir) => element.set_dir(dir.as_str()),
            Self::EnterKeyHint(hint) => element.set_attribute("enterkeyhint", &hint)?,
            Self::Id(id) => element.set_id(id),
            Self::Inert(inert) => element.set_inert(*inert),
            Self::InputMode(input) => element.set_attribute("inputmode", input.as_str())?,
            Self::Lang(lang) => element.set_lang(lang),
            Self::PopOver(p_o) => element.set_attribute("popover", &p_o)?,
            Self::PopOverTarget(p_oT) => element.set_attribute("popover-target", &p_oT)?,
            Self::SpellCheck(spellcheck) => element.set_spellcheck(*spellcheck),
            Self::TabIndex(tab_index) => element.set_tab_index(*tab_index),
            Self::Title(title) => element.set_title(&title),
            Self::Class(class) => {
                for toks in class{
                    toks.hydrate(element)?
                }
            }
        }
    
        Ok(())
    }
}
impl Hash for GlobalAttributes{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self{
            GlobalAttributes::Accesskey(_) => "g_access_key".hash(state),
            GlobalAttributes::Class(class) => class.hash(state),
            GlobalAttributes::ContentEditable(_) =>  "g_content_editable".hash(state),
            GlobalAttributes::Dir(_) => "g_dir".hash(state),
            GlobalAttributes::Dragable(_) => "g_draggable".hash(state),
            GlobalAttributes::EnterKeyHint(_) => "g_enter_key_hint".hash(state),
            GlobalAttributes::Hidden(_) => "g_hidden".hash(state),
            GlobalAttributes::Id(_) => "g_id".hash(state),
            GlobalAttributes::Inert(_) => "g_inert".hash(state),
            GlobalAttributes::InputMode(_) => "g_input_mode".hash(state),
            GlobalAttributes::Lang(_) => "g_lang".hash(state),
            GlobalAttributes::PopOver(_) => "g_pop_over".hash(state),
            GlobalAttributes::PopOverTarget(_) => "g_pop_over_trgt".hash(state),
            GlobalAttributes::SpellCheck(_) => "g_spell_check".hash(state),
            GlobalAttributes::TabIndex(_) => "g_tab_index".hash(state),
            GlobalAttributes::Title(_) => "g_title".hash(state),
            GlobalAttributes::_Data(_) => "g__data".hash(state)
        }
    }
}

impl PartialEq for GlobalAttributes {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for GlobalAttributes {}
#[derive(PartialEq, Eq, Debug)]
pub enum DirAttribute{
    LTR,
    RTL,
    AUTO
}

#[derive(PartialEq, Eq, Debug)]
pub enum InputMode{
    DECIMAL,
    EMAIL,
    NONE,
    NUMERIC,
    SEARCH,
    TEL,
    TEXT,
    URL
}
impl DirAttribute {
    pub fn as_str(&self) -> &str {
        match self {
            DirAttribute::LTR => "ltr",
            DirAttribute::RTL => "rtl",
            DirAttribute::AUTO => "auto",
        }
    }
}
impl InputMode {
    pub fn as_str(&self) -> &str {
        match self {
            InputMode::NONE => "none",
            InputMode::TEXT => "text",
            InputMode::DECIMAL => "decimal",
            InputMode::NUMERIC => "numeric",
            InputMode::TEL => "tel",
            InputMode::SEARCH => "search",
            InputMode::EMAIL => "email",
            InputMode::URL => "url",
        }
    }
}