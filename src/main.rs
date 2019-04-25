use std::collections::HashMap;

trait PluginSpecification {
    //type Plugin: Plugin;
    fn name(&self) -> &'static str;
    fn id(&self) -> std::any::TypeId;
    fn dependencies(&self) -> Vec<std::any::TypeId> {
        vec![]
    }
}

trait Plugin {
    type Specification: PluginSpecification;

    fn new(deps: Self::Specification) -> Self;
    fn specification() -> Self::Specification;

    fn name(&self) -> &'static str {
        Self::specification().name()
    }
}

struct DependencyGraph {
    adjacency_matrix: Vec<bool>,
    plugin_count: usize,
}

impl DependencyGraph {
    fn new(plugin_count: usize) -> Option<DependencyGraph> {
        if plugin_count == 0 {
            return None;
        }
        let size = plugin_count * plugin_count;
        let mut m = Vec::with_capacity(size);
        m.resize(size, false);
        Some(DependencyGraph {
            adjacency_matrix: m,
            plugin_count: plugin_count,
        })
    }

    fn addDependency(&mut self, what: usize, dependency: usize) -> bool {
        if what >= self.plugin_count {
            return false;
        }
        if dependency >= self.plugin_count {
            return false;
        }
        let pos = what * self.plugin_count + dependency;
        self.adjacency_matrix[pos] = true;
        true
    }
}

fn sort_specifications(specs: Vec<&PluginSpecification>) -> Vec<&PluginSpecification> {
    let mut graph = match DependencyGraph::new(specs.len()) {
        Some(graph) => { graph },
        None => { return specs; },
    };
    let mut sorted = vec![];
    let mut type_to_index = HashMap::new();
    for (idx, spec) in specs.iter().enumerate() {
        type_to_index.insert(spec.id(), idx);
    }
    for (idx, spec) in specs.iter().enumerate() {
        let deps : Vec<usize> = vec![];
        println!("plugin {} {} depends on {:?}", spec.name(), idx, deps);
        for dep in spec.dependencies() {
            graph.addDependency(idx, type_to_index[&dep]);
            println!("dep {}", type_to_index[&dep]);
        }
    }
    sorted
}

//----------------------------- web
struct WebPlugin {
}

struct WebPluginSpecification {
}

impl PluginSpecification for WebPluginSpecification {
    //type Plugin = WebPlugin;

    fn name(&self) -> &'static str {
        "web"
    }

    fn id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<WebPlugin>()
    }
    fn dependencies(&self) -> Vec<std::any::TypeId> {
        vec![
            std::any::TypeId::of::<LoggingPlugin>(),
            std::any::TypeId::of::<ProjectorPlugin>(),
            std::any::TypeId::of::<AppendlogPlugin>(),
        ]
    }
}

impl Plugin for WebPlugin {
    type Specification = WebPluginSpecification;

    fn new(deps: Self::Specification) -> Self {
        WebPlugin {
        }
    }

    fn specification() -> Self::Specification {
        WebPluginSpecification {
        }
    }
}

//----------------------------- logging
struct LoggingPlugin {
}

struct LoggingPluginSpecification {
}

impl Plugin for LoggingPlugin {
    type Specification = LoggingPluginSpecification;
    fn new(deps: Self::Specification) -> Self {
        LoggingPlugin {
        }
    }

    fn specification() -> Self::Specification {
        LoggingPluginSpecification {
        }
    }
}

impl PluginSpecification for LoggingPluginSpecification {
    fn name(&self) -> &'static str {
        "logging"
    }

    fn id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<LoggingPlugin>()
    }
}

//----------------------------- appendlog
struct AppendlogPlugin {
}

struct AppendlogPluginSpecification {
}

impl Plugin for AppendlogPlugin {
    type Specification = AppendlogPluginSpecification;
    fn new(deps: Self::Specification) -> Self {
        AppendlogPlugin {
        }
    }

    fn specification() -> Self::Specification {
        AppendlogPluginSpecification {
        }
    }
}

impl PluginSpecification for AppendlogPluginSpecification {
    fn name(&self) -> &'static str {
        "appendlog"
    }

    fn id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<AppendlogPlugin>()
    }
    fn dependencies(&self) -> Vec<std::any::TypeId> {
        vec![
            std::any::TypeId::of::<LoggingPlugin>(),
        ]
    }
}

//----------------------------- projector
struct ProjectorPlugin {
}

struct ProjectorPluginSpecification {
}

impl Plugin for ProjectorPlugin {
    type Specification = ProjectorPluginSpecification;
    fn new(deps: Self::Specification) -> Self {
        ProjectorPlugin {
        }
    }

    fn specification() -> Self::Specification {
        ProjectorPluginSpecification {
        }
    }
}

impl PluginSpecification for ProjectorPluginSpecification {
    fn name(&self) -> &'static str {
        "projector"
    }

    fn id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<ProjectorPlugin>()
    }
    fn dependencies(&self) -> Vec<std::any::TypeId> {
        vec![
            std::any::TypeId::of::<LoggingPlugin>(),
            std::any::TypeId::of::<AppendlogPlugin>(),
        ]
    }
}

fn main() {
    let spec_web = WebPlugin::specification();
    let spec_logging = LoggingPlugin::specification();
    let spec_appendlog = AppendlogPlugin::specification();
    let spec_projector = ProjectorPlugin::specification();
    let deps : Vec<&PluginSpecification> = vec![
        &spec_web,
        &spec_logging,
        &spec_appendlog,
        &spec_projector,
    ];
    let sorted_specs = sort_specifications(deps);
}
