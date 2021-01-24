
struct App;

impl Widget<usize> for App {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut usize, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &usize,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &usize, _data: &usize, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &usize,
        _env: &Env,
    ) -> Size {
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &usize, env: &Env) {

    }
}