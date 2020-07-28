extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

use glutin_window::GlutinWindow;
use piston::window::{WindowSettings, Size};
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};
use std::collections::LinkedList;
use rand::Rng;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Right, Left, Up, Down,
}

#[derive(Eq, PartialEq)]
enum FoodState{
    Fresh,
    Eaten,
}

#[derive(Copy, Clone)]
struct RGBTColor(f32, f32, f32, f32);

impl From<RGBTColor> for [f32; 4]{
    fn from(color: RGBTColor) -> Self {
        [color.0, color.1, color.2, color.3]
    }
}

struct Food {
    pos_x: i16,
    pos_y: i16,
    color: RGBTColor,
    state: FoodState,
}

impl Food{
    fn set_state(&mut self, state: FoodState){
        self.state = state;
    }
}

struct Snake {
    dir: Direction,
    body: LinkedList<Direction>,
    color: RGBTColor,
    pos_x: i16,
    pos_y: i16,
}

impl Snake{

    fn new(pos_x: i16, pos_y: i16,) -> Snake {
        let direction = Direction::Up;
        println!("size of Direction enum = {} bytes", std::mem::size_of_val(&direction));

        Snake {
            dir: Direction::Right,
            color: RGBTColor(1.0, 0.0, 0.0, 1.0),
            pos_x,
            pos_y,
            body: Default::default(),
        }
    }

    fn grow(&mut self){
        // self.body.push_front(self.dir.clone());
        self.body.push_back(*self.body.back().unwrap());
    }
}

struct Game{
    snake: Snake,
    snacks: Vec<Food>,
    square_size: i16,
    background_color: RGBTColor,
    game_paused: bool,
    field_width: i16,
    field_height: i16,
    level: u8,
}

impl Game {
    fn new(sqr_size: i16, field_size: (i16, i16)) -> Game {
        Game {
            snake: Snake::new(10, 10),
            snacks: Vec::new(),
            square_size: sqr_size,
            background_color: RGBTColor(0.0, 0.9, 0.3, 1.0),
            game_paused: true,
            field_width: field_size.0,
            field_height: field_size.1,
            level: 0,
        }
    }

    fn game_over(&mut self){
        self.game_paused = true;
        self.snake.pos_x = 10;
        self.snake.pos_y = 10;
        self.level = 0;
        self.snake.body.clear();
        self.next_level();
    }

    fn update(&mut self) {
        if self.snacks.len() < 1 {
            self.next_level();
        }

        self.snacks.retain(|elem| elem.state == FoodState::Fresh);

        for snack in &mut self.snacks {
            if self.snake.pos_x == snack.pos_x && self.snake.pos_y == snack.pos_y {

                snack.set_state(FoodState::Eaten);
                self.snake.grow();
            }
        }

        if !self.game_paused {
            self.move_snake();
        }
    }

    fn next_level(&mut self) {
        self.snacks.clear();
        self.level += 1;
        for _ in 0..=self.level {
            let mut thread_random_gen = rand::thread_rng();
            let x = thread_random_gen.gen_range(0, self.field_width);
            let y = thread_random_gen.gen_range(0, self.field_height);
            let snack = Food {
                pos_x: x as i16,
                pos_y: y as i16,
                color: RGBTColor(0.17, 0.1, 0.67, 1.0),
                state: FoodState::Fresh
            };
            self.snacks.push(snack);
        }
    }

    fn move_snake(&mut self) {
        use crate::Direction::{Right, Down, Left, Up};
        self.snake.body.pop_back();

        match self.snake.dir {
            Right => {
                self.snake.pos_x =
                    (self.snake.pos_x + 1 + self.field_width as i16) % self.field_width as i16
            },
            Left => {
                self.snake.pos_x =
                    (self.snake.pos_x - 1 + self.field_width as i16) % self.field_width as i16
            },
            Up => {
                self.snake.pos_y =
                    (self.snake.pos_y - 1 + self.field_height as i16) % self.field_height as i16
            },
            Down => {
                self.snake.pos_y =
                    (self.snake.pos_y + 1 + self.field_height as i16) % self.field_height as i16
            },
        }

        self.snake.body.push_front(self.snake.dir.clone());
    }

