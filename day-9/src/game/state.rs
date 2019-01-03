use intrusive_collections::{linked_list::CursorMut, LinkedList};

use super::marble::{Marble, MarbleAdapter};

const WEIRD_FREQUENCY: i64 = 23;
const WEIRD_DELTA: i64 = 7;

macro_rules! weird_move {
    ($cursor: expr) => {{
        for _ in 0..WEIRD_DELTA {
            move_wrapping!(prev, $cursor);
        }

        $cursor.remove().map(|marble| marble.value)
    }};
}

macro_rules! add_marble {
    ($cursor: expr, $value: expr) => {
        move_wrapping!(next, $cursor);
        $cursor.insert_after(Marble::new_node($value));
        move_wrapping!(next, $cursor);
    };
}

macro_rules! move_wrapping {
    (prev, $cursor: expr) => {
        call_method_wrapping!(CursorMut::move_prev, $cursor);
    };
    (next, $cursor: expr) => {
        call_method_wrapping!(CursorMut::move_next, $cursor);
    };
}

macro_rules! call_method_wrapping {
    ($method: path, $cursor: expr) => {
        $method(&mut $cursor);
        if $cursor.is_null() {
            $method(&mut $cursor);
        }
    };
}

pub(crate) struct GameState {
    players: Vec<i64>,
    #[allow(dead_code)]
    marbles: LinkedList<MarbleAdapter>,
}

impl GameState {
    pub fn produce(n_players: usize, max_marble: i64) -> Self {
        let mut marbles = LinkedList::new(MarbleAdapter::new());
        marbles.push_front(Marble::new_node(0));
        let mut cursor = marbles.cursor_mut();

        let mut players = vec![0; n_players];
        let mut current_player = 0;

        for marble in 1..=max_marble {
            if marble % WEIRD_FREQUENCY == 0 {
                players[current_player] += marble + weird_move!(cursor).unwrap();
            } else {
                add_marble!(cursor, marble);
            }

            current_player = (current_player + 1) % n_players;
        }

        Self { marbles, players }
    }

    pub fn high_score(&self) -> Option<i64> {
        self.players.iter().max().cloned()
    }
}

#[cfg(test)]
mod macro_tests {
    use super::*;

    #[test]
    fn test_move_wrapping_macro() {
        let mut list = LinkedList::new(MarbleAdapter::new());

        let marble = Marble::new_node(0);
        list.push_front(marble);

        let mut cursor = list.front_mut();
        assert_eq!(cursor.get().unwrap().value, 0);

        move_wrapping!(prev, cursor);
        assert_eq!(cursor.get().unwrap().value, 0);

        move_wrapping!(next, cursor);
        assert_eq!(cursor.get().unwrap().value, 0);

        let marble = Marble::new_node(1);
        cursor.insert_after(marble);
        assert_eq!(cursor.get().unwrap().value, 0);

        move_wrapping!(next, cursor);
        assert_eq!(cursor.get().unwrap().value, 1);

        move_wrapping!(next, cursor);
        assert_eq!(cursor.get().unwrap().value, 0);

        move_wrapping!(next, cursor);
        assert_eq!(cursor.get().unwrap().value, 1);

        move_wrapping!(prev, cursor);
        assert_eq!(cursor.get().unwrap().value, 0);

        move_wrapping!(prev, cursor);
        assert_eq!(cursor.get().unwrap().value, 1);

        move_wrapping!(prev, cursor);
        assert_eq!(cursor.get().unwrap().value, 0);
    }

    #[test]
    fn test_add_marble_macro() {
        let mut list = LinkedList::new(MarbleAdapter::new());

        let marble = Marble::new_node(0);
        list.push_front(marble);

        let mut cursor = list.front_mut();

        for i in 1..23 {
            add_marble!(cursor, i);
            assert_eq!(cursor.get().unwrap().value, i);
        }

        assert_eq!(
            list.iter().map(|x| x.value).collect::<Vec<_>>(),
            vec![0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
    }

    #[test]
    fn test_weird_move_macro() {
        let mut list = LinkedList::new(MarbleAdapter::new());

        let marble = Marble::new_node(0);
        list.push_front(marble);

        let mut cursor = list.front_mut();

        for i in 1..23 {
            add_marble!(cursor, i);
            assert_eq!(cursor.get().unwrap().value, i);
        }

        let removed = weird_move!(cursor);

        assert_eq!(cursor.get().unwrap().value, 19);
        assert_eq!(removed, Some(9));
        assert_eq!(
            list.iter().map(|x| x.value).collect::<Vec<_>>(),
            vec![0, 16, 8, 17, 4, 18, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_produce() {
        let game_state = GameState::produce(9, 25);
        assert_eq!(
            game_state
                .marbles
                .iter()
                .map(|x| x.value)
                .collect::<Vec<_>>(),
            vec![
                0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 25, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7,
                15
            ]
        );
        assert_eq!(game_state.players, vec![0, 0, 0, 0, 32, 0, 0, 0, 0]);
        assert_eq!(game_state.high_score(), Some(32));
    }

    #[test]
    fn test_game_high_score_examples() {
        macro_rules! check_high_score_expample {
            ($players: expr, $limit: expr, $expected: expr) => {
                let game_state = GameState::produce($players, $limit);
                assert_eq!(game_state.high_score(), Some($expected));
            };
        }

        check_high_score_expample!(9, 25, 32);
        check_high_score_expample!(10, 1618, 8317);
        check_high_score_expample!(13, 7999, 146373);
        check_high_score_expample!(17, 1104, 2764);
        check_high_score_expample!(21, 6111, 54718);
        check_high_score_expample!(30, 5807, 37305);
    }
}
