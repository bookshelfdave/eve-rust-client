/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdMail200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdMail200Ok {
  /// From whom the mail was sent
  #[serde(rename = "from")]
  from: Option<i32>,
  /// is_read boolean
  #[serde(rename = "is_read")]
  is_read: Option<bool>,
  /// labels array
  #[serde(rename = "labels")]
  labels: Option<Vec<i32>>,
  /// mail_id integer
  #[serde(rename = "mail_id")]
  mail_id: Option<i32>,
  /// Recipients of the mail
  #[serde(rename = "recipients")]
  recipients: Option<Vec<::models::GetCharactersCharacterIdMailRecipient>>,
  /// Mail subject
  #[serde(rename = "subject")]
  subject: Option<String>,
  /// When the mail was sent
  #[serde(rename = "timestamp")]
  timestamp: Option<String>
}

impl GetCharactersCharacterIdMail200Ok {
  /// 200 ok object
  pub fn new() -> GetCharactersCharacterIdMail200Ok {
    GetCharactersCharacterIdMail200Ok {
      from: None,
      is_read: None,
      labels: None,
      mail_id: None,
      recipients: None,
      subject: None,
      timestamp: None
    }
  }

  pub fn set_from(&mut self, from: i32) {
    self.from = Some(from);
  }

  pub fn with_from(mut self, from: i32) -> GetCharactersCharacterIdMail200Ok {
    self.from = Some(from);
    self
  }

  pub fn from(&self) -> Option<&i32> {
    self.from.as_ref()
  }

  pub fn reset_from(&mut self) {
    self.from = None;
  }

  pub fn set_is_read(&mut self, is_read: bool) {
    self.is_read = Some(is_read);
  }

  pub fn with_is_read(mut self, is_read: bool) -> GetCharactersCharacterIdMail200Ok {
    self.is_read = Some(is_read);
    self
  }

  pub fn is_read(&self) -> Option<&bool> {
    self.is_read.as_ref()
  }

  pub fn reset_is_read(&mut self) {
    self.is_read = None;
  }

  pub fn set_labels(&mut self, labels: Vec<i32>) {
    self.labels = Some(labels);
  }

  pub fn with_labels(mut self, labels: Vec<i32>) -> GetCharactersCharacterIdMail200Ok {
    self.labels = Some(labels);
    self
  }

  pub fn labels(&self) -> Option<&Vec<i32>> {
    self.labels.as_ref()
  }

  pub fn reset_labels(&mut self) {
    self.labels = None;
  }

  pub fn set_mail_id(&mut self, mail_id: i32) {
    self.mail_id = Some(mail_id);
  }

  pub fn with_mail_id(mut self, mail_id: i32) -> GetCharactersCharacterIdMail200Ok {
    self.mail_id = Some(mail_id);
    self
  }

  pub fn mail_id(&self) -> Option<&i32> {
    self.mail_id.as_ref()
  }

  pub fn reset_mail_id(&mut self) {
    self.mail_id = None;
  }

  pub fn set_recipients(&mut self, recipients: Vec<::models::GetCharactersCharacterIdMailRecipient>) {
    self.recipients = Some(recipients);
  }

  pub fn with_recipients(mut self, recipients: Vec<::models::GetCharactersCharacterIdMailRecipient>) -> GetCharactersCharacterIdMail200Ok {
    self.recipients = Some(recipients);
    self
  }

  pub fn recipients(&self) -> Option<&Vec<::models::GetCharactersCharacterIdMailRecipient>> {
    self.recipients.as_ref()
  }

  pub fn reset_recipients(&mut self) {
    self.recipients = None;
  }

  pub fn set_subject(&mut self, subject: String) {
    self.subject = Some(subject);
  }

  pub fn with_subject(mut self, subject: String) -> GetCharactersCharacterIdMail200Ok {
    self.subject = Some(subject);
    self
  }

  pub fn subject(&self) -> Option<&String> {
    self.subject.as_ref()
  }

  pub fn reset_subject(&mut self) {
    self.subject = None;
  }

  pub fn set_timestamp(&mut self, timestamp: String) {
    self.timestamp = Some(timestamp);
  }

  pub fn with_timestamp(mut self, timestamp: String) -> GetCharactersCharacterIdMail200Ok {
    self.timestamp = Some(timestamp);
    self
  }

  pub fn timestamp(&self) -> Option<&String> {
    self.timestamp.as_ref()
  }

  pub fn reset_timestamp(&mut self) {
    self.timestamp = None;
  }

}



