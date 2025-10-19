use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Mesh};
use ggez::input::keyboard::KeyCode;
use ggez::mint::Vector2;
use ggez::{Context, ContextBuilder, GameResult};

// Isometric projection constants
const TILE_WIDTH_HALF: f32 = 16.0;
const TILE_HEIGHT_HALF: f32 = 8.0;

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
            speed: 300.0,
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
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Get delta time for frame-independent movement
        let delta = ctx.time.delta().as_secs_f32();

        // Build velocity vector from WASD keyboard input
        // Using screen-space movement for better gameplay feel (equal perceived speed in all directions)
        let mut velocity = Vector2 { x: 0.0, y: 0.0 };

        if ctx.keyboard.is_key_pressed(KeyCode::W) {
            velocity.y -= 1.0; // Up on screen
        }
        if ctx.keyboard.is_key_pressed(KeyCode::S) {
            velocity.y += 1.0; // Down on screen
        }
        if ctx.keyboard.is_key_pressed(KeyCode::A) {
            velocity.x -= 1.0; // Left on screen
        }
        if ctx.keyboard.is_key_pressed(KeyCode::D) {
            velocity.x += 1.0; // Right on screen
        }

        // Normalize diagonal movement to prevent speed boost
        let length = ((velocity.x * velocity.x + velocity.y * velocity.y) as f32).sqrt();
        if length > 0.0 {
            velocity.x /= length;
            velocity.y /= length;
        }

        // Apply movement in screen space (velocity is already screen-space)
        // We move the player in screen coordinates, treating world pos as screen pos
        // The isometric transform is purely visual (rendering only)
        self.player.velocity = velocity;
        self.player.pos.x += velocity.x * self.player.speed * delta;
        self.player.pos.y += velocity.y * self.player.speed * delta;

        // Screen bounds collision: clamp player position to stay within green debug box
        // Green box: 600x400 at screen position (100, 100) to (700, 500)
        // Player pos is offset from screen center (400, 300), so:
        // Min pos: (100 - 400, 100 - 300) = (-300, -200)
        // Max pos: (700 - 400, 500 - 300) = (300, 200)
        // Account for player radius to keep entire circle inside bounds
        let min_x = -300.0 + self.player.radius;
        let max_x = 300.0 - self.player.radius;
        let min_y = -200.0 + self.player.radius;
        let max_y = 200.0 - self.player.radius;

        self.player.pos.x = self.player.pos.x.clamp(min_x, max_x);
        self.player.pos.y = self.player.pos.y.clamp(min_y, max_y);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear screen with black background
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Camera offset: keeps player centered on screen
        // World position (0,0) appears at screen center
        let screen_center = Vector2 { x: 400.0, y: 300.0 };
        let camera_offset = Vector2 {
            x: screen_center.x - self.player.pos.x,
            y: screen_center.y - self.player.pos.y,
        };

        // Draw debug bounds box in world space (moves with camera)
        // World bounds: 600x400 centered at world origin (0, 0)
        // Box corners: (-300, -200) to (300, 200) in world space
        let world_bounds_x = -300.0;
        let world_bounds_y = -200.0;
        let world_bounds_width = 600.0;
        let world_bounds_height = 400.0;

        // Transform world bounds to screen space using camera offset
        let bounds_rect = graphics::Rect::new(
            world_bounds_x + camera_offset.x,
            world_bounds_y + camera_offset.y,
            world_bounds_width,
            world_bounds_height,
        );
        let bounds_mesh = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            bounds_rect,
            Color::from_rgb(0, 255, 0), // Green debug box
        )?;
        canvas.draw(&bounds_mesh, DrawParam::default());

        // Camera follows player: always render player at screen center
        let screen_pos = screen_center;

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

        // Debug UI: Display player position and FPS in top-left corner
        let fps = ctx.time.fps();
        let debug_text = format!(
            "Pos: ({:.1}, {:.1}) | FPS: {:.0}",
            self.player.pos.x, self.player.pos.y, fps
        );

        let text = graphics::Text::new(debug_text);
        let text_pos = Vector2 { x: 10.0, y: 10.0 };
        canvas.draw(
            &text,
            DrawParam::default().dest(text_pos).color(Color::WHITE),
        );

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
