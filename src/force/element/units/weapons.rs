
    #[derive(Debug, Clone)]
    pub struct Weapon {
        pub name: String,
        pub pen: i32,
        pub damage: i32,
        pub rof: i32
    }

    pub fn sword() -> Weapon {
        return Weapon {
            name: String::from("sword"),
            pen: 1,
            damage: 2,
            rof: 1
        };
    }

    pub fn spear() -> Weapon {
        return Weapon {
            name: String::from("spear"),
            pen: 2,
            damage: 1,
            rof: 1,
        };
    }

    pub fn dagger() -> Weapon {
        return Weapon {
            name: String::from("dagger"),
            pen: 1,
            damage: 1,
            rof: 1
        }
    }

    pub fn horse_impact() -> Weapon {
        return Weapon {
            name: String::from("horse impact"),
            pen: 2,
            damage: 2,
            rof: 1
        }
    }

    pub fn cavalry_lance() -> Weapon {
        return Weapon {
            name: String::from("cavalry lance"),
            pen: 4,
            damage: 2,
            rof: 1
        }
    }