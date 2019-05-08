use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;

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

    fn new_plugin(&self, plugins: &Vec<Rc<dyn Plugin>>) -> Result<Rc<dyn Plugin>, PluginError>;
}

pub trait Plugin {
    fn as_any(&self) -> &dyn Any;

    fn run(&self);

    fn shutdown(&self);
}

pub fn get_dep<T: 'static>(deps: &Vec<Rc<dyn Plugin>>) -> Result<Rc<T>, PluginError> {
    println!("get dep {:?}", TypeId::of::<T>());
    for (idx, plugin) in deps.iter().enumerate() {
        if TypeId::of::<T>() == plugin.as_any().type_id() {
            match plugin.as_any().downcast_ref::<Rc<T>>() {
                None => return Err("could not downcast"),
                Some(plugin) => {
                    return Ok(plugin.clone());
                }
            }
        }
    }
    Err("cannot get dependency")
}
