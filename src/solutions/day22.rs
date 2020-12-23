use crate::solution::Solution;
use std::collections::{VecDeque, HashSet, HashMap};

pub struct Day;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Player {
    Player1,
    Player2,
}

type GameState = (VecDeque<usize>, VecDeque<usize>);
type GameResult = (Player, Option<VecDeque<usize>>);

fn game(mut state: GameState, cache: &mut HashMap<GameState, GameResult>, root: bool) -> GameResult {
    // if cache.len() % 100 == 0 {
        // println!("cache size: {}", cache.len());
        // println!("cache: {:?}", cache);
    // }
    // if let Some((winner, winner_cards)) = cache.get(&state) {
    //     return (*winner, winner_cards.clone());
    // }

    let original_state = state.clone();
    let mut previous_positions: HashSet<GameState> = HashSet::new();
    while state.0.len() > 0 && state.1.len() > 0 {
        if previous_positions.contains(&state) {
            // for position in previous_positions {
            //     cache.insert(position, (Player::Player1, None));
            // }
            if root {
                return (Player::Player1, Some(state.0));
            } else {
                return (Player::Player1, None);
            }
        }
        previous_positions.insert(state.clone());

        let cards = (state.0.pop_front().unwrap(), state.1.pop_front().unwrap());
        if cards.0 <= state.0.len() && cards.1 <= state.1.len() {
            let vec1: Vec<usize> = state.0.clone().into_iter().collect();
            let slice1 = vec1[..cards.0].to_vec();

            let vec2: Vec<usize> = state.1.clone().into_iter().collect();
            let slice2 = vec2[..cards.1].to_vec();

            let (winner, _) = game((VecDeque::from(slice1), VecDeque::from(slice2)), cache, false);

            if winner == Player::Player1 {
                state.0.push_back(cards.0);
                state.0.push_back(cards.1);
            } else {
                state.1.push_back(cards.1);
                state.1.push_back(cards.0);
            }
        } else if cards.0 > cards.1 {
            state.0.push_back(cards.0);
            state.0.push_back(cards.1);
        } else {
            state.1.push_back(cards.1);
            state.1.push_back(cards.0);
        }
    }

    let winner = match (state.0.len(), state.1.len(), root) {
        (_, 0, true) => (Player::Player1, Some(state.0)),
        (0, _, true) => (Player::Player2, Some(state.1)),
        (_, 0, false) => (Player::Player1, None),
        (0, _, false) => (Player::Player2, None),
        _ => panic!("something went wrong"),
    };

    // cache.insert(original_state, winner.clone());
    // for position in previous_positions {
    //     cache.insert(position, winner.clone());
    // }
    winner
}

impl Solution for Day {
    type Input = (VecDeque<usize>, VecDeque<usize>);
    type Output1 = i32;
    type Output2 = i32;

    fn get_input_file_path(&self) -> String {
        "input/day22".to_string()
    }

    fn parse_input(&self, puzzle_input: String) -> Self::Input {
        let players: Vec<VecDeque<usize>> = puzzle_input.split("\n\n").map(|p| {
            p.lines().skip(1).map(|n| n.parse().unwrap()).collect()
        }).collect();

        (players[0].clone(), players[1].clone())
    }

    fn part1(&self, input: &Self::Input) -> Self::Output1 {
        4
    }

    fn part2(&self, input: &Self::Input) -> Self::Output2 {
        println!("{:?}", input);
        let mut cache = HashMap::new();
        let (_, cards) = game(input.clone(), &mut cache, true);
        println!("{:?}", cards);

        let unwrapped = cards.unwrap();

        unwrapped.iter().enumerate().fold(0, |acc, (idx, card)| {
            acc + *card as i32 * (unwrapped.len() as i32 - idx as i32) as i32
         })
    }
}
