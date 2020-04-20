/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdStatsMining : mining object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdStatsMining {
  /// drone_mine integer
  #[serde(rename = "drone_mine")]
  drone_mine: Option<i64>,
  /// ore_arkonor integer
  #[serde(rename = "ore_arkonor")]
  ore_arkonor: Option<i64>,
  /// ore_bistot integer
  #[serde(rename = "ore_bistot")]
  ore_bistot: Option<i64>,
  /// ore_crokite integer
  #[serde(rename = "ore_crokite")]
  ore_crokite: Option<i64>,
  /// ore_dark_ochre integer
  #[serde(rename = "ore_dark_ochre")]
  ore_dark_ochre: Option<i64>,
  /// ore_gneiss integer
  #[serde(rename = "ore_gneiss")]
  ore_gneiss: Option<i64>,
  /// ore_harvestable_cloud integer
  #[serde(rename = "ore_harvestable_cloud")]
  ore_harvestable_cloud: Option<i64>,
  /// ore_hedbergite integer
  #[serde(rename = "ore_hedbergite")]
  ore_hedbergite: Option<i64>,
  /// ore_hemorphite integer
  #[serde(rename = "ore_hemorphite")]
  ore_hemorphite: Option<i64>,
  /// ore_ice integer
  #[serde(rename = "ore_ice")]
  ore_ice: Option<i64>,
  /// ore_jaspet integer
  #[serde(rename = "ore_jaspet")]
  ore_jaspet: Option<i64>,
  /// ore_kernite integer
  #[serde(rename = "ore_kernite")]
  ore_kernite: Option<i64>,
  /// ore_mercoxit integer
  #[serde(rename = "ore_mercoxit")]
  ore_mercoxit: Option<i64>,
  /// ore_omber integer
  #[serde(rename = "ore_omber")]
  ore_omber: Option<i64>,
  /// ore_plagioclase integer
  #[serde(rename = "ore_plagioclase")]
  ore_plagioclase: Option<i64>,
  /// ore_pyroxeres integer
  #[serde(rename = "ore_pyroxeres")]
  ore_pyroxeres: Option<i64>,
  /// ore_scordite integer
  #[serde(rename = "ore_scordite")]
  ore_scordite: Option<i64>,
  /// ore_spodumain integer
  #[serde(rename = "ore_spodumain")]
  ore_spodumain: Option<i64>,
  /// ore_veldspar integer
  #[serde(rename = "ore_veldspar")]
  ore_veldspar: Option<i64>
}

impl GetCharactersCharacterIdStatsMining {
  /// mining object
  pub fn new() -> GetCharactersCharacterIdStatsMining {
    GetCharactersCharacterIdStatsMining {
      drone_mine: None,
      ore_arkonor: None,
      ore_bistot: None,
      ore_crokite: None,
      ore_dark_ochre: None,
      ore_gneiss: None,
      ore_harvestable_cloud: None,
      ore_hedbergite: None,
      ore_hemorphite: None,
      ore_ice: None,
      ore_jaspet: None,
      ore_kernite: None,
      ore_mercoxit: None,
      ore_omber: None,
      ore_plagioclase: None,
      ore_pyroxeres: None,
      ore_scordite: None,
      ore_spodumain: None,
      ore_veldspar: None
    }
  }

  pub fn set_drone_mine(&mut self, drone_mine: i64) {
    self.drone_mine = Some(drone_mine);
  }

  pub fn with_drone_mine(mut self, drone_mine: i64) -> GetCharactersCharacterIdStatsMining {
    self.drone_mine = Some(drone_mine);
    self
  }

  pub fn drone_mine(&self) -> Option<&i64> {
    self.drone_mine.as_ref()
  }

  pub fn reset_drone_mine(&mut self) {
    self.drone_mine = None;
  }

  pub fn set_ore_arkonor(&mut self, ore_arkonor: i64) {
    self.ore_arkonor = Some(ore_arkonor);
  }

  pub fn with_ore_arkonor(mut self, ore_arkonor: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_arkonor = Some(ore_arkonor);
    self
  }

  pub fn ore_arkonor(&self) -> Option<&i64> {
    self.ore_arkonor.as_ref()
  }

  pub fn reset_ore_arkonor(&mut self) {
    self.ore_arkonor = None;
  }

