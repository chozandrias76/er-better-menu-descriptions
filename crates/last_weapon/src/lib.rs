pub mod retour;

use serde::{Deserialize, Serialize};
use std::{
    fmt::Display, fs::File, io::Read, path::Path, sync::atomic::AtomicUsize,
};

pub static LAST_WEAP_PTR: AtomicUsize = AtomicUsize::new(0);

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponData {
    pub disable_param_nt: u8,
    pub disable_param_reserve: u8,
    pub disable_param_reserve2: [u8; 2], // Padding
    pub behavior_variation_id: i32,
    pub sort_id: i32,
    pub wandering_equip_id: i32,
    pub weight: f32,
    pub weapon_weight_rate: f32,
    pub fix_price: i32,
    pub reinforce_price: i32,
    pub sell_value: i32,
    pub correct_strength: f32,
    pub correct_agility: f32,
    pub correct_magic: f32,
    pub correct_faith: f32,
    pub phys_guard_cut_rate: f32,
    pub mag_guard_cut_rate: f32,
    pub fire_guard_cut_rate: f32,
    pub thun_guard_cut_rate: f32,
    pub sp_effect_behavior_id: i32,
    pub sp_effect_behavior_id1: i32,
    pub sp_effect_behavior_id2: i32,
    pub resident_sp_effect_id: i32,
    pub resident_sp_effect_id1: i32,
    pub resident_sp_effect_id2: i32,
    pub material_set_id: i32,
    pub origin_equip_wep: [i32; 16],
    pub weak_a_damage_rate: f32,
    pub weak_b_damage_rate: f32,
    pub weak_c_damage_rate: f32,
    pub weak_d_damage_rate: f32,
    pub sleep_guard_resist_max_correct: f32,
    pub madness_guard_resist_max_correct: f32,
    pub sa_weapon_damage: f32,
    pub equip_model_id: u16,
    pub icon_id: u16,
    pub durability: u16,
    pub durability_max: u16,
    pub attack_throw_escape: u16,
    pub parry_damage_life: u16,
    pub attack_base_physics: u16,
    pub attack_base_magic: u16,
    pub attack_base_fire: u16,
    pub attack_base_thunder: u16,
    pub attack_base_stamina: u16,
    pub sa_durability: u16,
    pub guard_angle: u16,
    pub stamina_guard_def: u16,
    pub reinforce_type_id: u16,
    pub trophy_s_grade_id: u16,
    pub trophy_seq_id: u16,
    pub throw_atk_rate: u16,
    pub bow_dist_rate: u16,
    pub equip_model_category: u8,
    pub equip_model_gender: u8,
    pub weapon_category: u8,
    pub wep_motion_category: u8,
    pub guard_motion_category: u8,
    pub atk_material: u8,
    pub def_material: u8,
    pub def_sfx_material: u8,
    pub correct_type_physics: u8,
    pub sp_attribute: u8,
    pub sp_atk_category: u16,
    pub wep_motion_one_hand_id: u8,
    pub wep_motion_both_hand_id: u8,
    pub proper_strength: u8,
    pub proper_agility: u8,
    pub proper_magic: u8,
    pub proper_faith: u8,
    pub over_strength: u8,
    pub attack_base_parry: u8,
    pub defense_base_parry: u8,
    pub guard_base_repel: u8,
    pub attack_base_repel: u8,
    pub guard_cut_cancel_rate: u8,
    pub guard_level: u8,
    pub slash_guard_cut_rate: u8,
    pub blow_guard_cut_rate: u8,
    pub thrust_guard_cut_rate: u8,
    pub poison_guard_resist: u8,
    pub disease_guard_resist: u8,
    pub blood_guard_resist: u8,
    pub curse_guard_resist: u8,
}

impl Display for WeaponData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl WeaponData {
    pub fn name(&self) -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let path = path.parent().unwrap().parent().unwrap();
        let path = path.join("resources\\item_ids.txt");
        let file = File::open(&path);

        if let Ok(mut file) = file {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read item_ids.txt");
            let entries = contents.lines();
            for entry in entries {
                let parts: Vec<&str> = entry.split(':').collect();
                if parts.len() > 1
                    && parts[0].parse::<i32>().unwrap_or(0)
                        == self.origin_equip_wep[0]
                {
                    return parts[1].to_string().trim().to_string();
                }
            }
        }
        "".to_string()
    }
}
