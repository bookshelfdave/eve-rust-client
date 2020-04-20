/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetMarketsStructuresStructureId200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMarketsStructuresStructureId200Ok {
  /// duration integer
  #[serde(rename = "duration")]
  duration: i32,
  /// is_buy_order boolean
  #[serde(rename = "is_buy_order")]
  is_buy_order: bool,
  /// issued string
  #[serde(rename = "issued")]
  issued: String,
  /// location_id integer
  #[serde(rename = "location_id")]
  location_id: i64,
  /// min_volume integer
  #[serde(rename = "min_volume")]
  min_volume: i32,
  /// order_id integer
  #[serde(rename = "order_id")]
  order_id: i64,
  /// price number
  #[serde(rename = "price")]
  price: f64,
  /// range string
  #[serde(rename = "range")]
  range: String,
  /// type_id integer
  #[serde(rename = "type_id")]
  type_id: i32,
  /// volume_remain integer
  #[serde(rename = "volume_remain")]
  volume_remain: i32,
  /// volume_total integer
  #[serde(rename = "volume_total")]
  volume_total: i32
}

impl GetMarketsStructuresStructureId200Ok {
  /// 200 ok object
  pub fn new(duration: i32, is_buy_order: bool, issued: String, location_id: i64, min_volume: i32, order_id: i64, price: f64, range: String, type_id: i32, volume_remain: i32, volume_total: i32) -> GetMarketsStructuresStructureId200Ok {
    GetMarketsStructuresStructureId200Ok {
      duration: duration,
      is_buy_order: is_buy_order,
      issued: issued,
      location_id: location_id,
      min_volume: min_volume,
      order_id: order_id,
      price: price,
      range: range,
      type_id: type_id,
      volume_remain: volume_remain,
      volume_total: volume_total
    }
  }

  pub fn set_duration(&mut self, duration: i32) {
    self.duration = duration;
  }

  pub fn with_duration(mut self, duration: i32) -> GetMarketsStructuresStructureId200Ok {
    self.duration = duration;
    self
  }

  pub fn duration(&self) -> &i32 {
    &self.duration
  }


  pub fn set_is_buy_order(&mut self, is_buy_order: bool) {
    self.is_buy_order = is_buy_order;
  }

  pub fn with_is_buy_order(mut self, is_buy_order: bool) -> GetMarketsStructuresStructureId200Ok {
    self.is_buy_order = is_buy_order;
    self
  }

  pub fn is_buy_order(&self) -> &bool {
    &self.is_buy_order
  }


  pub fn set_issued(&mut self, issued: String) {
    self.issued = issued;
  }

  pub fn with_issued(mut self, issued: String) -> GetMarketsStructuresStructureId200Ok {
    self.issued = issued;
    self
  }

  pub fn issued(&self) -> &String {
    &self.issued
  }


  pub fn set_location_id(&mut self, location_id: i64) {
    self.location_id = location_id;
  }

  pub fn with_location_id(mut self, location_id: i64) -> GetMarketsStructuresStructureId200Ok {
    self.location_id = location_id;
    self
  }

  pub fn location_id(&self) -> &i64 {
    &self.location_id
  }


  pub fn set_min_volume(&mut self, min_volume: i32) {
    self.min_volume = min_volume;
  }

  pub fn with_min_volume(mut self, min_volume: i32) -> GetMarketsStructuresStructureId200Ok {
    self.min_volume = min_volume;
    self
  }

  pub fn min_volume(&self) -> &i32 {
    &self.min_volume
  }


  pub fn set_order_id(&mut self, order_id: i64) {
    self.order_id = order_id;
  }

  pub fn with_order_id(mut self, order_id: i64) -> GetMarketsStructuresStructureId200Ok {
    self.order_id = order_id;
    self
  }

  pub fn order_id(&self) -> &i64 {
    &self.order_id
  }


  pub fn set_price(&mut self, price: f64) {
    self.price = price;
  }

  pub fn with_price(mut self, price: f64) -> GetMarketsStructuresStructureId200Ok {
    self.price = price;
    self
  }

  pub fn price(&self) -> &f64 {
    &self.price
  }


  pub fn set_range(&mut self, range: String) {
    self.range = range;
  }

  pub fn with_range(mut self, range: String) -> GetMarketsStructuresStructureId200Ok {
    self.range = range;
    self
  }

  pub fn range(&self) -> &String {
    &self.range
  }


  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = type_id;
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetMarketsStructuresStructureId200Ok {
    self.type_id = type_id;
    self
  }

  pub fn type_id(&self) -> &i32 {
    &self.type_id
  }


  pub fn set_volume_remain(&mut self, volume_remain: i32) {
    self.volume_remain = volume_remain;
  }

  pub fn with_volume_remain(mut self, volume_remain: i32) -> GetMarketsStructuresStructureId200Ok {
    self.volume_remain = volume_remain;
    self
  }

  pub fn volume_remain(&self) -> &i32 {
    &self.volume_remain
  }


  pub fn set_volume_total(&mut self, volume_total: i32) {
    self.volume_total = volume_total;
  }

  pub fn with_volume_total(mut self, volume_total: i32) -> GetMarketsStructuresStructureId200Ok {
    self.volume_total = volume_total;
    self
  }

  pub fn volume_total(&self) -> &i32 {
    &self.volume_total
  }


}



