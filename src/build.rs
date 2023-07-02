use std::borrow::Cow;

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
pub struct BuildReport {
    /// Error that occurred but do not prevent the build from failing completely.
    ///
    /// Note: Error in `Result` enum in the build trait, `Build::build` function is a hard error. (kinda like a panic maybe)
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

    pub fn add_soft_error<S: Into<Cow<'static, str>>>(&mut self, message: S) -> &mut Self {
        self.soft_errors.push(message.into());
        self
    }

    pub fn add_warning<S: Into<Cow<'static, str>>>(&mut self, message: S) -> &mut Self {
        self.soft_errors.push(message.into());
        self
    }

    pub fn n0_add_soft_error<S: Into<Cow<'static, str>>>(mut self, message: S) -> Self {
        self.add_soft_error(message);
        self
    }

    pub fn n0_add_warning<S: Into<Cow<'static, str>>>(mut self, message: S) -> Self {
        self.add_warning(message);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.soft_errors.is_empty() && self.warnings.is_empty()
    }
}

pub trait Build {
    type Into;
    type BuildError;
    type Arg;

    fn build(self, arg: Self::Arg) -> (Result<Self::Into, Self::BuildError>, BuildReport);
    fn check(&self, with_method: BuildMethod) -> (Option<Self::BuildError>, BuildReport);
    fn build_method(&self) -> BuildMethod;
    fn set_build_method_to(&mut self, method: BuildMethod);
    fn set_build_method_recursivly_to(&mut self, method: BuildMethod);
}
