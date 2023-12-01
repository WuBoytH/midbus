use {
    smash::lua2cpp::L2CFighterCommon,
    once_cell::sync::Lazy,
    super::MIDBUS_SLOTS
};

unsafe extern "C" fn on_start(_fighter: &mut L2CFighterCommon) {
    Lazy::force(&MIDBUS_SLOTS);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}