use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Instruction {
    name: char,
    outbound: Vec<char>,
    inbound: Vec<char>,
}

#[derive(Debug)]
pub(crate) struct InstructionGraph {
    nodes: HashMap<char, Instruction>,
}

impl Instruction {
    pub fn new(name: char) -> Self {
        Self {
            name,
            outbound: Vec::new(),
            inbound: Vec::new(),
        }
    }

    pub fn add_new_link(&mut self, to: char) {
        self.outbound.push(to)
    }

    pub fn set_incoming(&mut self, from: char) {
        self.inbound.push(from);
    }
}

impl InstructionGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub(crate) fn add_link(&mut self, from: char, to: char) {
        self.nodes.entry(to).or_insert_with(|| Instruction::new(to)).set_incoming(from);
        let from_node = self.nodes.entry(from).or_insert_with(|| Instruction::new(from));

        from_node.add_new_link(to)
    }

    pub(crate) fn get_starting_nodes<'a>(&'a self) -> impl Iterator<Item = char> + 'a {
        self.nodes
            .iter()
            .filter_map(|(c, node)| {
                if node.inbound.is_empty() {
                    Some(*c)
                } else {
                    None
                }
            })
    }

    pub(crate) fn visit_node(&mut self, node: char) -> Vec<char> {
        let instruction = self.nodes.remove(&node).unwrap();

        instruction.outbound.into_iter()
            .filter(|to| {
                let dest = self.nodes.get_mut(to).unwrap();

                dest.inbound.retain(|from| from != &node);
                dest.inbound.is_empty()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_starting_node() {
        let mut graph = InstructionGraph::new();

        graph.add_link('C', 'F');
        graph.add_link('C', 'B');
        graph.add_link('B', 'G');

        assert_eq!(graph.get_starting_nodes().collect::<Vec<_>>(), vec!['C']);
    }
}
