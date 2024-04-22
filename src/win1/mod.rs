use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "master", script = "sound_win1", category = ACMD_SOUND )]
unsafe fn master_sound_win1(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 1 {
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_01"));
        }
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_master_win01"));
        }
    else {
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_02"));
        }
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_01"));
        }
        frame(agent.lua_state_agent, 41.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_master_win01"));
        }
        frame(agent.lua_state_agent, 64.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_02"));
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_03"));
        }

                      }
                     }

pub fn install() {
    smashline::install_acmd_scripts!(
     master_sound_win1
    );
}
