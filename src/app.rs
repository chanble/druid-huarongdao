
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc, Size
};
use druid::widget::prelude::*;
use druid::widget::{Label, Painter, BackgroundBrush, Flex};

use num_huarongdao::num_hrd::NumHrd;
use crate::row::{Row, RowState};

#[derive(Clone, Lens)]
pub struct AppState {
    hrd_data: Vec<Vec<usize>>,
}

impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        self.hrd_data == other.hrd_data
    }
}

impl AppState {
    pub fn new(hrd: NumHrd) -> Self {
        Self {
            hrd_data: hrd.as_2d_vec(),
        }
    }

    pub fn click(&mut self, n: usize) {
        
    }

    pub fn row(&self, index: u8) -> RowState {
        let mut row: Vec<usize> = Vec::new();
        row.extend_from_slice(&self.hrd_data[index as usize]);
        RowState::new(row)
    }

    pub fn get_hrd_data(&self) -> &Vec<Vec<usize>> {
        &self.hrd_data
    }
}

pub struct AppWidget {
    rows: Vec<Row>,
}

impl AppWidget {
    pub fn new(data: AppState) -> Self {
        let mut rows: Vec<Row> = Vec::new();
        for row in data.get_hrd_data() {
            let row = Row::new(RowState::new(row.clone()));
            rows.push(row);
        }
        Self {
            rows,
        }
    }
}

impl Widget<AppState> for AppWidget {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, _env: &Env) {

        if data.hrd_data != old_data.hrd_data {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        if self.rows.len() < 1 {
            return bc.max();
        }
        let row_size = self.rows[0].layout(ctx, bc, &data.row(0), env);
        bc.constrain(Size {
            width: row_size.width,
            height: row_size.height * self.rows.len() as f64,
        })
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let mut index = 0;
        for row in &mut self.rows {
            row.paint(ctx, &data.row(index), env);
            index += 1;
        }
    }
}
