use crate::components::{ActiveTradeComponent, InventoryComponent, PlayerComponent, TradeSessionComponent};
use crate::tuples::Pocket;
use spacetimedb::hash::Hash;
use spacetimedb::println;
use spacetimedb::spacetimedb;

#[spacetimedb(reducer)]
pub fn initiate_trade_session(identity: Hash, timestamp: u64, initiator_entity_id: u32, acceptor_entity_id: u32) {
    println!(
        "Attempting trade session between {} and {}",
        initiator_entity_id, acceptor_entity_id
    );

    let initiator =
        PlayerComponent::filter_entity_id_eq(initiator_entity_id).expect("The trade initiator doesn't exist!");
    let _acceptor =
        PlayerComponent::filter_entity_id_eq(acceptor_entity_id).expect("The trade acceptor doesn't exist!");

    if ActiveTradeComponent::filter_entity_id_eq(initiator_entity_id).is_some() {
        println!("Trade initiator is already in a trading session.");
        return;
    }

    if ActiveTradeComponent::filter_entity_id_eq(acceptor_entity_id).is_some() {
        println!("Trade acceptor is already in a trading session.");
        return;
    }

    // Make sure this identity owns this player
    if initiator.owner_id != identity {
        println!("This identity doesn't own this player! (allowed for now)");
    }

    // ToDo: We definitely need a way to get unique ids in sequential tables.
    let trade_session_entity_id = timestamp as u32;
    let initiator_offer_inventory_entity_id = trade_session_entity_id + 1;
    let acceptor_offer_inventory_entity_id = trade_session_entity_id + 2;

    // Create trade session
    let trade_session = TradeSessionComponent {
        entity_id: trade_session_entity_id,
        initiator_entity_id,
        acceptor_entity_id,
        initiator_offer_inventory_entity_id,
        acceptor_offer_inventory_entity_id,
        approved_by_acceptor: false,
        approved_by_initiator: false,
    };
    TradeSessionComponent::insert(trade_session);

    // Create trade components on each participant
    let acceptor_trade = ActiveTradeComponent {
        entity_id: acceptor_entity_id,
        trade_session_entity_id,
    };
    ActiveTradeComponent::insert(acceptor_trade);

    let initiator_trade = ActiveTradeComponent {
        entity_id: initiator_entity_id,
        trade_session_entity_id,
    };
    ActiveTradeComponent::insert(initiator_trade);

    // Create trade session inventories
    let initiator_offer = InventoryComponent {
        entity_id: initiator_offer_inventory_entity_id,
        pockets: Vec::<Pocket>::new(),
    };
    InventoryComponent::insert(initiator_offer);

    let acceptor_offer = InventoryComponent {
        entity_id: acceptor_offer_inventory_entity_id,
        pockets: Vec::<Pocket>::new(),
    };
    InventoryComponent::insert(acceptor_offer);
}

#[spacetimedb(reducer)]
pub fn add_to_trade(
    identity: Hash,
    _timestamp: u64,
    participant_entity_id: u32,
    source_pocket_id: u32,
    dest_pocket_id: u32,
) {
    let participant = PlayerComponent::filter_entity_id_eq(participant_entity_id).expect("This player doesn't exist!");

    // Make sure this identity owns this player
    if participant.owner_id != identity {
        println!("This identity doesn't own this player! (allowed for now)");
    }

    // Retrieve active trade session entity_id
    let active_session =
        ActiveTradeComponent::filter_entity_id_eq(participant_entity_id).expect("There is no ongoing trade.");

    // Retrieve and update trade session
    let mut session = TradeSessionComponent::filter_entity_id_eq(active_session.trade_session_entity_id)
        .expect("This trade session no longer exists.")
        .clone();

    let offer_inventory_entity_id = if session.initiator_entity_id == participant_entity_id {
        session.initiator_offer_inventory_entity_id
    } else {
        session.acceptor_offer_inventory_entity_id
    };

    // Contents changed, trade is no longer approved by anyone
    session.approved_by_acceptor = false;
    session.approved_by_initiator = false;
    TradeSessionComponent::update_entity_id_eq(active_session.trade_session_entity_id, session);

    // Remove from player inventory
    let mut player_inventory =
        InventoryComponent::filter_entity_id_eq(participant_entity_id).expect("Player has no inventory.");
    let pocket = player_inventory
        .get_pocket(source_pocket_id)
        .expect("Traded items do not exist");
    if !player_inventory.add(pocket.item_id, -pocket.item_count, Some(source_pocket_id)) {
        panic!("Failed to remove item from player inventory");
    }
    InventoryComponent::update_entity_id_eq(participant_entity_id, player_inventory);

    // Add to trade offer
    let mut offer_inventory =
        InventoryComponent::filter_entity_id_eq(offer_inventory_entity_id).expect("Trade session has no such offer");
    if !offer_inventory.add(pocket.item_id, pocket.item_count, Some(dest_pocket_id)) {
        panic!("Failed to add item to trade window");
    }
    InventoryComponent::update_entity_id_eq(offer_inventory_entity_id, offer_inventory);
}

