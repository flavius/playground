use std::any::{Any, TypeId};
use std::collections::HashMap;

pub type PluginError = &'static str;

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

    fn new_plugin(&self, plugins: &Vec<Box<dyn Plugin>>) -> Result<Box<dyn Plugin>, PluginError>;
}

pub trait Plugin {
    fn as_any(&self) -> &dyn Any;

    fn run(&self);

    fn shutdown(&self);
}

pub fn get_dep<T: 'static>(deps: &Vec<Box<dyn Plugin>>) -> Result<usize, PluginError> {
    for (idx, plugin) in deps.iter().enumerate() {
        if TypeId::of::<T>() == plugin.as_any().type_id() {
            return Ok(idx);
        }
    }
    Err("cannot get dependency")
}
