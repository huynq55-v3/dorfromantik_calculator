use crate::board::Board;
use crate::hex::AxialPos;
use crate::prng::{CustomRules, TileDeckGenerator};
use crate::tile::Tile;

pub struct GameState {
    pub seed: u64,
    pub rules: CustomRules,
    pub deck_generator: TileDeckGenerator,
    pub board: Board,
    pub current_tile: Tile,
    pub next_tile: Tile,
    pub tiles_remaining: u32,
    pub score: u32,
    pub perfect_count: u32,
    pub quests_completed: u32,
    pub flags_completed: u32,
    pub tiles_placed_count: u32,
    pub game_over: bool,
}

impl GameState {
    pub fn new(seed: u64, rules: CustomRules) -> Self {
        let mut deck_generator = TileDeckGenerator::new(seed, rules.clone());
        let current_tile = deck_generator.draw_next_tile();
        let next_tile = deck_generator.draw_next_tile();

        let initial_tiles = rules.tile_limit.unwrap_or(40);

        Self {
            seed,
            rules,
            deck_generator,
            board: Board::new(),
            current_tile,
            next_tile,
            tiles_remaining: initial_tiles,
            score: 0,
            perfect_count: 0,
            quests_completed: 0,
            flags_completed: 0,
            tiles_placed_count: 0,
            game_over: false,
        }
    }

    pub fn rotate_current_tile(&mut self) {
        self.current_tile.rotate_cw();
    }

    pub fn place_current_tile(&mut self, pos: AxialPos) -> bool {
        if self.game_over || self.tiles_remaining == 0 {
            return false;
        }

        // Enforce hard Tile Limit if configured
        if let Some(limit) = self.rules.tile_limit {
            if self.tiles_placed_count >= limit {
                self.game_over = true;
                return false;
            }
        }

        if let Some(res) = self.board.place_tile(pos, self.current_tile.clone()) {
            self.score += res.points_awarded;
            self.tiles_placed_count += 1;

            // If hard tile limit mode, remaining = limit - placed
            if let Some(limit) = self.rules.tile_limit {
                self.tiles_remaining = limit.saturating_sub(self.tiles_placed_count);
                if self.tiles_placed_count >= limit {
                    self.game_over = true;
                }
            } else {
                // Classic mode: Add bonus tiles to remaining stack
                self.tiles_remaining = self.tiles_remaining.saturating_sub(1) + res.tiles_added_to_deck;
                if self.tiles_remaining == 0 {
                    self.game_over = true;
                }
            }

            if res.is_perfect {
                self.perfect_count += 1;
            }
            self.quests_completed += res.quests_completed;
            self.flags_completed += res.flags_completed;

            // Advance to next tile deterministically
            self.current_tile = self.next_tile.clone();
            self.next_tile = self.deck_generator.draw_next_tile();

            true
        } else {
            false
        }
    }
}
