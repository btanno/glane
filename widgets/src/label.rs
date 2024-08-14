use super::*;

#[derive(Default, Debug)]
pub struct Style {
    pub font: Option<Font>,
}

#[derive(Debug)]
pub struct Label {
    id: Id,
    pub text: String,
    pub style: Style,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            text: text.into(),
            style: Default::default(),
        }
    }
}

impl HasId for Label {
    #[inline]
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Label {
    fn input(&mut self, _ctx: &Context, _input: &Input, _events: &mut Events) -> ControlFlow {
        ControlFlow::Continue
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
        let shape = bounding_box_with_str(font, &self.text);
        LogicalSize::new(shape.right - shape.left, shape.bottom - shape.top)
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        result.push(
            &lc,
            LayoutElement::text(
                self,
                WidgetState::None,
                LogicalRect::from_position_size(lc.rect.left_top(), size),
                self.text.clone(),
                false,
            ),
        );
    }
}
