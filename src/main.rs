// use std::fs::DirEntry;

use ggez;
use ggez::event;
use ggez::glam::vec2;
use ggez::graphics;
use ggez::{conf, Context, GameResult};
use nalgebra::Point;
// use ggez::input::keyboard::{KeyCode, KeyMods, KeyInput};
use std::f32::consts::PI;
use mint::Point2;
use rand::Rng;

const BORDER: f32 = 40.0;
const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;
const FIELD_WIDTH: f32 = WINDOW_WIDTH - BORDER;
const FIELD_HEIGHT: f32 = WINDOW_HEIGHT - BORDER;
// const CIRCLE_V: f32 = 4.5;
const SIGMA: f32 = 1.0;
const HOLES_RADIUS: f32 = 36.0;
const HOLES_POINTS: [Point2<f32>; 6]  = [
    Point2{x: 0.0 + HOLES_RADIUS, y: 0.0 + HOLES_RADIUS},
    Point2{x: WINDOW_WIDTH - HOLES_RADIUS, y: 0.0 + HOLES_RADIUS}, // DO not work
    Point2{x: WINDOW_WIDTH  - HOLES_RADIUS, y: WINDOW_HEIGHT - HOLES_RADIUS},
    Point2{x: 0.0 + HOLES_RADIUS, y: WINDOW_HEIGHT - HOLES_RADIUS},
    Point2{x: WINDOW_WIDTH/2.0 , y: WINDOW_HEIGHT - HOLES_RADIUS},
    Point2{x: WINDOW_WIDTH/2.0 , y: 0.0 + HOLES_RADIUS},
];
const BALL_RADIUS: f32 = 18.0;
const BALL_POINTS: [Point2<f32>; 6]  = [
    Point2{x: 0.0 + HOLES_RADIUS, y: 0.0 + HOLES_RADIUS},
    Point2{x: WINDOW_WIDTH - HOLES_RADIUS, y: 0.0 + HOLES_RADIUS}, // DO not work
    Point2{x: WINDOW_WIDTH  - HOLES_RADIUS, y: WINDOW_HEIGHT - HOLES_RADIUS},
    Point2{x: 0.0 + HOLES_RADIUS, y: WINDOW_HEIGHT - HOLES_RADIUS},
    Point2{x: WINDOW_WIDTH/2.0 , y: WINDOW_HEIGHT - HOLES_RADIUS},
    Point2{x: WINDOW_WIDTH/2.0 , y: 0.0 + HOLES_RADIUS},
];



struct Ball {
    position: Point2<f32>,
    radius: f32,
    color: graphics::Color,
    velocity: Point2<f32>,
    number: usize,
    mass: f32,
    // momentum: f32,
}
impl Ball {
    fn new(x:f32, y:f32 ,radius: f32, color: graphics::Color, velocity: Point2<f32>, number: usize) -> Ball {
        // Calculate the position based on the radius
        let position: Point2<f32>= Point2{
            x, 
            y};
        let mass: f32 = SIGMA*PI*radius.powf(2.0);

        // if x < FIELD_WIDTH - radius{
        //     position = Point2 {
        //             x: x+radius,
        //             y: y-radius,
        //     }; 
        // }
        // else{
        //     position = Point2 {
        //         x: x-radius,
        //         y: y-radius,
        //     }; 
        // }
        
        Ball {
            position,
            radius,
            color,
            velocity,
            // direction,
            number,
            mass,
            // momentum: mass*CIRCLE_V,
        }
    }
}

struct Balls{
    balls: Vec<Ball>,
}
impl Balls {
    fn new() -> Balls {
        let mut x: f32 = 300.0;
        let mut balls = Vec::new();
        for i in 1..=15{
            let ball :Ball = Ball::new(
                x, 
                100.0, 
                BALL_RADIUS, 
                graphics::Color::RED, 
                Point2 { 
                    x: 0.0, 
                    y: 0.0,
                },
                i);
            x += BALL_RADIUS*2.0;
            balls.push(ball);
        }
        Balls { balls }
    }

    fn moveball(){
        ()
    }
}


