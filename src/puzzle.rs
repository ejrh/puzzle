use std::collections::HashMap;

use bevy::app::{App, Plugin, Update};
use bevy::asset::{AssetApp, AssetEvent, AssetServer, Assets, Handle};
use bevy::gltf::Gltf;
use bevy::log::{debug, info, warn};
use bevy::picking::Pickable;
use bevy::prelude::{on_message, ChildOf, Commands, Component, Entity, IntoScheduleConfigs, MessageReader, Name, Query, Reflect, Res, ResMut, Transform, Visibility};
use bevy::scene::Scene;

use crate::clickable::Clickable;
use crate::item::{Item, Rotating};
use crate::logic::LogicMessage;
use crate::puzzle_def::{ItemDecoration, ItemDef, PartDef, Position, PuzzleDef, PuzzleDefLoader, ZoneDef};
use crate::utils::named_scene::NamedScene;
use crate::zone::Zone;

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
    puzzle_assets: Res<Assets<PuzzleDef>>,
    mut gltf_assets: ResMut<Assets<Gltf>>,
    mut scene_assets: ResMut<Assets<Scene>>,
    puzzles: Query<(Entity, &Puzzle)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let loaded_puzzles: Vec<_> = asset_events.read().filter_map(
        |e| if let AssetEvent::LoadedWithDependencies { id } = e { Some(*id) } else { None }
    ).collect();

    for (puzzle_id, puzzle) in puzzles {
        if loaded_puzzles.contains(&puzzle.0.id()) {
            let def = puzzle_assets.get(puzzle.0.id()).unwrap();

            let initial_scene = def.initial_layout.as_ref()
                .and_then(|il| il.label())
                .and_then(|l| {
                    def.initial_layout_handle.as_ref()
                        .and_then(|h| gltf_assets.get_mut(h))
                        .and_then(|g| g.named_scenes.get_mut(l))
                })
                .and_then(|h| scene_assets.get_mut(h));

            construct_puzzle(puzzle_id, def, initial_scene, &mut commands, &asset_server);
        }
    }
}

fn construct_puzzle(puzzle_id: Entity, def: &PuzzleDef, scene: Option<&mut Scene>, commands: &mut Commands, asset_server: &AssetServer) {
    info!("Constructing puzzle");

    let mut initial_positions = HashMap::new();

    if let Some(scene) = scene {
        info!("Using positions from {}", def.initial_layout.as_ref().unwrap());

        for (name, transform) in scene.world.query::<(&Name, &Transform)>().query(&scene.world) {
            initial_positions.insert(name.as_str(), transform);
        }
    }

    let name_map: HashMap<String, Entity> = def.parts.keys().map(
        |name| (name.clone(), commands.spawn((Name::new(name.clone()), ChildOf(puzzle_id))).id())
    ).collect();

    for (name, part) in &def.parts {
        let part_id = name_map[name];

        match part {
            PartDef::Zone(zone_def) => construct_zone(name, part_id, zone_def, &initial_positions, &name_map, commands),
            PartDef::Item(item_def) => construct_item(name, part_id, item_def, &initial_positions, &name_map, commands, asset_server),
        };
    }

    commands.write_message(LogicMessage::CreatedPuzzle(puzzle_id));
}

fn construct_zone(name: &str, zone_id: Entity, zone_def: &ZoneDef, initial_positions: &HashMap<&str, &Transform>, name_map: &HashMap<String, Entity>, commands: &mut Commands) {
    info!("Constructing zone: {} (entity {})", name, zone_id);

    let transform = pick_transform(initial_positions, name, &zone_def.position);

    commands.entity(zone_id).insert((
        Zone,
        transform,
        Visibility::default(),
    ));

    if let Some(clickable) = &zone_def.clickable {
        let clickable_name = format!("{}/clickable", name);
        let clickable_transform = pick_transform(initial_positions, &clickable_name, &clickable.position);
        commands.spawn((
            Pickable { should_block_lower: false, is_hoverable: true },
            Name::new(clickable_name),
            Clickable(clickable.radius),
            ChildOf(zone_id),
            clickable_transform,
        ));
    }
}

fn construct_item(name: &str, item_id: Entity, item_def: &ItemDef, initial_positions: &HashMap<&str, &Transform>, name_map: &HashMap<String, Entity>, commands: &mut Commands,
                  asset_server: &AssetServer) {
    info!("Constructing item: {} (entity {})", name, item_id);

    let transform = pick_transform(initial_positions, name, &item_def.position);

    let Some((gltf_path, scene_name)) = item_def.gltf_scene.split_once('#')
    else { panic!() };

    let gltf = asset_server.load(gltf_path.to_owned());

    commands.entity(item_id).insert((
        Item,
        transform,
        Visibility::default(),
        NamedScene { gltf, scene_name: scene_name.to_owned() },
    ));
    if matches!(item_def.decoration, ItemDecoration::Rotating) {
        commands.entity(item_id).insert((
            Rotating,
        ));
    }
}

fn pick_transform(initial_positions: &HashMap<&str, &Transform>, name: &str, position: &Option<Position>) -> Transform {
    if let Some(new_transform) = initial_positions.get(name) {
        debug!("Using scene position for {}", name);
        **new_transform
    } else if let Some(position) = position {
        debug!("Using data file position for {}", name);
        position.to_transform()
    } else {
        warn!("No position for {}", name);
        Transform::default()
    }
}
