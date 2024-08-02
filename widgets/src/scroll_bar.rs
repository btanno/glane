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

#[derive(Debug)]
pub enum Message {
    Changed(usize),
}

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
    fn input(&mut self, _ctx: &Context, _input: &Input, _events: &mut Vec<Event>) {}
    fn apply(&mut self, _funcs: &mut ApplyFuncs) {}
    fn size(&self, _ctx: &LayoutContext) -> LogicalSize<f32> {
        (0.0, 0.0).into()
    }
    fn layout(&self, _lc: LayoutContext, _result: &mut LayoutConstructor) {}
}

pub struct ScrollBar {
    id: Id,
    style: Style,
    pub len: usize,
    pub thumb: Thumb,
    current: usize,
    d: f32,
}

impl ScrollBar {
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

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn add(&mut self, d: usize) {
        self.current += d;
    }

    pub fn sub(&mut self, d: usize) {
        self.current -= d;
    }
}

impl HasId for ScrollBar {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for ScrollBar {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Vec<Event>) {
        let Some(layout) = ctx.find_layout(self).next() else {
            return;
        };
        let Some(thumb_layout) = ctx.find_layout(&self.thumb).next() else {
            return;
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
                                (self.len - self.thumb.len) as f32 * p as f32 / height as f32;
                            self.current =
                                (current.floor() as usize).min(self.len - self.thumb.len - 1);
                            events.push(Event::new(self, Message::Changed(self.current)));
                        }
                        Direction::Horizontal => {}
                    }
                }
            }
            _ => {}
        }
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
                result.push_back(LayoutElement::area(self, WidgetState::None, rect));
                result.push_back(LayoutElement::area(
                    &self.thumb,
                    self.thumb.widget_state,
                    thumb_rect,
                ));
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
                result.push_back(LayoutElement::area(self, WidgetState::None, rect));
                result.push_back(LayoutElement::area(
                    &self.thumb,
                    self.thumb.widget_state,
                    thumb_rect,
                ));
            }
        }
    }
}

impl WidgetMessage for ScrollBar {
    type Message = Message;
}
