use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};

struct GameState {
    // Empty for now - will add game state fields in future stories
}

impl GameState {
    fn new() -> GameResult<GameState> {
        Ok(GameState {})
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Game logic will go here in future stories
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear screen with black background
        let canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
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
