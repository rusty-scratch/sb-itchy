use crate::{context::ProjectContextData, resource::Resource, uid::Uid};
use std::collections::HashMap;

// use crate::{
//     build_context::ProjectContext,
//     resource::Resource,
//     target::{SpriteBuilder, StageBuilder},
//     uid::Uid,
//     Build, BuildMethod, BuildReport,
// };
// use sb_sbity::{
//     monitor::Monitor,
//     project::{Meta, Project},
//     target::SpriteOrStage,
// };

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectBuilder {
    pub stage_builder: StageBuilder,
    pub sprite_builders: Vec<SpriteBuilder>,
    pub monitors: Vec<sb_sbity::monitor::Monitor>,
    pub meta: sb_sbity::project::Meta,
}

impl ProjectBuilder {
    // pub fn set_stage(&mut self, stage_builder: StageBuilder) -> &mut Self {
    //     self.stage_builder = stage_builder;
    //     self
    // }

    // pub fn add_sprite(&mut self, sprite_builder: SpriteBuilder) -> &mut Self {
    //     self.sprite_builders.push(sprite_builder);
    //     self
    // }

    fn build(self, project_context: ProjectContextData) -> sb_sbity::project::Project {
        let ProjectBuilder {
            stage_builder,
            sprite_builders,
            monitors,
            meta,
        } = self;

        let all_broadcasts: HashMap<String, Uid> = stage_builder
            .target
            .broadcasts
            .iter()
            .chain(sprite_builders.iter().flat_map(|sb| &sb.target.broadcasts))
            .map(|(name, uid)| (name.clone(), uid.clone()))
            .collect::<HashMap<_, _>>();

        let mut targets = Vec::with_capacity(1 + sprite_builders.len());
        let (stage, global_varlist_buf) = stage_builder.build(res_buf, &all_broadcasts);
        targets.push(sb_sbity::target::SpriteOrStage::Stage(stage));
        targets.extend(sprite_builders.into_iter().map(|sprite_builder| {
            sb_sbity::target::SpriteOrStage::Sprite(sprite_builder.build(
                res_buf,
                &global_varlist_buf,
                &all_broadcasts,
            ))
        }));
        let project = sb_sbity::project::Project {
            meta,
            extensions: serde_json::value::Value::Array(vec![]),
            monitors,
            targets,
        };
        project
    }
}

impl Default for ProjectBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        ProjectBuilder {
            stage_builder:   StageBuilder::default(),
            sprite_builders: Vec::default(),
            monitors:        Vec::default(),
            meta: sb_sbity::project::Meta {
                semver: "3.0.0".to_owned(),
                vm:     "0.2.0-prerelease.20220222132735".to_owned(),
                agent:  "sb-itchy/0.1.0".to_owned(),
            },
        }
    }
}
