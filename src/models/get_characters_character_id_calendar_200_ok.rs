/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdCalendar200Ok : event

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdCalendar200Ok {
  /// event_date string
  #[serde(rename = "event_date")]
  event_date: Option<String>,
  /// event_id integer
  #[serde(rename = "event_id")]
  event_id: Option<i32>,
  /// event_response string
  #[serde(rename = "event_response")]
  event_response: Option<String>,
  /// importance integer
  #[serde(rename = "importance")]
  importance: Option<i32>,
  /// title string
  #[serde(rename = "title")]
  title: Option<String>
}

impl GetCharactersCharacterIdCalendar200Ok {
  /// event
  pub fn new() -> GetCharactersCharacterIdCalendar200Ok {
    GetCharactersCharacterIdCalendar200Ok {
      event_date: None,
      event_id: None,
      event_response: None,
      importance: None,
      title: None
    }
  }

  pub fn set_event_date(&mut self, event_date: String) {
    self.event_date = Some(event_date);
  }

  pub fn with_event_date(mut self, event_date: String) -> GetCharactersCharacterIdCalendar200Ok {
    self.event_date = Some(event_date);
    self
  }

  pub fn event_date(&self) -> Option<&String> {
    self.event_date.as_ref()
  }

  pub fn reset_event_date(&mut self) {
    self.event_date = None;
  }

  pub fn set_event_id(&mut self, event_id: i32) {
    self.event_id = Some(event_id);
  }

  pub fn with_event_id(mut self, event_id: i32) -> GetCharactersCharacterIdCalendar200Ok {
    self.event_id = Some(event_id);
    self
  }

  pub fn event_id(&self) -> Option<&i32> {
    self.event_id.as_ref()
  }

  pub fn reset_event_id(&mut self) {
    self.event_id = None;
  }

  pub fn set_event_response(&mut self, event_response: String) {
    self.event_response = Some(event_response);
  }

  pub fn with_event_response(mut self, event_response: String) -> GetCharactersCharacterIdCalendar200Ok {
    self.event_response = Some(event_response);
    self
  }

  pub fn event_response(&self) -> Option<&String> {
    self.event_response.as_ref()
  }

  pub fn reset_event_response(&mut self) {
    self.event_response = None;
  }

  pub fn set_importance(&mut self, importance: i32) {
    self.importance = Some(importance);
  }

  pub fn with_importance(mut self, importance: i32) -> GetCharactersCharacterIdCalendar200Ok {
    self.importance = Some(importance);
    self
  }

  pub fn importance(&self) -> Option<&i32> {
    self.importance.as_ref()
  }

  pub fn reset_importance(&mut self) {
    self.importance = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> GetCharactersCharacterIdCalendar200Ok {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

}



