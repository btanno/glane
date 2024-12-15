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
        for (i, child) in self.children.iter().enumerate() {
            if child.size_types().height != SizeType::Fix {
                continue;
            }
            let s = child.size(ctx);
            size.width = s.width.max(size.width);
            size.height += self.max_height.map_or(s.height, |mh| s.height.min(mh));
            if i < self.children.len() - 1 {
                size.height += self.space;
            }
        }
        let flexible_count = self
            .children
            .iter()
            .filter(|child| child.size_types().height == SizeType::Flexible)
            .count();
        if flexible_count > 0 {
            size.height += ctx.rect.size().height - size.height;
            let last = self.children.last().unwrap();
            size.height += self.space
                * (flexible_count
                    - if last.size_types().width == SizeType::Flexible {
                        1
                    } else {
                        0
                    }) as f32;
        }
        size.height = size.height.min(ctx.rect.size().height);
        size
    }

    fn size_types(&self) -> SizeTypes {
        self.children
            .iter()
            .fold(SizeTypes::fix(), |r, child| SizeTypes {
                width: if child.size_types().width == SizeType::Flexible {
                    SizeType::Flexible
                } else {
                    r.width
                },
                height: if child.size_types().height == SizeType::Flexible {
                    SizeType::Flexible
                } else {
                    r.height
                },
            })
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        let mut rect = lc.rect;
        let mut sizes = Vec::with_capacity(self.children.len());
        let mut max_width = 0.0f32;
        let mut h = 0.0;
        for (i, child) in self.children.iter().enumerate() {
            if child.size_types().height == SizeType::Flexible {
                h += self.space;
                sizes.push(None);
            } else {
                let s = child.size(&lc);
                max_width = max_width.max(s.width);
                let height = self.max_height.map_or(s.height, |mh| s.height.min(mh));
                if h + height <= size.height {
                    sizes.push(Some(LogicalSize::new(s.width, height)));
                    h += height;
                    if i < self.children.len() - 1 {
                        h += self.space;
                    }
                } else {
                    let height = size.height - h;
                    if height > 0.0 {
                        sizes.push(Some(LogicalSize::new(s.width, height)));
                    } else {
                        sizes.push(None);
                    }
                }
            }
        }
        let flexible_count = self
            .children
            .iter()
            .filter(|child| child.size_types().height == SizeType::Flexible)
            .count();
        let flexible_height = if flexible_count > 0 {
            (size.height - h) / flexible_count as f32
        } else {
            0.0
        };
        let flexible_height = self
            .max_height
            .map_or(flexible_height, |mh| mh.min(flexible_height));
        for (i, child) in self.children.iter().enumerate() {
            let s = match child.size_types().height {
                SizeType::Flexible => {
                    let size = child.size(&lc);
                    let width = max_width.max(size.width);
                    LogicalSize::new(width, flexible_height)
                }
                _ => {
                    if let Some(size) = sizes[i] {
                        size
                    } else {
                        continue;
                    }
                }
            };
            child.layout(
                lc.next(
                    self,
                    LogicalRect::from_position_size(rect.left_top(), s),
                    lc.layer,
                    lc.selected,
                ),
                result,
            );
            if i < self.children.len() - 1 {
                rect.top += s.height + self.space;
            }
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