    fn react_on(&mut self, button: &Button) {
        use crate::Direction::{Right, Down, Left, Up};

        match button {
            Button::Keyboard(key) => {
                match key {
                    Key::Space => {
                        self.game_paused = !self.game_paused;
                    },
                    Key::W | Key::Up => {
                        if self.snake.dir != Down { self.snake.dir = Up }
                    },
                    Key::A | Key::Left => {
                        if self.snake.dir != Right { self.snake.dir = Left }
                    },
                    Key::D | Key::Right => {
                        if self.snake.dir != Left { self.snake.dir = Right }
                    },
                    Key::S | Key::Down => {
                        if self.snake.dir != Up { self.snake.dir = Down }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
}


struct Renderer{
    window: GlutinWindow,
    gl: GlGraphics,
}

impl Renderer {
    fn new(win_size: (i32, i32)) -> Renderer {
        let gl_version_value = OpenGL::V3_2;
        let win_size = Size{
            width: win_size.0 as u32,
            height: win_size.1 as u32,
        };

        Renderer {
            window: WindowSettings::new(
                "Snake Game", win_size)
                .opengl(gl_version_value)
                .exit_on_esc(true)
                .build()
                .unwrap(),
            gl: GlGraphics::new(gl_version_value),
        }
    }

    fn render_game(&mut self, game: &mut Game, arg: &RenderArgs) {

        self.gl.draw(
            arg.viewport(),
            |_c, gl| { graphics::clear(<[f32; 4]>::from(game.background_color), gl); }
        );

        let sq_size = game.square_size as i16;
        let half_size = sq_size / 2;

        for snack in &game.snacks {
            let snack_circle = graphics::ellipse::circle(
                (snack.pos_x * sq_size + half_size) as f64,
                (snack.pos_y * sq_size + half_size) as f64,
                half_size as f64,
            );

            self.gl.draw(arg.viewport(), |c, gl| {
                graphics::ellipse(
                    snack.color.into(),
                    snack_circle,
                    c.transform,
                    gl,
                );
            });
        }

        let mut segment_pos_x = game.snake.pos_x;
        let mut segment_pos_y = game.snake.pos_y;

        let mut reset_game = false;

        let mut last_direction = game.snake.dir;

        for body_segment in &game.snake.body {
            match body_segment {
                Direction::Right => {
                    segment_pos_x = (segment_pos_x - 1 + game.field_width) % game.field_width;
                },
                Direction::Left => {
                    segment_pos_x = (segment_pos_x + 1 + game.field_width) % game.field_width;
                },
                Direction::Up => {
                    segment_pos_y = (segment_pos_y + 1 + game.field_height) % game.field_height;
                },
                Direction::Down => {
                    segment_pos_y = (segment_pos_y - 1 + game.field_height) % game.field_height;
                },
            }

            if segment_pos_x == game.snake.pos_x
                && segment_pos_y == game.snake.pos_y {
                reset_game = true;
                break;
            }

            if last_direction == *body_segment {
                let segment_square = graphics::rectangle::square(
                    (segment_pos_x * game.square_size) as f64,
                    (segment_pos_y * game.square_size) as f64,
                    game.square_size as f64,
                );

                self.gl.draw(arg.viewport(), |c, gl| {
                    graphics::rectangle(game.snake.color.into(), segment_square, c.transform, gl);
                });
            } else {


                let mut segm_arc_x = (segment_pos_x * game.square_size) as f64;
                let mut segm_arc_y = (segment_pos_y * game.square_size) as f64;
                let mut segm_start = 0.0;
                let mut segm_end = 0.0;


            }
        }

        self.gl.draw(arg.viewport(), |c, gl| {
            graphics::rectangle(game.snake.color.into(),
                                graphics::rectangle::square(
                                    (game.snake.pos_x * game.square_size) as f64,
                                    (game.snake.pos_y * game.square_size) as f64,
                                    game.square_size as f64,
                                ),
                                c.transform, gl);
        });

        let mut head_x = (game.snake.pos_x * game.square_size) as f64;
        let mut head_y = (game.snake.pos_y * game.square_size) as f64;
        {
            match game.snake.dir{
                Direction::Right => {
                    head_x = head_x + 0.75 * game.square_size as f64;
                    head_y = head_y + 0.25 * game.square_size as f64;},
                Direction::Left => {
                    head_x = head_x - 0.25 * game.square_size as f64;
                    head_y = head_y + 0.25 * game.square_size as f64;},
                Direction::Up => {
                    head_x = head_x + 0.25 * game.square_size as f64;
                    head_y = head_y - 0.25 * game.square_size as f64;},
                Direction::Down => {
                    head_x = head_x + 0.25 * game.square_size as f64;
                    head_y = head_y + 0.75 * game.square_size as f64;},
            };
        }

        let snake_head = graphics::rectangle::square(
            head_x,
            head_y,
            0.5 * game.square_size as f64
        );

        let arc: (f64, f64) = match game.snake.dir {
            Direction::Right => {(0.5 * -std::f64::consts::PI, 0.5 * std::f64::consts::PI)},
            Direction::Left => {(0.5 * std::f64::consts::PI, 1.5 * std::f64::consts::PI)},
            Direction::Up => {(std::f64::consts::PI, 2.0 * std::f64::consts::PI)},
            Direction::Down => {(0.0, std::f64::consts::PI)},
        };

        self.gl.draw(arg.viewport(), |c, gl| {
            graphics::circle_arc(
                game.snake.color.into(), 0.25 * <f64>::from(game.square_size),
                arc.0,
                arc.1,
                snake_head,
                c.transform,
                gl)
        });

        if reset_game {
            game.game_over();
        }
    }
}


fn main() {

    let field_size: (i16, i16) = (20, 20);
    let sq_sz: i16 = 40;

    println!("field size: [{}, {}]", field_size.0, field_size.1);

    let mut events = Events::new(EventSettings::new()).ups(8);

    let mut rnr = Renderer::new((
        (field_size.0 as i32) * sq_sz as i32,
        (field_size.1 as i32) * sq_sz as i32,
    ));

    let mut game = Game::new(sq_sz, field_size);

    while let Some(e) = events.next(&mut rnr.window) {
        if let Some(r) = e.render_args() {
            rnr.render_game(&mut game, &r);
        }

        if let Some(_) = e.update_args() {
            game.update();
        }

        if let Some(key) = e.button_args() {

            if key.state == ButtonState::Press {
                game.react_on(&key.button);
            }
        }
    }
}
