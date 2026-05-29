use core::fmt::{Display, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Character {
    health: u16,
    defence: u16,
    damage: u16,
}

impl Character {
    fn do_damage(&self, other: &mut Self) {
        let before = other.health;
        other.health = match self.damage.saturating_sub(other.defence) {
            1.. => other
                .health
                .saturating_sub(self.damage.saturating_sub(other.defence)),
            _ => other.health - 1,
        };
        println!("did {} damage", before - other.health);
    }

    fn equip_item(&mut self, item: &Item) {
        match item {
            Item::Weapon { cost: _, damage } => self.damage += damage,
            Item::Armor { cost: _, armor } => self.defence += armor,
            Item::DamageRing { cost: _, damage } => self.damage += damage,
            Item::DefenceRing { cost: _, armor } => self.defence += armor,
        }
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Item {
    Weapon { cost: u16, damage: u16 },
    Armor { cost: u16, armor: u16 },
    DamageRing { cost: u16, damage: u16 },
    DefenceRing { cost: u16, armor: u16 },
}

impl Item {
    fn new_weapon(cost: u16, damage: u16) -> Item {
        Item::Weapon { cost, damage }
    }

    fn new_armor(cost: u16, armor: u16) -> Item {
        Item::Armor { cost, armor }
    }

    fn new_damage_ring(cost: u16, damage: u16) -> Item {
        Item::DamageRing { cost, damage }
    }

    fn new_defence_ring(cost: u16, armor: u16) -> Item {
        Item::DefenceRing { cost, armor }
    }

    fn get_cost(&self) -> u16 {
        match self {
            Item::Weapon { cost, damage: _ } => *cost,
            Item::Armor { cost, armor: _ } => *cost,
            Item::DamageRing { cost, damage: _ } => *cost,
            Item::DefenceRing { cost, armor: _ } => *cost,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Winner {
    A,
    B,
}

impl Display for Winner {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Winner::A => f.write_char('A'),
            Winner::B => f.write_char('B'),
        }
    }
}

/// a always takes the first turn
fn determine_winner(mut a: Character, mut b: Character) -> Winner {
    loop {
        println!("starting round...");

        if !a.is_alive() {
            println!("B won!");
            return Winner::B;
        }

        println!("A's turn to attack\na: {:?}\nb: {:?}", a, b);
        a.do_damage(&mut b);
        println!("b: {:?}", b);
        if !b.is_alive() {
            println!("A won!");
            return Winner::A;
        }

        println!("B's turn to attack\na: {:?}\nb: {:?}", a, b);
        b.do_damage(&mut a);
        println!("a: {:?}", a);

        println!("round over!");
    }
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let lines: Vec<&str> = input.split("\n").collect();

    let player = Character {
        health: 100,
        defence: 0,
        damage: 0,
    };

    let boss = Character {
        health: u16::from_str_radix(lines[0].split(": ").last().unwrap(), 10)
            .expect("Unable to extract health"),
        damage: u16::from_str_radix(lines[1].split(": ").last().unwrap(), 10)
            .expect("Unable to extract damage"),
        defence: u16::from_str_radix(lines[2].split(": ").last().unwrap(), 10)
            .expect("Unable to extract defence"),
    };

    let weapons = [
        Item::new_weapon(8, 4),
        Item::new_weapon(10, 5),
        Item::new_weapon(25, 6),
        Item::new_weapon(40, 7),
        Item::new_weapon(74, 8),
    ];

    let armors = [
        Some(Item::new_armor(13, 1)),
        Some(Item::new_armor(31, 2)),
        Some(Item::new_armor(53, 3)),
        Some(Item::new_armor(75, 4)),
        Some(Item::new_armor(102, 5)),
        None,
    ];

    let rings = [
        Some(Item::new_damage_ring(25, 1)),
        Some(Item::new_damage_ring(50, 2)),
        Some(Item::new_damage_ring(100, 3)),
        Some(Item::new_defence_ring(20, 1)),
        Some(Item::new_defence_ring(40, 2)),
        Some(Item::new_defence_ring(80, 3)),
        None,
    ];

    println!("player: {:?}", player);
    println!("boss: {:?}", boss);

    let mut cheepest_win = 65535;

    for weapon in weapons {
        for armor in armors {
            for ring_a in rings {
                for ring_b in rings {
                    if ring_a.is_none() && ring_b.is_none() || ring_a != ring_b {
                        println!(
                            "checking combo {:?} {:?} {:?} {:?}...",
                            weapon, armor, ring_a, ring_b
                        );

                        let mut player = player.clone();
                        let boss = boss.clone();

                        player.equip_item(&weapon);

                        if let Some(armor) = armor {
                            player.equip_item(&armor);
                        }
                        if let Some(ring_a) = ring_a {
                            player.equip_item(&ring_a);
                        }
                        if let Some(ring_b) = ring_b {
                            player.equip_item(&ring_b);
                        }

                        let winner = determine_winner(player, boss);

                        if winner == Winner::A {
                            println!("Player won");
                            let mut cost = weapon.get_cost();
                            if let Some(armor) = armor {
                                cost += armor.get_cost();
                            }

                            if let Some(ring_a) = ring_a {
                                cost += ring_a.get_cost();
                            }

                            if let Some(ring_b) = ring_b {
                                cost += ring_b.get_cost();
                            }
                            if cost < cheepest_win {
                                cheepest_win = cost;
                            }
                        } else {
                            println!("Boss won");
                        }
                    }
                }
            }
        }
    }

    println!("cheepest win cost {} coins", cheepest_win);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_do_damage() {
        let mut player = Character {
            health: 100,
            damage: 8,
            defence: 300,
        };

        let mut boss = Character {
            health: 100,
            damage: 8,
            defence: 3,
        };

        player.do_damage(&mut boss);

        assert_eq!(boss.health, 95);

        boss.do_damage(&mut player);

        assert_eq!(player.health, 99);
    }

    #[test]
    fn test_determine_winner() {
        let player = Character {
            health: 8,
            damage: 5,
            defence: 5,
        };

        let boss = Character {
            health: 12,
            damage: 7,
            defence: 2,
        };

        assert_eq!(determine_winner(player, boss), Winner::A);
    }

    #[test]
    fn test_do_damage() {
        let mut player = Character {
            health: 8,
            damage: 5,
            defence: 5,
        };

        let mut boss = Character {
            health: 12,
            damage: 7,
            defence: 2,
        };

        player.do_damage(&mut boss);
        assert_eq!(
            boss,
            Character {
                health: 9,
                damage: 7,
                defence: 2,
            },
        );

        boss.do_damage(&mut player);
        assert_eq!(
            player,
            Character {
                health: 6,
                damage: 5,
                defence: 5,
            }
        )
    }
}
