
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use crate::cow_tils::CowTag;
use crate::cow_nstants::CowNstants;

pub struct CowUiPlugin;

impl Plugin for CowUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(update);
    }
}

fn setup(
    mut egui_context: ResMut<EguiContext>,
    window : Res<Windows>,
){
    let window_id = window.get_primary().unwrap().id();
    egui_context.ctx_for_window_mut(window_id).set_fonts(egui::FontDefinitions::default());
}

fn update(
    mut contexts: ResMut<EguiContext>,
    query: Query<&CowTag>,
    mut cownstants : ResMut<CowNstants>,
){

    let frame = egui::Frame {
        rounding: 0.0.into(),
        outer_margin: 3.5.into(), // so the stroke is within the bounds
        stroke: egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_gray(0),
        },
        inner_margin: 10.0.into(),
        fill: egui::Color32::from_gray(180),
        ..Default::default()
    };

    egui::Window::new("COWindow")
        .frame(frame)
        .default_width(cownstants.ui_size.x)
        .default_height(cownstants.ui_size.y)
        .default_pos(egui::Pos2::new(cownstants.ui_position.x, cownstants.ui_position.y))
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::LEFT_TOP, egui::Vec2::new(0.0, 0.0))
        .show(contexts.ctx_mut(), |ui| {
        
        let total_cows = query.iter().count();

        ui.label(format!("Total Cows: {}", total_cows));

        ui.separator();

        ui.add(egui::Slider::new(&mut cownstants.cow_size, 0.0..=10.0).text("Cow Size"));
        ui.add(egui::Slider::new(&mut cownstants.COWttraction_force, -0.5..=0.5).text("Cow Attract Force"));
        ui.add(egui::Slider::new(&mut cownstants.max_attraction_distance, 100.0..=2000.0).text("Max CowTTraction Distance"));
        ui.add(egui::Slider::new(&mut cownstants.max_repulsion_distance, 0.0..=1000.0).text("Max Repulsion Distance"));
    });
}

pub fn mouse_click_grabbed(pos: Vec2, ui_pos : Vec2, ui_size : Vec2) -> bool {
    let ui_x2 = ui_pos.x + ui_size.x;
    let ui_y2 = ui_pos.y + ui_size.y;

    if pos.x > ui_pos.x && pos.x < ui_x2 && pos.y > ui_pos.y && pos.y < ui_y2 {
        return true;
    }

    return false;
}