#[spacetimedb(reducer)]
pub fn remove_from_trade(
    identity: Hash,
    _timestamp: u64,
    participant_entity_id: u32,
    source_pocket_id: u32,
    dest_pocket_id: u32,
) {
    let participant = PlayerComponent::filter_entity_id_eq(participant_entity_id).expect("This player doesn't exist!");

    // Make sure this identity owns this player
    if participant.owner_id != identity {
        println!("This identity doesn't own this player! (allowed for now)");
    }

    // Retrieve active trade session entity_id
    let active_session =
        ActiveTradeComponent::filter_entity_id_eq(participant_entity_id).expect("There is no ongoing trade.");

    // Retrieve and update trade session
    let mut session = TradeSessionComponent::filter_entity_id_eq(active_session.trade_session_entity_id)
        .expect("This trade session no longer exists.")
        .clone();

    let offer_inventory_entity_id = if session.initiator_entity_id == participant_entity_id {
        session.initiator_offer_inventory_entity_id
    } else {
        session.acceptor_offer_inventory_entity_id
    };

    // Contents changed, trade is no longer approved by anyone
    session.approved_by_acceptor = false;
    session.approved_by_initiator = false;
    TradeSessionComponent::update_entity_id_eq(active_session.trade_session_entity_id, session);

    // Remove from trade offer
    let mut offer_inventory =
        InventoryComponent::filter_entity_id_eq(offer_inventory_entity_id).expect("Trade session has no such offer");
    let pocket = offer_inventory
        .get_pocket(source_pocket_id)
        .expect("Traded items do not exist");
    if !offer_inventory.add(pocket.item_id, -pocket.item_count, Some(source_pocket_id)) {
        panic!("Failed to remove item from trade inventory");
    }
    InventoryComponent::update_entity_id_eq(offer_inventory_entity_id, offer_inventory);

    // Add to player inventory
    let mut player_inventory =
        InventoryComponent::filter_entity_id_eq(participant_entity_id).expect("Player has no inventory.");
    if !player_inventory.add(pocket.item_id, pocket.item_count, Some(dest_pocket_id)) {
        panic!("Failed to add item to player inventory");
    }
    InventoryComponent::update_entity_id_eq(participant_entity_id, player_inventory);
}

#[spacetimedb(reducer)]
pub fn toggle_accept_trade(identity: Hash, _timestamp: u64, participant_entity_id: u32) {
    let participant = PlayerComponent::filter_entity_id_eq(participant_entity_id).expect("This player doesn't exist!");

    // Make sure this identity owns this player
    if participant.owner_id != identity {
        println!("This identity doesn't own this player! (allowed for now)");
    }

    // Retrieve active trade session entity_id
    let active_session =
        ActiveTradeComponent::filter_entity_id_eq(participant_entity_id).expect("There is no trade to approve.");

    // Retrieve and update trade session
    let mut session = TradeSessionComponent::filter_entity_id_eq(active_session.trade_session_entity_id)
        .expect("This trade session no longer exists.");

    if session.acceptor_entity_id == participant_entity_id {
        session.approved_by_acceptor = !session.approved_by_acceptor;
    } else if session.initiator_entity_id == participant_entity_id {
        session.approved_by_initiator = !session.approved_by_initiator;
    } else {
        panic!("This player is not part of the trade session. How is this possible?");
    }
    let close_session = session.approved_by_acceptor && session.approved_by_initiator;
    TradeSessionComponent::update_entity_id_eq(active_session.trade_session_entity_id, session);

    // If session is approved by both parties, conclude it succesfully
    if close_session {
        close_trade_session(active_session.trade_session_entity_id, true);
    }
}

