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

    fn layout(&self, ctx: LayoutContext, result: &mut LayoutConstructor) {
        self.child.layout(
            ctx.next(
                LogicalRect::from_positions(self.position, ctx.rect.right_bottom()),
            ),
            result,
        );
    }
}

pub struct Column {
    id: Id,
    children: Vec<Box<dyn Widget>>,
}

impl Column {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Id::new(),
            children: vec![],
        }
    }

    #[inline]
    pub fn push(&mut self, widget: impl Widget) {
        self.children.push(Box::new(widget));
    }

    #[inline]
    pub fn erase(&mut self, object: &impl HasId) {
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
            size.height += s.height;
        }
        size
    }

    fn layout(&self, ctx: LayoutContext, result: &mut LayoutConstructor) {
        let mut pt = ctx.rect.left_top();
        let size = self.size(&ctx);
        for child in self.children.iter() {
            let s = child.size(&ctx);
            child.layout(
                ctx.next(
                    LogicalRect::from_position_size(pt, LogicalSize::new(size.width, s.height)),
                ),
                result,
            );
            pt.y += s.height;
        }
    }
}

impl HasChildren for Column {
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
}

impl Row {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Id::new(),
            children: vec![],
        }
    }

    #[inline]
    pub fn erase(&mut self, object: &impl HasId) {
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

    fn layout(&self, ctx: LayoutContext, result: &mut LayoutConstructor) {
        let mut pt = ctx.rect.left_top();
        let size = self.size(&ctx);
        for child in self.children.iter() {
            let s = child.size(&ctx);
            child.layout(
                ctx.next(LogicalRect::from_position_size(
                    pt,
                    LogicalSize::new(s.width, size.height),
                )),
                result,
            );
            pt.x += s.width;
        }
    }
}

impl HasChildren for Row {
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
