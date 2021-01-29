
use druid::widget::{Label, Painter, CrossAxisAlignment};
use druid::{
    Widget, WidgetPod, EventCtx, Event, Env, theme, Color, Data, PaintCtx,
    Size, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints,
    RenderContext, WidgetExt
};

#[derive(Debug, Clone, Data)]
pub(crate) struct SquareState {
    digit: usize,
}

impl SquareState {
    pub fn new(d: usize) -> Self {
        SquareState {
            digit: d,
        }
    }
    pub fn into_string(&self) -> String {
        format!("{}", self.digit)
    }
}

pub struct Square {
    label: Label<String>,
}

impl Square {
    pub fn new(state: SquareState) -> Self {
        let label_string = state.into_string();
        Self {
            label: Self::from_usize(label_string),
        }
    }

    fn from_usize(text: String) -> Label<String> {

        let mut label = Label::new(text)
            .with_text_size(24.);
        label
    }
}

impl Widget<SquareState> for Square {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut SquareState, _env: &Env) {

    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &SquareState,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &SquareState, data: &SquareState, _env: &Env) {
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &SquareState,
        env: &Env,
    ) -> Size {
        self.label.layout(ctx, bc, &data.into_string(), env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &SquareState, env: &Env) {
        self.label.paint(ctx, &data.into_string(), env);
    }
}