use bevy::prelude::*;
use rand::Rng;
use rand::seq::IndexedRandom;

pub const GRID: usize = 4;

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Game>();
    }
}

#[derive(Resource)]
pub struct Game {
    pub board: [[u32; GRID]; GRID],
    pub score: u32,
    pub best_score: u32,
    pub won: bool,
    pub game_over: bool,
    // Incremented after valid moves and resets so the UI can trigger short animations.
    pub revision: u64,
    pub motions: Vec<TileMotion>,
    pub spawned_cell: Option<(usize, usize)>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            board: [[0; GRID]; GRID],
            score: 0,
            best_score: 0,
            won: false,
            game_over: false,
            revision: 0,
            motions: Vec::new(),
            spawned_cell: None,
        };
        game.spawn_random_tile();
        game.spawn_random_tile();
        game
    }

    pub fn reset(&mut self) {
        // Keep the best score across rounds while rebuilding the board from scratch.
        let best_score = self.best_score.max(self.score);
        *self = Self {
            board: [[0; GRID]; GRID],
            score: 0,
            best_score,
            won: false,
            game_over: false,
            revision: self.revision + 1,
            motions: Vec::new(),
            spawned_cell: None,
        };
        self.spawn_random_tile();
        self.spawned_cell = self.spawn_random_tile();
    }

    pub fn make_move(&mut self, direction: Direction) -> bool {
        if self.game_over {
            return false;
        }

        let old_board = self.board;
        let mut new_board = [[0; GRID]; GRID];
        let mut motions = Vec::new();
        let mut gained_score = 0;

        match direction {
            Direction::Left => {
                for row in 0..GRID {
                    gained_score += move_line(
                        &old_board,
                        [(row, 0), (row, 1), (row, 2), (row, 3)],
                        &mut new_board,
                        &mut motions,
                    );
                }
            }
            Direction::Right => {
                for row in 0..GRID {
                    gained_score += move_line(
                        &old_board,
                        [(row, 3), (row, 2), (row, 1), (row, 0)],
                        &mut new_board,
                        &mut motions,
                    );
                }
            }
            Direction::Up => {
                for col in 0..GRID {
                    gained_score += move_line(
                        &old_board,
                        [(0, col), (1, col), (2, col), (3, col)],
                        &mut new_board,
                        &mut motions,
                    );
                }
            }
            Direction::Down => {
                for col in 0..GRID {
                    gained_score += move_line(
                        &old_board,
                        [(3, col), (2, col), (1, col), (0, col)],
                        &mut new_board,
                        &mut motions,
                    );
                }
            }
        }

        self.board = new_board;

        if old_board == self.board {
            return false;
        }

        // A valid move updates score, spawns exactly one new tile, then checks terminal state.
        self.score += gained_score;
        self.best_score = self.best_score.max(self.score);
        self.won = self.won || self.board.iter().flatten().any(|value| *value >= 2048);
        self.spawned_cell = self.spawn_random_tile();
        self.game_over = !self.can_move();
        self.revision += 1;
        self.motions = motions;
        true
    }

    pub fn can_move(&self) -> bool {
        for row in 0..GRID {
            for col in 0..GRID {
                let value = self.board[row][col];
                if value == 0
                    || row + 1 < GRID && self.board[row + 1][col] == value
                    || col + 1 < GRID && self.board[row][col + 1] == value
                {
                    return true;
                }
            }
        }
        false
    }

    fn spawn_random_tile(&mut self) -> Option<(usize, usize)> {
        let empty_cells = self
            .board
            .iter()
            .enumerate()
            .flat_map(|(row, cells)| {
                cells
                    .iter()
                    .enumerate()
                    .filter(|(_, value)| **value == 0)
                    .map(move |(col, _)| (row, col))
            })
            .collect::<Vec<_>>();

        if let Some(&(row, col)) = empty_cells.choose(&mut rand::rng()) {
            let value = if rand::rng().random_bool(0.9) { 2 } else { 4 };
            self.board[row][col] = value;
            Some((row, col))
        } else {
            None
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Debug)]
pub struct TileMotion {
    pub value: u32,
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub merged: bool,
}

