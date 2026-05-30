#[derive(Debug, Clone, Copy, PartialEq)]
enum Effect {
    None,
    ArmorBuff,
    Poison,
    Recharge,
}

impl Effect {
    fn apply_effect(&self, player: &mut Character, boss: &mut Character) {
        match self {
            Effect::None => {}
            Effect::ArmorBuff => {}
            Effect::Poison => {}
            Effect::Recharge => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn get_mana_cost(&self) -> u16 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn get_instant_damage(&self) -> u16 {
        match self {
            Spell::MagicMissile => 4,
            Spell::Drain => 2,
            Spell::Shield => 0,
            Spell::Poison => 0,
            Spell::Recharge => 0,
        }
    }

    fn get_instant_heal(&self) -> u16 {
        match self {
            Spell::MagicMissile => 0,
            Spell::Drain => 2,
            Spell::Shield => 0,
            Spell::Poison => 0,
            Spell::Recharge => 0,
        }
    }

    fn get_effect(&self) -> Effect {
        match self {
            Spell::MagicMissile => Effect::None,
            Spell::Drain => Effect::None,
            Spell::Shield => Effect::ArmorBuff,
            Spell::Poison => Effect::Poison,
            Spell::Recharge => Effect::Recharge,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Character {
    Boss {
        health: u16,
        damage: u16,
    },
    Player {
        health: u16,
        defence: u16,
        mana: u16,
    },
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let lines: Vec<&str> = input.split("\n").collect();

    let player = Character::Player {
        health: 50,
        defence: 0,
        mana: 500,
    };

    let boss = Character::Boss {
        health: u16::from_str_radix(lines[0].split(": ").last().unwrap(), 10)
            .expect("Unable to extract health"),
        damage: u16::from_str_radix(lines[1].split(": ").last().unwrap(), 10)
            .expect("Unable to extract damage"),
    };
}
