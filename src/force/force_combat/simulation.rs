pub mod simulation {
    use crate::force::force_combat::calc_supply_capacity;
    use crate::units;
    use crate::force;
    use crate::force::force_combat::CombatLog;
    use force::{Force, Element};
    use units::{Unit, weapons};
    use weapons::Weapons::*;
    use rand::Rng;


    pub const MISSED: &str = "missed";
    pub const HIT: &str = "hit";
    pub const PENNED: &str = "penned";
    pub const KILLED: &str = "killed";

    pub const BASE_HIT_RATE: f32 = 0.2;
    pub const OUT_OF_SUPPLY_ENGAGEMENT_CHANCE: f32 = 0.5;

    pub fn hit_check(weapon: &Weapon, target: &Element, has_supply: bool) -> bool {
        let mut rng = rand::thread_rng();
        let i: f32 = rng.gen();
        let foritification_modifier = 
            0.1 
            * (target.fortification.clone() as f32 / 10.0) 
            * (1.0 - ((target.unit_type.size as f32 - 1.0) / 100.0));
        
        // 0.1 * 5/10 * (1-((1-1) / 100)) = 0.05
        // 0.1 * 1/10 * (1-(8-1) / 100)) = 
        // if weapon.name == String::from("dagger") {
        //     print!("{:?} ", 0.2 - foritification_modifier);
        // }
        let mut hit_chance = BASE_HIT_RATE - foritification_modifier;
        if !has_supply {
            hit_chance *= OUT_OF_SUPPLY_ENGAGEMENT_CHANCE;
        }

        return i < BASE_HIT_RATE - foritification_modifier && target_attackable(weapon, &target.unit_type.clone());
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
        let pen_chance = weapon.damage as f32/(target.hp * 2) as f32;
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

    pub fn calculate_losses(attacker: Force, target: Force) -> (Force, Vec<CombatLog>) {
        let mut working_target = target.clone();
        let mut combat_log: Vec<CombatLog> = vec![];
        let has_supply = calc_supply_capacity(&(attacker.clone())) > attacker.supply_used;
        for element in attacker.forces {
            for _i in 1..(element.count + 1) {
                for weapon in &element.unit_type.weapons {
                    for _j in 1..(weapon.rof + 1) {
                        let target_option = random_target(&mut working_target);
                        if target_option.is_some() {
                            let target_element = target_option.unwrap();
                            let hit = hit_check(weapon, &target_element, has_supply);
                            let penned = hit && pen_check(weapon, &target_element.unit_type);
                            let killed = penned && kill_check(weapon, &target_element.unit_type);
                            let mut verb = MISSED;
                            if hit {
                                verb = HIT;
                            }
                            if penned {
                                verb = PENNED;
                            }
                            if killed {
                                verb = KILLED;
                                target_element.count -= 1;
                            }
                            combat_log.push(
                                CombatLog {
                                    attacker_force_name: attacker.name.clone(),
                                    attacker_unit_name: element.unit_type.name.clone(),
                                    weapon_name: weapon.name.clone(),
                                    target_force_name: target.name.clone(),
                                    target_unit_name: target_element.unit_type.name.clone(),
                                    verb: String::from(verb),
                                }
                            );
                        }
                    }
                }
            }
        }
        return (working_target, combat_log);
    }
}