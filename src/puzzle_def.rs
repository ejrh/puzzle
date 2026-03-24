use std::collections::HashMap;

use bevy::asset::{Asset, AssetLoader, AssetPath, Handle, LoadContext};
use bevy::asset::io::Reader;
use bevy::gltf::Gltf;
use bevy::math::{Dir3, Quat, Vec3};
use bevy::prelude::{Reflect, Transform};
use bevy::reflect::TypePath;
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, Deserialize, Reflect)]
pub struct PuzzleDef {
    pub initial_layout: Option<AssetPath<'static>>,
    #[serde(default)]
    pub parts: HashMap<String, PartDef>,

    #[serde(skip)]
    pub initial_layout_handle: Option<Handle<Gltf>>,
}

#[derive(Deserialize, Reflect)]
pub enum PartDef {
    Zone(ZoneDef),
    Item(ItemDef),
}

#[derive(Deserialize, Reflect)]
pub struct ZoneDef {
    pub position: Option<Position>,
    pub clickable: Option<Clickable>,
}

#[derive(Deserialize, Reflect)]
pub struct ItemDef {
    pub position: Option<Position>,
    pub gltf_scene: String,
    pub state: ItemState,
    #[serde(default)]
    pub active_in: Vec<String>,
    #[serde(default)]
    pub decoration: ItemDecoration,
}

#[derive(Deserialize, Reflect)]
pub enum ItemState {
    Locked,
    Open,
    Active,
}

#[derive(Default, Deserialize, Reflect)]
pub enum ItemDecoration {
    #[default]
    None,
    Rotating,
}

#[derive(Deserialize, Reflect)]
pub struct Clickable {
    pub position: Option<Position>,
    pub radius: f32,
}

#[derive(Deserialize, Reflect)]
pub struct Position {
    pub translation: Vec3,
    pub looking_at: Option<Vec3>,
    pub rotation: Option<Vec3>,
}

impl Position {
    pub fn to_transform(&self) -> Transform {
        let mut tf = Transform::from_translation(self.translation);
        if let Some(looking_at) = self.looking_at {
            tf = tf.looking_at(looking_at, Dir3::Y);
        }
        if let Some(rotation) = self.rotation {
            tf = tf.with_rotation(Quat::from_scaled_axis(rotation));
        }
        tf
    }
}

#[derive(Default, TypePath)]
pub struct PuzzleDefLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PuzzleDefLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for PuzzleDefLoader {
    type Asset = PuzzleDef;
    type Settings = ();
    type Error = PuzzleDefLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let options = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
            .with_default_extension(ron::extensions::Extensions::UNWRAP_VARIANT_NEWTYPES);

        let mut puzzle_def = options.from_bytes::<PuzzleDef>(&bytes)?;

        if let Some(initial_layout) = &puzzle_def.initial_layout {
            let gltf_path = initial_layout.path().to_owned();
            puzzle_def.initial_layout_handle = Some(load_context.load(gltf_path));
        }

        Ok(puzzle_def)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_file() {
        const PATH: &str = "assets/puzzle.ron";
        let options = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
            .with_default_extension(ron::extensions::Extensions::UNWRAP_VARIANT_NEWTYPES);
        let f = std::fs::File::open(PATH).unwrap();
        let str = std::io::read_to_string(f).unwrap();
        let puzzle = options.from_str::<PuzzleDef>(&str).unwrap();

        assert!(!puzzle.parts.is_empty());
    }
}
