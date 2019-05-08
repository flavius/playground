use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;

pub type PluginError = &'static str;

pub trait AsAny {
    fn as_any(self: Rc<Self>) -> Rc<dyn Any>;
}

pub trait Specification {
    fn new() -> Self
    where
        Self: Sized;
    //type Plugin: Plugin;
    fn name(&self) -> &'static str;
    fn id(&self) -> std::any::TypeId;
    fn dependencies(&self) -> Vec<std::any::TypeId> {
        vec![]
    }
    fn as_any(&self) -> &dyn Any;

    fn new_plugin(&self, plugins: &Vec<Rc<dyn Plugin>>) -> Result<Rc<dyn Plugin>, PluginError>;
}

pub trait Plugin: Any + AsAny {
    fn run(&self);
    fn shutdown(&self);
}

impl<T: Plugin + 'static> AsAny for T {
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

pub fn get_dep<T: Plugin>(deps: &Vec<Rc<dyn Plugin>>) -> Result<Rc<T>, PluginError> {
    let type_id = TypeId::of::<T>();
    let first_match = deps.iter().find(|&plugin| plugin.as_ref().type_id() == type_id);
    match first_match {
        None => Err("cannot get dependency"),
        Some(first_match) => {
            let cloned = Rc::clone(first_match);
            let dependency = cloned.as_any().downcast().expect("Cannot downcast dependency");
            Ok(dependency)
        }
    }
}
