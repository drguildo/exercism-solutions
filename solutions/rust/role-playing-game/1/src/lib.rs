// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            if self.level >= 10 {
                return Some(Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                });
            } else {
                return Some(Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                });
            }
        }

        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana_amount) = self.mana {
            if mana_cost > mana_amount {
                0
            } else {
                self.mana = Some(mana_amount - mana_cost);
                mana_cost * 2
            }
        } else {
            if (mana_cost > self.health) {
                self.health = 0;
            } else {
                self.health -= mana_cost;
            }
            0
        }
    }
}
