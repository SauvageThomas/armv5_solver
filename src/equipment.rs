#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: &'static str,
    pub init: i8,
    pub attack: u8,
    pub defence: u8,
    pub damage: u8,
}

pub const GREAT_SWORD: Weapon = Weapon {
    name: "Great Sword",
    init: 2,
    attack: 5,
    defence: 2,
    damage: 9,
};

pub const POLE_AXE: Weapon = Weapon {
    name: "Pole Axe",
    init: 1,
    attack: 5,
    defence: 0,
    damage: 11,
};

pub const POLE_ARM: Weapon = Weapon {
    name: "Pole Arm",
    init: 3,
    attack: 4,
    defence: 1,
    damage: 8,
};


#[derive(Debug, Clone)]
pub struct Armor {
    pub name: &'static str,
    pub protection: i8,
}

pub const FULL_CHAIN_MAIL: Armor = Armor {
    name: "Full Mail",
    protection: 9,
};

pub const CUSTOM: Armor = Armor {
    name: "Full Mail",
    protection: 6,
};

pub const PLATE_FULL_MAIL_CAMAIL_LVL_1: Armor = Armor {
    name: "Plate and Full Mail +1 With Camail +1",
    protection: 15,
};

pub const PLATE_FULL_MAIL_LVL_1: Armor = Armor {
    name: "Plate and Full Mail +1",
    protection: 13,
};