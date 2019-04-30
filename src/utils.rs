use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;
use std::fmt;

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

    fn add_dependency(&mut self, what: usize, dependency: usize) -> bool {
        if what >= self.plugin_count {
            return false;
        }
        if dependency >= self.plugin_count {
            return false;
        }
        if what == dependency {
            return false;
        }
        if let Some(mut path) = self.path(dependency, what) {
            //println!("\tcycle detected from {} to {}: {:?}", dependency, what, path);
            return false;
        }
        let pos = what * self.plugin_count + dependency;
        self.adjacency_matrix[pos] = true;
        //println!("\tplugin {} depends on {}", what, dependency);
        true
    }

    fn path(&self, from: usize, to: usize) -> Option<Vec<usize>> {
        //println!("\t\t=== path from {} to {}", from, to);
        let mut path = vec![];
        let mut visited = vec![false; self.plugin_count];
        visited[from] = true;
        let mut queue = VecDeque::new();
        let mut matrix = self.adjacency_matrix.clone();
        queue.push_back(from);
        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            let neighbours = self.outset(node, &matrix);
            //println!("\t\t neighbours of {} = {:?}", node, neighbours);
            for neighbour in neighbours {
                if visited[neighbour] == false {
                    visited[neighbour] = true;
                    let idx = node * self.plugin_count + neighbour;
                    //println!("remove outgoing edge at {}", idx);
                    matrix[idx] = false;
                    queue.push_back(neighbour);
                    path.push(neighbour);
                }
            }
        }
        //println!("\t\t=== path from {} to {}: {:?}", from, to, path);
        if path.len() > 0 && path.last().unwrap() == &to {
            Some(path)
        } else {
            None
        }
    }

    fn column<'a>(&self, colnum: usize, matrix: impl Into<Option<&'a Vec<bool>>>) -> Vec<bool> {
        let matrix = matrix.into().unwrap_or(&self.adjacency_matrix);
        let mut col = vec![];
        if colnum >= self.plugin_count {
            return col;
        }
        for row in 0..self.plugin_count {
            let idx = row * self.plugin_count + colnum;
            col.push(matrix[idx]);
        }
        col
    }

    fn row<'a>(&self, rownum: usize, matrix: impl Into<Option<&'a Vec<bool>>>) -> Vec<bool> {
        let matrix = matrix.into().unwrap_or(&self.adjacency_matrix);
        let mut row = vec![];
        if rownum >= self.plugin_count {
            return row;
        }
        let start = rownum * self.plugin_count;
        let end = start + self.plugin_count;
        for idx in start..end {
            row.push(matrix[idx]);
        }
        row
    }

    //fn remove_outedges<'a>(&self, rownum: usize, matrix: impl Into<Option<&'a Vec<bool>>>) {
    //    let matrix = matrix.into().unwrap_or(&self.adjacency_matrix);
    //    let start = rownum * self.plugin_count;
    //    let end = start + self.plugin_count;
    //    for idx in start..end {
    //        matrix[idx] = false;
    //    }
    //}

    fn inset<'a>(&self, nodenum: usize, matrix: impl Into<Option<&'a Vec<bool>>>) -> Vec<usize> {
        let candidates = self.column(nodenum, matrix);
        let mut set = vec![];
        for (idx, val) in candidates.iter().enumerate() {
            if *val {
                set.push(idx);
            }
        }
        set
    }

    fn outset<'a>(&self, nodenum: usize, matrix: impl Into<Option<&'a Vec<bool>>>) -> Vec<usize> {
        let candidates = self.row(nodenum, matrix);
        let mut set = vec![];
        for (idx, val) in candidates.iter().enumerate() {
            if *val {
                set.push(idx);
            }
        }
        set
    }

    fn only_outgoing_nodes<'a>(&self, matrix: impl Into<Option<&'a Vec<bool>>>) -> VecDeque<usize> {
        let matrix = matrix.into().unwrap_or(&self.adjacency_matrix);
        let mut nodes_with_indegree_zero = VecDeque::new();
        for nodenum in 0..self.plugin_count {
            let inset = self.inset(nodenum, matrix);
            if inset.len() == 0 {
                nodes_with_indegree_zero.push_back(nodenum);
            }
        }
        nodes_with_indegree_zero
    }

    fn topologically_sorted(&self) -> Vec<usize> {
        let mut sorted = vec![];
        let mut noinedges = self.only_outgoing_nodes(None);
        //println!("only out: {:?}", noinedges);
        let mut matrix = self.adjacency_matrix.clone();
        while noinedges.len() > 0 {
            let head = noinedges.pop_front().unwrap();
            let out_nodes = self.outset(head, &matrix);
            sorted.push(head);
            //println!("out {} to {:?}", head, out_nodes);
            for out_node in out_nodes {
                let idx = head * self.plugin_count + out_node;
                //println!("resetting edge at {} value {}", idx, matrix[idx]);
                matrix[idx] = false;
                if self.inset(out_node, &matrix).len() == 0 {
                    noinedges.push_back(out_node);
                }
            }
        }
        //sorted.reverse();
        sorted
    }

}

