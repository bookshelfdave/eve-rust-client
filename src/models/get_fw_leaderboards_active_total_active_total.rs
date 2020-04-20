/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFwLeaderboardsActiveTotalActiveTotal : active_total object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFwLeaderboardsActiveTotalActiveTotal {
  /// Amount of kills
  #[serde(rename = "amount")]
  amount: Option<i32>,
  /// faction_id integer
  #[serde(rename = "faction_id")]
  faction_id: Option<i32>
}

impl GetFwLeaderboardsActiveTotalActiveTotal {
  /// active_total object
  pub fn new() -> GetFwLeaderboardsActiveTotalActiveTotal {
    GetFwLeaderboardsActiveTotalActiveTotal {
      amount: None,
      faction_id: None
    }
  }

  pub fn set_amount(&mut self, amount: i32) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: i32) -> GetFwLeaderboardsActiveTotalActiveTotal {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&i32> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_faction_id(&mut self, faction_id: i32) {
    self.faction_id = Some(faction_id);
  }

  pub fn with_faction_id(mut self, faction_id: i32) -> GetFwLeaderboardsActiveTotalActiveTotal {
    self.faction_id = Some(faction_id);
    self
  }

  pub fn faction_id(&self) -> Option<&i32> {
    self.faction_id.as_ref()
  }

  pub fn reset_faction_id(&mut self) {
    self.faction_id = None;
  }

}



