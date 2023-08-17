use sb_sbity::asset as sbity_asset;

use crate::{name::Name, resource::Resource};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostumeBuilder {
    pub rotation_center_x: i64,
    pub rotation_center_y: i64,
    pub asset: AssetBuilder,
}

impl CostumeBuilder {
    pub fn new(asset_builder: AssetBuilder) -> CostumeBuilder {
        CostumeBuilder {
            asset: asset_builder,
            rotation_center_x: 0,
            rotation_center_y: 0,
        }
    }

    pub fn build(self) -> (sbity_asset::Costume, Resource) {
        let CostumeBuilder {
            rotation_center_x,
            rotation_center_y,
            asset,
        } = self;
        let (asset, resource) = asset.build();
        (
            sbity_asset::Costume {
                rotation_center_x: rotation_center_x.into(),
                rotation_center_y: rotation_center_y.into(),
                bitmap_resolution: Some(1),
                asset,
            },
            resource,
        )
    }
}

/// Not really sure what to do here yet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundBuilder {
    pub rate: u64,
    pub sample_count: u64,
    pub format: Option<String>,
    pub asset: AssetBuilder,
}

impl SoundBuilder {
    pub fn build(self) -> (sbity_asset::Sound, Resource) {
        let SoundBuilder {
            rate,
            sample_count,
            format,
            asset,
        } = self;
        let (asset, resource) = asset.build();
        (
            sbity_asset::Sound {
                rate,
                sample_count,
                format,
                asset,
            },
            resource,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetBuilder {
    pub name: Name,
    pub resource: Resource,
}

impl AssetBuilder {
    pub fn new<'a, S: Into<&'a str>>(name: S, resource: Resource) -> AssetBuilder {
        AssetBuilder {
            name: Name::from(name.into()),
            resource,
        }
    }

    pub fn build(self) -> (sbity_asset::Asset, Resource) {
        let AssetBuilder { name, resource } = self;
        let file_extension = resource.file_extension();
        let md5_hash = resource.md5_hash();
        let asset = sbity_asset::Asset {
            asset_id: md5_hash.to_owned(),
            name: name.to_string(),
            md5ext: Some(md5_hash.to_owned() + "." + file_extension),
            data_format: file_extension.to_string(),
        };
        (asset, resource)
    }
}
