use super::*;

pub struct InnerFrame {
    id: Id,
    viewport: LogicalSize<f32>,
    position: LogicalPosition<f32>,
    size: LogicalSize<f32>,
    vscroll: VScrollBar,
    hscroll: HScrollBar,
    children: Vec<Box<dyn Widget>>,
}

impl InnerFrame {
    #[inline]
    pub fn new(viewport: impl Into<LogicalSize<f32>>, size: impl Into<LogicalSize<f32>>) -> Self {
        let viewport = viewport.into();
        let size = size.into();
        Self {
            id: Id::new(),
            viewport,
            position: LogicalPosition::new(0.0, 0.0),
            size,
            vscroll: VScrollBar::new(size.height.ceil() as usize, viewport.height.ceil() as usize),
            hscroll: HScrollBar::new(size.width.ceil() as usize, viewport.width.ceil() as usize),
            children: vec![],
        }
    }
}

impl HasId for InnerFrame {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for InnerFrame {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        let area = ctx.find_layout(self).find_map(|l| l.as_area());
        if let Some(area) = area {
            match input {
                Input::MouseInput(ev) => {
                    if !area.rect.contains(&ev.mouse_state.position) {
                        return ControlFlow::Continue;
                    }
                }
                Input::CursorMoved(ev) => {
                    if !area.rect.contains(&ev.mouse_state.position) {
                        return ControlFlow::Continue;
                    }
                }
                _ => {}
            }
        } else {
            return ControlFlow::Continue;
        }
        for child in self.children.iter_mut() {
            if ctx.find_layout(child.as_ref()).count() > 0 {
                if child.input(ctx, input, events) == ControlFlow::Break {
                    return ControlFlow::Break;
                }
            }
        }
        if self.vscroll.input(ctx, input, events) == ControlFlow::Break {
            return ControlFlow::Break;
        }
        if self.hscroll.input(ctx, input, events) == ControlFlow::Break {
            return ControlFlow::Break;
        }
        if let Some(vs) = events
            .iter()
            .rev()
            .find_map(|event| event.message(&self.vscroll))
        {
            let scroll_bar::Message::Changed(ev) = vs;
            self.position.y = *ev as f32;
        } else if let Some(hs) = events
            .iter()
            .rev()
            .find_map(|event| event.message(&self.hscroll))
        {
            let scroll_bar::Message::Changed(ev) = hs;
            self.position.x = *ev as f32;
        }
        ControlFlow::Continue
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
        self.vscroll.apply(funcs);
        self.hscroll.apply(funcs);
        self.children
            .iter_mut()
            .for_each(|child| child.apply(funcs));
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        let vs = self.vscroll.size(ctx);
        let hs = self.hscroll.size(ctx);
        LogicalSize::new(
            self.viewport.width + vs.width,
            self.viewport.height + hs.height,
        )
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        result.push(
            &lc,
            LayoutElement::area(
                self,
                WidgetState::None,
                LogicalRect::from_position_size(lc.rect.left_top(), self.viewport),
                false,
            ),
        );
        result.push(
            &lc,
            LayoutElement::start_clipping(
                self,
                LogicalRect::from_position_size(lc.rect.left_top(), self.viewport),
            ),
        );
        let mut position = LogicalPosition::new(-self.position.x, -self.position.y);
        for child in self.children.iter() {
            let size = child.size(&lc);
            let range = position.x + size.width >= 0.0 && position.y + size.height >= 0.0;
            if range {
                child.layout(
                    lc.next(
                        LogicalRect::from_position_size(
                            LogicalPosition::new(
                                lc.rect.left + position.x,
                                lc.rect.top + position.y,
                            ),
                            LogicalSize::new(self.size.width, self.size.height - position.y),
                        ),
                        lc.layer,
                        lc.selected,
                    ),
                    result,
                );
            }
            position.y += size.height;
            if position.y >= self.viewport.height {
                break;
            }
        }
        result.push(
            &lc,
            LayoutElement::end_clipping(
                self,
                LogicalRect::from_position_size(lc.rect.left_top(), self.viewport),
            ),
        );
        let vscroll_size = self.vscroll.size(&lc.next(
            LogicalRect::from_position_size(lc.rect.left_top(), self.viewport),
            lc.layer,
            lc.selected,
        ));
        self.vscroll.layout(
            lc.next(
                LogicalRect::from_position_size(
                    LogicalPosition::new(
                        lc.rect.left + self.viewport.width - vscroll_size.width,
                        lc.rect.top,
                    ),
                    vscroll_size,
                ),
                lc.layer,
                lc.selected,
            ),
            result,
        );
        let hscroll_size = self.hscroll.size(&lc.next(
            LogicalRect::from_position_size(
                lc.rect.left_top(),
                LogicalSize::new(
                    self.viewport.width - vscroll_size.width,
                    self.viewport.height,
                ),
            ),
            lc.layer,
            lc.selected,
        ));
        self.hscroll.layout(
            lc.next(
                LogicalRect::from_position_size(
                    LogicalPosition::new(
                        lc.rect.left,
                        lc.rect.top + self.viewport.height - hscroll_size.height,
                    ),
                    hscroll_size,
                ),
                lc.layer,
                lc.selected,
            ),
            result,
        );
    }
}

impl HasChildren for InnerFrame {
    fn len(&self) -> usize {
        self.children.len()
    }

    fn push(&mut self, child: impl Widget) {
        self.children.push(Box::new(child));
    }

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
