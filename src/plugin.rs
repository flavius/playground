use std::any::{Any, TypeId};
use std::collections::HashMap;


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

    //fn new_plugin(&self, deps: &HashMap<TypeId, Box<dyn Plugin>>) -> Option<Box<dyn Plugin>>;
    fn new_plugin(&self, plugins: &Vec<Box<dyn Plugin>>) -> Result<Box<dyn Plugin>, crate::InfrastructureError>;

    //fn plugin_index(&self, deps: &Vec<Box<dyn Plugin>>) -> Option<usize> {
    //    for (idx, plugin) in deps.iter().enumerate() {
    //        if plugin.as_any().downcast_ref::<T>().is_some() {
    //            return Some(idx);
    //        }
    //    }
    //    None
    //}
}

pub trait Plugin {
    fn as_any(&self) -> &dyn Any;
}

pub fn get_dep<T: 'static>(deps: &Vec<Box<dyn Plugin>>) -> Result<usize, crate::InfrastructureError> {
    for (idx, plugin) in deps.iter().enumerate() {
        if TypeId::of::<T>() == plugin.as_any().type_id() {
            return Ok(idx);
        }
    }
    Err("cannot get dependency")
}
