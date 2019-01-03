use intrusive_collections::{intrusive_adapter, LinkedListLink};

#[derive(Debug, Default)]
pub struct Marble {
    link: LinkedListLink,
    pub(super) value: i64,
}

impl Marble {
    fn new(value: i64) -> Self {
        let mut marble: Self = Default::default();
        marble.value = value;

        marble
    }

    pub(super) fn new_node(value: i64) -> Box<Self> {
        Box::new(Self::new(value))
    }
}

intrusive_adapter!(pub MarbleAdapter = Box<Marble>: Marble { link: LinkedListLink });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marble_impl() {
        let marble = Marble::new(5);
        assert_eq!(marble.value, 5);

        let marble_node = Marble::new_node(7);
        assert_eq!(marble_node.value, 7);
    }
}
