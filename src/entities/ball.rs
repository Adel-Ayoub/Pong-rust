use crate::entities::Entity;
use crate::updatable::Updatable;
use crate::WINDOW_HEIGHT;
use tetra::graphics::{DrawParams, Texture};
use tetra::math::Vec2;
use tetra::Context;

const BALL_SPEED: f32 = 7.0;

pub struct Ball {
    pub entity: Entity,
    pub velocity: Vec2<f32>,
}

impl Ball {
    pub fn new(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Ball {
        Ball {
            entity: Entity { position, texture },
            velocity,
        }
    }
}

impl Ball {
    pub fn draw<P>(&self, ctx: &mut Context, params: P)
    where
        P: Into<DrawParams>,
    {
        self.entity.draw(ctx, params)
    }
}

impl Updatable for Ball {
    fn update(&mut self, _ctx: &mut Context) {
        self.velocity /=
            (self.velocity.x * self.velocity.x + self.velocity.y * self.velocity.y).sqrt();
        self.entity.position += self.velocity * BALL_SPEED;

        if self.entity.position.y <= 0.0
            || self.entity.position.y + self.entity.dimensions().1 >= WINDOW_HEIGHT as f32
        {
            self.velocity.y = -self.velocity.y;
        }
    }
}
