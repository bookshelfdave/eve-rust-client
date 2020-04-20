/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PostCorporationsCorporationIdAssetsLocations200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostCorporationsCorporationIdAssetsLocations200Ok {
  /// item_id integer
  #[serde(rename = "item_id")]
  item_id: i64,
  #[serde(rename = "position")]
  position: ::models::PostCorporationsCorporationIdAssetsLocationsPosition
}

impl PostCorporationsCorporationIdAssetsLocations200Ok {
  /// 200 ok object
  pub fn new(item_id: i64, position: ::models::PostCorporationsCorporationIdAssetsLocationsPosition) -> PostCorporationsCorporationIdAssetsLocations200Ok {
    PostCorporationsCorporationIdAssetsLocations200Ok {
      item_id: item_id,
      position: position
    }
  }

  pub fn set_item_id(&mut self, item_id: i64) {
    self.item_id = item_id;
  }

  pub fn with_item_id(mut self, item_id: i64) -> PostCorporationsCorporationIdAssetsLocations200Ok {
    self.item_id = item_id;
    self
  }

  pub fn item_id(&self) -> &i64 {
    &self.item_id
  }


  pub fn set_position(&mut self, position: ::models::PostCorporationsCorporationIdAssetsLocationsPosition) {
    self.position = position;
  }

  pub fn with_position(mut self, position: ::models::PostCorporationsCorporationIdAssetsLocationsPosition) -> PostCorporationsCorporationIdAssetsLocations200Ok {
    self.position = position;
    self
  }

  pub fn position(&self) -> &::models::PostCorporationsCorporationIdAssetsLocationsPosition {
    &self.position
  }


}



