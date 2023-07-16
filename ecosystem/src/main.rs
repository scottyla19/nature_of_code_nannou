use animals::*;
use nannou::prelude::*;
// use rand::{
//     distributions::{Distribution, Standard},
//     Rng,
// };
mod animals;

// enum Animals {
//     FrogAnimal(Frog),
//     FishAnimal(Fish),
//     FlyAnimal(Fly),
//     SnakeAnimal(Snake)
// }
// impl Distribution<Animals> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Animals {
//         // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
//         match rng.gen_range(0..=3) { // rand 0.8
//             0 => Animals::FrogAnimal(Frog::new()),
//             1 => Animals::FishAnimal(Fish::new()),
//             2 => Animals::FlyAnimal(Fly::new()),
//             _ => Animals::SnakeAnimal(Snake::new()),
//         }
//     }
// }
struct Model{
    frogs: Vec<Frog>,
    fishes: Vec<Fish>,
    flys: Vec<Fly>,
    snakes: Vec<Snake>
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();

    let rect = app.window_rect();
    
    let mut a: Vec<Frog> = Vec::new();
    for _ in 0..random_range(2, 10) {
        let x = random_range(rect.left(), rect.right());
        let y = random_range(rect.bottom(), rect.top());
        a.push(Frog::from(vec2(x,y), &app.window_rect()));
    }

    let mut b: Vec<Fish> = Vec::new();
    for _ in 0..random_range(2, 10) {
        let x = random_range(150.0, rect.right());
        let y = random_range(150.0, rect.top());
        b.push(Fish::from(vec2(x,y), &app.window_rect()));
    }

    let mut c: Vec<Fly> = Vec::new();
    for _ in 0..random_range(2, 10) {
        let x = random_range(rect.left(), rect.right());
        let y = random_range(rect.bottom(), rect.top());
        c.push(Fly::from(vec2(x,y)));
    }

    let mut d: Vec<Snake> = Vec::new();
    for _ in 0..random_range(2, 10) {
        let x = random_range(rect.left(), rect.right());
        let y = random_range(rect.bottom(), rect.top());
        d.push(Snake::from(vec2(x,y), &app.window_rect()));
    }

    Model {
        frogs: a,
        fishes: b,
        flys: c,
        snakes: d
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse = app.mouse.position();
    for f  in &mut model.frogs{
        
        f.update( app.time);
    }
    for f  in &mut model.fishes{
        f.update((app.elapsed_frames() as f64) * 0.03);
    }
    for f  in &mut model.flys{
        let t = (app.elapsed_frames() as f64) * 0.03;
        f.update(mouse, t);
    }
    for f  in &mut model.snakes{
        
        f.update( (app.elapsed_frames() as f64) * 0.03);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // let mut snake_draw = draw.rotate(vec2(0.0,0.0).angle_between(vec2(0.0,0.0)));

    let rect = app.window_rect();
    draw.background().color(WHITE);
    draw.ellipse()
    .xy(rect.top_right())
    .w(rect.w())
    .h(rect.h())
    .color(STEELBLUE);

    for f  in &model.frogs{
        f.display(&draw);
    }
    for f  in &model.fishes{
        f.display(&draw);
    }
    for f  in &model.flys{
        f.display(&draw);
    }
    for f  in &model.snakes{
        // draw.rotate(f.velocity.angle_between(vec2(0.0,0.0)));
        f.display(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
    // snake_draw.to_frame(app, &frame).unwrap();
}


