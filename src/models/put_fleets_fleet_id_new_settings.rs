/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PutFleetsFleetIdNewSettings : new_settings object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PutFleetsFleetIdNewSettings {
  /// Should free-move be enabled in the fleet
  #[serde(rename = "is_free_move")]
  is_free_move: Option<bool>,
  /// New fleet MOTD in CCP flavoured HTML
  #[serde(rename = "motd")]
  motd: Option<String>
}

impl PutFleetsFleetIdNewSettings {
  /// new_settings object
  pub fn new() -> PutFleetsFleetIdNewSettings {
    PutFleetsFleetIdNewSettings {
      is_free_move: None,
      motd: None
    }
  }

  pub fn set_is_free_move(&mut self, is_free_move: bool) {
    self.is_free_move = Some(is_free_move);
  }

  pub fn with_is_free_move(mut self, is_free_move: bool) -> PutFleetsFleetIdNewSettings {
    self.is_free_move = Some(is_free_move);
    self
  }

  pub fn is_free_move(&self) -> Option<&bool> {
    self.is_free_move.as_ref()
  }

  pub fn reset_is_free_move(&mut self) {
    self.is_free_move = None;
  }

  pub fn set_motd(&mut self, motd: String) {
    self.motd = Some(motd);
  }

  pub fn with_motd(mut self, motd: String) -> PutFleetsFleetIdNewSettings {
    self.motd = Some(motd);
    self
  }

  pub fn motd(&self) -> Option<&String> {
    self.motd.as_ref()
  }

  pub fn reset_motd(&mut self) {
    self.motd = None;
  }

}



