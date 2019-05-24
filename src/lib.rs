use std::ops::{Deref, DerefMut};
use std::any::{Any, TypeId};

trait Plugin : Any + AsAnyPlugin {
    fn run(&mut self);
    fn shutdown(&mut self);
}

trait AsAnyPlugin {
    fn as_any(self: &Self) -> &dyn Any;
    fn as_any_mut(self: &mut Self) -> &mut dyn Any;
}

impl<T: Plugin + 'static> AsAnyPlugin for T {
    fn as_any(self: &Self) -> &dyn Any {
        self
    }
    fn as_any_mut(self: &mut Self) -> &mut dyn Any {
        self
    }
}

enum MyPlugin {
    Web(Web),
    Logging(Logging),
}

impl MyPlugin {
    fn run(&mut self, logger: &mut Box<dyn LogWriter>) {
        logger.log_raw("BEFORE casted run".to_owned());
        self.as_plugin_mut().run();
        logger.log_raw("AFTER casted run".to_owned());
    }
    fn shutdown(&mut self, logger: &mut Box<dyn LogWriter>) {
        logger.log_raw("BEFORE casted shutdown".to_owned());
        self.as_plugin_mut().shutdown();
        logger.log_raw("AFTER casted shutdown".to_owned());
    }

    // match boilerplate is centralized here
    // Avoidable and optimizable with crates such as "enum_dispatch"
    fn as_plugin_mut(&mut self) -> &mut dyn Plugin {
        match self {
            MyPlugin::Web(plugin) => plugin,
            MyPlugin::Logging(plugin) => plugin,
        }
    }
    fn as_plugin(&mut self) -> &dyn Plugin {
        self.as_plugin_mut()
    }
    fn plugin_id(&mut self) -> TypeId {
        let plugin = self.as_plugin_mut();
        let any_plugin = plugin.as_any();
        any_plugin.type_id()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct PluginList(Vec<MyPlugin>);

impl PluginList {
    fn new() -> Self {
        Self (
            vec![],
        )
    }
    fn register(&mut self, mut my_plugin: MyPlugin) {
        let plugin_id = my_plugin.plugin_id();
        self.0.push(my_plugin);
    }
    fn get_plugin<'a, T: Plugin + 'static + 'a>(&'a mut self) -> Option<&'a mut T> {
        let type_id = TypeId::of::<T>();
        for my_plugin in self.0.iter_mut() {
            if my_plugin.plugin_id() == type_id {
                let mut plugin = my_plugin.as_plugin_mut();
                return plugin.as_any_mut().downcast_mut::<T>();
            }
        }
        None
    }
    fn iter_rev_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut MyPlugin> {
        self.0.iter_mut().rev()
    }
}

impl Deref for PluginList {
    type Target = Vec<MyPlugin>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PluginList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
