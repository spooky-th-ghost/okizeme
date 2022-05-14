use crate::{
    Hitbox,
    StunValues,
    ComboedState
};
pub struct Combo {
    hit_count: u8,
    valid: bool,
    hitstun_modifier: u8,
    damage_scaling: f32,
    total_damage: u8,
    breakpoints: Vec<u8>
}

impl Combo {
    fn new(hitbox: Hitbox) -> Self {
        Combo {  
            hit_count: 1, 
            valid: true, 
            hitstun_modifier: 0, 
            damage_scaling: hitbox.proration(), 
            total_damage: hitbox.damage(), 
            breakpoints: Vec::new()  
        }
    }

    fn add(&mut self, hitbox: Hitbox, missed_tech: bool) -> u8 {
        if missed_tech {
            self.valid = false;
            self.breakpoints.push(self.hit_count);
        }
        let stun_values = StunValues::from_attack_level(hitbox.level());
        // TODO: Add method here to add to the hitcount and re-calculate scaling
        // possibly reset scaling to some value when a breakpoint occurs
        self.hit_count += 1;
        // call scaled damage here to get the adjusted value, add it to the combos total damage
        // and then return how much damage was dealt
        let adjusted_damage = self.scaled_damage(hitbox.damage());
        self.total_damage += adjusted_damage;
        adjusted_damage
    }

    fn scaled_damage(&self, base_damage: u8) -> u8 {
        (base_damage as f32 * self.damage_scaling) as u8
    }
}
