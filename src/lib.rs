//! Hello there!
//! Nothing much here be here are some note when building:
//!  - Scratch won't load if there are layer order collision; make sure all your Sprite have different layer order; Stage should be 0.
//!  - Scratch also won't load if there are no costume in Sprite or Stage; make sure to have atleast one!
//!
//! More documentation will made later if a lot of people actually uses this crate.
//!
//! Feel free to ask in github discussion. I will make sure to answer all of you questions if no one do so!

use std::borrow::Cow;

pub mod asset;
pub mod block;
pub mod comment;
pub mod data;
pub mod project;
pub mod stack;
pub mod target;

pub mod opcode;
pub mod resource;
pub mod uid;

pub mod build_context;

pub mod export;
pub mod import;

pub mod blocks;

/// Allow user to pick how they want their thing to be builded.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
enum BuildMethod {
    /// Try to make the result like you're working in Scratch Editor.
    /// This is commonly used for generating monitors, and maybe resolving field blocks if you're not already inserting one.
    /// But not sure if I actually will implement those.
    Fancy,
    /// Build as is. What you pass in there for build will spat right out as that.
    /// Some thing may looks off.
    /// Will return error in some cases (Most won't).
    #[default]
    AsIs,
    /// Try to build as is and ignore any error.
    /// May cause the project to be unloadable by Scratch,
    /// but if used properly may grant you the power of funny shenanigan.
    ForcedAsIs,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BuildWarnings(pub Vec<Cow<'static, str>>);

pub trait Build {
    type Builded;
    type BuildError;
    type CheckResult;

    fn build(self) -> Result<(Self::Builded, BuildWarnings), (Self::BuildError, BuildWarnings)>;

    /// Check if there will be any error when build
    fn check(&self, with_method: BuildMethod) -> Option<Self::CheckResult>;
    /// Get currently assigned build method
    fn current_build_method(&self) -> BuildMethod;
    /// Make current builder uses this passed method
    fn set_build_method(&mut self, method: BuildMethod);
    /// Make current builder uses this passed method and also their
    fn set_build_method_recursive(&mut self, method: BuildMethod);
}

pub mod prelude {
    pub use self::{
        asset::{AssetBuilder, CostumeBuilder, SoundBuilder},
        block::{
            BlockBuilder, BlockFieldBuilder, BlockInputBuilder, BlockNormalBuilder,
            BlockVarListBuilder, FieldKind, StackOrValue, VarListFrom,
        },
        build_context::{GlobalVarListContext, TargetContext},
        comment::CommentBuilder,
        data::{ListBuilder, VariableBuilder},
        opcode::StandardOpCode,
        project::ProjectBuilder,
        resource::{Resource, ResourceError},
        stack::StackBuilder,
        target::{SpriteBuilder, StageBuilder, TargetBuilder},
        uid::Uid,
    };
    use super::*;
    pub use sb_sbity::block::{BlockInputValue, ShadowInputType};
}
