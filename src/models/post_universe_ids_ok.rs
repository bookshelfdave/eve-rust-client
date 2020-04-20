/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PostUniverseIdsOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostUniverseIdsOk {
  /// agents array
  #[serde(rename = "agents")]
  agents: Option<Vec<::models::PostUniverseIdsAgent>>,
  /// alliances array
  #[serde(rename = "alliances")]
  alliances: Option<Vec<::models::PostUniverseIdsAlliance>>,
  /// characters array
  #[serde(rename = "characters")]
  characters: Option<Vec<::models::PostUniverseIdsCharacter>>,
  /// constellations array
  #[serde(rename = "constellations")]
  constellations: Option<Vec<::models::PostUniverseIdsConstellation>>,
  /// corporations array
  #[serde(rename = "corporations")]
  corporations: Option<Vec<::models::PostUniverseIdsCorporation>>,
  /// factions array
  #[serde(rename = "factions")]
  factions: Option<Vec<::models::PostUniverseIdsFaction>>,
  /// inventory_types array
  #[serde(rename = "inventory_types")]
  inventory_types: Option<Vec<::models::PostUniverseIdsInventoryType>>,
  /// regions array
  #[serde(rename = "regions")]
  regions: Option<Vec<::models::PostUniverseIdsRegion>>,
  /// stations array
  #[serde(rename = "stations")]
  stations: Option<Vec<::models::PostUniverseIdsStation>>,
  /// systems array
  #[serde(rename = "systems")]
  systems: Option<Vec<::models::PostUniverseIdsSystem>>
}

impl PostUniverseIdsOk {
  /// 200 ok object
  pub fn new() -> PostUniverseIdsOk {
    PostUniverseIdsOk {
      agents: None,
      alliances: None,
      characters: None,
      constellations: None,
      corporations: None,
      factions: None,
      inventory_types: None,
      regions: None,
      stations: None,
      systems: None
    }
  }

  pub fn set_agents(&mut self, agents: Vec<::models::PostUniverseIdsAgent>) {
    self.agents = Some(agents);
  }

  pub fn with_agents(mut self, agents: Vec<::models::PostUniverseIdsAgent>) -> PostUniverseIdsOk {
    self.agents = Some(agents);
    self
  }

  pub fn agents(&self) -> Option<&Vec<::models::PostUniverseIdsAgent>> {
    self.agents.as_ref()
  }

  pub fn reset_agents(&mut self) {
    self.agents = None;
  }

  pub fn set_alliances(&mut self, alliances: Vec<::models::PostUniverseIdsAlliance>) {
    self.alliances = Some(alliances);
  }

  pub fn with_alliances(mut self, alliances: Vec<::models::PostUniverseIdsAlliance>) -> PostUniverseIdsOk {
    self.alliances = Some(alliances);
    self
  }

  pub fn alliances(&self) -> Option<&Vec<::models::PostUniverseIdsAlliance>> {
    self.alliances.as_ref()
  }

  pub fn reset_alliances(&mut self) {
    self.alliances = None;
  }

  pub fn set_characters(&mut self, characters: Vec<::models::PostUniverseIdsCharacter>) {
    self.characters = Some(characters);
  }

  pub fn with_characters(mut self, characters: Vec<::models::PostUniverseIdsCharacter>) -> PostUniverseIdsOk {
    self.characters = Some(characters);
    self
  }

  pub fn characters(&self) -> Option<&Vec<::models::PostUniverseIdsCharacter>> {
    self.characters.as_ref()
  }

  pub fn reset_characters(&mut self) {
    self.characters = None;
  }

  pub fn set_constellations(&mut self, constellations: Vec<::models::PostUniverseIdsConstellation>) {
    self.constellations = Some(constellations);
  }

  pub fn with_constellations(mut self, constellations: Vec<::models::PostUniverseIdsConstellation>) -> PostUniverseIdsOk {
    self.constellations = Some(constellations);
    self
  }

  pub fn constellations(&self) -> Option<&Vec<::models::PostUniverseIdsConstellation>> {
    self.constellations.as_ref()
  }

  pub fn reset_constellations(&mut self) {
    self.constellations = None;
  }

  pub fn set_corporations(&mut self, corporations: Vec<::models::PostUniverseIdsCorporation>) {
    self.corporations = Some(corporations);
  }

  pub fn with_corporations(mut self, corporations: Vec<::models::PostUniverseIdsCorporation>) -> PostUniverseIdsOk {
    self.corporations = Some(corporations);
    self
  }

  pub fn corporations(&self) -> Option<&Vec<::models::PostUniverseIdsCorporation>> {
    self.corporations.as_ref()
  }

  pub fn reset_corporations(&mut self) {
    self.corporations = None;
  }

  pub fn set_factions(&mut self, factions: Vec<::models::PostUniverseIdsFaction>) {
    self.factions = Some(factions);
  }

  pub fn with_factions(mut self, factions: Vec<::models::PostUniverseIdsFaction>) -> PostUniverseIdsOk {
    self.factions = Some(factions);
    self
  }

  pub fn factions(&self) -> Option<&Vec<::models::PostUniverseIdsFaction>> {
    self.factions.as_ref()
  }

  pub fn reset_factions(&mut self) {
    self.factions = None;
  }

  pub fn set_inventory_types(&mut self, inventory_types: Vec<::models::PostUniverseIdsInventoryType>) {
    self.inventory_types = Some(inventory_types);
  }

  pub fn with_inventory_types(mut self, inventory_types: Vec<::models::PostUniverseIdsInventoryType>) -> PostUniverseIdsOk {
    self.inventory_types = Some(inventory_types);
    self
  }

  pub fn inventory_types(&self) -> Option<&Vec<::models::PostUniverseIdsInventoryType>> {
    self.inventory_types.as_ref()
  }

  pub fn reset_inventory_types(&mut self) {
    self.inventory_types = None;
  }

  pub fn set_regions(&mut self, regions: Vec<::models::PostUniverseIdsRegion>) {
    self.regions = Some(regions);
  }

  pub fn with_regions(mut self, regions: Vec<::models::PostUniverseIdsRegion>) -> PostUniverseIdsOk {
    self.regions = Some(regions);
    self
  }

  pub fn regions(&self) -> Option<&Vec<::models::PostUniverseIdsRegion>> {
    self.regions.as_ref()
  }

  pub fn reset_regions(&mut self) {
    self.regions = None;
  }

  pub fn set_stations(&mut self, stations: Vec<::models::PostUniverseIdsStation>) {
    self.stations = Some(stations);
  }

  pub fn with_stations(mut self, stations: Vec<::models::PostUniverseIdsStation>) -> PostUniverseIdsOk {
    self.stations = Some(stations);
    self
  }

  pub fn stations(&self) -> Option<&Vec<::models::PostUniverseIdsStation>> {
    self.stations.as_ref()
  }

  pub fn reset_stations(&mut self) {
    self.stations = None;
  }

  pub fn set_systems(&mut self, systems: Vec<::models::PostUniverseIdsSystem>) {
    self.systems = Some(systems);
  }

  pub fn with_systems(mut self, systems: Vec<::models::PostUniverseIdsSystem>) -> PostUniverseIdsOk {
    self.systems = Some(systems);
    self
  }

  pub fn systems(&self) -> Option<&Vec<::models::PostUniverseIdsSystem>> {
    self.systems.as_ref()
  }

  pub fn reset_systems(&mut self) {
    self.systems = None;
  }

}



