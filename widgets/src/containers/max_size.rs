use super::*;

#[derive(Debug)]
pub struct MaxSize {
    id: Id,
    child: Box<dyn Widget>,
    pub size: LogicalSize<Option<f32>>,
}

impl MaxSize {
    #[inline]
    pub fn new(width: Option<f32>, height: Option<f32>, child: impl Widget) -> Self {
        Self {
            id: Id::new(),
            child: Box::new(child),
            size: LogicalSize::new(width, height),
        }
    }
}

impl HasId for MaxSize {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for MaxSize {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        self.child.input(ctx, input, events)
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
        self.child.apply(funcs);
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        let size = self.child.size(ctx);
        LogicalSize::new(
            self.size.width.map_or(size.width, |m| size.width.min(m)),
            self.size.height.map_or(size.height, |m| size.height.min(m)),
        )
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        self.child.layout(
            lc.next(
                LogicalRect::from_position_size(lc.rect.left_top(), size),
                lc.layer,
                lc.selected,
            ),
            result,
        );
    }
}
