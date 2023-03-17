use std::collections::HashMap;
use crate::Force;
use crate::force::force_combat::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CombatLog {
    pub attacker_force_name: String,
    pub attacker_unit_name: String,
    pub weapon_name: String,
    pub target_force_name: String,
    pub target_unit_name: String,
    pub verb: String,
}

fn generate_log(log: &CombatLog) -> String {
    return format!(
        "{} {} {} {} {} with {}", 
        log.attacker_force_name, 
        log.attacker_unit_name,
        log.verb,
        log.target_force_name,
        log.target_unit_name,
        log.weapon_name
    )
}

fn print_log_for_verb(map: &HashMap<CombatLog, i32>, verb: String) {
    for (log, count) in map {
        if log.verb == verb {
            println!("{:?}x {}", count, generate_log(log));
        }
    }
    println!("--------------------");
}

fn group_and_print_logs(logs: Vec<CombatLog>) {
    let mut freq: HashMap<CombatLog, i32> = HashMap::new();
    for log in logs {
        let current_count = freq.get(&log);
        if current_count.is_none() {
            freq.insert(log, 1);
        } else {
            freq.insert(log, current_count.unwrap() + 1);
        }
    }
    // print_log_for_verb(&freq, String::from(MISSED));
    print_log_for_verb(&freq, String::from(HIT));
    print_log_for_verb(&freq, String::from(PENNED));
    print_log_for_verb(&freq, String::from(KILLED));
}

pub fn print_combat_log(logs: &Vec<CombatLog>, force_name: String) {
    let mut events_a = logs.clone();
    events_a.retain(|l| l.attacker_force_name == force_name );
    group_and_print_logs(events_a);
}

pub fn print_combatants(force: &Force){
    println!("{}: ", force.name);
    for element in force.forces.clone() {
        println!("{}x {}", element.count, element.unit_type.name)
    }
    println!("");
}