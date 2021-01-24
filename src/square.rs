
use druid::{
    theme, Color, Data, Lens, LocalizedString, RenderContext, WidgetExt,
    PaintCtx, Env,
};

use druid::widget::prelude::*;
use druid::widget::{Widget, Label, Painter, BackgroundBrush};

pub struct Square;

impl Widget<usize> for Square {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut usize, _env: &Env) {}

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &usize, _env: &Env) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &usize, _data: &usize, _env: &Env) {}

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &usize,
        _env: &Env,
    ) -> Size {
        bc.constrain((50.0, 50.0))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &usize, env: &Env) {
        let mut background = if ctx.is_hot() {
            BackgroundBrush::Color(Color::rgb8(200, 55, 55))
        } else {
            BackgroundBrush::Color(Color::rgb8(30, 210, 170))
        };
        background.paint(ctx, data, env);

        let painter = Painter::new(|ctx, _, env| {
            let bounds = ctx.size().to_rect();
    
            ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));
    
            if ctx.is_hot() {
                ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
            }
    
            if ctx.is_active() {
                ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
            }
        });

        Label::new(format!("{}", data))
            .with_text_size(24.)
            .center()
            .background(painter)
            .expand();
    }
}