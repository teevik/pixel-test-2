use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use bevy_inspector_egui::egui::{Align2, Ui};

fn sized_text(ui: &mut Ui, text: impl Into<String>, size: f32) {
    ui.label(egui::RichText::new(text).size(size));
}

/// System to generate user interface with egui
pub fn fps_ui(mut egui_context: ResMut<EguiContext>, diagnostics: Res<Diagnostics>) {
    let ctx = egui_context.ctx_mut();

    egui::Area::new("fps")
        .anchor(Align2::RIGHT_TOP, [-16., 4.])
        .show(ctx, |ui| {
            let size = 15.0;
            if let Some(diag) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(avg) = diag.smoothed() {
                    sized_text(ui, format!("FPS: {:.0}", avg), size);
                }
            }
        });
}
