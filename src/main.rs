use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Mesh};
use ggez::mint::Vector2;
use ggez::{Context, ContextBuilder, GameResult};

// Isometric projection constants
const TILE_WIDTH_HALF: f32 = 32.0;
const TILE_HEIGHT_HALF: f32 = 16.0;

// Player entity
#[allow(dead_code)] // Some fields used in future stories
struct Player {
    pos: Vector2<f32>,      // World position
    velocity: Vector2<f32>, // Current velocity
    speed: f32,             // Movement speed (pixels/sec)
    hp: i32,                // Current health
    max_hp: i32,            // Maximum health
    radius: f32,            // Collision radius
    last_shot_time: f32,    // Cooldown tracking
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            pos: Vector2 { x, y },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            speed: 150.0,
            hp: 100,
            max_hp: 100,
            radius: 16.0,
            last_shot_time: 0.0,
        }
    }
}

// Convert world coordinates to screen coordinates using isometric projection
fn world_to_screen(world_pos: Vector2<f32>, camera_offset: Vector2<f32>) -> Vector2<f32> {
    let screen_x = (world_pos.x - world_pos.y) * TILE_WIDTH_HALF + camera_offset.x;
    let screen_y = (world_pos.x + world_pos.y) * TILE_HEIGHT_HALF + camera_offset.y;
    Vector2 {
        x: screen_x,
        y: screen_y,
    }
}

struct GameState {
    player: Player,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        // Initialize player at world origin - camera will position it at screen center
        let player = Player::new(0.0, 0.0);

        Ok(GameState { player })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Game logic will go here in future stories
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear screen with black background
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Transform player world position to screen coordinates
        let camera_offset = Vector2 { x: 400.0, y: 300.0 };
        let screen_pos = world_to_screen(self.player.pos, camera_offset);

        // Create yellow circle mesh for player sprite
        let player_color = Color::from_rgb(255, 215, 0); // #FFD700 yellow
        let circle = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vector2 { x: 0.0, y: 0.0 },
            self.player.radius,
            0.1,
            player_color,
        )?;

        // Draw player at screen position
        canvas.draw(&circle, DrawParam::default().dest(screen_pos));

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    // Create game context with window configuration
    let (ctx, event_loop) = ContextBuilder::new("springfield_meltdown", "Lassi Viitakoski")
        .window_setup(WindowSetup::default().title("Springfield Meltdown - Prototype"))
        .window_mode(WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    // Create game state
    let state = GameState::new()?;

    // Run game loop at 60 FPS (ggez default)
    event::run(ctx, event_loop, state)
}
