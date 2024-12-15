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
        let mut size = LogicalSize::new(0.0, 0.0);
        for (i, child) in self.children.iter().enumerate() {
            if child.size_types().width != SizeType::Fix {
                continue;
            }
            let s = child.size(ctx);
            size.width += s.width;
            if i < self.children.len() - 1 {
                size.width += self.space;
            }
            size.height = s.height.max(size.height);
        }
        let flexible_count = self
            .children
            .iter()
            .filter(|child| child.size_types().width == SizeType::Flexible)
            .count();
        if flexible_count > 0 {
            size.width += ctx.rect.size().width - size.width;
            let last = self.children.last().unwrap();
            size.width += self.space
                * (flexible_count
                    - if last.size_types().width == SizeType::Flexible {
                        1
                    } else {
                        0
                    }) as f32;
        }
        size.width = size.width.min(ctx.rect.size().width);
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
        let mut max_height = 0.0f32;
        let mut w = 0.0;
        for (i, child) in self.children.iter().enumerate() {
            if child.size_types().width == SizeType::Flexible {
                w += self.space;
                sizes.push(None);
            } else {
                let s = child.size(&lc);
                max_height = max_height.max(s.height);
                let width = s.width;
                if w + width <= size.width {
                    sizes.push(Some(LogicalSize::new(width, s.height)));
                    w += width;
                    if i < self.children.len() - 1 {
                        w += self.space;
                    }
                } else {
                    let width = size.width - w;
                    if width > 0.0 {
                        sizes.push(Some(LogicalSize::new(width, s.height)));
                    } else {
                        sizes.push(None);
                    }
                };
            }
        }
        let flexible_count = self
            .children
            .iter()
            .filter(|child| child.size_types().width == SizeType::Flexible)
            .count();
        let flexible_width = if flexible_count > 0 {
            (size.width - w) / flexible_count as f32
        } else {
            0.0
        };
        for (i, child) in self.children.iter().enumerate() {
            let s = match child.size_types().width {
                SizeType::Flexible => {
                    let size = child.size(&lc);
                    let height = max_height.max(size.height);
                    LogicalSize::new(flexible_width, height)
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
                rect.left += s.width + self.space;
            }
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