  pub fn set_ore_bistot(&mut self, ore_bistot: i64) {
    self.ore_bistot = Some(ore_bistot);
  }

  pub fn with_ore_bistot(mut self, ore_bistot: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_bistot = Some(ore_bistot);
    self
  }

  pub fn ore_bistot(&self) -> Option<&i64> {
    self.ore_bistot.as_ref()
  }

  pub fn reset_ore_bistot(&mut self) {
    self.ore_bistot = None;
  }

  pub fn set_ore_crokite(&mut self, ore_crokite: i64) {
    self.ore_crokite = Some(ore_crokite);
  }

  pub fn with_ore_crokite(mut self, ore_crokite: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_crokite = Some(ore_crokite);
    self
  }

  pub fn ore_crokite(&self) -> Option<&i64> {
    self.ore_crokite.as_ref()
  }

  pub fn reset_ore_crokite(&mut self) {
    self.ore_crokite = None;
  }

  pub fn set_ore_dark_ochre(&mut self, ore_dark_ochre: i64) {
    self.ore_dark_ochre = Some(ore_dark_ochre);
  }

  pub fn with_ore_dark_ochre(mut self, ore_dark_ochre: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_dark_ochre = Some(ore_dark_ochre);
    self
  }

  pub fn ore_dark_ochre(&self) -> Option<&i64> {
    self.ore_dark_ochre.as_ref()
  }

  pub fn reset_ore_dark_ochre(&mut self) {
    self.ore_dark_ochre = None;
  }

  pub fn set_ore_gneiss(&mut self, ore_gneiss: i64) {
    self.ore_gneiss = Some(ore_gneiss);
  }

  pub fn with_ore_gneiss(mut self, ore_gneiss: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_gneiss = Some(ore_gneiss);
    self
  }

  pub fn ore_gneiss(&self) -> Option<&i64> {
    self.ore_gneiss.as_ref()
  }

  pub fn reset_ore_gneiss(&mut self) {
    self.ore_gneiss = None;
  }

  pub fn set_ore_harvestable_cloud(&mut self, ore_harvestable_cloud: i64) {
    self.ore_harvestable_cloud = Some(ore_harvestable_cloud);
  }

  pub fn with_ore_harvestable_cloud(mut self, ore_harvestable_cloud: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_harvestable_cloud = Some(ore_harvestable_cloud);
    self
  }

  pub fn ore_harvestable_cloud(&self) -> Option<&i64> {
    self.ore_harvestable_cloud.as_ref()
  }

  pub fn reset_ore_harvestable_cloud(&mut self) {
    self.ore_harvestable_cloud = None;
  }

  pub fn set_ore_hedbergite(&mut self, ore_hedbergite: i64) {
    self.ore_hedbergite = Some(ore_hedbergite);
  }

  pub fn with_ore_hedbergite(mut self, ore_hedbergite: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_hedbergite = Some(ore_hedbergite);
    self
  }

  pub fn ore_hedbergite(&self) -> Option<&i64> {
    self.ore_hedbergite.as_ref()
  }

  pub fn reset_ore_hedbergite(&mut self) {
    self.ore_hedbergite = None;
  }

  pub fn set_ore_hemorphite(&mut self, ore_hemorphite: i64) {
    self.ore_hemorphite = Some(ore_hemorphite);
  }

  pub fn with_ore_hemorphite(mut self, ore_hemorphite: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_hemorphite = Some(ore_hemorphite);
    self
  }

  pub fn ore_hemorphite(&self) -> Option<&i64> {
    self.ore_hemorphite.as_ref()
  }

  pub fn reset_ore_hemorphite(&mut self) {
    self.ore_hemorphite = None;
  }

  pub fn set_ore_ice(&mut self, ore_ice: i64) {
    self.ore_ice = Some(ore_ice);
  }

  pub fn with_ore_ice(mut self, ore_ice: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_ice = Some(ore_ice);
    self
  }

  pub fn ore_ice(&self) -> Option<&i64> {
    self.ore_ice.as_ref()
  }

  pub fn reset_ore_ice(&mut self) {
    self.ore_ice = None;
  }

  pub fn set_ore_jaspet(&mut self, ore_jaspet: i64) {
    self.ore_jaspet = Some(ore_jaspet);
  }

  pub fn with_ore_jaspet(mut self, ore_jaspet: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_jaspet = Some(ore_jaspet);
    self
  }

