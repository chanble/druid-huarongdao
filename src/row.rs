
pub struct Row {
    
}

impl Widget<Vec<usize>> for Row {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut Vec<usize>, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Vec<usize>,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Vec<usize>, _data: &Vec<usize>, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &Vec<usize>,
        _env: &Env,
    ) -> Size {
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Vec<usize>, env: &Env) {
        
    }
}