use std::collections::HashMap;

use bevy::app::{App, Plugin, Update};
use bevy::asset::{AssetApp, AssetEvent, AssetServer, Assets, Handle};
use bevy::log::info;
use bevy::math::{Dir3, Quat};
use bevy::prelude::{on_message, ChildOf, Commands, Component, Entity, IntoScheduleConfigs, MessageReader, Name, Query, Reflect, Res, Transform, Visibility};

use crate::clickable::Clickable;
use crate::item::{Item, Rotating};
use crate::puzzle_def::{ItemDecoration, ItemDef, PartDef, PuzzleDef, PuzzleDefLoader, ZoneDef, ZoneState};
use crate::utils::named_scene::NamedScene;
use crate::zone::{ActiveInZones, CurrentZone, UnzoomTo, Zone, ZoneCamera};

pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<PuzzleDef>();
        app.init_asset_loader::<PuzzleDefLoader>();

        app.add_systems(Update, check_puzzle_loaded.run_if(on_message::<AssetEvent<PuzzleDef>>));
    }
}

#[derive(Component, Reflect)]
#[require(Transform, Visibility)]
pub struct Puzzle(pub Handle<PuzzleDef>);

fn check_puzzle_loaded(
    mut asset_events: MessageReader<AssetEvent<PuzzleDef>>,
    assets: Res<Assets<PuzzleDef>>,
    puzzles: Query<(Entity, &Puzzle)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let loaded_puzzles: Vec<_> = asset_events.read().filter_map(
        |e| if let AssetEvent::LoadedWithDependencies { id } = e { Some(*id) } else { None }
    ).collect();

    for (puzzle_id, puzzle) in puzzles {
        if loaded_puzzles.contains(&puzzle.0.id()) {
            let def = assets.get(puzzle.0.id()).unwrap();
            construct_puzzle(puzzle_id, def, &mut commands, &asset_server);
        }
    }
}

fn construct_puzzle(puzzle_id: Entity, def: &PuzzleDef, commands: &mut Commands, asset_server: &AssetServer) {
    info!("Constructing puzzle");

    let name_map: HashMap<String, Entity> = def.parts.keys().map(
        |name| (name.clone(), commands.spawn((Name::new(name.clone()), ChildOf(puzzle_id))).id())
    ).collect();

    for (name, part) in &def.parts {
        let part_id = name_map[name];

        match part {
            PartDef::Zone(zone_def) => construct_zone(name, part_id, zone_def, &name_map, commands),
            PartDef::Item(item_def) => construct_item(name, part_id, item_def, &name_map, commands, asset_server),
        };
    }
}

fn construct_zone(name: &str, zone_id: Entity, zone_def: &ZoneDef, name_map: &HashMap<String, Entity>, commands: &mut Commands) {
    info!("Constructing zone: {} (entity {})", name, zone_id);

    commands.entity(zone_id).insert((
        Zone,
        ActiveInZones(zone_def.active_in.iter().map(|n| name_map[n]).collect()),
        Transform::from_translation(zone_def.clickable.0),
        Visibility::default(),
        ZoneCamera(Transform::from_translation(zone_def.camera.0).looking_at(zone_def.camera.1, Dir3::Y)),
    ));
    if zone_def.clickable.1 >= 0.0 {
        commands.entity(zone_id).insert((
            Clickable(zone_def.clickable.1),
        ));
    }
    if let Some(back_to) = &zone_def.back_to {
        commands.entity(zone_id).insert((
            UnzoomTo(name_map[back_to]),
        ));
    }

    if matches!(zone_def.state, ZoneState::Current) {
        commands.insert_resource(CurrentZone(Some(zone_id)));
    }
}

fn construct_item(name: &str, item_id: Entity, item_def: &ItemDef, name_map: &HashMap<String, Entity>, commands: &mut Commands,
                  asset_server: &AssetServer) {
    info!("Constructing item: {} (entity {})", name, item_id);

    let Some((gltf_path, scene_name)) = item_def.gltf_scene.split_once('#')
    else { panic!() };

    let gltf = asset_server.load(gltf_path.to_owned());

    commands.entity(item_id).insert((
        Item,
        ActiveInZones(item_def.active_in.iter().map(|n| name_map[n]).collect()),
        Transform::from_translation(item_def.position).with_rotation(Quat::from_scaled_axis(item_def.rotation)),
        Visibility::default(),
        NamedScene { gltf, scene_name: scene_name.to_owned() },
    ));
    if matches!(item_def.decoration, ItemDecoration::Rotating) {
        commands.entity(item_id).insert((
            Rotating,
        ));
    }
}