  pub fn ore_jaspet(&self) -> Option<&i64> {
    self.ore_jaspet.as_ref()
  }

  pub fn reset_ore_jaspet(&mut self) {
    self.ore_jaspet = None;
  }

  pub fn set_ore_kernite(&mut self, ore_kernite: i64) {
    self.ore_kernite = Some(ore_kernite);
  }

  pub fn with_ore_kernite(mut self, ore_kernite: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_kernite = Some(ore_kernite);
    self
  }

  pub fn ore_kernite(&self) -> Option<&i64> {
    self.ore_kernite.as_ref()
  }

  pub fn reset_ore_kernite(&mut self) {
    self.ore_kernite = None;
  }

  pub fn set_ore_mercoxit(&mut self, ore_mercoxit: i64) {
    self.ore_mercoxit = Some(ore_mercoxit);
  }

  pub fn with_ore_mercoxit(mut self, ore_mercoxit: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_mercoxit = Some(ore_mercoxit);
    self
  }

  pub fn ore_mercoxit(&self) -> Option<&i64> {
    self.ore_mercoxit.as_ref()
  }

  pub fn reset_ore_mercoxit(&mut self) {
    self.ore_mercoxit = None;
  }

  pub fn set_ore_omber(&mut self, ore_omber: i64) {
    self.ore_omber = Some(ore_omber);
  }

  pub fn with_ore_omber(mut self, ore_omber: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_omber = Some(ore_omber);
    self
  }

  pub fn ore_omber(&self) -> Option<&i64> {
    self.ore_omber.as_ref()
  }

  pub fn reset_ore_omber(&mut self) {
    self.ore_omber = None;
  }

  pub fn set_ore_plagioclase(&mut self, ore_plagioclase: i64) {
    self.ore_plagioclase = Some(ore_plagioclase);
  }

  pub fn with_ore_plagioclase(mut self, ore_plagioclase: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_plagioclase = Some(ore_plagioclase);
    self
  }

  pub fn ore_plagioclase(&self) -> Option<&i64> {
    self.ore_plagioclase.as_ref()
  }

  pub fn reset_ore_plagioclase(&mut self) {
    self.ore_plagioclase = None;
  }

  pub fn set_ore_pyroxeres(&mut self, ore_pyroxeres: i64) {
    self.ore_pyroxeres = Some(ore_pyroxeres);
  }

  pub fn with_ore_pyroxeres(mut self, ore_pyroxeres: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_pyroxeres = Some(ore_pyroxeres);
    self
  }

  pub fn ore_pyroxeres(&self) -> Option<&i64> {
    self.ore_pyroxeres.as_ref()
  }

  pub fn reset_ore_pyroxeres(&mut self) {
    self.ore_pyroxeres = None;
  }

  pub fn set_ore_scordite(&mut self, ore_scordite: i64) {
    self.ore_scordite = Some(ore_scordite);
  }

  pub fn with_ore_scordite(mut self, ore_scordite: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_scordite = Some(ore_scordite);
    self
  }

  pub fn ore_scordite(&self) -> Option<&i64> {
    self.ore_scordite.as_ref()
  }

  pub fn reset_ore_scordite(&mut self) {
    self.ore_scordite = None;
  }

  pub fn set_ore_spodumain(&mut self, ore_spodumain: i64) {
    self.ore_spodumain = Some(ore_spodumain);
  }

  pub fn with_ore_spodumain(mut self, ore_spodumain: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_spodumain = Some(ore_spodumain);
    self
  }

  pub fn ore_spodumain(&self) -> Option<&i64> {
    self.ore_spodumain.as_ref()
  }

  pub fn reset_ore_spodumain(&mut self) {
    self.ore_spodumain = None;
  }

  pub fn set_ore_veldspar(&mut self, ore_veldspar: i64) {
    self.ore_veldspar = Some(ore_veldspar);
  }

  pub fn with_ore_veldspar(mut self, ore_veldspar: i64) -> GetCharactersCharacterIdStatsMining {
    self.ore_veldspar = Some(ore_veldspar);
    self
  }

  pub fn ore_veldspar(&self) -> Option<&i64> {
    self.ore_veldspar.as_ref()
  }

  pub fn reset_ore_veldspar(&mut self) {
    self.ore_veldspar = None;
  }

}



