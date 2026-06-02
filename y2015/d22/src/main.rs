use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
enum EffectState {
    Inactive,
    Active(u8),
}

impl EffectState {
    fn tick(&mut self) {
        match self {
            EffectState::Inactive => {}
            EffectState::Active(0) => *self = EffectState::Inactive,
            EffectState::Active(i) => *self = EffectState::Active(*i - 1),
        }
    }

    fn is_active(&self) -> bool {
        match self {
            EffectState::Inactive => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Effect {
    None,
    ArmorBuff,
    Poison,
    Recharge,
}

impl Effect {
    fn apply_effect(&self, player: &mut Player, boss: &mut Boss) -> bool {
        match self {
            Effect::None => true,
            Effect::ArmorBuff => {
                if player.shield_state == EffectState::Inactive {
                    player.shield_state = EffectState::Active(6);
                    true
                } else {
                    false
                }
            }
            Effect::Poison => {
                if boss.poison_state == EffectState::Inactive {
                    boss.poison_state = EffectState::Active(6);
                    true
                } else {
                    false
                }
            }
            Effect::Recharge => {
                if player.shield_state == EffectState::Inactive {
                    player.shield_state = EffectState::Active(5);
                    true
                } else {
                    false
                }
            }
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
struct Boss {
    health: u16,
    damage: u16,
    poison_state: EffectState,
}

impl Boss {
    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn do_damage(&mut self, player: &mut Player) {
        player.health = if player.shield_state.is_active() {
            player.health.saturating_sub(self.damage - 7)
        } else {
            player.health.saturating_sub(self.damage)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Player {
    health: u16,
    defence: u16,
    mana: u16,
    shield_state: EffectState,
    recharge_state: EffectState,
}

impl Player {
    fn cast_spell(&mut self, spell: &Spell, boss: &mut Boss) -> bool {
        if self.mana >= spell.get_mana_cost() {
            if spell.get_effect().apply_effect(self, boss) {
                self.health += spell.get_instant_heal();
                boss.health -= spell.get_instant_damage();

                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
enum Winner {
    Player(u16),
    Boss,
}

impl Ord for Winner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Winner::Boss, Winner::Boss) => Ordering::Equal,
            (Winner::Player(_), Winner::Boss) => Ordering::Less,
            (Winner::Boss, Winner::Player(_)) => Ordering::Greater,
            (Winner::Player(a), Winner::Player(b)) => {
                if a == b {
                    Ordering::Equal
                } else if a > b {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    }
}

fn determine_minimum(player: Player, boss: Boss) -> Winner {
    [
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Shield,
        Spell::Poison,
        Spell::Recharge,
    ]
    .into_iter()
    .map(|spell| {
        let mut player = player.clone();
        let mut boss = boss.clone();

        if player.cast_spell(&spell, &mut boss) {
            if boss.is_alive() {
                boss.do_damage(&mut player);
                if player.is_alive() {
                    determine_minimum(player, boss)
                } else {
                    Winner::Boss
                }
            } else {
                Winner::Player(spell.get_mana_cost())
            }
        } else {
            Winner::Boss
        }
    })
    .min()
    .unwrap()
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let lines: Vec<&str> = input.split("\n").collect();

    let player = Player {
        health: 50,
        defence: 0,
        mana: 500,
        shield_state: EffectState::Inactive,
        recharge_state: EffectState::Inactive,
    };

    let boss = Boss {
        health: u16::from_str_radix(lines[0].split(": ").last().unwrap(), 10)
            .expect("Unable to extract health"),
        damage: u16::from_str_radix(lines[1].split(": ").last().unwrap(), 10)
            .expect("Unable to extract damage"),
        poison_state: EffectState::Inactive,
    };

    println!("{:?}", determine_minimum(player, boss))
}
