//! Hello there!
//! Nothing much here be here are some note when building:
//!  - Scratch won't load if there are layer order collision; make sure all your Sprite have different layer order; Stage should be 0.
//!  - Scratch also won't load if there are no costume in Sprite or Stage; make sure to have atleast one!
//!
//! More documentation will made later if a lot of people actually uses this crate.
//!
//! Feel free to ask in github discussion. I will make sure to answer all of you questions if no one do so!

use std::{borrow::Cow, fmt};

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
pub enum BuildMethod {
    /// Try to make the result like you're working in Scratch Editor.
    /// This is commonly used for generating monitors, and maybe resolving field blocks if you're not already inserting one.
    /// But not sure if I will actually implement those.
    Fancy,
    /// Build as is. What you pass in there for build will spat right out as that.
    /// Some thing may looks off.
    /// Will return error in some cases (Most won't).
    #[default]
    AsIs,
    // /// Like `AsIs`
    // /// Try to build as is and ignore any error.
    // /// May cause the project to be unloadable by Scratch,
    // /// but if used properly may grant you the power of funny shenanigan.
    // ForcedAsIs,
}

// note that this thingy is temporary to a more robust error handling, probably

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct BuildReport {
    /// Error that occurred but do not prevent the build from failing completely.
    ///
    /// Note: Error in `Result` enum in build trait `build` function is a hard error. (kinda like a panic maybe)
    /// Prevents build from finishing in that builder completely.
    pub soft_errors: Vec<Cow<'static, str>>,
    /// Warning
    pub warnings: Vec<Cow<'static, str>>,
}

impl BuildReport {
    pub fn new() -> BuildReport {
        BuildReport {
            soft_errors: vec![],
            warnings: vec![],
        }
    }

    pub fn add_soft_error<S: Into<Cow<'static, str>>>(&mut self, message: S) {
        self.soft_errors.push(message.into())
    }

    pub fn add_warning<S: Into<Cow<'static, str>>>(&mut self, message: S) {
        self.soft_errors.push(message.into())
    }

    pub fn is_empty(&self) -> bool {
        self.soft_errors.is_empty() && self.warnings.is_empty()
    }
}

impl std::fmt::Display for BuildReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#?}")
    }
}

/// Trait for this crate's builder
///
/// Having build method stored inside the builder instead of passing by a function
/// allows for fine-grained tuning of how you want your things builded.
pub trait Build {
    type Builded;
    type BuildError;
    type Context;

    fn build(
        self,
        context: Self::Context,
    ) -> (Result<Self::Builded, Self::BuildError>, BuildReport);

    /// Check if there will be any error/warnings when build
    fn check(&self, with_method: BuildMethod) -> (Option<Self::BuildError>, BuildReport);

    /// Get currently assigned build method
    fn build_method(&self) -> BuildMethod;
    /// Make current builder uses this passed method
    fn set_build_method_to(&mut self, method: BuildMethod);
    /// Make current builder uses this passed method and also their inner builders
    fn set_build_method_recursivly_to(&mut self, method: BuildMethod);
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
