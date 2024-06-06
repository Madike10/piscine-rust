use boss::Boss;
use member::Member;

use crate::member::Role;

pub mod boss;
pub mod member;

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    //recruit: an associated function which adds a Member to the members vector.
    // It should accept a name, and an age. The member's role should be set to Associate
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members.push(Member::new(name, Role::Associate, age))
    }
    pub fn attack(&mut self, other_mob: &mut Mob) {
        fn get_power_score(mob: &Mob) -> u32 {
            mob.members.iter().map(|member| match member.role {
                Role::Underboss => 4,
                Role::Caporegime => 3,
                Role::Soldier => 2,
                Role::Associate => 1,
            }).sum()
        }
        let self_score = get_power_score(self);
        let other_score = get_power_score(other_mob);
        if self_score > other_score {
            other_mob.members.pop();
            if other_mob.members.is_empty() {
                self.cities.append(&mut other_mob.cities);
                self.wealth += other_mob.wealth;
                other_mob.wealth = 0;
            }
        } else if self_score <= other_score {
            self.members.pop();
        }
    }
    //steal: an associated function which receives a Mob to target, and an u32 value to steal.
    // The 'self' mob steals the value from the wealth of the target mob, and adds the value to its own wealth.
    // Only as much money as the target mob has can be stolen.
    pub fn steal(&mut self, target: &mut Mob, amount: u32) {
        self.wealth += amount.min(target.wealth);
        target.wealth -= amount.min(target.wealth);
    }
    //conquer_city: an associated function which receives a vector of Mob, a city name and an u8 value.
    // The city name and u8 value are added to its list of cities if none of the other mobs in the vector have a city with the same name.
    pub fn conquer_city(&mut self, mobs: Vec<Mob>, city_name: String, value: u8) {
        for mob in mobs {
            let has_city = mob.cities.iter().any(|(name, _)| name == &city_name);
            if !has_city {
                self.cities.push((city_name, value));
                break;
            }
        }
    }
}
