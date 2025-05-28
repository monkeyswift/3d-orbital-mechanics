use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, reply};
use clap::Parser;

//1. Grid - using a floor grid, might add a grid that centers on
//a celestial body instead, will decide later.
//2. console commands

// to add time-indexing, playing a system backwards, changing evolution
// speed, etc, I'm going to make console commands. I may add a full
// GUI later on if time permits. 

// There will be an entity that tracks time, holds the number of bodies
// being simulated(for faster calculations), and uses this time to index
// the ephemeris and display what's relevant. 

#[derive(Parser, ConsoleCommand)]
#[command(name = "set_time")]
pub struct SetTime { pub t: f64 }

#[derive(Parser, ConsoleCommand)]
#[command(name = "forward_time")]
pub struct ForwardTime;

#[derive(Parser, ConsoleCommand)]
#[command(name = "backward_time")]
pub struct BackwardTime;

#[derive(Parser, ConsoleCommand)]
#[command(name = "sim_speed")]
pub struct SimSpeed { pub speed: f32 }


fn set_time_cmd() {

}

fn forward_t_cmd() {

}

fn backward_t_cmd() {

}

fn sim_speed_cmd() {

}

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_console_command::<SetTime, _>(set_time_cmd)
            .add_console_command::<ForwardTime, _>(forward_t_cmd)
            .add_console_command::<BackwardTime, _>(backward_t_cmd)
            .add_console_command::<SimSpeed, _>(sim_speed_cmd);
    }
}