struct Hole{
    position: Point2<f32>,
    radius: f32,
}
struct Holes{
    holes: Vec<Hole>,
}
impl Holes {
    fn new () -> Holes{
        let mut holes:Vec<Hole> = Vec::new();
        for i in 0..HOLES_POINTS.len(){
            holes.push(Hole { position: HOLES_POINTS[i], radius: HOLES_RADIUS })
        }
        Holes{holes}
    }
}


fn clamp(ball: &mut Ball) {
    let (min_x, min_y, max_x, max_y) = (
        ball.radius + BORDER,
        ball.radius + BORDER,
        FIELD_WIDTH - ball.radius,
        FIELD_HEIGHT - ball.radius,
    );

    if ball.position.x < min_x {
        ball.position.x = min_x;
        ball.velocity.x *= -1.0;
    } else if ball.position.x > max_x {
        ball.position.x = max_x;
        ball.velocity.x *= -1.0;
    }

    if ball.position.y < min_y {
        ball.position.y = min_y;
        ball.velocity.y *= -1.0;
    } else if ball.position.y > max_y {
        ball.position.y = max_y;
        ball.velocity.y *= -1.0;
    }
}

// fn clamp(ball: &mut Circle) {
//     let min_x = ball.radius;
//     let min_y = ball.radius;
//     let max_x = WINDOW_WIDTH - ball.radius;
//     let max_y = WINDOW_HEIGHT - ball.radius;

//     ball.position.x = ball.position.x.clamp(min_x, max_x);
//     ball.position.y = ball.position.y.clamp(min_y, max_y);
// }


fn collision(balls: &[Ball]) -> (bool, Vec<(usize, usize)>) {
    let mut collided_pairs = Vec::new();
    let mut collision_detected = false;

    for i in 0..balls.len() - 1 {
        for j in i + 1..balls.len() {
            let ball1 = &balls[i];
            let ball2 = &balls[j];
            let dx = ball1.position.x - ball2.position.x;
            let dy = ball1.position.y - ball2.position.y;
            let distance_squared = dx * dx + dy * dy;
            let min_distance = ball1.radius + ball2.radius;
            if distance_squared < min_distance * min_distance {
                collided_pairs.push((i, j)); // Collision detected, add the pair to the list.
                collision_detected = true;
            }
        }
    }

    (collision_detected, collided_pairs)
}

fn momentum_conservation(collided_balls: &[(usize, usize)], balls: &mut [Ball]) {
    for i in 0..collided_balls.len() {
        let (index1, index2) = collided_balls[i];

        // Calculate the new velocities separately for x and y components.
        let delta_x = balls[index2].position.x - balls[index1].position.x;
        let delta_y = balls[index2].position.y - balls[index1].position.y;
        let distance = (delta_x * delta_x + delta_y * delta_y).sqrt();

        let normal_x = delta_x / distance;
        let normal_y = delta_y / distance;

        let relative_velocity_x = balls[index2].velocity.x - balls[index1].velocity.x;
        let relative_velocity_y = balls[index2].velocity.y - balls[index1].velocity.y;

        let dot_product = normal_x * relative_velocity_x + normal_y * relative_velocity_y;

        if dot_product < 0.0 {
            let impulse = (2.0 * dot_product)
                / (1.0 / balls[index1].mass + 1.0 / balls[index2].mass);
            let impulse_x = impulse * normal_x;
            let impulse_y = impulse * normal_y;

            // Update velocities for both balls.
            balls[index1].velocity.x += impulse_x / balls[index1].mass;
            balls[index1].velocity.y += impulse_y / balls[index1].mass;
            balls[index2].velocity.x -= impulse_x / balls[index2].mass;
            balls[index2].velocity.y -= impulse_y / balls[index2].mass;
        }
    }
}


struct MainState {
    balls: Balls,
    holes: Holes,
}

