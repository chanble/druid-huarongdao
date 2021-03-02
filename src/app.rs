
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc, Size, Point, Rect, MouseButton,
};
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::widget::{Label, Painter, BackgroundBrush, Flex, CrossAxisAlignment};

use num_huarongdao::num_hrd::{NumHrd, Direction};

#[derive(Clone, Lens)]
pub struct AppData {
    pub hrd_data: NumHrd,
    pub state: String,
}

impl Data for AppData {
    fn same(&self, other: &Self) -> bool {
        self.hrd_data == other.hrd_data
    }
}


pub struct HuarongDaoWidget {
    pub cell_size: Size,
}

impl HuarongDaoWidget {
    fn grid_pos(&self, p: Point, grid_size: u8) -> Option<(u8, u8)> {
        let w0 = self.cell_size.width;
        let h0 = self.cell_size.height;
        if p.x < 0.0 || p.y < 0.0 || w0 == 0.0 || h0 == 0.0 {
            return None;
        }
        let row = (p.x / w0) as u8;
        let col = (p.y / h0) as u8;
        if row >= grid_size || col >= grid_size {
            return None;
        }
        Some((row, col))
    }
}

impl Widget<AppData> for HuarongDaoWidget {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {
        match event {
            Event::MouseUp(e) => {
                if e.button == MouseButton::Left {
                    let pos = e.pos;
                    println!("MouseUp pos: {}", pos);
                    if let Some(grid_pos) = self.grid_pos(e.pos, *data.hrd_data.size()) {
                        data.hrd_data.move_num_by_point(grid_pos);
                    }
                }
            },
            Event::MouseDown(e) => {
                let pos = e.pos;
                // println!("MouseDown pos: {}", pos);

            },
            Event::MouseMove(e) => {
                // 
            },
            _ => {}
        }
    }

    
    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppData,
        _env: &Env,
    ) {
    }

    
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppData, data: &AppData, _env: &Env) {
        if data.hrd_data != old_data.hrd_data {
            ctx.request_paint();
        }
    }

    
    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppData,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    
    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &Env) {
        let grid_size = *data.hrd_data.size();
        let size: Size = ctx.size();
        let w0 = size.width / grid_size as f64;
        let h0 = size.height / grid_size as f64;
        self.cell_size = Size {
            width: w0,
            height: h0,
        };
        for row in 0..grid_size {
            for col in 0..grid_size {
                let txt_num = *data.hrd_data.get_by_point((row, col)).unwrap();
                let point = Point {
                    x: w0 * col as f64,
                    y: h0 * row as f64,
                };
                let rect = Rect::from_origin_size(point, self.cell_size);
                let border = Rect::from_origin_size(point, self.cell_size);
                
                ctx.stroke(border, &Color::rgb8(0, 0, 0), 5.0);
                ctx.fill(rect, &Color::rgb8(0x71, 0x71, 0x71));
                
                let rect_center = rect.center();
                let font_size = 48.0;
                let half_font_size = font_size / 2.0;
                let text = ctx.text();
                let layout = text
                    .new_text_layout(txt_num.get_n().to_string())
                    .font(FontFamily::MONOSPACE, font_size)
                    .text_color(Color::rgb8(128, 0, 0))
                    .build()
                    .unwrap();
                ctx.draw_text(&layout, (rect_center.x - half_font_size, rect_center.y - half_font_size));
            }
        }
    }
}
