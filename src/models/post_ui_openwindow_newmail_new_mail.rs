/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PostUiOpenwindowNewmailNewMail : new_mail object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostUiOpenwindowNewmailNewMail {
  /// body string
  #[serde(rename = "body")]
  body: String,
  /// recipients array
  #[serde(rename = "recipients")]
  recipients: Vec<i32>,
  /// subject string
  #[serde(rename = "subject")]
  subject: String,
  /// to_corp_or_alliance_id integer
  #[serde(rename = "to_corp_or_alliance_id")]
  to_corp_or_alliance_id: Option<i32>,
  /// Corporations, alliances and mailing lists are all types of mailing groups. You may only send to one mailing group, at a time, so you may fill out either this field or the to_corp_or_alliance_ids field
  #[serde(rename = "to_mailing_list_id")]
  to_mailing_list_id: Option<i32>
}

impl PostUiOpenwindowNewmailNewMail {
  /// new_mail object
  pub fn new(body: String, recipients: Vec<i32>, subject: String) -> PostUiOpenwindowNewmailNewMail {
    PostUiOpenwindowNewmailNewMail {
      body: body,
      recipients: recipients,
      subject: subject,
      to_corp_or_alliance_id: None,
      to_mailing_list_id: None
    }
  }

  pub fn set_body(&mut self, body: String) {
    self.body = body;
  }

  pub fn with_body(mut self, body: String) -> PostUiOpenwindowNewmailNewMail {
    self.body = body;
    self
  }

  pub fn body(&self) -> &String {
    &self.body
  }


  pub fn set_recipients(&mut self, recipients: Vec<i32>) {
    self.recipients = recipients;
  }

  pub fn with_recipients(mut self, recipients: Vec<i32>) -> PostUiOpenwindowNewmailNewMail {
    self.recipients = recipients;
    self
  }

  pub fn recipients(&self) -> &Vec<i32> {
    &self.recipients
  }


  pub fn set_subject(&mut self, subject: String) {
    self.subject = subject;
  }

  pub fn with_subject(mut self, subject: String) -> PostUiOpenwindowNewmailNewMail {
    self.subject = subject;
    self
  }

  pub fn subject(&self) -> &String {
    &self.subject
  }


  pub fn set_to_corp_or_alliance_id(&mut self, to_corp_or_alliance_id: i32) {
    self.to_corp_or_alliance_id = Some(to_corp_or_alliance_id);
  }

  pub fn with_to_corp_or_alliance_id(mut self, to_corp_or_alliance_id: i32) -> PostUiOpenwindowNewmailNewMail {
    self.to_corp_or_alliance_id = Some(to_corp_or_alliance_id);
    self
  }

  pub fn to_corp_or_alliance_id(&self) -> Option<&i32> {
    self.to_corp_or_alliance_id.as_ref()
  }

  pub fn reset_to_corp_or_alliance_id(&mut self) {
    self.to_corp_or_alliance_id = None;
  }

  pub fn set_to_mailing_list_id(&mut self, to_mailing_list_id: i32) {
    self.to_mailing_list_id = Some(to_mailing_list_id);
  }

  pub fn with_to_mailing_list_id(mut self, to_mailing_list_id: i32) -> PostUiOpenwindowNewmailNewMail {
    self.to_mailing_list_id = Some(to_mailing_list_id);
    self
  }

  pub fn to_mailing_list_id(&self) -> Option<&i32> {
    self.to_mailing_list_id.as_ref()
  }

  pub fn reset_to_mailing_list_id(&mut self) {
    self.to_mailing_list_id = None;
  }

}



