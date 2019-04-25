pub trait Specification {
    //type Plugin: Plugin;
    fn name(&self) -> &'static str;
    fn id(&self) -> std::any::TypeId;
    fn dependencies(&self) -> Vec<std::any::TypeId> {
        vec![]
    }
}

pub trait Plugin {
    type Specification: Specification;

    fn new(deps: Self::Specification) -> Self;
    fn specification() -> Self::Specification;

    fn name(&self) -> &'static str {
        Self::specification().name()
    }
}
