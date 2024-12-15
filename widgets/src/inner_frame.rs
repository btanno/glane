use super::*;
use std::cell::RefCell;

pub struct InnerFrame {
    id: Id,
    position: LogicalPosition<f32>,
    size: LogicalSize<f32>,
    vscroll: RefCell<VScrollBar>,
    hscroll: RefCell<HScrollBar>,
    children: Vec<Box<dyn Widget>>,
    entered: bool,
}

impl InnerFrame {
    #[inline]
    pub fn new(size: impl Into<LogicalSize<f32>>) -> Self {
        let size = size.into();
        Self {
            id: Id::new(),
            position: LogicalPosition::new(0.0, 0.0),
            size,
            vscroll: RefCell::new(VScrollBar::new(size.height.ceil() as usize, 1)),
            hscroll: RefCell::new(HScrollBar::new(size.width.ceil() as usize, 1)),
            children: vec![],
            entered: false,
        }
    }

    #[inline]
    pub fn virtual_size(&self) -> LogicalSize<f32> {
        self.size
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
        let left = if let Some(area) = area {
            let rc = area.rect;
            match input {
                Input::MouseInput(ev) => {
                    if rc.contains(&ev.mouse_state.position) {
                        self.entered = true;
                        None
                    } else if self.entered {
                        self.entered = false;
                        Some(Input::CursorLeft(CursorLeft {
                            mouse_state: ev.mouse_state.clone(),
                        }))
                    } else {
                        None
                    }
                }
                Input::CursorMoved(ev) => {
                    if rc.contains(&ev.mouse_state.position) {
                        self.entered = true;
                        None
                    } else if self.entered {
                        self.entered = false;
                        Some(Input::CursorLeft(CursorLeft {
                            mouse_state: ev.mouse_state.clone(),
                        }))
                    } else {
                        None
                    }
                }
                Input::MouseWheel(ev) => {
                    if self.entered || rc.contains(&ev.mouse_state.position) {
                        let mut vbar = self.vscroll.borrow_mut();
                        let a = ctx.default_font.as_ref().map(|df| df.size).unwrap_or(1.0) as isize;
                        let d = a * ev.distance as isize;
                        vbar.advance(d);
                        self.position.y = (self.position.y + d as f32)
                            .max(0.0)
                            .min(self.size.height - vbar.thumb.len as isize as f32);
                    }
                    None
                }
                _ => None,
            }
        } else {
            None
        };
        let layer = if let Some(area) = area { area.layer } else { 0 };
        for child in self.children.iter_mut() {
            if let Some(i) = left.as_ref() {
                if child.input(ctx, i, events) == ControlFlow::Break {
                    return ControlFlow::Break;
                }
            } else if self.entered {
                if child.input(ctx, input, events) == ControlFlow::Break {
                    return ControlFlow::Break;
                }
            } else {
                let layered_children = ctx.layout.iter().filter(|l| {
                    (l.handle().id() == child.id()
                        || l.ancestors().iter().any(|a| a.id() == child.id()))
                        && l.layer() > layer
                });
                if layered_children.count() > 0
                    && child.input(ctx, input, events) == ControlFlow::Break
                {
                    return ControlFlow::Break;
                }
            }
        }
        if self.vscroll.borrow_mut().input(ctx, input, events) == ControlFlow::Break {
            return ControlFlow::Break;
        }
        if self.hscroll.borrow_mut().input(ctx, input, events) == ControlFlow::Break {
            return ControlFlow::Break;
        }
        use std::ops::Deref;
        let vscroll = self.vscroll.borrow();
        let hscroll = self.hscroll.borrow();
        if let Some(vs) = events
            .iter()
            .rev()
            .find_map(|event| event.message(vscroll.deref()))
        {
            let scroll_bar::Message::Changed(ev) = vs;
            self.position.y = *ev as f32;
        } else if let Some(hs) = events
            .iter()
            .rev()
            .find_map(|event| event.message(hscroll.deref()))
        {
            let scroll_bar::Message::Changed(ev) = hs;
            self.position.x = *ev as f32;
        }
        ControlFlow::Continue
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
        self.vscroll.borrow_mut().apply(funcs);
        self.hscroll.borrow_mut().apply(funcs);
        self.children
            .iter_mut()
            .for_each(|child| child.apply(funcs));
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        ctx.rect.size()
    }

    fn size_types(&self) -> SizeTypes {
        SizeTypes::flexible()
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let viewport = {
            let size = lc.rect.size();
            let vs_size = self.vscroll.borrow().size(&lc);
            let hs_size = self.hscroll.borrow().size(&lc);
            LogicalSize::new(size.width - vs_size.width, size.height - hs_size.height)
        };
        {
            let mut hscroll = self.hscroll.borrow_mut();
            let mut vscroll = self.vscroll.borrow_mut();
            hscroll.thumb.len = viewport.width.ceil() as usize;
            vscroll.thumb.len = viewport.height.ceil() as usize;
        }
        result.push(
            &lc,
            LayoutElement::area(
                self,
                WidgetState::None,
                LogicalRect::from_position_size(lc.rect.left_top(), viewport),
                &lc.ancestors,
                lc.layer,
                false,
            ),
        );
        result.push(
            &lc,
            LayoutElement::start_clipping(
                self,
                LogicalRect::from_position_size(lc.rect.left_top(), viewport),
                &lc.ancestors,
                lc.layer,
            ),
        );
        let mut position = LogicalPosition::new(-self.position.x, -self.position.y);
        for child in self.children.iter() {
            let size = child.size(&lc);
            let range = position.x + size.width >= 0.0 && position.y + size.height >= 0.0;
            if range {
                child.layout(
                    lc.next(
                        self,
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
            if position.y >= viewport.height {
                break;
            }
        }
        result.push(
            &lc,
            LayoutElement::end_clipping(
                self,
                LogicalRect::from_position_size(lc.rect.left_top(), viewport),
                &lc.ancestors,
                lc.layer,
            ),
        );
        let hscroll_size = self.hscroll.borrow().size(&lc.next(
            self,
            LogicalRect::from_position_size(lc.rect.left_top(), viewport),
            lc.layer,
            lc.selected,
        ));
        let vscroll_size = self.vscroll.borrow().size(&lc.next(
            self,
            LogicalRect::from_position_size(
                lc.rect.left_top(),
                LogicalSize::new(viewport.width, viewport.height + hscroll_size.height),
            ),
            lc.layer,
            lc.selected,
        ));
        self.vscroll.borrow().layout(
            lc.next(
                self,
                LogicalRect::from_position_size(
                    LogicalPosition::new(lc.rect.left + viewport.width, lc.rect.top),
                    vscroll_size,
                ),
                lc.layer,
                lc.selected,
            ),
            result,
        );
        self.hscroll.borrow().layout(
            lc.next(
                self,
                LogicalRect::from_position_size(
                    LogicalPosition::new(lc.rect.left, lc.rect.top + viewport.height),
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
