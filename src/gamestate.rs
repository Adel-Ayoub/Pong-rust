use crate::entities::{ball::Ball, player::Player};
use crate::updatable::Updatable;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use tetra::graphics::{Color, DrawParams, Texture};
use tetra::input::Key;
use tetra::math::Vec2;
use tetra::{graphics, window, Context, Result, State};

pub struct GameState {
    first_player: Player,
    second_player: Player,
    ball: Ball,
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.first_player.update(ctx);
        self.second_player.update(ctx);

        let first_player_bounds = self.first_player.entity.bounds();
        let second_player_bounds = self.second_player.entity.bounds();
        let ball_bounds = self.ball.entity.bounds();

        let player_hit = if ball_bounds.intersects(&first_player_bounds) {
            Some(&self.first_player)
        } else if ball_bounds.intersects(&second_player_bounds) {
            Some(&self.second_player)
        } else {
            None
        };

        if let Some(player) = player_hit {
            self.ball.velocity.x = -(self.ball.velocity.x + self.ball.velocity.x.signum());
            self.ball.velocity.y += (player.entity.center().y - self.ball.entity.center().y)
                / player.entity.dimensions().1;
        }

        self.ball.update(ctx);

        if self.ball.entity.position.x < 0.0 {
            window::quit(ctx);
            println!("Player 2 wins!");
        }

        if self.ball.entity.position.x > WINDOW_WIDTH as f32 {
            window::quit(ctx);
            println!("Player 1 wins!");
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

        self.first_player.draw(ctx, DrawParams::default());
        self.second_player.draw(ctx, DrawParams::default());
        self.ball.draw(ctx, DrawParams::default());

        Ok(())
    }
}

impl GameState {
    pub fn new(ctx: &mut Context, resources_path: &str) -> tetra::Result<GameState> {
        let first_player_texture =
            Texture::new(ctx, resources_path.to_string() + "first_player.png")?;
        let first_player_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - first_player_texture.height()) as f32 / 2.0,
        );

        let second_player_texture =
            Texture::new(ctx, resources_path.to_string() + "second_player.png")?;
        let second_player_position = Vec2::new(
            (WINDOW_WIDTH - second_player_texture.width()) as f32 - 16.0,
            (WINDOW_HEIGHT - second_player_texture.height()) as f32 / 2.0,
        );

        let ball_texture = Texture::new(ctx, resources_path.to_string() + "ball.png")?;
        let ball_position = Vec2::new(
            (WINDOW_WIDTH - ball_texture.width()) as f32 / 2.0,
            (WINDOW_HEIGHT - ball_texture.height()) as f32 / 2.0,
        );

        Ok(GameState {
            first_player: Player::new(first_player_texture, first_player_position, Key::Z, Key::S),
            second_player: Player::new(
                second_player_texture,
                second_player_position,
                Key::Up,
                Key::Down,
            ),
            ball: Ball::new(ball_texture, ball_position, Vec2 { x: 1.0, y: 0.0 }),
        })
    }
}
