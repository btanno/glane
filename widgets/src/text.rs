use super::*;

pub struct Style {
    font: Option<Font>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            font: None,
        }
    }
}

pub struct Text {
    id: Id,
    style: Style,
    pub text: String,
}

impl Text {
    #[inline]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            style: Default::default(),
            text: text.into(),
        }
    }
}

impl HasId for Text {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Text {
    fn input(&mut self, _ctx: &Context, _input: &Input, _events: &mut Vec<Event>) {}

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
    }
    
    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        let font = self
            .style
            .font
            .as_ref()
            .unwrap_or_else(|| ctx.ctx.default_font.as_ref().unwrap());
        let shape = bounding_box_with_str(&font, &self.text);
        LogicalSize::new(shape.right - shape.left, shape.bottom - shape.top)
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        result.push_back(LayoutElement::text(
            self,
            WidgetState::None,
            LogicalRect::from_position_size(lc.rect.left_top(), size),
            self.text.clone(),
        ));
    }
}
