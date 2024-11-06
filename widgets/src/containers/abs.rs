use super::*;

#[derive(Debug)]
pub struct Abs<T> {
    id: Id,
    pub child: T,
    pub position: LogicalPosition<f32>,
}

impl<T> Abs<T>
where
    T: Widget,
{
    #[inline]
    pub fn new(child: T, position: impl Into<LogicalPosition<f32>>) -> Self {
        Self {
            id: Id::new(),
            child,
            position: position.into(),
        }
    }
}

impl<T> HasId for Abs<T>
where
    T: Widget,
{
    #[inline]
    fn id(&self) -> Id {
        self.id
    }
}

impl<T> Widget for Abs<T>
where
    T: Widget,
{
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        self.child.input(ctx, input, events)
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
        self.child.apply(funcs);
    }

    fn size(&self, _ctx: &LayoutContext) -> LogicalSize<f32> {
        LogicalSize::new(0.0, 0.0)
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        self.child.layout(
            lc.next(
                LogicalRect::from_positions(self.position, lc.rect.right_bottom()),
                lc.layer,
                lc.selected,
            ),
            result,
        );
    }
}