use std::collections::HashMap;

// use sb_sbity::{
//     asset::{Costume, Sound},
//     block::Block,
//     broadcast::Broadcast,
//     comment::Comment,
//     list::List,
//     string_hashmap::StringHashMap,
//     target::{RotationStyle, Sprite, Stage, Target, VideoState},
//     variable::Variable,
// };

use sb_sbity::string_hashmap::StringHashMap;

// use crate::{
//     asset::{CostumeBuilder, SoundBuilder},
//     build_context::{ProjectContext, TargetContext},
//     comment::CommentBuilder,
//     data::{ListBuilder, VariableBuilder},
//     resource::Resource,
//     stack::StackBuilder,
//     uid::Uid,
//     Build, BuildMethod, BuildReport,
// };
use crate::{
    build::{
        asset::{CostumeBuilder, SoundBuilder},
        context::{GlobalScriptingCtx, LocalScriptingCtx, ProjectCtxMut},
    },
    name::Name,
    uid::Uid,
};

use super::{
    comment::CommentBuilder,
    context::{GlobalScriptingContextData, LocalScriptingContextData},
    data::{ListBuilder, VariableBuilder},
    stack::StackBuilder,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TargetBuilder {
    pub name: String,
    pub variables: HashMap<Name, VariableBuilder>,
    pub lists: HashMap<Name, ListBuilder>,
    pub broadcasts: HashMap<Name, Uid>,
    pub block_stackes: Vec<StackBuilder>,
    pub comments: HashMap<Uid, sb_sbity::comment::Comment>,
    pub costumes: Vec<CostumeBuilder>,
    pub sounds: Vec<SoundBuilder>,
    pub current_costume: u64,
    pub layer_order: u64,
    pub volume: f64,
}

impl TargetBuilder {
    pub fn add_variable(&mut self, name: Name, variable_builder: VariableBuilder) -> &mut Self {
        self.variables.insert(name, variable_builder);
        self
    }

    pub fn add_list(&mut self, name: Name, list_builder: ListBuilder) -> &mut Self {
        self.lists.insert(name, list_builder);
        self
    }

    pub fn add_broadcast(&mut self, name: Name, uid: Uid) -> &mut Self {
        self.broadcasts.insert(name, uid);
        self
    }

    pub fn add_block_stack(&mut self, stack_builder: StackBuilder) -> &mut Self {
        self.block_stackes.push(stack_builder);
        self
    }

    pub fn add_comment(&mut self, comment_builder: CommentBuilder, uid: Uid) -> &mut Self {
        let comment = comment_builder.build();
        self.comments.insert(uid, comment);
        self
    }

    pub fn add_costume(&mut self, costume_builder: CostumeBuilder) -> &mut Self {
        self.costumes.push(costume_builder);
        self
    }

    pub fn add_sound(&mut self, sound_builder: SoundBuilder) -> &mut Self {
        self.sounds.push(sound_builder);
        self
    }

    // /// When global_varlist_buf suppose to be none when the Stage itself is building.
    // /// The .1 return value is going to return Some when stage itself is also building.
    // pub fn build(
    //     self,
    //     project_ctx: ProjectCtx,
    //     global_scripting_ctx: GlobalScriptingCtx,
    //     // res_buf: &mut Vec<Resource>,
    //     // global_varlist_ctx: Option<&GlobalVarListContext>,
    //     // all_broadcasts: &HashMap<String, Uid>,
    // ) -> (Target, Option<GlobalVarListContext>) {
    //     let TargetBuilder {
    //         name,
    //         variables,
    //         lists,
    //         broadcasts,
    //         block_stackes,
    //         comments,
    //         costumes,
    //         sounds,
    //         current_costume,
    //         layer_order,
    //         volume,
    //     } = self;
    //     let variables: HashMap<String, Variable> = variables
    //         .into_iter()
    //         .map(|(var_name, var_builder)| {
    //             let (var, uid) = var_builder.build(var_name);
    //             (uid.into_inner(), var)
    //         })
    //         .collect();
    //     let lists: HashMap<String, List> = lists
    //         .into_iter()
    //         .map(|(list_name, list_builder)| {
    //             let (list, uid) = list_builder.build(list_name);
    //             (uid.into_inner(), list)
    //         })
    //         .collect();
    //     let broadcasts: HashMap<String, Broadcast> = broadcasts
    //         .into_iter()
    //         .map(|(name, uid)| (uid.into_inner(), Broadcast { name }))
    //         .collect();

    //     let mut comments = comments;
    //     let variable_ctx: HashMap<String, Uid> = variables
    //         .iter()
    //         .map(|(uid, var)| (var.name.clone(), Uid::new(uid)))
    //         .collect();
    //     let list_ctx: HashMap<String, Uid> = lists
    //         .iter()
    //         .map(|(uid, list)| (list.name.clone(), Uid::new(uid)))
    //         .collect();
    //     let blocks: HashMap<String, Block> = block_stackes
    //         .into_iter()
    //         .flat_map(|stack_builder| {
    //             let builded_stack = stack_builder.build(
    //                 &Uid::generate(),
    //                 &mut comments,
    //                 &match global_varlist_ctx {
    //                     Some(global_varlist_ctx) => TargetContext {
    //                         global_vars: &global_varlist_ctx.vars,
    //                         global_lists: &global_varlist_ctx.lists,
    //                         this_sprite_vars: &variable_ctx,
    //                         this_sprite_lists: &list_ctx,
    //                         all_broadcasts,
    //                     },
    //                     None => TargetContext {
    //                         global_vars: &variable_ctx,
    //                         global_lists: &list_ctx,
    //                         this_sprite_vars: &variable_ctx,
    //                         this_sprite_lists: &list_ctx,
    //                         all_broadcasts,
    //                     },
    //                 },
    //             );
    //             builded_stack
    //                 .into_iter()
    //                 .map(|(uid, block)| (uid.into_inner(), block))
    //         })
    //         .collect();
    //     let comments: HashMap<String, Comment> = comments
    //         .into_iter()
    //         .map(|(uid, comment)| (uid.into_inner(), comment))
    //         .collect();
    //     let costumes: Vec<Costume> = costumes
    //         .into_iter()
    //         .map(|costume_builder| costume_builder.build(res_buf))
    //         .collect();
    //     let sounds: Vec<Sound> = sounds
    //         .into_iter()
    //         .map(|sound_builder| sound_builder.build(res_buf))
    //         .collect();
    //     let target = Target {
    //         name,
    //         variables: StringHashMap(variables),
    //         lists: StringHashMap(lists),
    //         broadcasts: StringHashMap(broadcasts),
    //         blocks: StringHashMap(blocks),
    //         comments: StringHashMap(comments),
    //         current_costume: current_costume as i64,
    //         costumes,
    //         sounds,
    //         layer_order: layer_order as i64,
    //         volume: volume.into(),
    //     };
    //     (
    //         target,
    //         match global_varlist_ctx {
    //             Some(_) => None,
    //             None => Some(GlobalVarListContext {
    //                 vars: variable_ctx,
    //                 lists: list_ctx,
    //             }),
    //         },
    //     )
    // }

    pub fn build_get_globals(
        self,
        project_ctx: ProjectCtxMut,
    ) -> (sb_sbity::target::Target, GlobalScriptingContextData) {
        let TargetBuilder {
            name,
            variables,
            lists,
            broadcasts,
            block_stackes,
            comments,
            costumes,
            sounds,
            current_costume,
            layer_order,
            volume,
        } = self;
        let builded_variables: HashMap<Uid, sb_sbity::variable::Variable> = variables
            .into_iter()
            .map(|(var_name, var_builder)| {
                let var = var_builder.build(var_name);
                (Uid::generate(), var)
            })
            .collect();
        let builded_lists: HashMap<Uid, sb_sbity::list::List> = lists
            .into_iter()
            .map(|(list_name, list_builder)| {
                let list = list_builder.build(list_name);
                (Uid::generate(), list)
            })
            .collect();
        let builded_broadcasts: HashMap<Uid, sb_sbity::broadcast::Broadcast> = broadcasts
            .into_iter()
            .map(|(name, uid)| {
                (
                    uid,
                    sb_sbity::broadcast::Broadcast {
                        name: name.to_string(),
                    },
                )
            })
            .collect();

        let mut comments = comments;
        let variable_ctx: HashMap<Name, Uid> = builded_variables
            .iter()
            .map(|(uid, variable)| (Name::from(&variable.name), uid.clone()))
            .collect();
        let list_ctx: HashMap<Name, Uid> = builded_lists
            .iter()
            .map(|(uid, list)| (Name::from(&list.name), uid.clone()))
            .collect();
        let global_scripting_context_data = GlobalScriptingContextData {
            global_variables: variable_ctx,
            global_lists: list_ctx,
            broadcasts,
        };
        let local_scripting_context_data = LocalScriptingContextData {
            variables: HashMap::new(),
            lists: HashMap::new(),
            comments: HashMap::new(),
        };
        let blocks: HashMap<String, sb_sbity::block::Block> = block_stackes
            .into_iter()
            .flat_map(|stack_builder| {
                let builded_stack = stack_builder.build(
                    Uid::generate(),
                    &mut global_scripting_context_data,
                    &mut local_scripting_context_data,
                );
                builded_stack
                    .into_iter()
                    .map(|(uid, block)| (uid.to_string(), block))
            })
            .collect();
        let builded_comments: HashMap<String, sb_sbity::comment::Comment> = comments
            .into_iter()
            .map(|(uid, comment)| (uid.to_string(), comment))
            .collect();
        let builded_costumes: Vec<sb_sbity::asset::Costume> = costumes
            .into_iter()
            .map(|costume_builder| costume_builder.build(project_ctx))
            .collect();
        let builded_sounds: Vec<sb_sbity::asset::Sound> = sounds
            .into_iter()
            .map(|sound_builder| sound_builder.build(project_ctx))
            .collect();
        let target = sb_sbity::target::Target {
            name,
            variables: StringHashMap(
                builded_variables
                    .into_iter()
                    .map(|(uid, var)| (uid.to_string(), var))
                    .collect(),
            ),
            lists: StringHashMap(
                builded_lists
                    .into_iter()
                    .map(|(uid, list)| (uid.to_string(), list))
                    .collect(),
            ),
            broadcasts: StringHashMap(
                builded_broadcasts
                    .into_iter()
                    .map(|(uid, broadcast)| (uid.to_string(), broadcast))
                    .collect(),
            ),
            blocks: StringHashMap(blocks),
            comments: StringHashMap(builded_comments),
            current_costume: current_costume as i64,
            costumes: builded_costumes,
            sounds: builded_sounds,
            layer_order: layer_order as i64,
            volume: volume.into(),
        };
        (
            target,
            match global_varlist_ctx {
                Some(_) => None,
                None => Some(GlobalVarListContext {
                    vars: variable_ctx,
                    lists: list_ctx,
                }),
            },
        )
    }
}

impl Default for TargetBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        TargetBuilder {
            name:            "".to_owned(),
            variables:       HashMap::default(),
            lists:           HashMap::default(),
            broadcasts:      HashMap::default(),
            block_stackes:   Vec::default(),
            comments:        HashMap::default(),
            costumes:        Vec::default(),
            sounds:          Vec::default(),
            current_costume: 0,
            layer_order:     0,
            volume:          100.,
            build_method:    BuildMethod::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StageBuilder {
    pub target: TargetBuilder,
    pub tempo: i64,
    pub video_state: sb_sbity::target::VideoState,
    pub video_transparency: i64,
    // Not availiable yet.
    // TODO: do this.
    // text_to_speech_language: (),
}

impl StageBuilder {
    pub fn build(self, project_ctx: ProjectCtxMut) -> sb_sbity::target::Stage {
        let StageBuilder {
            target,
            tempo,
            video_state,
            video_transparency,
            build_method,
        } = self;
        let (target, Some(global_var_list)) = target.build_get_globals(res_buf, None, all_broadcasts) else {
            panic!("stage suppose to return what global var they had");
        };
        let stage = Stage {
            target,
            tempo: tempo.into(),
            video_state,
            video_transparency: video_transparency.into(),
            text_to_speech_language: None,
            is_stage: true,
        };
        (Ok(stage), global_var_list)
    }
}

impl Default for StageBuilder {
    fn default() -> Self {
        StageBuilder {
            target: TargetBuilder {
                name: "Stage".to_owned(),
                ..Default::default()
            },
            tempo: 60,
            video_state: VideoState::On,
            video_transparency: 50,
            build_method: BuildMethod::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpriteBuilder {
    pub target: TargetBuilder,
    pub visible: bool,
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub direction: f64,
    pub draggable: bool,
    pub rotation_style: sb_sbity::target::RotationStyle,
}

impl SpriteBuilder {
    pub fn build(
        self,
        res_buf: &mut Vec<Resource>,
        global_varlist_buf: &GlobalVarListContext,
        all_broadcasts: &HashMap<String, Uid>,
    ) -> Sprite {
        let SpriteBuilder {
            target,
            visible,
            x,
            y,
            size,
            direction,
            draggable,
            rotation_style,
        } = self;
        Sprite {
            target: target
                .build_get_globals(res_buf, Some(global_varlist_buf), all_broadcasts)
                .0,
            visible,
            x: x.into(),
            y: y.into(),
            size: size.into(),
            direction: direction.into(),
            draggable,
            rotation_style,
            is_stage: false,
        }
    }
}

impl Default for SpriteBuilder {
    fn default() -> Self {
        SpriteBuilder {
            target: TargetBuilder::default(),
            visible: true,
            x: 0.,
            y: 0.,
            size: 100.,
            direction: 90.,
            draggable: false,
            rotation_style: sb_sbity::target::RotationStyle::AllAround,
        }
    }
}
