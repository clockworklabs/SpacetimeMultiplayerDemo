use crate::tables::ServerGlobals;

pub fn get_globals() -> ServerGlobals {
    ServerGlobals::filter_by_version(&0).unwrap()
}

pub fn update_globals(globals: ServerGlobals) {
    ServerGlobals::update_by_version(&0, globals);
}

pub fn next_entity_id() -> u64 {
    let mut globals = get_globals().clone();
    globals.entity_id_counter += 1;
    let result = globals.entity_id_counter;
    update_globals(globals);

    result
}
