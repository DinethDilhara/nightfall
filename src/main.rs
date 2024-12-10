/*
 * Create a very very simple snowfall / sparkle animation.
 *
 * Used for my album Nightfall:
 * - https://www.youtube.com/watch?v=usocnehtnlA
 *
 * Author: Dave Eddy <dave@daveeddy.com>
 * Date: April 21, 2024
 * License: MIT
 */

use macroquad::prelude::*;

const GAME_SCREEN_WIDTH: f32 = 1920.;
const GAME_SCREEN_HEIGHT: f32 = 1080.;
const NUM_SPARKLES: i32 = 500;

struct Sparkle {
    position: Vec2,
    speed: f32,
    cycle: f32,
    radius: f32,
}

impl Sparkle {
    fn draw(&self, color: Color) {
        // Simple sine-wave for sparkling
        let alpha = (self.cycle.sin() + 1.0) / 2.0;

        // Set the color with varying alpha
        let mut color = color;
        color.a = alpha;

        // Draw the circle
        draw_circle(self.position.x, self.position.y, self.radius, color);
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Nightfall"),
        window_width: GAME_SCREEN_WIDTH as i32,
        window_height: GAME_SCREEN_HEIGHT as i32,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    // Default speed and color
    let sparkle_speed = 2.0;
    let mut sparkle_color = Color::new(1.0, 1.0, 1.0, 1.0); // Start with white

    // TODO: seed this with something? the date?
    rand::srand(0);

    // create sparkles
    let mut sparkles = vec![];
    for _i in 0..NUM_SPARKLES {
        let x = rand::gen_range(0.0, GAME_SCREEN_WIDTH);
        let y = rand::gen_range(0.0, GAME_SCREEN_HEIGHT);
        let sparkle = Sparkle {
            position: vec2(x, y),
            radius: rand::gen_range(0.5, 3.0),
            speed: sparkle_speed * rand::gen_range(1.0, 1.5),
            cycle: rand::gen_range(0.0, 90.0),
        };
        sparkles.push(sparkle);
    }

    loop {
        let delta = get_frame_time();

        // Check for keyboard input to change color
        if is_key_pressed(KeyCode::R) {
            sparkle_color = Color::new(1.0, 0.0, 0.0, 1.0); // Red
        } else if is_key_pressed(KeyCode::G) {
            sparkle_color = Color::new(0.0, 1.0, 0.0, 1.0); // Green
        } else if is_key_pressed(KeyCode::B) {
            sparkle_color = Color::new(0.0, 0.0, 1.0, 1.0); // Blue
        } else if is_key_pressed(KeyCode::W) {
            sparkle_color = Color::new(1.0, 1.0, 1.0, 1.0); // White
        }

        // quit if ESC is pressed
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // move the sparkles
        for sparkle in sparkles.iter_mut() {
            // update cycle - used for sine-wave twinkling
            sparkle.cycle += delta;

            // move the sparkles down and to the left (5x faster down than to
            // the left)
            sparkle.position.y += sparkle.speed * delta * 10.;
            sparkle.position.x -= sparkle.speed * delta * 2.;

            // wrap around particles
            while sparkle.position.y >= GAME_SCREEN_HEIGHT {
                sparkle.position.y -= GAME_SCREEN_HEIGHT;
            }
            while sparkle.position.x < 0.0 {
                sparkle.position.x += GAME_SCREEN_WIDTH;
            }
        }

        // draw the screen
        clear_background(BLACK);

        for sparkle in sparkles.iter() {
            sparkle.draw(sparkle_color);
        }

        next_frame().await
    }
}
