
pub mod Weapons {
    #[derive(Debug, Clone)]
    pub struct Weapon {
        pub name: String,
        pub pen: i32,
        pub damage: i32,
        pub rof: i32,
        pub range: i32,
        pub supply_usage: i32,
    }

    // pen/kill chances = (pen||dmg)/(armour/hp * 2);
    // 1/1 = 50%
    // 1/2 = 25%
    // 2/1 = 100%

    struct DamageProfile {
        pub pen: i32,
        pub damage: i32,
        pub range: i32,
    }

    static PROFILE_556: DamageProfile = DamageProfile {
        pen: 5,
        damage: 3,
        range: 5
    };

    static PROFILE_762: DamageProfile = DamageProfile {
        pen: 8,
        damage: 4,
        range: 6
    };

    static PROFILE_50_C: DamageProfile = DamageProfile {
        pen: 16,
        damage: 5,
        range: 7
    };

    static PROFILE_LAT: DamageProfile = DamageProfile {
        pen: 50,
        damage: 35,
        range: 5,
    };


    // ROFs
    static HUMAN_SINGLE_SHOT_ROF: i32 = 1;
    static HUMAN_SEMI_AUTO_ROF: i32 = 2;
    static HUMAN_FULL_AUTO_ROF: i32 = 6;


    pub fn sword() -> Weapon {
        return Weapon {
            name: String::from("sword"),
            pen: 1,
            damage: 2,
            rof: 1,
            range: 1,
            supply_usage: 1
        };
    }

    pub fn spear() -> Weapon {
        return Weapon {
            name: String::from("spear"),
            pen: 2,
            damage: 1,
            rof: 1,
            range: 1,
            supply_usage: 1
        };
    }

    pub fn dagger() -> Weapon {
        return Weapon {
            name: String::from("dagger"),
            pen: 1,
            damage: 1,
            rof: 1,
            range: 1,
            supply_usage: 1
        }
    }

    pub fn horse_impact() -> Weapon {
        return Weapon {
            name: String::from("horse impact"),
            pen: 2,
            damage: 2,
            rof: 1,
            range: 1,
            supply_usage: 1
        }
    }

    pub fn cavalry_lance() -> Weapon {
        return Weapon {
            name: String::from("cavalry lance"),
            pen: 4,
            damage: 2,
            rof: 1,
            range: 1,
            supply_usage: 1
        }
    }

    pub fn short_bow() -> Weapon {
        return Weapon {
            name: String::from("short bow"),
            pen: 2,
            damage: 2,
            rof: 2,
            range: 2,
            supply_usage: 2
        }
    }

    pub fn long_bow() -> Weapon {
        return Weapon {
            name: String::from("long bow"),
            pen: 3,
            damage: 2,
            rof: 1,
            range: 3,
            supply_usage: 2
        }
    }

    // Modern Weapons

    pub fn rifle() -> Weapon {
        return Weapon {
            name: String::from("PW"),
            pen: PROFILE_556.pen,
            damage: PROFILE_556.damage,
            range: PROFILE_556.range,
            rof: HUMAN_SEMI_AUTO_ROF,
            supply_usage: 2
        }
    }

    pub fn automatic_rifle() -> Weapon {
        return Weapon {
            name: String::from("APW"),
            pen: PROFILE_556.pen,
            damage: PROFILE_556.damage,
            range: PROFILE_556.range,
            rof: HUMAN_FULL_AUTO_ROF,
            supply_usage: 12
        }
    }

    pub fn machine_gun() -> Weapon {
        return Weapon {
            name: String::from("MG"),
            pen: PROFILE_762.pen,
            damage: PROFILE_762.damage,
            range: PROFILE_762.range,
            rof: HUMAN_FULL_AUTO_ROF,
            supply_usage: 15
        }
    }

    pub fn heavy_machine_gun() -> Weapon {
        return Weapon {
            name: String::from("HMG"),
            pen: PROFILE_50_C.pen,
            damage: PROFILE_50_C.damage,
            range: PROFILE_50_C.range,
            rof: HUMAN_SEMI_AUTO_ROF,
            supply_usage: 18
        }
    }

    pub fn light_atgm() -> Weapon {
        return Weapon {
            name: String::from("LAT"),
            pen: PROFILE_LAT.pen,
            damage: PROFILE_LAT.damage,
            range: PROFILE_LAT.range,
            rof: HUMAN_SINGLE_SHOT_ROF,
            supply_usage: 12
        }
    }
}



// 5/25 = 5/50 = 10%
// 8/30 = 8/60 = 