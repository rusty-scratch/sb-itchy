use std::collections::HashMap;

use crate::{name::Name, resource::Resource, uid::Uid};

// pub type StackScriptingContext = Context<StackScriptingContextData>;
// pub type LocalScriptingContext = Context<LocalScriptingContextData>;
// pub type GlobalScriptingContext = Context<GlobalScriptingContextData>;

// #[derive(Debug, Default, Clone)]
// pub struct Context<T> {
//     inner: Arc<RwLock<T>>,
// }

// impl<T> Context<T> {
//     pub fn new(data: T) -> Context<T> {
//         Context {
//             inner: Arc::new(RwLock::new(data)),
//         }
//     }

//     pub fn read(&self) -> sync::LockResult<sync::RwLockReadGuard<'_, T>> {
//         self.inner.read()
//     }

//     pub fn try_read(&self) -> sync::TryLockResult<sync::RwLockReadGuard<'_, T>> {
//         self.inner.try_read()
//     }

//     pub fn write(&self) -> sync::LockResult<sync::RwLockWriteGuard<'_, T>> {
//         self.inner.write()
//     }

//     pub fn try_write(&self) -> sync::TryLockResult<sync::RwLockWriteGuard<'_, T>> {
//         self.inner.try_write()
//     }

//     pub fn wait_unwrap(self) -> T {
//         let mut arc = self.inner;
//         loop {
//             match Arc::try_unwrap(arc) {
//                 Ok(data) => return data.,
//                 Err(e) => arc = e,
//             }
//         }
//     }
// }

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ProjectContextData {
    pub resources: Vec<Resource>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GlobalScriptingContextData {
    pub global_variables: HashMap<Name, Uid>,
    pub global_lists: HashMap<Name, Uid>,
    pub broadcasts: HashMap<Name, Uid>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct LocalScriptingContextData {
    pub variables: HashMap<Name, Uid>,
    pub lists: HashMap<Name, Uid>,
    pub comments: HashMap<Uid, sb_sbity::comment::Comment>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockStackContextData {
    pub blocks: HashMap<Uid, sb_sbity::block::Block>,
}
