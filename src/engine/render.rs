use super::transform::Transform;
use crate::context::Context;
use crate::sprite::Sprite;
use lazy_static::*;
use specs::prelude::*;

lazy_static! {
    static ref CTX: Context = { Context::from_id("game").unwrap() };
}

pub struct SysRender;

impl<'a> System<'a> for SysRender {
    type SystemData = (ReadStorage<'a, Transform>, ReadStorage<'a, Sprite>);

    fn run(&mut self, (transforms, sprites): Self::SystemData) {
        for (t, s) in (&transforms, &sprites).join() {
            let t: &Transform = t;
            let s: &Sprite = s;
            let ctx: &Context = &CTX;

            ctx.draw(
                s.image(),
                *t.position().x() as u32,
                *t.position().y() as u32,
            )
            .unwrap();
        }
    }
}
