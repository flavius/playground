use std::any::Any;

pub trait Specification {
    fn new() -> Self where Self: Sized;
    //type Plugin: Plugin;
    fn name(&self) -> &'static str;
    fn id(&self) -> std::any::TypeId;
    fn dependencies(&self) -> Vec<std::any::TypeId> {
        vec![]
    }
    fn as_any(&self) -> &dyn Any;
}

pub trait Plugin {
}

//TODO: auto impl of Debug for Specification and Plugin
