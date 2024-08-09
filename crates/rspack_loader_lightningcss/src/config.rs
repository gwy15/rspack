use lightningcss::targets::Browsers;
use rspack_cacheable::{
  cacheable,
  with::{AsOption, AsPreset},
};
use serde::Deserialize;

#[cacheable]
#[derive(Debug, Deserialize)]
pub struct Draft {
  pub custom_media: bool,
}

#[cacheable]
#[derive(Debug, Deserialize)]
pub struct NonStandard {
  pub deep_selector_combinator: bool,
}

#[cacheable]
#[derive(Debug, Deserialize)]
pub struct PseudoClasses {
  pub hover: Option<String>,
  pub active: Option<String>,
  pub focus: Option<String>,
  pub focus_visible: Option<String>,
  pub focus_within: Option<String>,
}

#[cacheable]
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Config {
  pub error_recovery: Option<bool>,
  #[with(AsOption<AsPreset>)]
  pub targets: Option<Browsers>,
  pub include: Option<u32>,
  pub exclude: Option<u32>,
  pub draft: Option<Draft>,
  pub non_standard: Option<NonStandard>,
  pub pseudo_classes: Option<PseudoClasses>,
  pub unused_symbols: Option<Vec<String>>,
}
