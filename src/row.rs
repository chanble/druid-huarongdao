use std::vec::Vec;

use druid::widget::{Flex, Painter};
use druid::{
    Widget, WidgetPod, EventCtx, Event, Env, theme, Color, Data, PaintCtx, Size,
    LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints,
};
use crate::square::{Square, SquareState};

#[derive(Debug, Clone)]
pub struct RowState {
    digits: Vec<usize>,
}

impl Data for RowState {
    fn same(&self, other: &Self) -> bool {
        self.digits == other.digits
    }
}
impl RowState {
    pub fn new(digits: Vec<usize>) -> Self {
        Self {
            digits,
        }
    }

    pub fn get_square(&self, index: usize) -> SquareState {
        SquareState::new(self.digits[index])
    }
}
pub struct Row {
    squares: Vec<Square>,
}

impl Row {
    pub fn new(state: RowState) -> Self {
        let mut squares: Vec<Square> = Vec::new();
        for i in state.digits {
            let square = Square::new(SquareState::new(
                i
            ));
            squares.push(square);
        }
        Self {
            squares
        }
    }
}

impl Widget<RowState> for Row {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut RowState, _env: &Env) {

    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &RowState,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &RowState, data: &RowState, _env: &Env) {
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &RowState,
        env: &Env,
    ) -> Size {
        let mut width = 0f64;
        let mut height = 0f64;
        let mut index = 0;
        if self.squares.len() < 1 {
            return bc.max();
        }
        for square in &mut self.squares {
            let square_size = square.layout(ctx, bc, &data.get_square(index.clone()), env);
            width += square_size.width;
            height = square_size.height;
            index += 1;
        }
        bc.constrain(Size {
            width,
            height,
        })
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &RowState, env: &Env) {
        let mut flex = Flex::row();
        let mut index = 0;
        for square in &mut self.squares {
            square.paint(ctx, &data.get_square(index.clone()), env);
            flex.add_flex_child(*square, 1.0);
            index += 1;
        }
    }
}