use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::lua_const::*
    },
    smashline::*,
    once_cell::sync::Lazy,
    super::MIDBUS_SLOTS
};

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_KOOPA {
            return;
        }
        Lazy::force(&MIDBUS_SLOTS);
    }
}

pub fn install() {
    install_agent_resets!(
        agent_reset
    );
}
