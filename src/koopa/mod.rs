use {
    std::path::Path,
    smash::{
        lua2cpp::*,
        phx::{Hash40, Vector3f},
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    once_cell::sync::Lazy
};

pub static MIDBUS_SLOTS: Lazy<Vec<bool>> = Lazy::new(|| {
    let base_path = Path::new("mods:/fighter/koopa/model/body");
    let mut vec = Vec::new();
    let mut counter = 0;
    loop {
        let model_path = base_path.join(&format!("c{:02}", counter));
            vec.push(model_path.join("midbus.marker").exists());
        if counter == 256 {
            break;
        }
        counter += 1;
    }
    vec
});

#[acmd_script( agent = "koopa_breath", script = "game_move", category = ACMD_GAME )]
unsafe fn koopa_breath_move(weapon: &mut L2CAgentBase) {
    Lazy::force(&MIDBUS_SLOTS);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_color = WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize;
    if sv_battle_object::category(owner_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && sv_battle_object::kind(owner_id) == *FIGHTER_KIND_KOOPA
    && MIDBUS_SLOTS[owner_color] {
        if macros::is_excute(weapon) {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 1.8, 55, 30, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(weapon.module_accessor);
        }
        frame(weapon.lua_state_agent, 7.0);
        if macros::is_excute(weapon) {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 1.8, 55, 30, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        }
    }
    else {
        if macros::is_excute(weapon) {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 1.8, 55, 30, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(weapon.module_accessor);
        }
        frame(weapon.lua_state_agent, 7.0);
        if macros::is_excute(weapon) {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 1.8, 55, 30, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        koopa_breath_move
    );
}
