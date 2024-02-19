use super::*;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug)]
pub struct Style {
    pub font: Option<Font>,
    pub width: Option<std::num::NonZeroU32>,
    pub padding: LogicalRect<f32>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            font: None,
            width: None,
            padding: LogicalRect::new(0.5, 0.3, 0.5, 0.3),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    GetFocus,
    Changed,
}

pub struct TextBox {
    id: Id,
    widget_state: WidgetState,
    style: Style,
    front_text: Vec<char>,
    back_text: Vec<char>,
}

impl TextBox {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Id::new(),
            widget_state: WidgetState::None,
            style: Style::default(),
            front_text: vec![],
            back_text: vec![],
        }
    }
}

impl HasId for TextBox {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for TextBox {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Vec<Event>) {
        match input {
            Input::MouseInput(m) => {}
            Input::KeyInput(k) => {
                if k.key_state == KeyState::Pressed {
                    match k.vkey {
                        VirtualKey::Left => {
                            if let Some(c) = self.front_text.pop() {
                                self.back_text.push(c);
                            }
                        }
                        VirtualKey::Right => {
                            if let Some(c) = self.back_text.pop() {
                                self.front_text.push(c);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Input::CharInput(c) => match c {
                '\x08' => {
                    self.front_text.pop();
                }
                _ => {
                    self.front_text.extend(c.nfc());
                }
            },
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
        let text = self
            .front_text
            .iter()
            .chain(self.back_text.iter())
            .collect::<String>();
        let rect = bounding_box_with_str(&font, &text);
        let width = if let Some(ref width) = self.style.width {
            width.get()
        } else {
            6
        };
        let size = {
            let size = font.bounding_size();
            let size = LogicalSize::new(size.width * width as f32, size.height);
            LogicalSize::new(
                if rect.right <= size.width {
                    size.width
                } else {
                    rect.right
                },
                rect.bottom,
            )
        };
        LogicalSize::new(
            size.width + self.style.padding.left + self.style.padding.right,
            size.height + self.style.padding.top + self.style.padding.bottom,
        )
    }

    fn layout(&self, ctx: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&ctx);
        let font = self
            .style
            .font
            .as_ref()
            .unwrap_or_else(|| ctx.ctx.default_font.as_ref().unwrap());
        let rect = LogicalRect::from_position_size(ctx.rect.left_top(), size);
        result.push_back(LayoutElement::area(self, self.widget_state, rect));
        let text = self
            .front_text
            .iter()
            .chain(self.back_text.iter().rev())
            .collect::<String>();
        let rect = LogicalRect::new(
            rect.left + self.style.padding.left,
            rect.top + self.style.padding.top,
            rect.right - self.style.padding.right,
            rect.bottom - self.style.padding.bottom,
        );
        result.push_back(LayoutElement::text(self, self.widget_state, rect, text));
        let front_text = self.front_text.iter().collect::<String>();
        let front_rect = bounding_box_with_str(&font, &front_text);
        let rect = LogicalRect::from_position_size(
            LogicalPosition::new(rect.left, rect.top),
            LogicalSize::new(
                front_rect.right - front_rect.left,
                front_rect.bottom - front_rect.top,
            ),
        );
        let cursor_rect = LogicalRect::from_position_size(
            LogicalPosition::new(rect.left + front_rect.right - 1.0, rect.top),
            LogicalSize::new(2.0, rect.bottom - rect.top),
        );
        result.push_back(LayoutElement::cursor(self, self.widget_state, cursor_rect));
    }
}

impl WidgetMessage for TextBox {
    type Message = Message;
}
