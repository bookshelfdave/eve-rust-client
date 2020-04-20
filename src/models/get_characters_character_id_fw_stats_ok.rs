/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdFwStatsOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdFwStatsOk {
  /// The given character's current faction rank
  #[serde(rename = "current_rank")]
  current_rank: Option<i32>,
  /// The enlistment date of the given character into faction warfare. Will not be included if character is not enlisted in faction warfare
  #[serde(rename = "enlisted_on")]
  enlisted_on: Option<String>,
  /// The faction the given character is enlisted to fight for. Will not be included if character is not enlisted in faction warfare
  #[serde(rename = "faction_id")]
  faction_id: Option<i32>,
  /// The given character's highest faction rank achieved
  #[serde(rename = "highest_rank")]
  highest_rank: Option<i32>,
  #[serde(rename = "kills")]
  kills: ::models::GetCharactersCharacterIdFwStatsKills,
  #[serde(rename = "victory_points")]
  victory_points: ::models::GetCharactersCharacterIdFwStatsVictoryPoints
}

impl GetCharactersCharacterIdFwStatsOk {
  /// 200 ok object
  pub fn new(kills: ::models::GetCharactersCharacterIdFwStatsKills, victory_points: ::models::GetCharactersCharacterIdFwStatsVictoryPoints) -> GetCharactersCharacterIdFwStatsOk {
    GetCharactersCharacterIdFwStatsOk {
      current_rank: None,
      enlisted_on: None,
      faction_id: None,
      highest_rank: None,
      kills: kills,
      victory_points: victory_points
    }
  }

  pub fn set_current_rank(&mut self, current_rank: i32) {
    self.current_rank = Some(current_rank);
  }

  pub fn with_current_rank(mut self, current_rank: i32) -> GetCharactersCharacterIdFwStatsOk {
    self.current_rank = Some(current_rank);
    self
  }

  pub fn current_rank(&self) -> Option<&i32> {
    self.current_rank.as_ref()
  }

  pub fn reset_current_rank(&mut self) {
    self.current_rank = None;
  }

  pub fn set_enlisted_on(&mut self, enlisted_on: String) {
    self.enlisted_on = Some(enlisted_on);
  }

  pub fn with_enlisted_on(mut self, enlisted_on: String) -> GetCharactersCharacterIdFwStatsOk {
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

  pub fn with_faction_id(mut self, faction_id: i32) -> GetCharactersCharacterIdFwStatsOk {
    self.faction_id = Some(faction_id);
    self
  }

  pub fn faction_id(&self) -> Option<&i32> {
    self.faction_id.as_ref()
  }

  pub fn reset_faction_id(&mut self) {
    self.faction_id = None;
  }

  pub fn set_highest_rank(&mut self, highest_rank: i32) {
    self.highest_rank = Some(highest_rank);
  }

  pub fn with_highest_rank(mut self, highest_rank: i32) -> GetCharactersCharacterIdFwStatsOk {
    self.highest_rank = Some(highest_rank);
    self
  }

  pub fn highest_rank(&self) -> Option<&i32> {
    self.highest_rank.as_ref()
  }

  pub fn reset_highest_rank(&mut self) {
    self.highest_rank = None;
  }

  pub fn set_kills(&mut self, kills: ::models::GetCharactersCharacterIdFwStatsKills) {
    self.kills = kills;
  }

  pub fn with_kills(mut self, kills: ::models::GetCharactersCharacterIdFwStatsKills) -> GetCharactersCharacterIdFwStatsOk {
    self.kills = kills;
    self
  }

  pub fn kills(&self) -> &::models::GetCharactersCharacterIdFwStatsKills {
    &self.kills
  }


  pub fn set_victory_points(&mut self, victory_points: ::models::GetCharactersCharacterIdFwStatsVictoryPoints) {
    self.victory_points = victory_points;
  }

  pub fn with_victory_points(mut self, victory_points: ::models::GetCharactersCharacterIdFwStatsVictoryPoints) -> GetCharactersCharacterIdFwStatsOk {
    self.victory_points = victory_points;
    self
  }

  pub fn victory_points(&self) -> &::models::GetCharactersCharacterIdFwStatsVictoryPoints {
    &self.victory_points
  }


}



