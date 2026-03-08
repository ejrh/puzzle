use bevy::asset::{AssetEvent, Assets, Handle};
use bevy::gltf::Gltf;
use bevy::log::warn;
use bevy::prelude::{Changed, Commands, Component, Entity, MessageReader, Query, Reflect, Res};
use bevy::scene::SceneRoot;

#[derive(Component, Reflect)]
pub struct NamedScene {
    pub gltf: Handle<Gltf>,
    pub scene_name: String,
}

pub fn update_named_scenes(
    mut events: MessageReader<AssetEvent<Gltf>>,
    gltfs: Res<Assets<Gltf>>,
    scenes: Query<(Entity, &NamedScene, Option<&SceneRoot>)>,
    changed_scenes: Query<(), Changed<NamedScene>>,
    mut commands: Commands,
) {
    /* If no asset events, and no changed NamedScene components, then do nothing */
    if changed_scenes.is_empty() && events.read().last().is_none() {
        return;
    }

    for (id, scene, current_root) in scenes.iter() {
        if let Some(file) = gltfs.get(scene.gltf.id()) {
            if let Some(scene_handle) = file.named_scenes.get(scene.scene_name.as_str()) {
                /* Don't update it if it's already correct */
                if let Some(SceneRoot(h)) = current_root {
                    if *h == *scene_handle { continue; }
                }

                commands.entity(id).insert(SceneRoot(scene_handle.clone()));
            } else {
                warn!("No scene named: {}", scene.scene_name);
            }
        }
    }
}
