use super::*;

#[derive(Debug)]
pub struct Style {
    pub font: Option<Font>,
    pub padding: LogicalRect<f32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Message {
    Clicked,
}

pub struct Button {
    id: Id,
    pub text: String,
    pub style: Style,
    state: WidgetState,
}

impl Button {
    #[inline]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            text: text.into(),
            style: Style {
                font: None,
                padding: LogicalRect::new(7.0, 3.0, 7.0, 3.0),
            },
            state: WidgetState::None,
        }
    }
}

impl HasId for Button {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Button {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Vec<Event>) {
        let Some(layout) = ctx.find_layout(self).nth(0) else {
            return;
        };
        match input {
            Input::MouseInput(m) => {
                let prev_state = self.state;
                let state = WidgetState::current(layout.rect(), &m.mouse_state);
                let clicked = m.button == MouseButton::Left
                    && m.button_state == ButtonState::Released
                    && state == WidgetState::Hover;
                if clicked {
                    events.push(Event::new(self, Message::Clicked));
                }
                if state != prev_state {
                    events.push(Event::new(self, StateChanged::new(state, prev_state)));
                    self.state = state;
                }
            }
            Input::CursorMoved(m) => {
                let prev_state = self.state;
                let state = WidgetState::current(layout.rect(), &m.mouse_state);
                if state != prev_state {
                    events.push(Event::new(self, StateChanged::new(state, prev_state)));
                    self.state = state;
                }
            }
            _ => {}
        }
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        let font = self
            .style
            .font
            .as_ref()
            .unwrap_or_else(|| ctx.ctx.default_font.as_ref().unwrap());
        let rect = bounding_box_with_str(&font, &self.text);
        LogicalSize::new(
            rect.right + self.style.padding.left + self.style.padding.right,
            rect.bottom + self.style.padding.top + self.style.padding.bottom,
        )
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        let rect = LogicalRect::from_position_size(lc.rect.left_top(), size);
        result.push_back(LayoutElement::area(self, self.state, rect));
        let rect = LogicalRect::new(
            rect.left + self.style.padding.left,
            rect.top + self.style.padding.top,
            rect.right - self.style.padding.right,
            rect.bottom - self.style.padding.bottom,
        );
        result.push_back(LayoutElement::text(
            self,
            self.state,
            rect,
            self.text.clone(),
        ));
    }
}

impl WidgetMessage for Button {
    type Message = Message;
}
