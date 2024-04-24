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
unsafe extern "C" fn wiifit_sound_win1e(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wiifit_win08"));
        }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_h01"));
    }
    frame(agent.lua_state_agent, 124.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
    }
}else {
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_wiifit_smash_h01"));
        }
        frame(agent.lua_state_agent, 124.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_wiifit_smash_s01"));
        }
    }
}

pub fn install() {
    Agent::new("wiifit")
        .sound_acmd("sound_win1e", wiifit_sound_win1e)
        .install();
}