fn move_line(
    board: &[[u32; GRID]; GRID],
    positions: [(usize, usize); GRID],
    new_board: &mut [[u32; GRID]; GRID],
    motions: &mut Vec<TileMotion>,
) -> u32 {
    let tiles = positions
        .iter()
        .copied()
        .filter_map(|position| {
            let value = board[position.0][position.1];
            (value != 0).then_some((position, value))
        })
        .collect::<Vec<_>>();

    let mut score = 0;
    let mut target_index = 0;
    let mut index = 0;

    while index < tiles.len() {
        let target = positions[target_index];
        let (from, value) = tiles[index];

        if index + 1 < tiles.len() && value == tiles[index + 1].1 {
            let merged_value = value * 2;
            new_board[target.0][target.1] = merged_value;
            score += merged_value;

            motions.push(TileMotion {
                value,
                from,
                to: target,
                merged: true,
            });
            motions.push(TileMotion {
                value,
                from: tiles[index + 1].0,
                to: target,
                merged: true,
            });

            index += 2;
        } else {
            new_board[target.0][target.1] = value;
            motions.push(TileMotion {
                value,
                from,
                to: target,
                merged: false,
            });

            index += 1;
        }

        target_index += 1;
    }

    score
}

#[cfg(test)]
fn slide_and_merge(line: [u32; GRID]) -> ([u32; GRID], u32) {
    // 2048 merges each adjacent equal pair once per move, then pads empty cells at the end.
    let values = line
        .into_iter()
        .filter(|value| *value != 0)
        .collect::<Vec<_>>();
    let mut merged = Vec::with_capacity(GRID);
    let mut score = 0;
    let mut index = 0;

    while index < values.len() {
        if index + 1 < values.len() && values[index] == values[index + 1] {
            let value = values[index] * 2;
            merged.push(value);
            score += value;
            index += 2;
        } else {
            merged.push(values[index]);
            index += 1;
        }
    }

    while merged.len() < GRID {
        merged.push(0);
    }

    ([merged[0], merged[1], merged[2], merged[3]], score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slide_moves_tiles_into_empty_space() {
        let (line, score) = slide_and_merge([0, 2, 0, 4]);

        assert_eq!(line, [2, 4, 0, 0]);
        assert_eq!(score, 0);
    }

    #[test]
    fn slide_merges_each_pair_once() {
        let (line, score) = slide_and_merge([2, 2, 2, 2]);

        assert_eq!(line, [4, 4, 0, 0]);
        assert_eq!(score, 8);
    }

    #[test]
    fn slide_keeps_cascaded_merge_for_next_turn() {
        let (line, score) = slide_and_merge([4, 4, 8, 0]);

        assert_eq!(line, [8, 8, 0, 0]);
        assert_eq!(score, 8);
    }

    #[test]
    fn board_can_move_when_empty_cell_exists() {
        let game = Game {
            board: [[2, 4, 2, 4], [4, 2, 4, 2], [2, 4, 0, 4], [4, 2, 4, 2]],
            score: 0,
            best_score: 0,
            won: false,
            game_over: false,
            revision: 0,
            motions: Vec::new(),
            spawned_cell: None,
        };

        assert!(game.can_move());
    }

    #[test]
    fn board_can_move_when_neighbor_matches() {
        let game = Game {
            board: [[2, 4, 2, 4], [4, 2, 4, 2], [2, 4, 8, 8], [4, 2, 4, 2]],
            score: 0,
            best_score: 0,
            won: false,
            game_over: false,
            revision: 0,
            motions: Vec::new(),
            spawned_cell: None,
        };

        assert!(game.can_move());
    }

    #[test]
    fn board_cannot_move_when_full_and_no_matches() {
        let game = Game {
            board: [[2, 4, 2, 4], [4, 2, 4, 2], [2, 4, 2, 4], [4, 2, 4, 2]],
            score: 0,
            best_score: 0,
            won: false,
            game_over: false,
            revision: 0,
            motions: Vec::new(),
            spawned_cell: None,
        };

        assert!(!game.can_move());
    }
}
