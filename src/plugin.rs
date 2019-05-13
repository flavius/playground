use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub type PluginError = &'static str;

pub trait AsAnyPlugin {
    fn as_any(self: Rc<Self>) -> Rc<dyn Any>;
}

pub trait AsAnySpecification {
    fn as_any(self: &Self) -> &dyn Any;
}

pub trait Specification: Any + AsAnySpecification {
    fn new() -> Self
    where
        Self: Sized;
    //type Plugin: Plugin;
    fn name(&self) -> &'static str;
    fn id(&self) -> std::any::TypeId;
    fn dependencies(&self) -> Vec<std::any::TypeId> {
        vec![]
    }

    fn new_plugin(&self, plugins: &Vec<Rc<RefCell<dyn Plugin>>>) -> Result<Rc<RefCell<dyn Plugin>>, PluginError>;
}

pub trait Plugin: Any + AsAnyPlugin {
    fn run(&mut self);
    fn shutdown(&self);
}

impl<T: Plugin + 'static> AsAnyPlugin for T {
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

impl<T: Specification + 'static> AsAnySpecification for T {
    fn as_any(self: &Self) -> &dyn Any {
        self
    }
}

pub fn get_dep<T: Plugin>(deps: &Vec<Rc<RefCell<dyn Plugin>>>) -> Result<Rc<RefCell<T>>, PluginError> {
    let type_id = TypeId::of::<T>();
    let first_match: Option<&Rc<RefCell<dyn Plugin>>> = deps.iter().find(|&plugin| plugin.as_ref().type_id() == type_id);
    match first_match {
        None => Err("cannot get dependency"),
        Some(first_match) => {
            let cloned: Rc<RefCell<T>> = Rc::clone(first_match);
            let dependency = cloned.as_any().downcast::<RefCell<T>>().expect("Cannot downcast dependency");
            Ok(dependency)
        }
    }
}
