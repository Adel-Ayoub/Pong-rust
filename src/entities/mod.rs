use tetra::math::Vec2;
use tetra::Context;

pub(crate) mod ball;
pub(crate) mod player;

pub struct Entity {
    pub position: Vec2<f32>,
    pub texture: Texture,
}

impl Entity {
    pub fn dimensions(&self) -> (f32, f32) {
        (self.texture.width() as f32, self.texture.height() as f32)
    }

    pub fn center(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.dimensions().0 / 2.0),
            self.position.y + (self.dimensions().1 / 2.0),
        )
    }

    pub fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.dimensions().0,
            self.dimensions().1,
        )
    }
}

impl Entity {
    pub fn draw<P>(&self, ctx: &mut Context, params: P)
    where
        P: Into<DrawParams>,
    {
        let mut params = params.into();
        params = DrawParams {
            position: params.position + self.position,
            ..params
        };
        self.texture.draw(ctx, params);
    }
}
