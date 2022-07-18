use spacetimedb_bindgen::spacetimedb;
use spacetimedb_bindings::hash::Hash;

#[spacetimedb(tuple)]
pub struct Position {
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
}

#[spacetimedb(tuple)]
pub struct Rotation {
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub rot_w: f32,
}

#[spacetimedb(table(2))]
pub struct Character {
    #[primary_key]
    ident: CharacterIdentity,
    character_name: String,
}

#[spacetimedb(tuple)]
pub struct CharacterIdentity {
    pub player_id: Hash,
    pub index: u32,
}

#[spacetimedb(table(1))]
pub struct Player {
    #[primary_key]
    pub player_id: Hash,
    pub creation_time: u64,
    pub position: Position,
    pub rotation: Rotation,
    pub moving: bool,
}

#[spacetimedb(reducer)]
pub fn move_player(player_id: Hash, _timestamp: u64, position: Position, rotation: Rotation, moving: bool) {
    let player = Player::filter_player_id_eq(player_id.clone());
    match player {
        Some(mut player) => {
            player.position = position;
            player.rotation = rotation;
            player.moving = moving;

            // Update player position
            Player::update_player_id_eq(player_id, player);
        },
        None => {
            // Player not found!
        }
    }
}

#[spacetimedb(reducer)]
pub fn create_new_player(player_id: Hash, timestamp: u64, position: Position, rotation: Rotation) {
    // let player = Player {
    //     player_id,
    //     creation_time: timestamp,
    //     position,
    //     rotation,
    //     moving: false
    // };
    //
    // Player::insert(player);

    let ident = CharacterIdentity {
        player_id,
        index: 0,
    };

    Character::insert(Character {
        ident: ident.clone(),
        character_name: "My Character".to_string(),
    });

    let got_character = Character::filter_ident_eq(ident);
    match got_character {
        Some(a) => {
            if a.character_name.eq("My Character".to_string()) {
                panic!("Success!");
            }

            panic!("Failure 2");
        }, None => {
            panic!("Failure!");
        }
    }
}