#![feature(let_else)]
#![feature(async_closure)]

pub mod cell;

use cell::Cellule;
use gloo::timers::callback::Interval;
use log::info;
use rand::Rng;
use web_sys::window;
use yew::html::Scope;
use yew::prelude::*;

pub enum GOFMsg {
    Random,
    Start,
    Step,
    Reset,
    Stop,
    ToggleCellule(usize),
    Tick,
}

pub struct GameOfLife {
    active: bool,
    cellules: Vec<Cellule>,
    inner_width: usize,
    cellules_width: usize,
    cellules_height: usize,
    _interval: Interval,
}

impl GameOfLife {
    pub fn random_mutate(&mut self) {
        for cellule in self.cellules.iter_mut() {
            if rand::thread_rng().gen() {
                cellule.set_alive();
            } else {
                cellule.set_dead();
            }
        }
    }

    fn reset(&mut self) {
        for cellule in self.cellules.iter_mut() {
            cellule.set_dead();
        }
    }

    fn step(&mut self) {
        let mut to_dead = Vec::new();
        let mut to_live = Vec::new();
        for row in 0..self.cellules_height {
            for col in 0..self.cellules_width {
                let neighbors = self.neighbors(row as isize, col as isize);

                let current_idx = self.row_col_as_idx(row as isize, col as isize);
                if self.cellules[current_idx].is_alive() {
                    if Cellule::alone(&neighbors) || Cellule::overpopulated(&neighbors) {
                        to_dead.push(current_idx);
                    }
                } else if Cellule::can_be_revived(&neighbors) {
                    to_live.push(current_idx);
                }
            }
        }
        to_dead
            .iter()
            .for_each(|idx| self.cellules[*idx].set_dead());
        to_live
            .iter()
            .for_each(|idx| self.cellules[*idx].set_alive());
    }

    fn neighbors(&self, row: isize, col: isize) -> [Cellule; 8] {
        [
            self.cellules[self.row_col_as_idx(row + 1, col)],
            self.cellules[self.row_col_as_idx(row + 1, col + 1)],
            self.cellules[self.row_col_as_idx(row + 1, col - 1)],
            self.cellules[self.row_col_as_idx(row - 1, col)],
            self.cellules[self.row_col_as_idx(row - 1, col + 1)],
            self.cellules[self.row_col_as_idx(row - 1, col - 1)],
            self.cellules[self.row_col_as_idx(row, col - 1)],
            self.cellules[self.row_col_as_idx(row, col + 1)],
        ]
    }

    fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cellules_height as isize);
        let col = wrap(col, self.cellules_width as isize);

        row * self.cellules_width + col
    }

    fn view_cellule(&self, idx: usize, cellule: &Cellule, link: &Scope<Self>) -> Html {
        let cellule_status = {
            if cellule.is_alive() {
                "cellule-live"
            } else {
                "cellule-dead"
            }
        };
        html! {
            <div key={idx} class={classes!("game-cellule", cellule_status)}
                onclick={link.callback(move |_| GOFMsg::ToggleCellule(idx))}>
            </div>
        }
    }
}
impl Component for GameOfLife {
    type Message = GOFMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| GOFMsg::Tick);
        let interval = Interval::new(2000, move || callback.emit(()));

        let window = window().expect("There should be a window");
        let inner_width = window
            .inner_width()
            .expect("a window should have an inner width")
            .as_f64()
            .expect("Should be a number");
        let inner_height = window
            .inner_height()
            .expect("a window should have an inner height")
            .as_f64()
            .expect("Should be a number");

        info!("{}x{}", inner_width, inner_height);

        let (cellules_width, cellules_height) = (
            ((inner_width - 2.0 * (inner_width / 25.0)) / 25.0).ceil() as usize + 1,
            ((inner_height - 2.0 * (inner_height / 25.0)) / 25.0).ceil() as usize,
        );

        info!("{} {}", cellules_width, cellules_height);

        ctx.link()
            .send_message_batch(vec![GOFMsg::Random, GOFMsg::Start]);

        Self {
            active: false,
            cellules: vec![Cellule::new_dead(); cellules_width * cellules_height],
            inner_width: inner_width as usize,
            cellules_width,
            cellules_height,
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GOFMsg::Random => {
                self.random_mutate();
                log::info!("Random");
                true
            }
            GOFMsg::Start => {
                self.active = true;
                log::info!("Start");
                false
            }
            GOFMsg::Step => {
                self.step();
                true
            }
            GOFMsg::Reset => {
                self.reset();
                log::info!("Reset");
                true
            }
            GOFMsg::Stop => {
                self.active = false;
                log::info!("Stop");
                false
            }
            GOFMsg::ToggleCellule(idx) => {
                let cellule = self.cellules.get_mut(idx).unwrap();
                cellule.toggle();
                true
            }
            GOFMsg::Tick => {
                if self.active {
                    self.step();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows =
            self.cellules
                .chunks(self.cellules_width)
                .enumerate()
                .map(|(y, cellules)| {
                    let idx_offset = y * self.cellules_width;

                    let cells = cellules
                        .iter()
                        .enumerate()
                        .map(|(x, cell)| self.view_cellule(idx_offset + x, cell, ctx.link()));
                    html! {
                        <div key={y} class="game-row" style={format!("min-width: {}px; height: 25px", self.inner_width + 50)}>
                            { for cells }
                        </div>
                    }
                });

        html! {
            <div>
                <section class="game-container">
                    <section class="game-area">
                        <div class="game-of-life">
                            { for cell_rows }
                        </div>
                        <div class="game-buttons">
                            <button class="game-button" onclick={ctx.link().callback(|_| GOFMsg::Random)}>{ "Random" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| GOFMsg::Step)}>{ "Step" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| GOFMsg::Start)}>{ "Start" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| GOFMsg::Stop)}>{ "Stop" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| GOFMsg::Reset)}>{ "Reset" }</button>
                        </div>
                    </section>
                </section>
                <footer class="app-footer">
                    <strong class="footer-text">
                      { "Game of Life - a yew experiment " }
                    </strong>
                    <a href="https://github.com/yewstack/yew" target="_blank">{ "source" }</a>
                </footer>
            </div>
        }
    }
}

fn wrap(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        coord + range
    } else if coord >= range {
        coord - range
    } else {
        coord
    };
    result as usize
}
