use crate::board::Board;
use crate::hex::AxialPos;
use crate::prng::TileDeckGenerator;
use crate::tile::Tile;

pub struct GameState {
    pub seed: u64,
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
    pub fn new(seed: u64, initial_deck_size: u32) -> Self {
        let mut deck_generator = TileDeckGenerator::new(seed);
        let current_tile = deck_generator.draw_next_tile();
        let next_tile = deck_generator.draw_next_tile();

        Self {
            seed,
            deck_generator,
            board: Board::new(),
            current_tile,
            next_tile,
            tiles_remaining: initial_deck_size,
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

        if let Some(res) = self.board.place_tile(pos, self.current_tile.clone()) {
            self.score += res.points_awarded;
            self.tiles_placed_count += 1;
            self.tiles_remaining = self.tiles_remaining.saturating_sub(1) + res.tiles_added_to_deck;

            if res.is_perfect {
                self.perfect_count += 1;
            }
            self.quests_completed += res.quests_completed;
            self.flags_completed += res.flags_completed;

            // Advance to next tile
            self.current_tile = self.next_tile.clone();
            self.next_tile = self.deck_generator.draw_next_tile();

            if self.tiles_remaining == 0 {
                self.game_over = true;
            }

            true
        } else {
            false
        }
    }
}
