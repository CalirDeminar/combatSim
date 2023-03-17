pub mod simulation {
    use crate::units;
    use crate::force;
    use force::{Force, Element};
    use units::{Unit, weapons};
    use weapons::Weapon;
    use rand::Rng;


    pub const MISSED: &str = "missed";
    pub const HIT: &str = "hit";
    pub const PENNED: &str = "penned";
    pub const KILLED: &str = "killed";

    pub fn hit_check(weapon: &Weapon, target: &Unit) -> bool {
        let mut rng = rand::thread_rng();
    
        let i: f32 = rng.gen();
        return i < 0.2 && target_attackable(weapon, target);
    }

    pub fn pen_check(weapon: &Weapon, target: &Unit) -> bool {
        let mut rng = rand::thread_rng();
        let i: f32 = rng.gen();
        let pen_chance = weapon.pen as f32/(target.armour * 2) as f32;
        return i < pen_chance.clamp(0.0, 1.0);
    }
    
    pub fn kill_check(weapon: &Weapon, target: &Unit) -> bool {
        let mut rng = rand::thread_rng();
        let i: f32 = rng.gen();
        let pen_chance = weapon.pen as f32/(target.hp * 2) as f32;
        return i < pen_chance.clamp(0.0, 1.0);
    }

    pub fn random_target(force: &mut Force) -> Option<&mut Element>  {
        let elements = &mut force.forces;
        let total_forces = elements.into_iter().fold(0, |acc, e| acc + (e.count * e.unit_type.size));
        let odds = elements.into_iter().map(|e| ((e.count * e.unit_type.size) as f32 / total_forces as f32, e));
        let mut i: f32 = rand::thread_rng().gen();
        for entry in odds {
            if i < entry.0 {
                return Some(entry.1);
            }
            i -= entry.0;
        }
        return None;
    }

    pub fn target_attackable(weapon: &Weapon, target: &Unit) -> bool {
        let mut max_target_range = weapon.range;
        for w in target.weapons.clone() {
            if w.range > max_target_range {
                max_target_range = w.range;
            }
        }
        let attackable_odds = weapon.range as f32 / max_target_range as f32;
        let i: f32 = rand::thread_rng().gen();
        return i < attackable_odds;
    }
}