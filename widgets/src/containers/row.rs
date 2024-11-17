use super::*;

#[derive(Debug)]
pub struct Row {
    id: Id,
    children: Vec<Box<dyn Widget>>,
    pub space: f32,
}

impl Row {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Id::new(),
            children: vec![],
            space: 10.0,
        }
    }
}

impl HasId for Row {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Row {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        for child in self.children.iter_mut() {
            if child.input(ctx, input, events) == ControlFlow::Break {
                return ControlFlow::Break;
            }
        }
        ControlFlow::Continue
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
        for child in self.children.iter_mut() {
            child.apply(funcs);
        }
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        let mut size = LogicalSize::new(ctx.rect.size().width, 0.0);
        for child in self.children.iter() {
            let s = child.size(ctx);
            size.height = s.height.max(size.height);
        }
        size
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        let mut rect = lc.rect;
        for child in self.children.iter().take(self.children.len() - 1) {
            let s = child.size(&lc.next(self, rect, lc.layer, lc.selected));
            let h = (size.height - s.height) / 2.0;
            let r = rect.left + s.width;
            let rb = LogicalPosition::new(
                if r > lc.rect.right { lc.rect.right } else { r },
                rect.bottom,
            );
            child.layout(
                lc.next(
                    self,
                    LogicalRect::from_positions(LogicalPosition::new(rect.left, rect.top + h), rb),
                    lc.layer,
                    lc.selected,
                ),
                result,
            );
            rect.left += s.width + self.space;
        }
        if let Some(child) = self.children.last() {
            let s = child.size(&lc.next(self, rect, lc.layer, lc.selected));
            let h = (size.height - s.height) / 2.0;
            let rb = LogicalPosition::new(lc.rect.right, rect.bottom);
            child.layout(
                lc.next(
                    self,
                    LogicalRect::from_positions(LogicalPosition::new(rect.left, rect.top + h), rb),
                    lc.layer,
                    lc.selected,
                ),
                result,
            );
        }
    }
}

impl HasChildren for Row {
    #[inline]
    fn len(&self) -> usize {
        self.children.len()
    }

    #[inline]
    fn push(&mut self, child: impl Widget) {
        self.children.push(Box::new(child));
    }

    #[inline]
    fn erase(&mut self, object: &impl HasId) {
        let Some(index) = self
            .children
            .iter()
            .position(|child| child.id() == object.id())
        else {
            return;
        };
        self.children.remove(index);
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}
