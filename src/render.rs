use crate::context::Context;
use crate::sprite::Sprite;
use crate::transform::Position;
use lazy_static::*;
use specs::prelude::*;

lazy_static! {
    static ref CTX: Context = { Context::from_id("game").unwrap() };
}

pub struct SysRender;

impl<'a> System<'a> for SysRender {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

    fn run(&mut self, (transforms, sprites): Self::SystemData) {
        for (p, s) in (&transforms, &sprites).join() {
            let p: &Position = p;
            let s: &Sprite = s;
            let ctx: &Context = &CTX;

            ctx.draw(s.image(), *p.get().x() as u32, *p.get().y() as u32)
                .unwrap();
        }
    }
}
