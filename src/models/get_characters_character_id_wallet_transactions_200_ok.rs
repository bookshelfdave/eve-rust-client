/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdWalletTransactions200Ok : wallet transaction

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdWalletTransactions200Ok {
  /// client_id integer
  #[serde(rename = "client_id")]
  client_id: i32,
  /// Date and time of transaction
  #[serde(rename = "date")]
  date: String,
  /// is_buy boolean
  #[serde(rename = "is_buy")]
  is_buy: bool,
  /// is_personal boolean
  #[serde(rename = "is_personal")]
  is_personal: bool,
  /// journal_ref_id integer
  #[serde(rename = "journal_ref_id")]
  journal_ref_id: i64,
  /// location_id integer
  #[serde(rename = "location_id")]
  location_id: i64,
  /// quantity integer
  #[serde(rename = "quantity")]
  quantity: i32,
  /// Unique transaction ID
  #[serde(rename = "transaction_id")]
  transaction_id: i64,
  /// type_id integer
  #[serde(rename = "type_id")]
  type_id: i32,
  /// Amount paid per unit
  #[serde(rename = "unit_price")]
  unit_price: f64
}

impl GetCharactersCharacterIdWalletTransactions200Ok {
  /// wallet transaction
  pub fn new(client_id: i32, date: String, is_buy: bool, is_personal: bool, journal_ref_id: i64, location_id: i64, quantity: i32, transaction_id: i64, type_id: i32, unit_price: f64) -> GetCharactersCharacterIdWalletTransactions200Ok {
    GetCharactersCharacterIdWalletTransactions200Ok {
      client_id: client_id,
      date: date,
      is_buy: is_buy,
      is_personal: is_personal,
      journal_ref_id: journal_ref_id,
      location_id: location_id,
      quantity: quantity,
      transaction_id: transaction_id,
      type_id: type_id,
      unit_price: unit_price
    }
  }

  pub fn set_client_id(&mut self, client_id: i32) {
    self.client_id = client_id;
  }

  pub fn with_client_id(mut self, client_id: i32) -> GetCharactersCharacterIdWalletTransactions200Ok {
    self.client_id = client_id;
    self
  }

  pub fn client_id(&self) -> &i32 {
    &self.client_id
  }


  pub fn set_date(&mut self, date: String) {
    self.date = date;
  }

  pub fn with_date(mut self, date: String) -> GetCharactersCharacterIdWalletTransactions200Ok {
    self.date = date;
    self
  }

  pub fn date(&self) -> &String {
    &self.date
  }


  pub fn set_is_buy(&mut self, is_buy: bool) {
    self.is_buy = is_buy;
  }

  pub fn with_is_buy(mut self, is_buy: bool) -> GetCharactersCharacterIdWalletTransactions200Ok {
    self.is_buy = is_buy;
    self
  }

  pub fn is_buy(&self) -> &bool {
    &self.is_buy
  }


  pub fn set_is_personal(&mut self, is_personal: bool) {
    self.is_personal = is_personal;
  }

  pub fn with_is_personal(mut self, is_personal: bool) -> GetCharactersCharacterIdWalletTransactions200Ok {
    self.is_personal = is_personal;
    self
  }

  pub fn is_personal(&self) -> &bool {
    &self.is_personal
  }


  pub fn set_journal_ref_id(&mut self, journal_ref_id: i64) {
    self.journal_ref_id = journal_ref_id;
  }

  pub fn with_journal_ref_id(mut self, journal_ref_id: i64) -> GetCharactersCharacterIdWalletTransactions200Ok {
    self.journal_ref_id = journal_ref_id;
    self
  }

  pub fn journal_ref_id(&self) -> &i64 {
    &self.journal_ref_id
  }


  pub fn set_location_id(&mut self, location_id: i64) {
    self.location_id = location_id;
  }

  pub fn with_location_id(mut self, location_id: i64) -> GetCharactersCharacterIdWalletTransactions200Ok {
    self.location_id = location_id;
    self
  }

  pub fn location_id(&self) -> &i64 {
    &self.location_id
  }


  pub fn set_quantity(&mut self, quantity: i32) {
    self.quantity = quantity;
  }

  pub fn with_quantity(mut self, quantity: i32) -> GetCharactersCharacterIdWalletTransactions200Ok {
    self.quantity = quantity;
    self
  }

  pub fn quantity(&self) -> &i32 {
    &self.quantity
  }


  pub fn set_transaction_id(&mut self, transaction_id: i64) {
    self.transaction_id = transaction_id;
  }

  pub fn with_transaction_id(mut self, transaction_id: i64) -> GetCharactersCharacterIdWalletTransactions200Ok {
    self.transaction_id = transaction_id;
    self
  }

  pub fn transaction_id(&self) -> &i64 {
    &self.transaction_id
  }


  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = type_id;
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetCharactersCharacterIdWalletTransactions200Ok {
    self.type_id = type_id;
    self
  }

  pub fn type_id(&self) -> &i32 {
    &self.type_id
  }


  pub fn set_unit_price(&mut self, unit_price: f64) {
    self.unit_price = unit_price;
  }

  pub fn with_unit_price(mut self, unit_price: f64) -> GetCharactersCharacterIdWalletTransactions200Ok {
    self.unit_price = unit_price;
    self
  }

  pub fn unit_price(&self) -> &f64 {
    &self.unit_price
  }


}



