/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdFwStatsOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdFwStatsOk {
  /// The enlistment date of the given corporation into faction warfare. Will not be included if corporation is not enlisted in faction warfare
  #[serde(rename = "enlisted_on")]
  enlisted_on: Option<String>,
  /// The faction the given corporation is enlisted to fight for. Will not be included if corporation is not enlisted in faction warfare
  #[serde(rename = "faction_id")]
  faction_id: Option<i32>,
  #[serde(rename = "kills")]
  kills: ::models::GetCorporationsCorporationIdFwStatsKills,
  /// How many pilots the enlisted corporation has. Will not be included if corporation is not enlisted in faction warfare
  #[serde(rename = "pilots")]
  pilots: Option<i32>,
  #[serde(rename = "victory_points")]
  victory_points: ::models::GetCorporationsCorporationIdFwStatsVictoryPoints
}

impl GetCorporationsCorporationIdFwStatsOk {
  /// 200 ok object
  pub fn new(kills: ::models::GetCorporationsCorporationIdFwStatsKills, victory_points: ::models::GetCorporationsCorporationIdFwStatsVictoryPoints) -> GetCorporationsCorporationIdFwStatsOk {
    GetCorporationsCorporationIdFwStatsOk {
      enlisted_on: None,
      faction_id: None,
      kills: kills,
      pilots: None,
      victory_points: victory_points
    }
  }

  pub fn set_enlisted_on(&mut self, enlisted_on: String) {
    self.enlisted_on = Some(enlisted_on);
  }

  pub fn with_enlisted_on(mut self, enlisted_on: String) -> GetCorporationsCorporationIdFwStatsOk {
    self.enlisted_on = Some(enlisted_on);
    self
  }

  pub fn enlisted_on(&self) -> Option<&String> {
    self.enlisted_on.as_ref()
  }

  pub fn reset_enlisted_on(&mut self) {
    self.enlisted_on = None;
  }

  pub fn set_faction_id(&mut self, faction_id: i32) {
    self.faction_id = Some(faction_id);
  }

  pub fn with_faction_id(mut self, faction_id: i32) -> GetCorporationsCorporationIdFwStatsOk {
    self.faction_id = Some(faction_id);
    self
  }

  pub fn faction_id(&self) -> Option<&i32> {
    self.faction_id.as_ref()
  }

  pub fn reset_faction_id(&mut self) {
    self.faction_id = None;
  }

  pub fn set_kills(&mut self, kills: ::models::GetCorporationsCorporationIdFwStatsKills) {
    self.kills = kills;
  }

  pub fn with_kills(mut self, kills: ::models::GetCorporationsCorporationIdFwStatsKills) -> GetCorporationsCorporationIdFwStatsOk {
    self.kills = kills;
    self
  }

  pub fn kills(&self) -> &::models::GetCorporationsCorporationIdFwStatsKills {
    &self.kills
  }


  pub fn set_pilots(&mut self, pilots: i32) {
    self.pilots = Some(pilots);
  }

  pub fn with_pilots(mut self, pilots: i32) -> GetCorporationsCorporationIdFwStatsOk {
    self.pilots = Some(pilots);
    self
  }

  pub fn pilots(&self) -> Option<&i32> {
    self.pilots.as_ref()
  }

  pub fn reset_pilots(&mut self) {
    self.pilots = None;
  }

  pub fn set_victory_points(&mut self, victory_points: ::models::GetCorporationsCorporationIdFwStatsVictoryPoints) {
    self.victory_points = victory_points;
  }

  pub fn with_victory_points(mut self, victory_points: ::models::GetCorporationsCorporationIdFwStatsVictoryPoints) -> GetCorporationsCorporationIdFwStatsOk {
    self.victory_points = victory_points;
    self
  }

  pub fn victory_points(&self) -> &::models::GetCorporationsCorporationIdFwStatsVictoryPoints {
    &self.victory_points
  }


}