impl MainState {
    pub fn new(number_balls: usize) -> Self {
        let (screen_w, screen_h) = (FIELD_WIDTH, FIELD_HEIGHT);
        // let radius:f32 = 12.0;
        let colors: Vec<graphics::Color> = vec![
            graphics::Color::WHITE,
            graphics::Color::BLACK,
            graphics::Color::RED,
            // graphics::Color::GREEN,
            graphics::Color::BLUE,
        ];
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        let mut balls = Balls::new();
        let holes = Holes{holes: Vec::new()};
        let state: MainState =  MainState { 
            balls: balls,
            holes: holes
        };


        // for i in 0..number_balls{
        //     let color: graphics::Color = colors[i];
        //     let radius = rng.gen_range(0.0..100.0);
        //     let x: f32 = rng.gen_range(0.0 + radius..screen_w - radius);
        //     let y: f32 = rng.gen_range(0.0 + radius..screen_h - radius);
        //     let velocity: Point2<f32> = Point2 { x: 2.0, y: 5.5 };
        //     state.add_ball(x, y, radius, color, velocity)
        // }
        state
    }
    
    // pub fn add_ball(&mut self, x:f32, y:f32 ,radius: f32, color: graphics::Color, velocity: Point2<f32>){
    //     let ball: Ball = Ball::new(x, y, radius, color, velocity);
    //     self.balls.push(ball);
    // }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        // let k_ctx = &ctx.keyboard;
        // if k_ctx.is_key_pressed(KeyCode::W) {
        //     self.ball1.position.x += 4.5;
        // }
        
        // for ball in &mut self.balls{
        //     ball.position.x += ball.velocity.x;
        //     ball.position.y += ball.velocity.y;
        //     clamp(ball);
        // }
        
        // let (collision_bool, collided_balls) = collision(&mut self.balls);
        // if collision_bool{
        //     momentum_conservation(&collided_balls, &mut self.balls);
        //     println!("Colision");
        // }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {

        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::GREEN);

        let border = graphics::Rect::new(
            0.0, 
            0.0, 
            WINDOW_WIDTH, 
            WINDOW_HEIGHT,
        );
        // let rect = graphics::Mesh::new_rectangle(
        //     ctx,
        //     graphics::DrawMode::stroke(BORDER),
        //     border,
        //     graphics::Color {
        //         r: 139.0,
        //         g: 69.0,
        //         b: 19.0,
        //         a: 1.0,
        //     },
        // );

        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(BORDER*2.0),
            border,
            graphics::Color::from_rgb(165, 42, 42),
        )?;
        canvas.draw(&rect, graphics::DrawParam::default());

        for i in 0..HOLES_POINTS.len(){
            let ball_mesh: graphics::Mesh = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                HOLES_POINTS[i],
                HOLES_RADIUS,
                0.1,
                graphics::Color::BLACK,
            )?;
            canvas.draw(&ball_mesh, graphics::DrawParam::default());
        }

        for ball in &mut self.balls.balls{
            let ball_mesh: graphics::Mesh = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                ball.position,
                ball.radius,
                0.1,
                ball.color,
            )?;
            canvas.draw(&ball_mesh, graphics::DrawParam::default());

            let mut text = graphics::Text::new(format!("{}", ball.number));

            let mut number_position: Point2<f32> = Point2 { x: 0.0, y: 0.0 };

            if ball.number < 10 {
                number_position = Point2 { 
                    x: ball.position.x - ball.radius/3.0, 
                    y: ball.position.y - ball.radius/2.0, 
                };
                println!("2");
            } else {
                number_position = Point2 { 
                    x: ball.position.x - ball.radius/1.7, 
                    y: ball.position.y - ball.radius/2.0, 
                };
            };
            let text_dest = graphics::DrawParam::new()
                .dest(number_position)
                .color(graphics::Color::WHITE);
            canvas.draw(
                text
                    .set_scale(20.)
                    .set_bounds(vec2(16.0, 16.0)),
                    text_dest,
            );
            // offset += 1.0;
        
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}


fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("Rusty Pong", "M@ano")
        .window_setup(conf::WindowSetup::default().title("Rusty Pong"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()
        .unwrap();

    let mut state = MainState::new(4);
    event::run(ctx, event_loop, state);
    Ok(())
}
