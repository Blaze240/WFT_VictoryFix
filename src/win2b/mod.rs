use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};
unsafe extern "C" fn wiifit_sound_win2b(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win05"));
        }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wiifit_attackair_h01"));
    }
    frame(agent.lua_state_agent, 119.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
    }
}else {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win03"));
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_attackair_h01"));
        }
        frame(agent.lua_state_agent, 119.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

pub fn install() {
    Agent::new("wiifit")
        .sound_acmd("sound_win2b", wiifit_sound_win2b)
        .install();
}
