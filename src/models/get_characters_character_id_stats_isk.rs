/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdStatsIsk : isk object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdStatsIsk {
  /// in integer
  #[serde(rename = "in")]
  _in: Option<i64>,
  /// out integer
  #[serde(rename = "out")]
  out: Option<i64>
}

impl GetCharactersCharacterIdStatsIsk {
  /// isk object
  pub fn new() -> GetCharactersCharacterIdStatsIsk {
    GetCharactersCharacterIdStatsIsk {
      _in: None,
      out: None
    }
  }

  pub fn set__in(&mut self, _in: i64) {
    self._in = Some(_in);
  }

  pub fn with__in(mut self, _in: i64) -> GetCharactersCharacterIdStatsIsk {
    self._in = Some(_in);
    self
  }

  pub fn _in(&self) -> Option<&i64> {
    self._in.as_ref()
  }

  pub fn reset__in(&mut self) {
    self._in = None;
  }

  pub fn set_out(&mut self, out: i64) {
    self.out = Some(out);
  }

  pub fn with_out(mut self, out: i64) -> GetCharactersCharacterIdStatsIsk {
    self.out = Some(out);
    self
  }

  pub fn out(&self) -> Option<&i64> {
    self.out.as_ref()
  }

  pub fn reset_out(&mut self) {
    self.out = None;
  }

}



