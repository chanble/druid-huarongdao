use std::time::Duration;
use rand::prelude::*;
use druid::{
    Color, Data, Lens, RenderContext, Widget, WidgetExt,
    Size, Point, Rect, MouseButton, TimerToken,
};
use druid::piet::{FontFamily, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::widget::{Label, Flex, Button};

use num_huarongdao::num_hrd::{NumHrd, Direction};

static TIMER_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Clone, PartialEq, Debug)]
pub enum State {
    Preparation,
    Ready,
    Running,
    Finished,
}

#[derive(Clone, Lens)]
pub struct AppData {
    pub hrd_data: NumHrd,
    win: bool,
    duration: String,
    steps: String,
    tip: String,
    pub state: State,
}

impl Data for AppData {
    fn same(&self, other: &Self) -> bool {
        self.hrd_data == other.hrd_data
        && self.win == other.win
        && self.duration == other.duration
        && self.steps == other.steps
        && self.tip == other.tip
    }
}

impl AppData {

    pub fn new(hrd: NumHrd) -> Self {
        Self {
            hrd_data: hrd,
            win: false,
            duration: format!("0"),
            steps: format!("0"),
            tip: format!(""),
            state: State::Preparation,
        }
    }
    pub fn set_win(&mut self) {
        self.win = true;
        self.tip = format!("恭喜您！");
        self.state = State::Finished;
    }

    pub fn incr_duration(&mut self) {
        let d_usize = self.duration.parse::<usize>().unwrap();
        self.duration = format!("{}", d_usize + 1);
    }

    pub fn incr_step(&mut self) {
        let d_usize = self.steps.parse::<usize>().unwrap();
        println!("d usize: {}", d_usize);
        self.steps = format!("{}", d_usize + 1);
    }

    pub fn start(&mut self) {
        self.steps = format!("0");
        self.tip = format!("");
        self.duration = format!("0");
        self.win = false;
        self.state = State::Running;
    }
}

pub struct HuarongDaoWidget {
    pub cell_size: Size,
    pub timer_id: TimerToken,
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
        Some((col, row))
    }
}

impl Widget<AppData> for HuarongDaoWidget {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {
        match event {
            Event::WindowConnected => {
            }
            Event::MouseUp(e) => {
                if e.button == MouseButton::Left {
                    if data.state == State::Running {
                        let pos = e.pos;
                        println!("MouseUp pos: {}", pos);
                        if let Some(grid_pos) = self.grid_pos(e.pos, *data.hrd_data.size()) {
                            println!("grid pos: {:?}", grid_pos);
                            if data.hrd_data.move_num_by_point(grid_pos) {
                                data.incr_step();
                            }
                        }
                        if data.hrd_data.is_win() {
                            data.set_win();
                            self.timer_id = TimerToken::INVALID;
                        }
                    }
                }
            },
            Event::Timer(id) => {
                // println!("event timer id: {:?}, timer_id: {:?}", *id, self.timer_id);
                if *id == self.timer_id && data.state == State::Running {
                    data.incr_duration();
                    self.timer_id = ctx.request_timer(TIMER_INTERVAL);
                }
            },
            Event::MouseDown(_e) => {
                // println!("MouseDown pos: {}", pos);
            },
            Event::MouseMove(_e) => {
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

        if data.state == State::Running && self.timer_id == TimerToken::INVALID {
            self.timer_id = ctx.request_timer(TIMER_INTERVAL);
        }
        if !data.same(old_data) {
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
        let max_size = bc.max();
        let min_side = max_size.height.min(max_size.width);
        Size {
            width: min_side,
            height: min_side,
        }
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
                let txt_num = *data.hrd_data.num_by_point((row, col)).unwrap();
                let point = Point {
                    x: w0 * col as f64,
                    y: h0 * row as f64,
                };
                let rect = Rect::from_origin_size(point, self.cell_size);
                let border = Rect::from_origin_size(point, self.cell_size);
                ctx.stroke(border, &Color::rgb8(0, 0, 0), 5.0);
                ctx.fill(rect, &Color::rgb8(0x71, 0x71, 0x71));
                let txt_num_n = txt_num.get_n();
                if txt_num_n > 0 {
                    let rect_center = rect.center();
                    let font_size = self.cell_size.width * 0.6;
                    let half_font_size = font_size / 2.0;
                    let text = ctx.text();
                    let layout = text
                        .new_text_layout(txt_num_n.to_string())
                        .font(FontFamily::MONOSPACE, font_size)
                        .text_color(Color::rgb8(128, 0, 0))
                        .build()
                        .unwrap();
                    ctx.draw_text(&layout, (rect_center.x - half_font_size, rect_center.y - half_font_size));
                }
            }
        }
    }
}

fn start_game(_ctx: &mut EventCtx, data: &mut AppData, _env: &Env) {

    let for_end: usize = *data.hrd_data.size() as usize * 100usize;

    for _ in 0..for_end {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0..4);
        let direction = match r {
            0 => Direction::Top,
            1 => Direction::Bottom,
            2 => Direction::Left,
            _ => Direction::Right,
        };
        data.hrd_data.zero_move(&direction).unwrap();
    }
    println!("shuffle over");
    data.state = State::Ready;
    if data.state == State::Ready {
        data.start();
    }
    _ctx.request_paint();
}

pub fn make_widget() -> impl Widget<AppData> {

    Flex::column()
        .with_flex_child(
            Label::new("数字华容道")
                .with_text_size(52.0)
                .padding(5.0),
            1.0,
        )
        .with_child(
            Flex::row()
            .with_child(Label::new("用时："))
            .with_child(Label::new(|data: &String, _env: &_| data.clone()).lens(AppData::duration))
            .with_child(Label::new("秒"))
            .with_child(Label::new("步数："))
            .with_child(Label::new(|data: &String, _env: &_| data.clone()).lens(AppData::steps))
            
        )
        .with_flex_child(
            HuarongDaoWidget {
                timer_id: TimerToken::INVALID,
                cell_size: Size {
                    width: 0.0,
                    height: 0.0,
                },
            },
            2.0,
        )
        .with_child(
            Button::new("开始游戏")
            .padding(15.0).on_click(start_game),
        )
        .with_child(
            Label::new(|data: &String, _env: &_| data.clone())
                .with_text_size(24.0)
                .with_text_color(Color::rgb8(0xf0, 0xf0, 0xea))
                .padding(3.0)
                .lens(AppData::tip)
        )
}