use crate::tables::Config;
use crate::tuples::Pocket;
use spacetimedb::spacetimedb;

#[spacetimedb(table)]
#[derive(Debug, Clone)]
pub struct InventoryComponent {
    #[unique]
    pub entity_id: u32,
    pub pockets: Vec<Pocket>,
}

impl InventoryComponent {
    pub fn get_pocket(&self, pocket_idx: u32) -> Option<Pocket> {
        for x in 0..self.pockets.len() {
            if self.pockets[x].pocket_idx == pocket_idx && self.pockets[x].item_count > 0 {
                return Some(self.pockets[x]);
            }
        }

        return None;
    }

    pub fn set_pocket(&mut self, pocket: Pocket) {
        // Try to find the pocket in the inventory
        for x in 0..self.pockets.len() {
            if self.pockets[x].pocket_idx == pocket.pocket_idx {
                self.pockets[x] = pocket;
                return;
            }
        }

        // We did not find this pocket, create a new pocket
        self.pockets.push(pocket);
    }

    pub fn delete_pocket(&mut self, pocket_idx: u32) {
        // Try to find the pocket in the inventory
        for x in 0..self.pockets.len() {
            if self.pockets[x].pocket_idx == pocket_idx {
                self.pockets.remove(x);
                return;
            }
        }
    }

    pub fn add(&mut self, item_id: u32, item_count: i32, index: Option<u32>) -> bool {
        // Check to see if this pocket index is bad
        let config = Config::filter_by_version(0).unwrap();

        // Change empty pocket index for the first EMPTY pocket index
        let pocket_idx = if index.is_none() {
            let mut idx = u32::MAX;
            for i in 0..config.max_player_inventory_slots {
                if self.get_pocket(i).is_none() {
                    idx = i;
                    break;
                }
            }
            if idx >= config.max_player_inventory_slots {
                return false;
            }
            idx
        } else {
            index.unwrap()
        };

        if pocket_idx >= config.max_player_inventory_slots {
            return false;
        }

        let pocket = match self.get_pocket(pocket_idx) {
            Some(mut pocket) => {
                assert_eq!(pocket.item_id, item_id, "Item ID mismatch");
                if pocket.item_count + item_count < 0 {
                    // removed more than what's available
                    return false;
                }
                pocket.item_count += item_count;
                pocket
            }
            None => Pocket {
                pocket_idx,
                item_id,
                item_count,
            },
        };

        if pocket.item_count == 0 {
            self.delete_pocket(pocket.pocket_idx);
        } else {
            self.set_pocket(pocket);
        }
        return true;
    }

    pub fn can_hold(&self, items: &Vec<(u32, i32)>) -> bool {
        let mut copy = self.clone();
        let mut success = true;
        for &(item_id, item_count) in items {
            success &= copy.add(item_id, item_count, None);
        }
        success
    }

    pub fn combine(&mut self, other: &InventoryComponent) -> bool {
        let other_items: Vec<(u32, i32)> = other.pockets.iter().map(|p| (p.item_id, p.item_count)).collect();
        if !self.can_hold(&other_items) {
            return false;
        }
        for (item_id, item_count) in other_items {
            self.add(item_id, item_count, None);
        }
        true
    }
}