impl fmt::Debug for DependencyGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.adjacency_matrix.len() == 0 {
            true => { write!(f, "[]") },
            false => {
                write!(f, "[\n")?;
                for (idx, value) in self.adjacency_matrix.iter().enumerate() {
                    if idx != 0 {
                        write!(f, ",")?;
                        if idx % self.plugin_count == 0 {
                            write!(f, "\n")?;
                        }
                    }
                    write!(f, "{}", if value == &true { 1 } else { 0 })?;
                }
                write!(f, "\n]")
            },
        }
    }
}

pub fn sort_specifications(specs: Vec<&plugin::Specification>) -> Option<Vec<&plugin::Specification>> {
    let mut type_to_index = HashMap::new();
    for (idx, spec) in specs.iter().enumerate() {
        type_to_index.insert(spec.id(), idx);
    }

    let mut graph = match DependencyGraph::new(specs.len()) {
        Some(graph) => { graph },
        None => { return None; },
    };
    for (idx, spec) in specs.iter().enumerate() {
        let deps : Vec<usize> = vec![];
        //println!("plugin {} {}", spec.name(), idx);
        //println!("G: {:?}", graph);
        for dep in spec.dependencies() {
            //println!("DEP: {}", type_to_index[&dep]);
            if !graph.add_dependency(idx, type_to_index[&dep]) {
                return None
            }
            //println!("after dep from {} to {} G: {:?}", idx, type_to_index[&dep], graph);
        }
    }
    let mut init_ids = graph.topologically_sorted();
    init_ids.reverse();
    let mut sorted = vec![];
    for idx in init_ids {
        sorted.push(specs[idx]);
    }
    Some(sorted)
}

pub fn initialize_plugins(specs: Vec<&plugin::Specification>) -> Vec<Box<plugin::Plugin>> {
    let mut plugins = vec![];
    plugins
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty_graph() {
        assert!(DependencyGraph::new(0).is_none());
    }

    #[test]
    fn dependencies() {
        let mut g = DependencyGraph::new(4).unwrap();
        assert_eq!(true, g.add_dependency(1, 0));
        assert_eq!(false, g.add_dependency(1, 1));
        assert_eq!(true, g.add_dependency(1, 2));
        assert_eq!(true, g.add_dependency(0, 2));
        assert_eq!(true, g.add_dependency(0, 2));
        assert_eq!(true, g.add_dependency(2, 3));
        assert_eq!(false, g.add_dependency(3, 0));
    }

    #[test]
    fn topo_sorting() {
        let mut g = DependencyGraph::new(4).unwrap();
        g.add_dependency(1, 0);
        g.add_dependency(1, 2);
        g.add_dependency(0, 2);
        g.add_dependency(2, 3);
        let sorted = g.topologically_sorted();
        assert_eq!(vec![1, 0, 2, 3], sorted);
    }

    #[test]
    fn realistic_deps() {
        let mut g = DependencyGraph::new(4).unwrap();
        assert_eq!(true, g.add_dependency(0, 1));
        assert_eq!(true, g.add_dependency(0, 2));
        assert_eq!(true, g.add_dependency(0, 3));
        assert_eq!(true, g.add_dependency(2, 1));
        assert_eq!(true, g.add_dependency(3, 1));
        assert_eq!(true, g.add_dependency(3, 2));
    }
}
