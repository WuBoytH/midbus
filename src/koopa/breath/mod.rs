use super::MIDBUS_SLOTS;

mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("koopa_breath");
    acmd::install(agent);
    agent.install();
}
