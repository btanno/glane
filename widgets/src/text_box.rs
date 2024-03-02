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
    composition_text: Vec<Clause>,
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
            composition_text: vec![],
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
            Input::MouseInput(m) => {
                if let Some(l) = ctx.layout.iter().find(|l| l.handle().is(self)) {
                    if l.rect().contains(&m.mouse_state.position) {
                        events.push(Event::new(self, SetFocus));
                    }
                }
            }
            Input::KeyInput(k) => {
                if ctx.has_focus(self) && k.key_state == KeyState::Pressed {
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
            Input::CharInput(c) => {
                if ctx.has_focus(self) {
                    match c {
                        '\x08' => {
                            self.front_text.pop();
                        }
                        _ => {
                            self.front_text.extend(c.nfc());
                        }
                    }
                }
            }
            Input::ImeBeginComposition => {}
            Input::ImeUpdateComposition(clauses) => {
                self.composition_text = clauses.clone();
            }
            Input::ImeEndComposition(Some(result)) => {
                if ctx.has_focus(self) {
                    self.front_text
                        .append(&mut result.chars().collect::<Vec<_>>());
                }
                self.composition_text.clear();
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
        let text = self
            .front_text
            .iter()
            .chain(
                self.composition_text
                    .iter()
                    .map(|t| t.string.iter())
                    .flatten(),
            )
            .chain(self.back_text.iter())
            .collect::<String>();
        let rect = bounding_box_with_str(&font, &text);
        let width = if let Some(ref width) = self.style.width {
            width.get()
        } else {
            5
        };
        let size = {
            let size = font.global_bounding_size();
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

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        let font = self
            .style
            .font
            .as_ref()
            .unwrap_or_else(|| lc.ctx.default_font.as_ref().unwrap());
        let mut rect = LogicalRect::from_position_size(lc.rect.left_top(), size);
        result.push_back(LayoutElement::area(self, self.widget_state, rect));
        rect.left -= self.style.padding.left;
        rect.top -= self.style.padding.top;
        rect.right += self.style.padding.right;
        rect.bottom += self.style.padding.bottom;
        if !self.front_text.is_empty() {
            let front_text = self.front_text.iter().collect::<String>();
            let front_rect = bounding_box_with_str(&font, &front_text);
            rect = LogicalRect::from_position_size(
                LogicalPosition::new(rect.left, rect.top),
                LogicalSize::new(
                    front_rect.right - front_rect.left,
                    front_rect.bottom - front_rect.top,
                ),
            );
            result.push_back(LayoutElement::text(
                self,
                self.widget_state,
                rect,
                front_text,
            ));
        }
        if lc.ctx.has_focus(self) {
            self.composition_text
                .iter()
                .map(|t| {
                    let s = t.string.iter().collect::<String>();
                    let b = bounding_box_with_str(&font, &s);
                    rect = LogicalRect::from_position_size(
                        LogicalPosition::new(rect.right, rect.top),
                        LogicalSize::new(b.right - b.left, b.bottom - b.top),
                    );
                    LayoutElement::composition_text(self, self.widget_state, rect, s, t.targeted)
                })
                .for_each(|elem| {
                    result.push_back(elem);
                });
            let cursor_rect = LogicalRect::from_position_size(
                LogicalPosition::new(rect.right, rect.top),
                LogicalSize::new(2.0, rect.bottom - rect.top),
            );
            result.push_back(LayoutElement::cursor(self, self.widget_state, cursor_rect));
        }
        if !self.back_text.is_empty() {
            let back_text = self.back_text.iter().collect::<String>();
            let back_rect = bounding_box_with_str(&font, &back_text);
            rect = LogicalRect::from_position_size(
                LogicalPosition::new(rect.left, rect.top),
                LogicalSize::new(
                    back_rect.right - back_rect.left,
                    back_rect.bottom - back_rect.top,
                ),
            );
            result.push_back(LayoutElement::text(
                self,
                self.widget_state,
                rect,
                back_text,
            ));
        }
    }
}

impl WidgetMessage for TextBox {
    type Message = Message;
}
