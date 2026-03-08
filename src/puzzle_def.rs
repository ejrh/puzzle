use std::collections::HashMap;

use bevy::asset::{Asset, AssetLoader, LoadContext};
use bevy::asset::io::Reader;
use bevy::math::Vec3;
use bevy::prelude::Reflect;
use bevy::reflect::TypePath;
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, Deserialize, Reflect)]
pub struct PuzzleDef {
    #[serde(default)]
    pub parts: HashMap<String, PartDef>,
}

#[derive(Deserialize, Reflect)]
pub enum PartDef {
    Zone(ZoneDef),
    Item(ItemDef),
}

#[derive(Deserialize, Reflect)]
pub struct ZoneDef {
    pub state: ZoneState,
    #[serde(default)]
    pub active_in: Vec<String>,
    pub back_to: Option<String>,
    pub camera: (Vec3, Vec3),
    #[serde(default)]
    pub clickable: (Vec3, f32),
}

#[derive(Deserialize, Reflect)]
pub enum ZoneState {
    Locked,
    Open,
    Current,
}

#[derive(Deserialize, Reflect)]
pub struct ItemDef {
    pub position: Vec3,
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
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let options = ron::Options::default().with_default_extension(ron::extensions::Extensions::UNWRAP_VARIANT_NEWTYPES);

        let puzzle_def = options.from_bytes::<PuzzleDef>(&bytes)?;

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
        let options = ron::Options::default().with_default_extension(ron::extensions::Extensions::UNWRAP_VARIANT_NEWTYPES);
        let f = std::fs::File::open(PATH).unwrap();
        let str = std::io::read_to_string(f).unwrap();
        let puzzle = options.from_str::<PuzzleDef>(&str).unwrap();

        assert!(!puzzle.parts.is_empty());
    }
}
