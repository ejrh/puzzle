use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::Color;
use bevy::ecs::{
    error::Result,
    query::{With},
    reflect::ReflectResource,
    resource::Resource,
    schedule::{IntoScheduleConfigs, SystemCondition},
    system::Query,
    world::World,
    system::{Res, ResMut},
};
use bevy::gizmos::gizmos::Gizmos;
use bevy::input::{ButtonInput, keyboard::KeyCode};
use bevy::light::{PointLight, SpotLight};
use bevy::math::Vec3;
use bevy::reflect::Reflect;
use bevy::state::{
    app::AppExtStates,
    condition::in_state,
    state::{
        States, NextState, State
    }
};
use bevy::transform::components::GlobalTransform;
use bevy::ui::Node;
use bevy_egui::{EguiContext, EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
enum DebugState {
    #[default]
    Off,
    On,
}

impl DebugState {
    fn inverse(&self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

#[derive(Reflect, Resource)]
#[reflect(Resource)]
struct DebugOptions {
    world_stats: bool,
    world_inspector: bool,
    show_lights: bool,
}

impl Default for DebugOptions {
    fn default() -> Self {
        Self {
            world_stats: true,
            world_inspector: true,
            show_lights: false,
        }
    }
}

#[macro_export]
macro_rules! debug_option {
    ($name: ident) => (in_state(DebugState::On).and(|options: Res<DebugOptions>| options.$name));
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EguiPlugin::default())
            .init_state::<DebugState>()
            .init_resource::<DebugOptions>()
            .add_systems(Startup, setup_debug)
            .add_systems(Update, toggle_debug)
            .add_systems(EguiPrimaryContextPass, debug_options_ui.run_if(in_state(DebugState::On)));

        app
            .add_systems(EguiPrimaryContextPass, world_stats.run_if(debug_option!(world_stats)))
            .add_plugins(WorldInspectorPlugin::new().run_if(debug_option!(world_inspector)))
            .add_systems(Update, show_lights.run_if(debug_option!(show_lights)));
    }
}

fn setup_debug(
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    egui_global_settings.enable_absorb_bevy_input_system = true;
}

fn toggle_debug(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<DebugState>>,
    mut next_state: ResMut<NextState<DebugState>>,
) {
    if keys.just_pressed(KeyCode::F5) {
        next_state.set(current_state.inverse());
    }
}

fn debug_options_ui(
    mut egui_contexts: EguiContexts,
    mut options: ResMut<DebugOptions>,
) -> Result {
    const DEFAULT_POS: (f32, f32) = (720., 16.);

    egui::Window::new("Debug Options")
        .default_pos(DEFAULT_POS)
        .show(egui_contexts.ctx_mut()?, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.checkbox(&mut options.world_stats, "World Stats");
                ui.checkbox(&mut options.world_inspector, "World Inspector");
                ui.heading("Gizmos");
                ui.checkbox(&mut options.show_lights, "Show Lights");
                ui.heading("Logging");
            });
        });
    Ok(())
}

fn world_stats(
    world: &mut World,
) {
    const DEFAULT_POS: (f32, f32) = (380., 16.);

    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("World Stats")
        .default_pos(DEFAULT_POS)
        .show(egui_context.get_mut(), |ui| {
            fn row(ui: &mut egui::Ui, name: &str, value: usize) {
                ui.label(name);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{}", value));
                });
                ui.end_row()
            }

            ui.horizontal_top(|ui| {
                egui::Grid::new("stats").show(ui, |ui| {
                    ui.heading("ECS");
                    ui.end_row();
                    row(ui, "Entities", world.entities().len() as usize);
                    row(ui, "Components", world.components().len());
                    row(ui, "Archetypes", world.archetypes().len());
                    row(ui, "UI Nodes", world.query::<&Node>().iter(world).count());
                });
            })
        });
}

fn show_lights(
    spot_lights: Query<(&SpotLight, &GlobalTransform)>,
    point_lights: Query<(&PointLight, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (_light, transform) in spot_lights.iter() {
        let from_point = transform.translation();
        let to_point = transform.transform_point(Vec3::new(0.0, 0.0, -2.0));
        gizmos.sphere(from_point, 1.0, Color::srgb(1.0, 1.0, 0.0));
        gizmos.arrow(from_point, to_point, Color::srgb(1.0, 1.0, 0.0));
    }

    for (_light, transform) in point_lights.iter() {
        let point = transform.translation();
        gizmos.sphere(point, 1.0, Color::srgb(1.0, 1.0, 0.0));
    }
}
