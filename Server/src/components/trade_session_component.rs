use spacetimedb::spacetimedb;

#[spacetimedb(table)]
#[derive(Clone)]
pub struct TradeSessionComponent {
    #[unique]
    pub entity_id: u32,
    pub initiator_entity_id: u32,
    pub acceptor_entity_id: u32,
    pub acceptor_offer_inventory_entity_id: u32,
    pub initiator_offer_inventory_entity_id: u32,
    pub approved_by_initiator: bool,
    pub approved_by_acceptor: bool,
}

#[spacetimedb(table)]
pub struct ActiveTradeComponent {
    #[unique]
    pub entity_id: u32,
    pub trade_session_entity_id: u32,
}
