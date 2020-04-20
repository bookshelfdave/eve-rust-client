/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdStatsMarket : market object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdStatsMarket {
  /// accept_contracts_courier integer
  #[serde(rename = "accept_contracts_courier")]
  accept_contracts_courier: Option<i64>,
  /// accept_contracts_item_exchange integer
  #[serde(rename = "accept_contracts_item_exchange")]
  accept_contracts_item_exchange: Option<i64>,
  /// buy_orders_placed integer
  #[serde(rename = "buy_orders_placed")]
  buy_orders_placed: Option<i64>,
  /// cancel_market_order integer
  #[serde(rename = "cancel_market_order")]
  cancel_market_order: Option<i64>,
  /// create_contracts_auction integer
  #[serde(rename = "create_contracts_auction")]
  create_contracts_auction: Option<i64>,
  /// create_contracts_courier integer
  #[serde(rename = "create_contracts_courier")]
  create_contracts_courier: Option<i64>,
  /// create_contracts_item_exchange integer
  #[serde(rename = "create_contracts_item_exchange")]
  create_contracts_item_exchange: Option<i64>,
  /// deliver_courier_contract integer
  #[serde(rename = "deliver_courier_contract")]
  deliver_courier_contract: Option<i64>,
  /// isk_gained integer
  #[serde(rename = "isk_gained")]
  isk_gained: Option<i64>,
  /// isk_spent integer
  #[serde(rename = "isk_spent")]
  isk_spent: Option<i64>,
  /// modify_market_order integer
  #[serde(rename = "modify_market_order")]
  modify_market_order: Option<i64>,
  /// search_contracts integer
  #[serde(rename = "search_contracts")]
  search_contracts: Option<i64>,
  /// sell_orders_placed integer
  #[serde(rename = "sell_orders_placed")]
  sell_orders_placed: Option<i64>
}

impl GetCharactersCharacterIdStatsMarket {
  /// market object
  pub fn new() -> GetCharactersCharacterIdStatsMarket {
    GetCharactersCharacterIdStatsMarket {
      accept_contracts_courier: None,
      accept_contracts_item_exchange: None,
      buy_orders_placed: None,
      cancel_market_order: None,
      create_contracts_auction: None,
      create_contracts_courier: None,
      create_contracts_item_exchange: None,
      deliver_courier_contract: None,
      isk_gained: None,
      isk_spent: None,
      modify_market_order: None,
      search_contracts: None,
      sell_orders_placed: None
    }
  }

  pub fn set_accept_contracts_courier(&mut self, accept_contracts_courier: i64) {
    self.accept_contracts_courier = Some(accept_contracts_courier);
  }

  pub fn with_accept_contracts_courier(mut self, accept_contracts_courier: i64) -> GetCharactersCharacterIdStatsMarket {
    self.accept_contracts_courier = Some(accept_contracts_courier);
    self
  }

  pub fn accept_contracts_courier(&self) -> Option<&i64> {
    self.accept_contracts_courier.as_ref()
  }

  pub fn reset_accept_contracts_courier(&mut self) {
    self.accept_contracts_courier = None;
  }

  pub fn set_accept_contracts_item_exchange(&mut self, accept_contracts_item_exchange: i64) {
    self.accept_contracts_item_exchange = Some(accept_contracts_item_exchange);
  }

  pub fn with_accept_contracts_item_exchange(mut self, accept_contracts_item_exchange: i64) -> GetCharactersCharacterIdStatsMarket {
    self.accept_contracts_item_exchange = Some(accept_contracts_item_exchange);
    self
  }

  pub fn accept_contracts_item_exchange(&self) -> Option<&i64> {
    self.accept_contracts_item_exchange.as_ref()
  }

  pub fn reset_accept_contracts_item_exchange(&mut self) {
    self.accept_contracts_item_exchange = None;
  }

  pub fn set_buy_orders_placed(&mut self, buy_orders_placed: i64) {
    self.buy_orders_placed = Some(buy_orders_placed);
  }

  pub fn with_buy_orders_placed(mut self, buy_orders_placed: i64) -> GetCharactersCharacterIdStatsMarket {
    self.buy_orders_placed = Some(buy_orders_placed);
    self
  }

  pub fn buy_orders_placed(&self) -> Option<&i64> {
    self.buy_orders_placed.as_ref()
  }

  pub fn reset_buy_orders_placed(&mut self) {
    self.buy_orders_placed = None;
  }

  pub fn set_cancel_market_order(&mut self, cancel_market_order: i64) {
    self.cancel_market_order = Some(cancel_market_order);
  }

  pub fn with_cancel_market_order(mut self, cancel_market_order: i64) -> GetCharactersCharacterIdStatsMarket {
    self.cancel_market_order = Some(cancel_market_order);
    self
  }

  pub fn cancel_market_order(&self) -> Option<&i64> {
    self.cancel_market_order.as_ref()
  }

  pub fn reset_cancel_market_order(&mut self) {
    self.cancel_market_order = None;
  }

  pub fn set_create_contracts_auction(&mut self, create_contracts_auction: i64) {
    self.create_contracts_auction = Some(create_contracts_auction);
  }

