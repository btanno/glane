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
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Vec<Event>) {
        self.child.input(ctx, input, events);
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
            lc.next(LogicalRect::from_positions(
                self.position,
                lc.rect.right_bottom(),
            )),
            result,
        );
    }
}

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
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Vec<Event>) {
        for child in self.children.iter_mut() {
            child.input(ctx, input, events);
        }
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
                lc.next(LogicalRect::from_position_size(
                    rect.left_top(),
                    LogicalSize::new(size.width, s.height),
                )),
                result,
            );
            rect.top += s.height + self.space;
            rect.bottom -= s.height + self.space;
        }
    }
}

impl HasChildren for Column {
    fn len(&self) -> usize {
        self.children.len()
    }

    fn push(&mut self, child: impl Widget) {
        self.children.push(Box::new(child));
    }

    fn erase(&mut self, object: impl HasId) {
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
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Vec<Event>) {
        for child in self.children.iter_mut() {
            child.input(ctx, input, events);
        }
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
            size.width += s.width;
            size.height = s.height.max(size.height);
        }
        size
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        let mut rect = lc.rect;
        for child in self.children.iter() {
            let s = child.size(&lc.next(rect));
            let h = (size.height - s.height) / 2.0;
            child.layout(
                lc.next(LogicalRect::from_position_size(
                    LogicalPosition::new(rect.left, rect.top + h),
                    LogicalSize::new(s.width, size.height),
                )),
                result,
            );
            rect.left += s.width + self.space;
            rect.right -= s.width + self.space;
        }
    }
}

impl HasChildren for Row {
    fn len(&self) -> usize {
        self.children.len()
    }

    fn push(&mut self, child: impl Widget) {
        self.children.push(Box::new(child));
    }

    fn erase(&mut self, object: impl HasId) {
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
