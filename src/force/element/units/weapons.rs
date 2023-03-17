
    #[derive(Debug, Clone)]
    pub struct Weapon {
        pub name: String,
        pub pen: i32,
        pub damage: i32,
        pub rof: i32,
        pub range: i32,
    }

    pub fn sword() -> Weapon {
        return Weapon {
            name: String::from("sword"),
            pen: 1,
            damage: 2,
            rof: 1,
            range: 1
        };
    }

    pub fn spear() -> Weapon {
        return Weapon {
            name: String::from("spear"),
            pen: 2,
            damage: 1,
            rof: 1,
            range: 1
        };
    }

    pub fn dagger() -> Weapon {
        return Weapon {
            name: String::from("dagger"),
            pen: 1,
            damage: 1,
            rof: 1,
            range: 1
        }
    }

    pub fn horse_impact() -> Weapon {
        return Weapon {
            name: String::from("horse impact"),
            pen: 2,
            damage: 2,
            rof: 1,
            range: 1
        }
    }

    pub fn cavalry_lance() -> Weapon {
        return Weapon {
            name: String::from("cavalry lance"),
            pen: 4,
            damage: 2,
            rof: 1,
            range: 1
        }
    }

    pub fn short_bow() -> Weapon {
        return Weapon {
            name: String::from("short bow"),
            pen: 2,
            damage: 2,
            rof: 2,
            range: 2
        }
    }

    pub fn long_bow() -> Weapon {
        return Weapon {
            name: String::from("long bow"),
            pen: 3,
            damage: 2,
            rof: 1,
            range: 3
        }
    }