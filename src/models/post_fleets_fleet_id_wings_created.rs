/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PostFleetsFleetIdWingsCreated : 201 created object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostFleetsFleetIdWingsCreated {
  /// The wing_id of the newly created wing
  #[serde(rename = "wing_id")]
  wing_id: i64
}

impl PostFleetsFleetIdWingsCreated {
  /// 201 created object
  pub fn new(wing_id: i64) -> PostFleetsFleetIdWingsCreated {
    PostFleetsFleetIdWingsCreated {
      wing_id: wing_id
    }
  }

  pub fn set_wing_id(&mut self, wing_id: i64) {
    self.wing_id = wing_id;
  }

  pub fn with_wing_id(mut self, wing_id: i64) -> PostFleetsFleetIdWingsCreated {
    self.wing_id = wing_id;
    self
  }

  pub fn wing_id(&self) -> &i64 {
    &self.wing_id
  }


}



