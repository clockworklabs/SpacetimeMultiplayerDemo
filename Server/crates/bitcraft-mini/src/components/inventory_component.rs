use crate::tuples::Pocket;
use spacetimedb::spacetimedb;

#[spacetimedb(table)]
#[derive(Debug)]
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
}
