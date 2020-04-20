/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdStatsInventory : inventory object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdStatsInventory {
  /// abandon_loot_quantity integer
  #[serde(rename = "abandon_loot_quantity")]
  abandon_loot_quantity: Option<i64>,
  /// trash_item_quantity integer
  #[serde(rename = "trash_item_quantity")]
  trash_item_quantity: Option<i64>
}

impl GetCharactersCharacterIdStatsInventory {
  /// inventory object
  pub fn new() -> GetCharactersCharacterIdStatsInventory {
    GetCharactersCharacterIdStatsInventory {
      abandon_loot_quantity: None,
      trash_item_quantity: None
    }
  }

  pub fn set_abandon_loot_quantity(&mut self, abandon_loot_quantity: i64) {
    self.abandon_loot_quantity = Some(abandon_loot_quantity);
  }

  pub fn with_abandon_loot_quantity(mut self, abandon_loot_quantity: i64) -> GetCharactersCharacterIdStatsInventory {
    self.abandon_loot_quantity = Some(abandon_loot_quantity);
    self
  }

  pub fn abandon_loot_quantity(&self) -> Option<&i64> {
    self.abandon_loot_quantity.as_ref()
  }

  pub fn reset_abandon_loot_quantity(&mut self) {
    self.abandon_loot_quantity = None;
  }

  pub fn set_trash_item_quantity(&mut self, trash_item_quantity: i64) {
    self.trash_item_quantity = Some(trash_item_quantity);
  }

  pub fn with_trash_item_quantity(mut self, trash_item_quantity: i64) -> GetCharactersCharacterIdStatsInventory {
    self.trash_item_quantity = Some(trash_item_quantity);
    self
  }

  pub fn trash_item_quantity(&self) -> Option<&i64> {
    self.trash_item_quantity.as_ref()
  }

  pub fn reset_trash_item_quantity(&mut self) {
    self.trash_item_quantity = None;
  }

}