#[spacetimedb(reducer)]
pub fn refuse_trade(identity: Hash, _timestamp: u64, participant_entity_id: u32) {
    let partipant = PlayerComponent::filter_entity_id_eq(participant_entity_id).expect("This player doesn't exist!");

    // Make sure this identity owns this player
    if partipant.owner_id != identity {
        println!("This identity doesn't own this player! (allowed for now)");
    }

    cancel_trade_session_with_participant(participant_entity_id);
}

pub fn cancel_trade_session_with_participant(participant_entity_id: u32) {
    // Retrieve active trade session entity_id
    if let Some(active_session) = ActiveTradeComponent::filter_entity_id_eq(participant_entity_id) {
        close_trade_session(active_session.trade_session_entity_id, false);
    }
}

pub fn close_trade_session(session_entity_id: u32, success: bool) {
    let session = TradeSessionComponent::filter_entity_id_eq(session_entity_id).unwrap();

    let can_trade = if success {
        // make sure both participants can receive every item of the trade
        let inventory = InventoryComponent::filter_entity_id_eq(session.acceptor_offer_inventory_entity_id)
            .expect("There is no acceptor offer in this trade session.");
        let items: Vec<(u32, i32)> = inventory.pockets.iter().map(|p| (p.item_id, p.item_count)).collect();
        if inventory.can_hold(&items) {
            let inventory = InventoryComponent::filter_entity_id_eq(session.initiator_offer_inventory_entity_id)
                .expect("There is no initiator offer in this trade session.");
            let items: Vec<(u32, i32)> = inventory.pockets.iter().map(|p| (p.item_id, p.item_count)).collect();
            inventory.can_hold(&items)
        } else {
            false
        }
    } else {
        false
    };

    if can_trade {
        // move offer contents into other participant's inventories
        let offer_inventory = InventoryComponent::filter_entity_id_eq(session.initiator_offer_inventory_entity_id)
            .expect("There is no initiator offer in this trade session.");
        let mut player_inventory = InventoryComponent::filter_entity_id_eq(session.acceptor_entity_id)
            .expect("There is no acceptor in this trade session.");
        player_inventory.combine(&offer_inventory);
        InventoryComponent::update_entity_id_eq(session.acceptor_entity_id, player_inventory);

        let offer_inventory = InventoryComponent::filter_entity_id_eq(session.acceptor_offer_inventory_entity_id)
            .expect("There is no acceptor offer in this trade session.");
        let mut player_inventory = InventoryComponent::filter_entity_id_eq(session.initiator_entity_id)
            .expect("There is no initiator in this trade session.");
        player_inventory.combine(&offer_inventory);
        InventoryComponent::update_entity_id_eq(session.initiator_entity_id, player_inventory);
    } else {
        // move offer contents back into each participant's inventories
        let offer_inventory = InventoryComponent::filter_entity_id_eq(session.initiator_offer_inventory_entity_id)
            .expect("There is no initiator offer in this trade session.");
        let mut player_inventory = InventoryComponent::filter_entity_id_eq(session.initiator_entity_id)
            .expect("There is no initiator in this trade session.");
        player_inventory.combine(&offer_inventory);
        InventoryComponent::update_entity_id_eq(session.initiator_entity_id, player_inventory);

        let offer_inventory = InventoryComponent::filter_entity_id_eq(session.acceptor_offer_inventory_entity_id)
            .expect("There is no acceptor offer in this trade session.");
        let mut player_inventory = InventoryComponent::filter_entity_id_eq(session.acceptor_entity_id)
            .expect("There is no acceptor in this trade session.");
        player_inventory.combine(&offer_inventory);
        InventoryComponent::update_entity_id_eq(session.acceptor_entity_id, player_inventory);
    }

    // delete everything session-related
    TradeSessionComponent::delete_entity_id_eq(session.entity_id);
    ActiveTradeComponent::delete_entity_id_eq(session.initiator_entity_id);
    ActiveTradeComponent::delete_entity_id_eq(session.acceptor_entity_id);
    InventoryComponent::delete_entity_id_eq(session.initiator_offer_inventory_entity_id);
    InventoryComponent::delete_entity_id_eq(session.acceptor_offer_inventory_entity_id);
}