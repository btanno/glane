use super::*;

pub struct Abs<T> {
    id: Id,
    child: T,
    pub position: LogicalPosition<f32>,
}

impl<T> Abs<T>
where
    T: Widget,
{
    #[inline]
    pub fn new(child: T, position: LogicalPosition<f32>) -> Self {
        Self {
            id: Id::new(),
            child,
            position,
        }
    }
}

impl<T> Widget for Abs<T>
where
    T: Widget,
{
    fn id(&self) -> Id {
        self.id
    }

    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Vec<Event>) {
        self.child.input(ctx, input, events);
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
        self.child.apply(funcs);
    }

    fn layout(&self, ctx: LayoutContext, result: &mut LayoutConstructor) {
        self.child.layout(
            ctx.next(
                LogicalRect::from_positions(self.position, ctx.rect.right_bottom()),
                ctx.z,
            ),
            result,
        );
    }
}