  pub fn with_create_contracts_auction(mut self, create_contracts_auction: i64) -> GetCharactersCharacterIdStatsMarket {
    self.create_contracts_auction = Some(create_contracts_auction);
    self
  }

  pub fn create_contracts_auction(&self) -> Option<&i64> {
    self.create_contracts_auction.as_ref()
  }

  pub fn reset_create_contracts_auction(&mut self) {
    self.create_contracts_auction = None;
  }

  pub fn set_create_contracts_courier(&mut self, create_contracts_courier: i64) {
    self.create_contracts_courier = Some(create_contracts_courier);
  }

  pub fn with_create_contracts_courier(mut self, create_contracts_courier: i64) -> GetCharactersCharacterIdStatsMarket {
    self.create_contracts_courier = Some(create_contracts_courier);
    self
  }

  pub fn create_contracts_courier(&self) -> Option<&i64> {
    self.create_contracts_courier.as_ref()
  }

  pub fn reset_create_contracts_courier(&mut self) {
    self.create_contracts_courier = None;
  }

  pub fn set_create_contracts_item_exchange(&mut self, create_contracts_item_exchange: i64) {
    self.create_contracts_item_exchange = Some(create_contracts_item_exchange);
  }

  pub fn with_create_contracts_item_exchange(mut self, create_contracts_item_exchange: i64) -> GetCharactersCharacterIdStatsMarket {
    self.create_contracts_item_exchange = Some(create_contracts_item_exchange);
    self
  }

  pub fn create_contracts_item_exchange(&self) -> Option<&i64> {
    self.create_contracts_item_exchange.as_ref()
  }

  pub fn reset_create_contracts_item_exchange(&mut self) {
    self.create_contracts_item_exchange = None;
  }

  pub fn set_deliver_courier_contract(&mut self, deliver_courier_contract: i64) {
    self.deliver_courier_contract = Some(deliver_courier_contract);
  }

  pub fn with_deliver_courier_contract(mut self, deliver_courier_contract: i64) -> GetCharactersCharacterIdStatsMarket {
    self.deliver_courier_contract = Some(deliver_courier_contract);
    self
  }

  pub fn deliver_courier_contract(&self) -> Option<&i64> {
    self.deliver_courier_contract.as_ref()
  }

  pub fn reset_deliver_courier_contract(&mut self) {
    self.deliver_courier_contract = None;
  }

  pub fn set_isk_gained(&mut self, isk_gained: i64) {
    self.isk_gained = Some(isk_gained);
  }

  pub fn with_isk_gained(mut self, isk_gained: i64) -> GetCharactersCharacterIdStatsMarket {
    self.isk_gained = Some(isk_gained);
    self
  }

  pub fn isk_gained(&self) -> Option<&i64> {
    self.isk_gained.as_ref()
  }

  pub fn reset_isk_gained(&mut self) {
    self.isk_gained = None;
  }

  pub fn set_isk_spent(&mut self, isk_spent: i64) {
    self.isk_spent = Some(isk_spent);
  }

  pub fn with_isk_spent(mut self, isk_spent: i64) -> GetCharactersCharacterIdStatsMarket {
    self.isk_spent = Some(isk_spent);
    self
  }

  pub fn isk_spent(&self) -> Option<&i64> {
    self.isk_spent.as_ref()
  }

  pub fn reset_isk_spent(&mut self) {
    self.isk_spent = None;
  }

  pub fn set_modify_market_order(&mut self, modify_market_order: i64) {
    self.modify_market_order = Some(modify_market_order);
  }

  pub fn with_modify_market_order(mut self, modify_market_order: i64) -> GetCharactersCharacterIdStatsMarket {
    self.modify_market_order = Some(modify_market_order);
    self
  }

  pub fn modify_market_order(&self) -> Option<&i64> {
    self.modify_market_order.as_ref()
  }

  pub fn reset_modify_market_order(&mut self) {
    self.modify_market_order = None;
  }

  pub fn set_search_contracts(&mut self, search_contracts: i64) {
    self.search_contracts = Some(search_contracts);
  }

  pub fn with_search_contracts(mut self, search_contracts: i64) -> GetCharactersCharacterIdStatsMarket {
    self.search_contracts = Some(search_contracts);
    self
  }

  pub fn search_contracts(&self) -> Option<&i64> {
    self.search_contracts.as_ref()
  }

  pub fn reset_search_contracts(&mut self) {
    self.search_contracts = None;
  }

  pub fn set_sell_orders_placed(&mut self, sell_orders_placed: i64) {
    self.sell_orders_placed = Some(sell_orders_placed);
  }

  pub fn with_sell_orders_placed(mut self, sell_orders_placed: i64) -> GetCharactersCharacterIdStatsMarket {
    self.sell_orders_placed = Some(sell_orders_placed);
    self
  }

  pub fn sell_orders_placed(&self) -> Option<&i64> {
    self.sell_orders_placed.as_ref()
  }

  pub fn reset_sell_orders_placed(&mut self) {
    self.sell_orders_placed = None;
  }

}



