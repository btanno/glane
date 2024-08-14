use super::*;

#[derive(Debug)]
pub struct Empty {
    id: Id,
}

impl Empty {
    #[inline]
    pub fn new() -> Self {
        Self { id: Id::new() }
    }
}

impl HasId for Empty {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Empty {
    fn input(&mut self, _ctx: &Context, _input: &Input, _events: &mut Events) -> ControlFlow {
        ControlFlow::Continue
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        ctx.rect.size()
    }

    fn layout(&self, _lc: LayoutContext, _result: &mut LayoutConstructor) {}
}

impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}