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

unsafe extern "C" fn reflet_sound_win3a(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win03"));
        }
        frame(agent.lua_state_agent, 97.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win1"));
        }
        frame(agent.lua_state_agent, 117.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win3_01"));
        }
        frame(agent.lua_state_agent, 120.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_smash_s01_win03"));
        }
    } else {
        frame(agent.lua_state_agent, 80.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win03"));
        }
        frame(agent.lua_state_agent, 97.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win1"));
        }
        frame(agent.lua_state_agent, 117.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win3_01"));
        }
        frame(agent.lua_state_agent, 120.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_smash_s01_win03"));
        }
    }
}
pub fn install() {
    Agent::new("reflet")
        .sound_acmd("sound_win3a", reflet_sound_win3a)
        .install();
}
