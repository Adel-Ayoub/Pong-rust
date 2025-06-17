//use tetra::graphics::{Texture, Drawable, DrawParams};
use crate::entities::Entity;
use crate::updatable::Updatable;
use crate::WINDOW_HEIGHT;
use tetra::graphics::{DrawParams, Texture};
use tetra::input::Key;
use tetra::math::Vec2;
use tetra::{input, Context};

const PLAYER_SPEED: f32 = 8.0;

pub struct Player {
    pub entity: Entity,
    pub up_key: Key,
    pub down_key: Key,
}

impl Player {
    pub fn new(texture: Texture, position: Vec2<f32>, up_key: Key, down_key: Key) -> Player {
        Player {
            entity: Entity { position, texture },
            up_key,
            down_key,
        }
    }
}

impl Player {
    pub fn draw<P>(&self, ctx: &mut Context, params: P)
    where
        P: Into<DrawParams>,
    {
        self.entity.draw(ctx, params)
    }
}

impl Updatable for Player {
    fn update(&mut self, ctx: &mut Context) {
        if input::is_key_down(ctx, self.up_key) {
            self.entity.position.y -= PLAYER_SPEED;
        }

        if input::is_key_down(ctx, self.down_key) {
            self.entity.position.y += PLAYER_SPEED;
        }

        if self.entity.position.y <= 16.0 {
            self.entity.position.y = 16.0
        } else if self.entity.position.y
            >= (WINDOW_HEIGHT - self.entity.texture.height()) as f32 - 16.0
        {
            self.entity.position.y = (WINDOW_HEIGHT - self.entity.texture.height()) as f32 - 16.0;
        }
    }
}
