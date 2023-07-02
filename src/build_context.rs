use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
    sync::{Arc, RwLock},
};

use crate::uid::Uid;

#[derive(Debug, PartialEq, Eq)]
struct Test<T>(T);

pub struct GlobalVarListContext {
    pub vars: HashMap<String, Uid>,
    pub lists: HashMap<String, Uid>,
}

pub struct TargetContext<'a> {
    pub global_vars: &'a HashMap<String, Uid>,
    pub global_lists: &'a HashMap<String, Uid>,
    pub this_sprite_vars: &'a HashMap<String, Uid>,
    pub this_sprite_lists: &'a HashMap<String, Uid>,
    pub all_broadcasts: &'a HashMap<String, Uid>,
}

fn new_project_context() -> ProjectContext {
    Context::new(ProjectContextInner {
        variables: HashMap::new(),
        lists: HashMap::new(),
        broadcasts: HashMap::new(),
        resources: Vec::new(),
    })
}

fn new_target_context() -> TargetContext {
    Context::new(TargetContextInner {
        variables: HashMap::new(),
        lists: HashMap::new(),
    })
}

pub type ProjectContext = Context<ProjectContextInner>;
pub type TargetContext2 = Context<TargetContextInner>;

/// Context to be use while building.
/// The Arc<Rwlock<C>> is currently have no use because multi-threading is not yet implemented but is planned.
#[derive(Debug, Default)]
pub struct Context<C>(Arc<RwLock<C>>);

impl<C> Context<C> {
    pub fn new(context: C) -> Context<C> {
        Context(Arc::new(RwLock::new(context)))
    }

    pub fn inner(&self) -> &RwLock<C> {
        &self.0
    }
}

impl<C> Clone for Context<C> {
    fn clone(&self) -> Self {
        Context(Arc::clone(&self.0))
    }
}

impl<C> PartialEq for Context<C>
where
    C: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let context = self.0.read().unwrap();
        let other_context = other.0.read().unwrap();
        *context == *other_context
    }
}

impl<C: Eq> Eq for Context<C> {
    fn assert_receiver_is_total_eq(&self) {}
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ProjectContextInner {
    variables: HashMap<String, Uid>,
    lists: HashMap<String, Uid>,
    broadcasts: HashMap<String, Uid>,
    resources: Vec<crate::resource::Resource>,
}

impl ProjectContextInner {
    pub fn variables(&self) -> &HashMap<String, Uid> {
        &self.variables
    }

    pub fn variables_mut(&mut self) -> &mut HashMap<String, Uid> {
        &mut self.variables
    }

    pub fn lists(&self) -> &HashMap<String, Uid> {
        &self.lists
    }

    pub fn lists_mut(&mut self) -> &mut HashMap<String, Uid> {
        &mut self.lists
    }

    pub fn broadcasts(&self) -> &HashMap<String, Uid> {
        &self.broadcasts
    }

    pub fn broadcasts_mut(&mut self) -> &mut HashMap<String, Uid> {
        &mut self.broadcasts
    }

    pub fn resources(&self) -> &Vec<crate::resource::Resource> {
        &self.resources
    }

    pub fn resources_mut(&mut self) -> &mut Vec<crate::resource::Resource> {
        &mut self.resources
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetContextInner {
    variables: HashMap<String, Uid>,
    lists: HashMap<String, Uid>,
}

impl TargetContextInner {
    pub fn variables(&self) -> &HashMap<String, Uid> {
        &self.variables
    }

    pub fn variables_mut(&mut self) -> &mut HashMap<String, Uid> {
        &mut self.variables
    }

    pub fn lists(&self) -> &HashMap<String, Uid> {
        &self.lists
    }

    pub fn lists_mut(&mut self) -> &mut HashMap<String, Uid> {
        &mut self.lists
    }
}
