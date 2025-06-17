use tetra::Context;

#[allow(unused_variables)]
pub trait Updatable {
    fn update(&mut self, ctx: &mut Context);
}
