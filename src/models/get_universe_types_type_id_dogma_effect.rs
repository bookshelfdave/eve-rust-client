/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetUniverseTypesTypeIdDogmaEffect : dogma_effect object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUniverseTypesTypeIdDogmaEffect {
  /// effect_id integer
  #[serde(rename = "effect_id")]
  effect_id: i32,
  /// is_default boolean
  #[serde(rename = "is_default")]
  is_default: bool
}

impl GetUniverseTypesTypeIdDogmaEffect {
  /// dogma_effect object
  pub fn new(effect_id: i32, is_default: bool) -> GetUniverseTypesTypeIdDogmaEffect {
    GetUniverseTypesTypeIdDogmaEffect {
      effect_id: effect_id,
      is_default: is_default
    }
  }

  pub fn set_effect_id(&mut self, effect_id: i32) {
    self.effect_id = effect_id;
  }

  pub fn with_effect_id(mut self, effect_id: i32) -> GetUniverseTypesTypeIdDogmaEffect {
    self.effect_id = effect_id;
    self
  }

  pub fn effect_id(&self) -> &i32 {
    &self.effect_id
  }


  pub fn set_is_default(&mut self, is_default: bool) {
    self.is_default = is_default;
  }

  pub fn with_is_default(mut self, is_default: bool) -> GetUniverseTypesTypeIdDogmaEffect {
    self.is_default = is_default;
    self
  }

  pub fn is_default(&self) -> &bool {
    &self.is_default
  }


}



