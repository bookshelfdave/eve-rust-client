/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdStarbasesStarbaseIdFuel : fuel object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdStarbasesStarbaseIdFuel {
  /// quantity integer
  #[serde(rename = "quantity")]
  quantity: i32,
  /// type_id integer
  #[serde(rename = "type_id")]
  type_id: i32
}

impl GetCorporationsCorporationIdStarbasesStarbaseIdFuel {
  /// fuel object
  pub fn new(quantity: i32, type_id: i32) -> GetCorporationsCorporationIdStarbasesStarbaseIdFuel {
    GetCorporationsCorporationIdStarbasesStarbaseIdFuel {
      quantity: quantity,
      type_id: type_id
    }
  }

  pub fn set_quantity(&mut self, quantity: i32) {
    self.quantity = quantity;
  }

  pub fn with_quantity(mut self, quantity: i32) -> GetCorporationsCorporationIdStarbasesStarbaseIdFuel {
    self.quantity = quantity;
    self
  }

  pub fn quantity(&self) -> &i32 {
    &self.quantity
  }


  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = type_id;
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetCorporationsCorporationIdStarbasesStarbaseIdFuel {
    self.type_id = type_id;
    self
  }

  pub fn type_id(&self) -> &i32 {
    &self.type_id
  }


}



