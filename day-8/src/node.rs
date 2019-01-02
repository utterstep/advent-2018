use std::{error::Error, fmt};

#[derive(Debug)]
pub(crate) struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
    value: Option<usize>,
}

#[derive(Debug)]
pub(crate) struct TooFewElementsError;

impl fmt::Display for TooFewElementsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong tree format: too few elements was specified")
    }
}

impl Error for TooFewElementsError {}

macro_rules! read_header {
    ($iter: expr) => {
        (
            $iter.next().ok_or(TooFewElementsError)?,
            $iter.next().ok_or(TooFewElementsError)?,
        )
    };
}

impl Node {
    fn new(children_count: usize, metadata_count: usize) -> Self {
        Self {
            children: Vec::with_capacity(children_count),
            metadata: Vec::with_capacity(metadata_count),
            value: None,
        }
    }

    pub(crate) fn from_iter(
        iter: &mut impl Iterator<Item = usize>,
    ) -> Result<Self, TooFewElementsError> {
        let (children, metadata) = read_header!(iter);

        let mut node = Self::new(children, metadata);
        node.parse_children(iter)?;
        node.parse_metadata(iter)?;

        Ok(node)
    }

    fn parse_children(
        &mut self,
        iter: &mut impl Iterator<Item = usize>,
    ) -> Result<(), TooFewElementsError> {
        for _ in 0..self.children.capacity() {
            self.children.push(Self::from_iter(iter)?);
        }

        Ok(())
    }

    fn parse_metadata(
        &mut self,
        iter: &mut impl Iterator<Item = usize>,
    ) -> Result<(), TooFewElementsError> {
        self.metadata.extend(iter.take(self.metadata.capacity()));

        if self.metadata.len() < self.metadata.capacity() {
            return Err(TooFewElementsError);
        }

        Ok(())
    }

    pub(crate) fn checksum(&self) -> usize {
        self.into_iter()
            .map(|node| node.metadata.iter().sum::<usize>())
            .sum()
    }

    pub(crate) fn value(&mut self) -> usize {
        if let Some(value) = self.value {
            return value;
        };

        let metadata = &self.metadata;
        let children = &mut self.children;

        let value = if children.is_empty() {
            metadata.iter().sum()
        } else {
            metadata
                .iter()
                .map(|idx| children.get_mut(*idx - 1).map_or(0, |child| child.value()))
                .sum()
        };

        self.value.replace(value);

        value
    }
}

impl<'a> IntoIterator for &'a Node {
    type Item = &'a Node;
    type IntoIter = NodeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            to_iterate: vec![self],
        }
    }
}

#[derive(Debug)]
pub(crate) struct NodeIterator<'a> {
    to_iterate: Vec<&'a Node>,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.to_iterate.pop();

        if let Some(node) = node {
            self.to_iterate.extend(node.children.iter());
        }

        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_parser() {
        let mut input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"
            .split_whitespace()
            .map(|n| n.parse().unwrap());

        let node = Node::from_iter(&mut input);
        assert!(node.is_ok());
        let node = node.unwrap();

        assert_eq!(node.children.len(), 2);
        assert_eq!(node.metadata, vec![1, 1, 2]);

        let node_b = &node.children[0];
        let node_c = &node.children[1];

        assert!(node_b.children.is_empty());
        assert_eq!(node_b.metadata, vec![10, 11, 12]);

        assert_eq!(node_c.children.len(), 1);
        assert_eq!(node_c.metadata, vec![2]);

        let node_d = &node_c.children[0];
        assert!(node_d.children.is_empty());
        assert_eq!(node_d.metadata, vec![99]);
    }

    #[test]
    fn test_node_iter() {
        let mut input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"
            .split_whitespace()
            .map(|n| n.parse().unwrap());

        let node = Node::from_iter(&mut input);
        assert!(node.is_ok());
        let node = node.unwrap();

        let nodes = node.into_iter().collect::<Vec<_>>();
        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[0].children.len(), 2);
        assert_eq!(nodes[0].metadata, vec![1, 1, 2]);

        assert_eq!(nodes[1].children.len(), 1);
        assert_eq!(nodes[1].metadata, vec![2]);

        assert!(nodes[2].children.is_empty());
        assert_eq!(nodes[2].metadata, vec![99]);

        assert!(nodes[3].children.is_empty());
        assert_eq!(nodes[3].metadata, vec![10, 11, 12]);
    }

    #[test]
    fn test_node_checksum() {
        let mut input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"
            .split_whitespace()
            .map(|n| n.parse().unwrap());

        let node = Node::from_iter(&mut input);
        assert!(node.is_ok());
        let node = node.unwrap();

        assert_eq!(node.checksum(), 138);
    }

    #[test]
    fn test_wrong_format() {
        let mut input = "3 0".split_whitespace().map(|n| n.parse().unwrap());

        assert!(Node::from_iter(&mut input).is_err());

        // incomplete root metadata
        let mut input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1"
            .split_whitespace()
            .map(|n| n.parse().unwrap());

        assert!(Node::from_iter(&mut input).is_err());
    }

    #[test]
    fn test_value() {
        let mut input = "0 3 1 2 3".split_whitespace().map(|n| n.parse().unwrap());

        let node = Node::from_iter(&mut input);
        assert!(node.is_ok());
        let mut node = node.unwrap();

        assert_eq!(node.value(), 6);

        let mut input = "2 3 0 2 1 2 0 3 5 6 7 1 2 3"
            .split_whitespace()
            .map(|n| n.parse().unwrap());

        let node = Node::from_iter(&mut input);
        assert!(node.is_ok());
        let mut node = node.unwrap();

        assert_eq!(node.value(), 21);
    }

    #[test]
    fn test_value_cache() {
        let mut input = "2 3 0 2 1 2 0 3 5 6 7 1 2 3"
            .split_whitespace()
            .map(|n| n.parse().unwrap());

        let node = Node::from_iter(&mut input);
        assert!(node.is_ok());
        let mut node = node.unwrap();

        assert!(node.value.is_none());
        assert_eq!(node.value(), 21);
        assert_eq!(node.value, Some(21));

        assert_eq!(node.value(), 21);
        assert_eq!(node.value, Some(21));
    }
}
