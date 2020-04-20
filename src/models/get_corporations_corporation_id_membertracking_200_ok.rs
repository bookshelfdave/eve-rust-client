/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdMembertracking200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdMembertracking200Ok {
  /// base_id integer
  #[serde(rename = "base_id")]
  base_id: Option<i32>,
  /// character_id integer
  #[serde(rename = "character_id")]
  character_id: i32,
  /// location_id integer
  #[serde(rename = "location_id")]
  location_id: Option<i64>,
  /// logoff_date string
  #[serde(rename = "logoff_date")]
  logoff_date: Option<String>,
  /// logon_date string
  #[serde(rename = "logon_date")]
  logon_date: Option<String>,
  /// ship_type_id integer
  #[serde(rename = "ship_type_id")]
  ship_type_id: Option<i32>,
  /// start_date string
  #[serde(rename = "start_date")]
  start_date: Option<String>
}

impl GetCorporationsCorporationIdMembertracking200Ok {
  /// 200 ok object
  pub fn new(character_id: i32) -> GetCorporationsCorporationIdMembertracking200Ok {
    GetCorporationsCorporationIdMembertracking200Ok {
      base_id: None,
      character_id: character_id,
      location_id: None,
      logoff_date: None,
      logon_date: None,
      ship_type_id: None,
      start_date: None
    }
  }

  pub fn set_base_id(&mut self, base_id: i32) {
    self.base_id = Some(base_id);
  }

  pub fn with_base_id(mut self, base_id: i32) -> GetCorporationsCorporationIdMembertracking200Ok {
    self.base_id = Some(base_id);
    self
  }

  pub fn base_id(&self) -> Option<&i32> {
    self.base_id.as_ref()
  }

  pub fn reset_base_id(&mut self) {
    self.base_id = None;
  }

  pub fn set_character_id(&mut self, character_id: i32) {
    self.character_id = character_id;
  }

  pub fn with_character_id(mut self, character_id: i32) -> GetCorporationsCorporationIdMembertracking200Ok {
    self.character_id = character_id;
    self
  }

  pub fn character_id(&self) -> &i32 {
    &self.character_id
  }


  pub fn set_location_id(&mut self, location_id: i64) {
    self.location_id = Some(location_id);
  }

  pub fn with_location_id(mut self, location_id: i64) -> GetCorporationsCorporationIdMembertracking200Ok {
    self.location_id = Some(location_id);
    self
  }

  pub fn location_id(&self) -> Option<&i64> {
    self.location_id.as_ref()
  }

  pub fn reset_location_id(&mut self) {
    self.location_id = None;
  }

  pub fn set_logoff_date(&mut self, logoff_date: String) {
    self.logoff_date = Some(logoff_date);
  }

  pub fn with_logoff_date(mut self, logoff_date: String) -> GetCorporationsCorporationIdMembertracking200Ok {
    self.logoff_date = Some(logoff_date);
    self
  }

  pub fn logoff_date(&self) -> Option<&String> {
    self.logoff_date.as_ref()
  }

  pub fn reset_logoff_date(&mut self) {
    self.logoff_date = None;
  }

  pub fn set_logon_date(&mut self, logon_date: String) {
    self.logon_date = Some(logon_date);
  }

  pub fn with_logon_date(mut self, logon_date: String) -> GetCorporationsCorporationIdMembertracking200Ok {
    self.logon_date = Some(logon_date);
    self
  }

  pub fn logon_date(&self) -> Option<&String> {
    self.logon_date.as_ref()
  }

  pub fn reset_logon_date(&mut self) {
    self.logon_date = None;
  }

  pub fn set_ship_type_id(&mut self, ship_type_id: i32) {
    self.ship_type_id = Some(ship_type_id);
  }

  pub fn with_ship_type_id(mut self, ship_type_id: i32) -> GetCorporationsCorporationIdMembertracking200Ok {
    self.ship_type_id = Some(ship_type_id);
    self
  }

  pub fn ship_type_id(&self) -> Option<&i32> {
    self.ship_type_id.as_ref()
  }

  pub fn reset_ship_type_id(&mut self) {
    self.ship_type_id = None;
  }

  pub fn set_start_date(&mut self, start_date: String) {
    self.start_date = Some(start_date);
  }

  pub fn with_start_date(mut self, start_date: String) -> GetCorporationsCorporationIdMembertracking200Ok {
    self.start_date = Some(start_date);
    self
  }

  pub fn start_date(&self) -> Option<&String> {
    self.start_date.as_ref()
  }

  pub fn reset_start_date(&mut self) {
    self.start_date = None;
  }

}



