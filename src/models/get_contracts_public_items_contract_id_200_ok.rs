/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetContractsPublicItemsContractId200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetContractsPublicItemsContractId200Ok {
  /// is_blueprint_copy boolean
  #[serde(rename = "is_blueprint_copy")]
  is_blueprint_copy: Option<bool>,
  /// true if the contract issuer has submitted this item with the contract, false if the isser is asking for this item in the contract
  #[serde(rename = "is_included")]
  is_included: bool,
  /// Unique ID for the item being sold. Not present if item is being requested by contract rather than sold with contract
  #[serde(rename = "item_id")]
  item_id: Option<i64>,
  /// Material Efficiency Level of the blueprint
  #[serde(rename = "material_efficiency")]
  material_efficiency: Option<i32>,
  /// Number of items in the stack
  #[serde(rename = "quantity")]
  quantity: i32,
  /// Unique ID for the item, used by the contract system
  #[serde(rename = "record_id")]
  record_id: i64,
  /// Number of runs remaining if the blueprint is a copy, -1 if it is an original
  #[serde(rename = "runs")]
  runs: Option<i32>,
  /// Time Efficiency Level of the blueprint
  #[serde(rename = "time_efficiency")]
  time_efficiency: Option<i32>,
  /// Type ID for item
  #[serde(rename = "type_id")]
  type_id: i32
}

impl GetContractsPublicItemsContractId200Ok {
  /// 200 ok object
  pub fn new(is_included: bool, quantity: i32, record_id: i64, type_id: i32) -> GetContractsPublicItemsContractId200Ok {
    GetContractsPublicItemsContractId200Ok {
      is_blueprint_copy: None,
      is_included: is_included,
      item_id: None,
      material_efficiency: None,
      quantity: quantity,
      record_id: record_id,
      runs: None,
      time_efficiency: None,
      type_id: type_id
    }
  }

  pub fn set_is_blueprint_copy(&mut self, is_blueprint_copy: bool) {
    self.is_blueprint_copy = Some(is_blueprint_copy);
  }

  pub fn with_is_blueprint_copy(mut self, is_blueprint_copy: bool) -> GetContractsPublicItemsContractId200Ok {
    self.is_blueprint_copy = Some(is_blueprint_copy);
    self
  }

  pub fn is_blueprint_copy(&self) -> Option<&bool> {
    self.is_blueprint_copy.as_ref()
  }

  pub fn reset_is_blueprint_copy(&mut self) {
    self.is_blueprint_copy = None;
  }

  pub fn set_is_included(&mut self, is_included: bool) {
    self.is_included = is_included;
  }

  pub fn with_is_included(mut self, is_included: bool) -> GetContractsPublicItemsContractId200Ok {
    self.is_included = is_included;
    self
  }

  pub fn is_included(&self) -> &bool {
    &self.is_included
  }


  pub fn set_item_id(&mut self, item_id: i64) {
    self.item_id = Some(item_id);
  }

  pub fn with_item_id(mut self, item_id: i64) -> GetContractsPublicItemsContractId200Ok {
    self.item_id = Some(item_id);
    self
  }

  pub fn item_id(&self) -> Option<&i64> {
    self.item_id.as_ref()
  }

  pub fn reset_item_id(&mut self) {
    self.item_id = None;
  }

  pub fn set_material_efficiency(&mut self, material_efficiency: i32) {
    self.material_efficiency = Some(material_efficiency);
  }

  pub fn with_material_efficiency(mut self, material_efficiency: i32) -> GetContractsPublicItemsContractId200Ok {
    self.material_efficiency = Some(material_efficiency);
    self
  }

  pub fn material_efficiency(&self) -> Option<&i32> {
    self.material_efficiency.as_ref()
  }

  pub fn reset_material_efficiency(&mut self) {
    self.material_efficiency = None;
  }

  pub fn set_quantity(&mut self, quantity: i32) {
    self.quantity = quantity;
  }

  pub fn with_quantity(mut self, quantity: i32) -> GetContractsPublicItemsContractId200Ok {
    self.quantity = quantity;
    self
  }

  pub fn quantity(&self) -> &i32 {
    &self.quantity
  }


  pub fn set_record_id(&mut self, record_id: i64) {
    self.record_id = record_id;
  }

  pub fn with_record_id(mut self, record_id: i64) -> GetContractsPublicItemsContractId200Ok {
    self.record_id = record_id;
    self
  }

  pub fn record_id(&self) -> &i64 {
    &self.record_id
  }


  pub fn set_runs(&mut self, runs: i32) {
    self.runs = Some(runs);
  }

  pub fn with_runs(mut self, runs: i32) -> GetContractsPublicItemsContractId200Ok {
    self.runs = Some(runs);
    self
  }

  pub fn runs(&self) -> Option<&i32> {
    self.runs.as_ref()
  }

  pub fn reset_runs(&mut self) {
    self.runs = None;
  }

  pub fn set_time_efficiency(&mut self, time_efficiency: i32) {
    self.time_efficiency = Some(time_efficiency);
  }

  pub fn with_time_efficiency(mut self, time_efficiency: i32) -> GetContractsPublicItemsContractId200Ok {
    self.time_efficiency = Some(time_efficiency);
    self
  }

  pub fn time_efficiency(&self) -> Option<&i32> {
    self.time_efficiency.as_ref()
  }

  pub fn reset_time_efficiency(&mut self) {
    self.time_efficiency = None;
  }

  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = type_id;
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetContractsPublicItemsContractId200Ok {
    self.type_id = type_id;
    self
  }

  pub fn type_id(&self) -> &i32 {
    &self.type_id
  }


}



