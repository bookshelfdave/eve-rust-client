/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PutFleetsFleetIdSquadsSquadIdNaming : naming object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PutFleetsFleetIdSquadsSquadIdNaming {
  /// name string
  #[serde(rename = "name")]
  name: String
}

impl PutFleetsFleetIdSquadsSquadIdNaming {
  /// naming object
  pub fn new(name: String) -> PutFleetsFleetIdSquadsSquadIdNaming {
    PutFleetsFleetIdSquadsSquadIdNaming {
      name: name
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> PutFleetsFleetIdSquadsSquadIdNaming {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



