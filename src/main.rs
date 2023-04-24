#![feature(slice_flatten)]
#![feature(exclusive_wrapper)]
#![feature(generic_const_exprs)]
#![feature(inherent_associated_types)]
#![feature(is_some_and)]

mod color;
mod game;

use crate::game::plugin::GamePlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_plugin(FrameTimeDiagnosticsPlugin);

    app.add_plugin(GamePlugin);

    app.insert_resource(ReportExecutionOrderAmbiguities);
    app.insert_resource(ClearColor(Color::rgb_u8(234, 231, 217)));

    app.run();
}
