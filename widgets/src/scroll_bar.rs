use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
pub struct Style {
    pub width: f32,
    pub min_thumb_size: f32,
    pub direction: Direction,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            width: 13.0,
            min_thumb_size: 7.0,
            direction: Direction::Vertical,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Changed(usize),
}

#[derive(Debug)]
pub struct Thumb {
    id: Id,
    pub len: usize,
    widget_state: WidgetState,
}

impl Thumb {
    fn new(len: usize) -> Self {
        Self {
            id: Id::new(),
            len,
            widget_state: WidgetState::None,
        }
    }
}

impl HasId for Thumb {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Thumb {
    fn input(&mut self, _ctx: &Context, _input: &Input, _events: &mut Events) -> ControlFlow {
        ControlFlow::Continue
    }

    fn apply(&mut self, _funcs: &mut ApplyFuncs) {}
    fn size(&self, _ctx: &LayoutContext) -> LogicalSize<f32> {
        (0.0, 0.0).into()
    }
    fn layout(&self, _lc: LayoutContext, _result: &mut LayoutConstructor) {}
}

#[derive(Debug)]
pub struct ScrollBar {
    id: Id,
    style: Style,
    pub len: usize,
    pub thumb: Thumb,
    current: usize,
    d: f32,
}

impl ScrollBar {
    #[inline]
    pub fn new(len: usize, thumb_len: usize) -> Self {
        Self {
            id: Id::new(),
            style: Style::default(),
            len,
            current: 0,
            thumb: Thumb::new(thumb_len),
            d: 0.0,
        }
    }

    #[inline]
    pub fn current(&self) -> usize {
        self.current
    }

    #[inline]
    pub fn advance(&mut self, d: isize) {
        use std::cmp::Ordering;
        match d.cmp(&0) {
            Ordering::Less => {
                let d = -d as usize;
                if self.current <= d {
                    self.current = 0;
                } else {
                    self.current -= d;
                }
            }
            Ordering::Greater => {
                let max = self.len - self.thumb.len;
                let d = d as usize;
                if self.current + d >= max {
                    self.current = max;
                } else {
                    self.current += d;
                }
            }
            Ordering::Equal => {}
        }
    }
}

impl HasId for ScrollBar {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for ScrollBar {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        let Some(layout) = ctx.find_layout(self).next() else {
            return ControlFlow::Continue;
        };
        let Some(thumb_layout) = ctx.find_layout(&self.thumb).next() else {
            return ControlFlow::Continue;
        };
        let size = layout.rect().size();
        let thumb_size = thumb_layout.rect().size();
        match input {
            Input::MouseInput(m) => {
                if thumb_layout.rect().contains(&m.mouse_state.position) {
                    if m.button == MouseButton::Left && m.button_state == ButtonState::Pressed {
                        self.d = m.mouse_state.position.y - thumb_layout.rect().top;
                        self.thumb.widget_state = WidgetState::Pressed;
                    } else {
                        self.thumb.widget_state = WidgetState::Hover;
                    }
                } else {
                    self.thumb.widget_state = WidgetState::None;
                }
            }
            Input::CursorMoved(m) => {
                if self.thumb.widget_state == WidgetState::Pressed {
                    match self.style.direction {
                        Direction::Vertical => {
                            let height = size.height - thumb_size.height;
                            let p = m.mouse_state.position.y - layout.rect().top - self.d;
                            let current =
                                (self.len - self.thumb.len) as f32 * p / height;
                            self.current =
                                (current.floor() as usize).min(self.len - self.thumb.len - 1);
                            events.push_message(self, Message::Changed(self.current));
                        }
                        Direction::Horizontal => {}
                    }
                }
            }
            _ => {}
        }
        ControlFlow::Continue
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        match self.style.direction {
            Direction::Vertical => {
                LogicalSize::new(self.style.width, ctx.rect.bottom - ctx.rect.top)
            }
            Direction::Horizontal => {
                LogicalSize::new(ctx.rect.right - ctx.rect.left, self.style.width)
            }
        }
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        match self.style.direction {
            Direction::Vertical => {
                let thumb_size = LogicalSize::new(
                    self.style.width,
                    (self.thumb.len as f32 / self.len as f32) * size.height,
                );
                let thumb_pt = LogicalPosition::new(
                    lc.rect.left,
                    lc.rect.top + (self.current as f32 / self.len as f32) * size.height,
                );
                let rect = LogicalRect::from_position_size(lc.rect.left_top(), size);
                let thumb_rect = LogicalRect::from_position_size(thumb_pt, thumb_size);
                result.push(
                    &lc,
                    LayoutElement::area(self, WidgetState::None, rect, false),
                );
                result.push(
                    &lc,
                    LayoutElement::area(&self.thumb, self.thumb.widget_state, thumb_rect, false),
                );
            }
            Direction::Horizontal => {
                let thumb_size = LogicalSize::new(
                    (self.thumb.len as f32 / self.len as f32) * size.width,
                    self.style.width,
                );
                let thumb_pt = LogicalPosition::new(
                    lc.rect.left + (self.current as f32 / self.len as f32) * size.width,
                    lc.rect.top,
                );
                let rect = LogicalRect::from_position_size(lc.rect.left_top(), size);
                let thumb_rect = LogicalRect::from_position_size(thumb_pt, thumb_size);
                result.push(
                    &lc,
                    LayoutElement::area(self, WidgetState::None, rect, false),
                );
                result.push(
                    &lc,
                    LayoutElement::area(&self.thumb, self.thumb.widget_state, thumb_rect, false),
                );
            }
        }
    }
}

impl WidgetMessage for ScrollBar {
    type Message = Message;
}
