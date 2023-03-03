
    #[derive(Debug)]
    pub struct Weapon {
        pub pen: i32,
        pub damage: i32,
        pub rof: i32
    }

    pub fn sword() -> Weapon {
        return Weapon {
            pen: 1,
            damage: 2,
            rof: 1
        };
    }

    pub fn spear() -> Weapon {
        return Weapon {
            pen: 2,
            damage: 1,
            rof: 1,
        };
    }