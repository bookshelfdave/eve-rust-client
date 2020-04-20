/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetWarsWarIdDefender : The defending corporation or alliance that declared this war, only contains either corporation_id or alliance_id

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWarsWarIdDefender {
  /// Alliance ID if and only if the defender is an alliance
  #[serde(rename = "alliance_id")]
  alliance_id: Option<i32>,
  /// Corporation ID if and only if the defender is a corporation
  #[serde(rename = "corporation_id")]
  corporation_id: Option<i32>,
  /// ISK value of ships the defender has killed
  #[serde(rename = "isk_destroyed")]
  isk_destroyed: f32,
  /// The number of ships the defender has killed
  #[serde(rename = "ships_killed")]
  ships_killed: i32
}

impl GetWarsWarIdDefender {
  /// The defending corporation or alliance that declared this war, only contains either corporation_id or alliance_id
  pub fn new(isk_destroyed: f32, ships_killed: i32) -> GetWarsWarIdDefender {
    GetWarsWarIdDefender {
      alliance_id: None,
      corporation_id: None,
      isk_destroyed: isk_destroyed,
      ships_killed: ships_killed
    }
  }

  pub fn set_alliance_id(&mut self, alliance_id: i32) {
    self.alliance_id = Some(alliance_id);
  }

  pub fn with_alliance_id(mut self, alliance_id: i32) -> GetWarsWarIdDefender {
    self.alliance_id = Some(alliance_id);
    self
  }

  pub fn alliance_id(&self) -> Option<&i32> {
    self.alliance_id.as_ref()
  }

  pub fn reset_alliance_id(&mut self) {
    self.alliance_id = None;
  }

  pub fn set_corporation_id(&mut self, corporation_id: i32) {
    self.corporation_id = Some(corporation_id);
  }

  pub fn with_corporation_id(mut self, corporation_id: i32) -> GetWarsWarIdDefender {
    self.corporation_id = Some(corporation_id);
    self
  }

  pub fn corporation_id(&self) -> Option<&i32> {
    self.corporation_id.as_ref()
  }

  pub fn reset_corporation_id(&mut self) {
    self.corporation_id = None;
  }

  pub fn set_isk_destroyed(&mut self, isk_destroyed: f32) {
    self.isk_destroyed = isk_destroyed;
  }

  pub fn with_isk_destroyed(mut self, isk_destroyed: f32) -> GetWarsWarIdDefender {
    self.isk_destroyed = isk_destroyed;
    self
  }

  pub fn isk_destroyed(&self) -> &f32 {
    &self.isk_destroyed
  }


  pub fn set_ships_killed(&mut self, ships_killed: i32) {
    self.ships_killed = ships_killed;
  }

  pub fn with_ships_killed(mut self, ships_killed: i32) -> GetWarsWarIdDefender {
    self.ships_killed = ships_killed;
    self
  }

  pub fn ships_killed(&self) -> &i32 {
    &self.ships_killed
  }


}



