/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PostCharactersCharacterIdContactsError520 : Error 520

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostCharactersCharacterIdContactsError520 {
  /// Error 520 message
  #[serde(rename = "error")]
  error: Option<String>
}

impl PostCharactersCharacterIdContactsError520 {
  /// Error 520
  pub fn new() -> PostCharactersCharacterIdContactsError520 {
    PostCharactersCharacterIdContactsError520 {
      error: None
    }
  }

  pub fn set_error(&mut self, error: String) {
    self.error = Some(error);
  }

  pub fn with_error(mut self, error: String) -> PostCharactersCharacterIdContactsError520 {
    self.error = Some(error);
    self
  }

  pub fn error(&self) -> Option<&String> {
    self.error.as_ref()
  }

  pub fn reset_error(&mut self) {
    self.error = None;
  }

}



