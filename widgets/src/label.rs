use super::*;

pub struct Style {
    pub font: Option<Font>,
}

impl Default for Style {
    fn default() -> Self {
        Self { font: None }
    }
}

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

impl Widget for Label {
    fn id(&self) -> Id {
        self.id
    }

    fn input(&mut self, _ctx: &Context, _input: &Input, _events: &mut Vec<Event>) {}

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
    }

    fn layout(&self, ctx: LayoutContext, result: &mut LayoutConstructor) {
        let font = self
            .style
            .font
            .as_ref()
            .unwrap_or_else(|| ctx.ctx.default_font.as_ref().unwrap());
        let shape = shape(&font, &self.text);
        result.push_back(
            self,
            shape + ctx.rect.left_top(),
            ctx.z,
            Some(self.text.clone()),
            WidgetState::None,
        );
    }
}

