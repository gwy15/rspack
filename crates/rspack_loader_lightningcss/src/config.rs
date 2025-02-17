use lightningcss::targets::Browsers;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Draft {
  pub custom_media: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonStandard {
  pub deep_selector_combinator: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PseudoClasses {
  pub hover: Option<String>,
  pub active: Option<String>,
  pub focus: Option<String>,
  pub focus_visible: Option<String>,
  pub focus_within: Option<String>,
}

#[derive(Debug, Default)]
pub struct Config {
  pub error_recovery: Option<bool>,
  pub targets: Option<Browsers>,
  pub include: Option<u32>,
  pub exclude: Option<u32>,
  pub draft: Option<Draft>,
  pub non_standard: Option<NonStandard>,
  pub pseudo_classes: Option<PseudoClasses>,
  pub unused_symbols: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RawConfig {
  pub error_recovery: Option<bool>,
  pub targets: Option<Vec<String>>,
  pub include: Option<u32>,
  pub exclude: Option<u32>,
  pub draft: Option<Draft>,
  pub non_standard: Option<NonStandard>,
  pub pseudo_classes: Option<PseudoClasses>,
  pub unused_symbols: Option<Vec<String>>,
}

impl TryFrom<RawConfig> for Config {
  type Error = rspack_error::Error;
  fn try_from(value: RawConfig) -> Result<Self, Self::Error> {
    Ok(Self {
      error_recovery: value.error_recovery,
      targets: value
        .targets
        .map(lightningcss::targets::Browsers::from_browserslist)
        .transpose()
        .map_err(|err| rspack_error::error!("Failed to parse browserslist: {}", err))?
        .flatten(),
      include: value.include,
      exclude: value.exclude,
      draft: value.draft,
      non_standard: value.non_standard,
      pseudo_classes: value.pseudo_classes,
      unused_symbols: value.unused_symbols,
    })
  }
}
