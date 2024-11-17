use super::*;

#[derive(Debug)]
pub struct Column {
    id: Id,
    children: Vec<Box<dyn Widget>>,
    pub space: f32,
    pub max_height: Option<f32>,
}

impl Column {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Id::new(),
            children: vec![],
            space: 10.0,
            max_height: None,
        }
    }
}

impl HasId for Column {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Column {
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
        let mut size = LogicalSize::new(0.0f32, 0.0);
        for child in self.children.iter() {
            let s = child.size(ctx);
            size.width = s.width.max(size.width);
            size.height += self.max_height.map_or(s.height, |mh| s.height.min(mh));
        }
        size
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        let mut rect = lc.rect;
        for child in self.children.iter() {
            let s = child.size(&lc);
            let s = self
                .max_height
                .map_or(s, |mh| LogicalSize::new(s.width, s.height.min(mh)));
            child.layout(
                lc.next(
                    self,
                    LogicalRect::from_position_size(
                        rect.left_top(),
                        LogicalSize::new(size.width, s.height),
                    ),
                    lc.layer,
                    lc.selected,
                ),
                result,
            );
            rect.top += s.height + self.space;
            rect.bottom -= s.height + self.space;
        }
    }
}

impl HasChildren for Column {
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

impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}