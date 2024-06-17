use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

unsafe extern "C" fn kamui_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        // No special fall
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("fall_special") {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
    }
}

pub fn install() {
    Agent::new("kamui")
        .on_line(Main, kamui_frame)
        .install();
}
