use super::*;

#[derive(Debug)]
pub struct VerticalPanes {
    id: Id,
    panes: [Box<dyn Widget>; 2],
    ratio: f32,
}

impl VerticalPanes {
    #[inline]
    pub fn new<T, U>(left: T, right: U, ratio: f32) -> (Self, Handle<T>, Handle<U>)
    where
        T: Widget,
        U: Widget,
    {
        assert!((0.0..=1.0).contains(&ratio));
        let left_handle = Handle::new(&left);
        let right_handle = Handle::new(&right);
        (
            Self {
                id: Id::new(),
                panes: [Box::new(left), Box::new(right)],
                ratio,
            },
            left_handle,
            right_handle,
        )
    }
}

impl HasId for VerticalPanes {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for VerticalPanes {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        for pane in self.panes.iter_mut() {
            if pane.input(ctx, input, events) == ControlFlow::Break {
                return ControlFlow::Break;
            }
        }
        ControlFlow::Continue
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
        self.panes.iter_mut().for_each(|pane| pane.apply(funcs));
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        ctx.rect.size()
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let left = LogicalRect::from_positions(
            lc.rect.left_top(),
            (lc.rect.right * self.ratio, lc.rect.bottom),
        );
        let right = LogicalRect::from_positions(
            (lc.rect.left + left.size().width, lc.rect.top),
            lc.rect.right_bottom(),
        );
        self.panes[0].layout(lc.next(left, lc.layer, lc.selected), result);
        self.panes[1].layout(lc.next(right, lc.layer, lc.selected), result);
    }
}

#[derive(Debug)]
pub struct HorizontalPanes {
    id: Id,
    panes: [Box<dyn Widget>; 2],
    ratio: f32,
}

impl HorizontalPanes {
    #[inline]
    pub fn new(top: impl Widget, bottom: impl Widget, ratio: f32) -> Self {
        assert!((0.0..=1.0).contains(&ratio));
        Self {
            id: Id::new(),
            panes: [Box::new(top), Box::new(bottom)],
            ratio,
        }
    }
}

impl HasId for HorizontalPanes {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for HorizontalPanes {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        for pane in self.panes.iter_mut() {
            if pane.input(ctx, input, events) == ControlFlow::Break {
                return ControlFlow::Break;
            }
        }
        ControlFlow::Continue
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
        self.panes.iter_mut().for_each(|pane| pane.apply(funcs));
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        ctx.rect.size()
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let top = LogicalRect::from_positions(
            lc.rect.left_top(),
            (lc.rect.right, lc.rect.bottom * self.ratio),
        );
        let bottom = LogicalRect::from_positions(
            (lc.rect.left, lc.rect.top + top.size().height),
            lc.rect.right_bottom(),
        );
        self.panes[0].layout(lc.next(top, lc.layer, lc.selected), result);
        self.panes[1].layout(lc.next(bottom, lc.layer, lc.selected), result);
    }
}
