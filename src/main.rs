mod board;
mod config;
mod game_state;
mod group;
mod hex;
mod prng;
mod tile;

use config::parse_config_string;
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
        window_title: "Dorfromantik Simulator & ConfigString Decoder".to_string(),
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

#[derive(PartialEq, Eq)]
enum ActiveField {
    ConfigString,
    SeedInput,
    TileLimitInput,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut mode = AppMode::SeedInput;
    let mut active_field = ActiveField::ConfigString;

    let mut config_input_text = "0720262fJmCw2gRsn6".to_string();
    let mut seed_input_text = "3103784960".to_string();
    let mut tile_limit_text = "250".to_string();
    let mut use_tile_limit = true;

    let mut game_state: Option<GameState> = None;

    let mut camera_offset = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let mut hex_radius = 45.0f32;
    let mut last_mouse_pos = mouse_position();

    loop {
        clear_background(Color::from_rgba(30, 32, 40, 255));

        match mode {
            AppMode::SeedInput => {
                let (mx, my) = mouse_position();

                let title = "DORFROMANTIK CONFIGSTRING DECODER & SIMULATOR";
                draw_text(title, screen_width() / 2.0 - 280.0, 70.0, 28.0, WHITE);

                // 1. ConfigString Input Field
                draw_text("ConfigString (18-char Base62, e.g. 0720262fJmCw2gRsn6):", screen_width() / 2.0 - 220.0, 125.0, 17.0, LIGHTGRAY);
                let cfg_rect = (screen_width() / 2.0 - 220.0, 140.0, 440.0, 40.0);
                let cfg_focused = active_field == ActiveField::ConfigString;
                
                draw_rectangle(cfg_rect.0, cfg_rect.1, cfg_rect.2, cfg_rect.3, Color::from_rgba(40, 44, 56, 255));
                draw_rectangle_lines(cfg_rect.0, cfg_rect.1, cfg_rect.2, cfg_rect.3, 2.0, if cfg_focused { GOLD } else { GRAY });
                draw_text(&config_input_text, cfg_rect.0 + 15.0, cfg_rect.1 + 27.0, 20.0, if cfg_focused { GOLD } else { WHITE });

                if mx >= cfg_rect.0 && mx <= cfg_rect.0 + cfg_rect.2 && my >= cfg_rect.1 && my <= cfg_rect.1 + cfg_rect.3 && is_mouse_button_pressed(MouseButton::Left) {
                    active_field = ActiveField::ConfigString;
                }

                // 2. Direct Seed Input Field
                draw_text("Random Seed (u32 / i32):", screen_width() / 2.0 - 220.0, 205.0, 17.0, LIGHTGRAY);
                let seed_rect = (screen_width() / 2.0 - 220.0, 220.0, 440.0, 40.0);
                let seed_focused = active_field == ActiveField::SeedInput;

                draw_rectangle(seed_rect.0, seed_rect.1, seed_rect.2, seed_rect.3, Color::from_rgba(40, 44, 56, 255));
                draw_rectangle_lines(seed_rect.0, seed_rect.1, seed_rect.2, seed_rect.3, 2.0, if seed_focused { GOLD } else { GRAY });
                draw_text(&seed_input_text, seed_rect.0 + 15.0, seed_rect.1 + 27.0, 20.0, if seed_focused { GOLD } else { WHITE });

                if mx >= seed_rect.0 && mx <= seed_rect.0 + seed_rect.2 && my >= seed_rect.1 && my <= seed_rect.1 + seed_rect.3 && is_mouse_button_pressed(MouseButton::Left) {
                    active_field = ActiveField::SeedInput;
                }

                // 3. Tile Limit Setting Input Field
                draw_text("Tile Limit Override (e.g. 250, 300, 500):", screen_width() / 2.0 - 220.0, 285.0, 17.0, LIGHTGRAY);
                let limit_rect = (screen_width() / 2.0 - 220.0, 300.0, 320.0, 40.0);
                let limit_focused = active_field == ActiveField::TileLimitInput;

                draw_rectangle(limit_rect.0, limit_rect.1, limit_rect.2, limit_rect.3, Color::from_rgba(40, 44, 56, 255));
                draw_rectangle_lines(limit_rect.0, limit_rect.1, limit_rect.2, limit_rect.3, 2.0, if limit_focused { GOLD } else if use_tile_limit { GREEN } else { GRAY });
                draw_text(
                    if use_tile_limit { &tile_limit_text } else { "OFF (Classic Mode)" },
                    limit_rect.0 + 15.0,
                    limit_rect.1 + 27.0,
                    20.0,
                    if limit_focused { GOLD } else if use_tile_limit { GREEN } else { GRAY },
                );

                if mx >= limit_rect.0 && mx <= limit_rect.0 + limit_rect.2 && my >= limit_rect.1 && my <= limit_rect.1 + limit_rect.3 && is_mouse_button_pressed(MouseButton::Left) {
                    active_field = ActiveField::TileLimitInput;
                    use_tile_limit = true;
                }

                // Toggle Tile Limit Button
                let toggle_rect = (screen_width() / 2.0 + 110.0, 300.0, 110.0, 40.0);
                let toggle_hover = mx >= toggle_rect.0 && mx <= toggle_rect.0 + toggle_rect.2 && my >= toggle_rect.1 && my <= toggle_rect.1 + toggle_rect.3;
                draw_rectangle(toggle_rect.0, toggle_rect.1, toggle_rect.2, toggle_rect.3, if toggle_hover { Color::from_rgba(80, 90, 110, 255) } else { Color::from_rgba(60, 70, 90, 255) });
                draw_rectangle_lines(toggle_rect.0, toggle_rect.1, toggle_rect.2, toggle_rect.3, 1.5, WHITE);
                draw_text("TOGGLE", toggle_rect.0 + 20.0, toggle_rect.1 + 25.0, 15.0, WHITE);
                if toggle_hover && is_mouse_button_pressed(MouseButton::Left) {
                    use_tile_limit = !use_tile_limit;
                }

                // Handle Keyboard Inputs for Active Field
                while let Some(c) = get_char_pressed() {
                    match active_field {
                        ActiveField::ConfigString => {
                            if (c.is_ascii_alphanumeric() || c == '-') && config_input_text.len() < 18 {
                                config_input_text.push(c);
                            }
                        }
                        ActiveField::SeedInput => {
                            if (c.is_ascii_digit() || c == '-') && seed_input_text.len() < 12 {
                                seed_input_text.push(c);
                            }
                        }
                        ActiveField::TileLimitInput => {
                            if c.is_ascii_digit() && tile_limit_text.len() < 6 {
                                tile_limit_text.push(c);
                            }
                        }
                    }
                }
                if is_key_pressed(KeyCode::Backspace) {
                    match active_field {
                        ActiveField::ConfigString => { config_input_text.pop(); }
                        ActiveField::SeedInput => { seed_input_text.pop(); }
                        ActiveField::TileLimitInput => { tile_limit_text.pop(); }
                    }
                }

                // Decode ConfigString Button
                let decode_rect = (screen_width() / 2.0 - 220.0, 375.0, 440.0, 45.0);
                let decode_hover = mx >= decode_rect.0 && mx <= decode_rect.0 + decode_rect.2 && my >= decode_rect.1 && my <= decode_rect.1 + decode_rect.3;
                draw_rectangle(decode_rect.0, decode_rect.1, decode_rect.2, decode_rect.3, if decode_hover { Color::from_rgba(180, 140, 40, 255) } else { Color::from_rgba(140, 100, 20, 255) });
                draw_rectangle_lines(decode_rect.0, decode_rect.1, decode_rect.2, decode_rect.3, 2.0, GOLD);
                draw_text("DECODE CONFIGSTRING & LOAD RULES", decode_rect.0 + 40.0, decode_rect.1 + 28.0, 18.0, WHITE);

                if decode_hover && is_mouse_button_pressed(MouseButton::Left) {
                    let (_decoded_seed, decoded_rules) = parse_config_string(&config_input_text);
                    if let Some(lim) = decoded_rules.tile_limit {
                        tile_limit_text = lim.to_string();
                        use_tile_limit = true;
                    }
                }

                // Start Game Button
                let btn_rect = (screen_width() / 2.0 - 120.0, 460.0, 240.0, 55.0);
                let btn_hover = mx >= btn_rect.0 && mx <= btn_rect.0 + btn_rect.2 && my >= btn_rect.1 && my <= btn_rect.1 + btn_rect.3;
                let btn_color = if btn_hover { Color::from_rgba(70, 170, 100, 255) } else { Color::from_rgba(40, 130, 70, 255) };

                draw_rectangle(btn_rect.0, btn_rect.1, btn_rect.2, btn_rect.3, btn_color);
                draw_rectangle_lines(btn_rect.0, btn_rect.1, btn_rect.2, btn_rect.3, 2.0, WHITE);
                draw_text("START GAME", btn_rect.0 + 45.0, btn_rect.1 + 36.0, 24.0, WHITE);

                if (btn_hover && is_mouse_button_pressed(MouseButton::Left)) || is_key_pressed(KeyCode::Enter) {
                    let seed = seed_input_text.parse::<i64>().map(|s| s as u64).unwrap_or(3103784960);
                    let (_d_seed, mut rules) = parse_config_string(&config_input_text);

                    // User typed value in tile_limit_text overrides default!
                    if !use_tile_limit {
                        rules.tile_limit = None;
                    } else if let Ok(lim) = tile_limit_text.parse::<u32>() {
                        rules.tile_limit = Some(lim);
                    }

                    game_state = Some(GameState::new(seed, rules));
                    mode = AppMode::Playing;
                }
            }

            AppMode::Playing => {
                let state = game_state.as_mut().unwrap();

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

                if is_mouse_button_pressed(MouseButton::Right) || is_key_pressed(KeyCode::R) {
                    state.rotate_current_tile();
                }

                let rel_x = mx - camera_offset.x;
                let rel_y = my - camera_offset.y;
                let hovered_hex = AxialPos::from_pixel(rel_x, rel_y, hex_radius);

                if is_mouse_button_pressed(MouseButton::Left)
                    && !is_key_down(KeyCode::LeftShift)
                    && !state.game_over
                {
                    state.place_current_tile(hovered_hex);
                }

                for (&pos, placed) in &state.board.tiles {
                    let (px, py) = pos.to_pixel(hex_radius);
                    let center = camera_offset + vec2(px, py);
                    draw_hex_tile(center, hex_radius, &placed.tile);
                }

                for &vpos in &state.board.valid_slots {
                    let (px, py) = vpos.to_pixel(hex_radius);
                    let center = camera_offset + vec2(px, py);
                    let is_hovered = vpos == hovered_hex;

                    if is_hovered && !state.game_over {
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

                draw_rectangle(
                    10.0,
                    10.0,
                    340.0,
                    220.0,
                    Color::from_rgba(20, 24, 32, 230),
                );
                draw_rectangle_lines(10.0, 10.0, 340.0, 220.0, 2.0, Color::from_rgba(60, 70, 90, 255));

                draw_text(&format!("Seed: {}", state.seed), 25.0, 35.0, 20.0, GOLD);
                draw_text(&format!("Score: {}", state.score), 25.0, 65.0, 24.0, WHITE);
                draw_text(
                    &format!("Tiles Placed: {}", state.tiles_placed_count),
                    25.0,
                    95.0,
                    20.0,
                    LIGHTGRAY,
                );

                if let Some(limit) = state.rules.tile_limit {
                    draw_text(
                        &format!("Tile Limit: {} / {}", state.tiles_placed_count, limit),
                        25.0,
                        125.0,
                        20.0,
                        GREEN,
                    );
                } else {
                    draw_text(
                        &format!("Deck Remaining: {}", state.tiles_remaining),
                        25.0,
                        125.0,
                        20.0,
                        GREEN,
                    );
                }

                draw_text(
                    &format!("Perfect Fits: {}", state.perfect_count),
                    25.0,
                    155.0,
                    18.0,
                    YELLOW,
                );
                draw_text(
                    &format!("Quests: {} | Flags: {}", state.quests_completed, state.flags_completed),
                    25.0,
                    180.0,
                    18.0,
                    ORANGE,
                );

                draw_text(
                    "Controls: [Left Click] Place | [Right Click / R] Rotate | [Middle Drag] Pan | [Scroll] Zoom",
                    20.0,
                    screen_height() - 20.0,
                    18.0,
                    LIGHTGRAY,
                );

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

                if state.game_over {
                    draw_rectangle(
                        0.0,
                        0.0,
                        screen_width(),
                        screen_height(),
                        Color::from_rgba(0, 0, 0, 190),
                    );
                    draw_text("GAME OVER!", screen_width() / 2.0 - 120.0, 300.0, 42.0, RED);
                    draw_text(
                        &format!("Final Score: {}", state.score),
                        screen_width() / 2.0 - 90.0,
                        360.0,
                        28.0,
                        GOLD,
                    );
                    draw_text(
                        &format!("Total Tiles Placed: {}", state.tiles_placed_count),
                        screen_width() / 2.0 - 110.0,
                        400.0,
                        22.0,
                        WHITE,
                    );
                }
            }
        }

        next_frame().await;
    }
}

fn draw_hex_tile(center: Vec2, radius: f32, tile: &Tile) {
    let mut vertices = [vec2(0.0, 0.0); 6];
    for i in 0..6 {
        let angle = std::f32::consts::PI / 180.0 * (60.0 * i as f32 - 30.0);
        vertices[i] = center + vec2(radius * angle.cos(), radius * angle.sin());
    }

    for i in 0..6 {
        let stype = tile.edges[i];
        let color = segment_color(stype);
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % 6];

        draw_triangle(center, p1, p2, color);
        draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, Color::from_rgba(20, 20, 20, 180));
    }

    draw_circle(center.x, center.y, radius * 0.25, Color::from_rgba(30, 35, 45, 230));

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
