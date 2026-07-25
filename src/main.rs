mod board;
mod game_state;
mod group;
mod hex;
mod prng;
mod tile;

use game_state::GameState;
use hex::AxialPos;
use macroquad::prelude::*;
use tile::{SegmentType, Tile};

fn segment_color(stype: SegmentType) -> Color {
    match stype {
        SegmentType::Grass => Color::from_rgba(136, 192, 87, 255),       // Light Green
        SegmentType::Village => Color::from_rgba(224, 90, 71, 255),      // Terracotta Red
        SegmentType::Forest => Color::from_rgba(45, 106, 79, 255),       // Dark Forest Green
        SegmentType::Agriculture => Color::from_rgba(244, 162, 97, 255), // Golden Yellow
        SegmentType::Water => Color::from_rgba(78, 168, 222, 255),       // Cyan Blue
        SegmentType::Train => Color::from_rgba(92, 77, 125, 255),        // Dark Purple/Charcoal
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dorfromantik Simulator".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

enum AppMode {
    SeedInput,
    Playing,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut mode = AppMode::SeedInput;
    let mut seed_input_text = "12345".to_string();
    let mut game_state: Option<GameState> = None;

    let mut camera_offset = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let mut hex_radius = 45.0f32;
    let mut last_mouse_pos = mouse_position();

    loop {
        clear_background(Color::from_rgba(30, 32, 40, 255));

        match mode {
            AppMode::SeedInput => {
                // Draw Seed Input Screen
                let title = "DORFROMANTIK SIMULATOR";
                draw_text(title, screen_width() / 2.0 - 180.0, 150.0, 32.0, WHITE);

                let label = "Enter Random Seed:";
                draw_text(label, screen_width() / 2.0 - 120.0, 250.0, 24.0, LIGHTGRAY);

                // Handle text input
                while let Some(c) = get_char_pressed() {
                    if c.is_ascii_digit() && seed_input_text.len() < 10 {
                        seed_input_text.push(c);
                    }
                }
                if is_key_pressed(KeyCode::Backspace) {
                    seed_input_text.pop();
                }

                // Input box
                draw_rectangle(
                    screen_width() / 2.0 - 150.0,
                    280.0,
                    300.0,
                    50.0,
                    Color::from_rgba(50, 54, 66, 255),
                );
                draw_rectangle_lines(
                    screen_width() / 2.0 - 150.0,
                    280.0,
                    300.0,
                    50.0,
                    2.0,
                    YELLOW,
                );
                draw_text(
                    &seed_input_text,
                    screen_width() / 2.0 - 130.0,
                    315.0,
                    28.0,
                    GOLD,
                );

                // Start Game Button
                let btn_rect = (screen_width() / 2.0 - 100.0, 370.0, 200.0, 50.0);
                let (mx, my) = mouse_position();
                let hovered = mx >= btn_rect.0
                    && mx <= btn_rect.0 + btn_rect.2
                    && my >= btn_rect.1
                    && my <= btn_rect.1 + btn_rect.3;

                let btn_color = if hovered {
                    Color::from_rgba(70, 160, 100, 255)
                } else {
                    Color::from_rgba(50, 120, 75, 255)
                };

                draw_rectangle(btn_rect.0, btn_rect.1, btn_rect.2, btn_rect.3, btn_color);
                draw_rectangle_lines(btn_rect.0, btn_rect.1, btn_rect.2, btn_rect.3, 2.0, WHITE);
                draw_text("START GAME", btn_rect.0 + 35.0, btn_rect.1 + 32.0, 22.0, WHITE);

                if (hovered && is_mouse_button_pressed(MouseButton::Left))
                    || is_key_pressed(KeyCode::Enter)
                {
                    let seed = seed_input_text.parse::<u64>().unwrap_or(12345);
                    game_state = Some(GameState::new(seed, 40));
                    mode = AppMode::Playing;
                }
            }

            AppMode::Playing => {
                let state = game_state.as_mut().unwrap();

                // 1. Camera Pan & Zoom controls
                let (mx, my) = mouse_position();

                if is_mouse_button_down(MouseButton::Middle)
                    || (is_mouse_button_down(MouseButton::Left) && is_key_down(KeyCode::LeftShift))
                {
                    camera_offset.x += mx - last_mouse_pos.0;
                    camera_offset.y += my - last_mouse_pos.1;
                }
                last_mouse_pos = (mx, my);

                let wheel = mouse_wheel().1;
                if wheel != 0.0 {
                    hex_radius = (hex_radius + wheel * 3.0).clamp(20.0, 120.0);
                }

                // 2. Rotate current tile (Right Click or R key)
                if is_mouse_button_pressed(MouseButton::Right) || is_key_pressed(KeyCode::R) {
                    state.rotate_current_tile();
                }

                // 3. Convert mouse pos to hex grid pos
                let rel_x = mx - camera_offset.x;
                let rel_y = my - camera_offset.y;
                let hovered_hex = AxialPos::from_pixel(rel_x, rel_y, hex_radius);

                // 4. Handle Tile Placement (Left Click)
                if is_mouse_button_pressed(MouseButton::Left)
                    && !is_key_down(KeyCode::LeftShift)
                    && !state.game_over
                {
                    state.place_current_tile(hovered_hex);
                }

                // 5. Draw Placed Hex Grid & Valid Slots
                for (&pos, placed) in &state.board.tiles {
                    let (px, py) = pos.to_pixel(hex_radius);
                    let center = camera_offset + vec2(px, py);
                    draw_hex_tile(center, hex_radius, &placed.tile);
                }

                // Draw valid empty placement slots
                for &vpos in &state.board.valid_slots {
                    let (px, py) = vpos.to_pixel(hex_radius);
                    let center = camera_offset + vec2(px, py);
                    let is_hovered = vpos == hovered_hex;

                    if is_hovered && !state.game_over {
                        // Draw preview of current tile on hovered slot
                        draw_hex_tile(center, hex_radius, &state.current_tile);
                        draw_hex_outline(center, hex_radius, YELLOW, 3.0);
                    } else {
                        draw_hex_outline(
                            center,
                            hex_radius,
                            Color::from_rgba(100, 110, 130, 150),
                            1.5,
                        );
                    }
                }

                // 6. Draw HUD & UI Overlays
                draw_rectangle(
                    10.0,
                    10.0,
                    320.0,
                    200.0,
                    Color::from_rgba(20, 24, 32, 220),
                );
                draw_rectangle_lines(10.0, 10.0, 320.0, 200.0, 2.0, Color::from_rgba(60, 70, 90, 255));

                draw_text(&format!("Seed: {}", state.seed), 25.0, 35.0, 20.0, GOLD);
                draw_text(&format!("Score: {}", state.score), 25.0, 65.0, 24.0, WHITE);
                draw_text(
                    &format!("Tiles Deck Remaining: {}", state.tiles_remaining),
                    25.0,
                    95.0,
                    20.0,
                    GREEN,
                );
                draw_text(
                    &format!("Perfect Placements: {}", state.perfect_count),
                    25.0,
                    125.0,
                    18.0,
                    YELLOW,
                );
                draw_text(
                    &format!("Quests Completed: {}", state.quests_completed),
                    25.0,
                    150.0,
                    18.0,
                    ORANGE,
                );
                draw_text(
                    &format!("Flags Completed: {}", state.flags_completed),
                    25.0,
                    175.0,
                    18.0,
                    SKYBLUE,
                );

                // Controls instructions
                draw_text(
                    "Controls: [Left Click] Place | [Right Click / R] Rotate | [Middle Drag] Pan | [Scroll] Zoom",
                    20.0,
                    screen_height() - 20.0,
                    18.0,
                    LIGHTGRAY,
                );

                // Current Tile Preview Box in Bottom-Left
                let preview_box_rect = (20.0, screen_height() - 170.0, 130.0, 130.0);
                draw_rectangle(
                    preview_box_rect.0,
                    preview_box_rect.1,
                    preview_box_rect.2,
                    preview_box_rect.3,
                    Color::from_rgba(25, 28, 38, 240),
                );
                draw_rectangle_lines(
                    preview_box_rect.0,
                    preview_box_rect.1,
                    preview_box_rect.2,
                    preview_box_rect.3,
                    2.0,
                    YELLOW,
                );
                draw_text(
                    "CURRENT TILE",
                    preview_box_rect.0 + 10.0,
                    preview_box_rect.1 + 20.0,
                    14.0,
                    WHITE,
                );
                let preview_center = vec2(preview_box_rect.0 + 65.0, preview_box_rect.1 + 75.0);
                draw_hex_tile(preview_center, 30.0, &state.current_tile);

                // Game Over Overlay
                if state.game_over {
                    draw_rectangle(
                        0.0,
                        0.0,
                        screen_width(),
                        screen_height(),
                        Color::from_rgba(0, 0, 0, 180),
                    );
                    draw_text("GAME OVER!", screen_width() / 2.0 - 120.0, 300.0, 42.0, RED);
                    draw_text(
                        &format!("Final Score: {}", state.score),
                        screen_width() / 2.0 - 90.0,
                        360.0,
                        28.0,
                        GOLD,
                    );
                }
            }
        }

        next_frame().await;
    }
}

/// Helper function to draw a 6-sided hex tile with colored triangle segments & quest text
fn draw_hex_tile(center: Vec2, radius: f32, tile: &Tile) {
    let mut vertices = [vec2(0.0, 0.0); 6];
    for i in 0..6 {
        let angle = std::f32::consts::PI / 180.0 * (60.0 * i as f32 - 30.0);
        vertices[i] = center + vec2(radius * angle.cos(), radius * angle.sin());
    }

    // Draw 6 colored triangular pie segments facing each edge
    for i in 0..6 {
        let stype = tile.edges[i];
        let color = segment_color(stype);
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % 6];

        draw_triangle(center, p1, p2, color);
        draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, Color::from_rgba(20, 20, 20, 180));
    }

    // Draw hex center circle
    draw_circle(center.x, center.y, radius * 0.25, Color::from_rgba(30, 35, 45, 230));

    // Draw Quest Badge if present
    if let Some(ref q) = tile.quest {
        let badge_color = if q.is_fulfilled {
            GREEN
        } else if q.is_flag {
            SKYBLUE
        } else {
            GOLD
        };

        draw_circle(center.x, center.y, radius * 0.35, badge_color);
        let qtext = format!("{}{}", q.target_count, if q.is_flag { "F" } else { "+" });
        draw_text(
            &qtext,
            center.x - 12.0,
            center.y + 5.0,
            16.0,
            BLACK,
        );
    }
}

fn draw_hex_outline(center: Vec2, radius: f32, color: Color, thickness: f32) {
    for i in 0..6 {
        let a1 = std::f32::consts::PI / 180.0 * (60.0 * i as f32 - 30.0);
        let a2 = std::f32::consts::PI / 180.0 * (60.0 * (i + 1) as f32 - 30.0);
        let p1 = center + vec2(radius * a1.cos(), radius * a1.sin());
        let p2 = center + vec2(radius * a2.cos(), radius * a2.sin());
        draw_line(p1.x, p1.y, p2.x, p2.y, thickness, color);
    }
}
