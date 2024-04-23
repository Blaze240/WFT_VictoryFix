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
unsafe extern "C" fn reflet_sound_win1b(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_reflet_win_lucina"));
        }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_reflet_fire_02_win01"));
    }
    frame(agent.lua_state_agent, 91.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_reflet_special_s02_win01"));
        macros::STOP_SE(agent, Hash40::new("se_reflet_fire_02_win01"));
    }
    frame(agent.lua_state_agent, 125.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_reflet_win1"));
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_03"));
    } 
}else {
        frame(agent.lua_state_agent, 46.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_fire_02_win01"));
        }
        frame(agent.lua_state_agent, 91.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_special_s02_win01"));
            macros::STOP_SE(agent, Hash40::new("se_reflet_fire_02_win01"));
        }
        frame(agent.lua_state_agent, 125.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_win1"));
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_03"));
        }
    }
}

pub fn install() {
    Agent::new("reflet")
        .sound_acmd("sound_win1b", reflet_sound_win1b)
        .install();
}
