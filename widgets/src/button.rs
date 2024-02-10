use super::*;

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
                padding: LogicalRect::new(5.0, 2.0, 5.0, 2.0),
            },
            state: WidgetState::None,
        }
    }
}

impl Widget for Button {
    fn id(&self) -> Id {
        self.id
    }

    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Vec<Event>) {
        let Some(layout) = ctx.find_layout(self).nth(0) else {
            return;
        };
        match input {
            Input::MouseInput(m) => {
                let prev_state = self.state;
                if contains(&layout.rect, &m.mouse_state.position) {
                    if m.button == MouseButton::Left {
                        match m.button_state {
                            ButtonState::Pressed => {
                                if prev_state != WidgetState::Pressed {
                                    events.push(Event::new(
                                        self,
                                        StateChanged::new(WidgetState::Pressed, prev_state),
                                    ));
                                    self.state = WidgetState::Pressed;
                                }
                            }
                            ButtonState::Released => {
                                events.push(Event::new(self, Message::Clicked));
                                if prev_state != WidgetState::Hover {
                                    events.push(Event::new(
                                        self,
                                        StateChanged::new(WidgetState::Hover, prev_state),
                                    ));
                                    self.state = WidgetState::Hover;
                                }
                            }
                        }
                    } else {
                        if prev_state != WidgetState::None {
                            events.push(Event::new(
                                self,
                                StateChanged::new(WidgetState::None, prev_state),
                            ));
                            self.state = WidgetState::None;
                        }
                    }
                } else {
                    if prev_state != WidgetState::None {
                        events.push(Event::new(
                            self,
                            StateChanged::new(WidgetState::None, prev_state),
                        ));
                        self.state = WidgetState::None;
                    }
                }
            }
            Input::CursorMoved(m) => {
                let prev = self.state;
                if contains(&layout.rect, &m.mouse_state.position) {
                    if m.mouse_state.buttons.contains(MouseButton::Left) {
                        if prev != WidgetState::Pressed {
                            events.push(Event::new(
                                self,
                                StateChanged::new(WidgetState::Pressed, prev),
                            ));
                            self.state = WidgetState::Pressed;
                        }
                    } else {
                        if prev != WidgetState::Hover {
                            events.push(Event::new(
                                self,
                                StateChanged::new(WidgetState::Hover, prev),
                            ));
                            self.state = WidgetState::Hover;
                        }
                    }
                } else {
                    if prev != WidgetState::None {
                        events.push(Event::new(self, StateChanged::new(WidgetState::None, prev)));
                        self.state = WidgetState::None;
                    }
                }
            }
            _ => {}
        }
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
    }

    fn layout(&self, ctx: LayoutContext, result: &mut LayoutConstructor) {
        let font = self
            .style
            .font
            .as_ref()
            .unwrap_or_else(|| ctx.ctx.default_font.as_ref().unwrap());
        let rect = ctx.rect.left_top() + shape(&font, &self.text);
        let rect = LogicalRect::new(
            rect.left,
            rect.top,
            rect.right + self.style.padding.left + self.style.padding.right,
            rect.bottom + self.style.padding.top + self.style.padding.bottom,
        );
        result.push_back(self, rect, 0.0, None, self.state);
        let rect = LogicalRect::new(
            rect.left + self.style.padding.left,
            rect.top + self.style.padding.top,
            rect.right - self.style.padding.right,
            rect.bottom - self.style.padding.bottom,
        );
        result.push_back(self, rect, 0.0, Some(self.text.clone()), self.state);
    }
}

impl WidgetMessage for Button {
    type Message = Message;
}
