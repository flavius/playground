use std::collections::HashMap;

use crate::plugin;

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

pub fn sort_specifications(specs: Vec<&plugin::Specification>) -> Vec<&plugin::Specification> {
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

