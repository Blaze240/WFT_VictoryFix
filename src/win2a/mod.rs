use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

unsafe extern "C" fn reflet_sound_win2a(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 47.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win02"));
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win2_01"));
        }
        frame(agent.lua_state_agent, 123.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_smash_s01_win02"));
        }
    } else {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win02"));
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win2_01"));
        }
        frame(agent.lua_state_agent, 123.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_smash_s01_win02"));
        }
    }
}
pub fn install() {
    Agent::new("reflet")
        .sound_acmd("sound_win2a", reflet_sound_win2a)
        .install();
}
