use crate::game::constants::PIXEL_SIMULATION_TIMESTEP;
use crate::game::systems::fps_ui::fps_ui;
use crate::game::systems::setup::setup;
use bevy::prelude::*;
use bevy::time::FixedTimestep;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);

        app.add_system(fps_ui);

        // app.add_system(update_pixel_simulation.label(SystemLabels::UpdatePixelSimulation));
        // app.add_system(
        //     simulate_pixel_simulation
        //         .label(SystemLabels::RenderPixelSimulation)
        //         .with_run_criteria(FixedTimestep::step(PIXEL_SIMULATION_TIMESTEP as f64)),
        // );
        //
        // app.add_system(
        //     render_pixel_simulation
        //         .label(SystemLabels::SimulatePixelSimulation)
        //         .after(SystemLabels::UpdatePixelSimulation),
        // );
    }
}
