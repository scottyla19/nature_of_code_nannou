use nannou::noise::NoiseFn;
use nannou::noise::Perlin;
use nannou::prelude::*;
// use nannou::color;
// use nannou::rand::Rng;
// use rand::prelude::*;
// use nannou::noise;
use rand_distr::{Normal, Distribution};


struct Model {  
    pts: Vec<Point2>,
    mouse_pos: Vec2
}


fn main() {
    nannou::app(model).update(update).loop_mode(LoopMode::refresh_sync()).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().event(event).view(view).build().unwrap();
    
    // draw background once so that we don't have to draw every point every time in the view fn
    let draw = app.draw();
    draw.background().color(WHITE);

    Model {
        pts: vec!{pt2(0f32, 0f32)},
        mouse_pos: pt2(0f32, 0f32)
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(pos) => model.mouse_pos = pos,
        _ => ()
    } 
}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();
    let start_pt = *model.pts.get(model.pts.len() - 2).unwrap();
    let end_pt = *model.pts.last().unwrap();

    // exercise I.4: Gaussian color
    let color_dist = Normal::new(128.0, 127.0).unwrap();
    let r = map_range(color_dist.sample(&mut rand::thread_rng()), -400.0, 500.0, 0.0, 1.0);
    let g = map_range(color_dist.sample(&mut rand::thread_rng()), -400.0, 500.0, 0.0, 1.0);
    let b = map_range(color_dist.sample(&mut rand::thread_rng()), -400.0, 500.0, 0.0, 1.0);
    let col = Rgb::new(r, g, b);

    draw.line()
    .start(start_pt)
    .end(end_pt)
    .weight(4.0)
    .color(col);
    draw.to_frame(app, &frame).unwrap();
    
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let prev_pt = model.pts.last().unwrap();
   

    // first example
//     let new_pt = pt2(prev_pt.x + random_range(-1.0, 1.0),
//                            prev_pt.y + random_range(-1.0, 1.0)
// ); 

    // example I.3: 40% to the right
    // let mut rng = nannou::rand::thread_rng();
    // let new_pt:Point2 = match rng.gen::<f32> (){
    //     d if d < 0.4 => pt2(prev_pt.x + 1.0, prev_pt.y),
    //     d if d < 0.6 && d >= 0.4 => pt2(prev_pt.x - 1.0, prev_pt.y),
    //     d if d < 0.8 && d >= 0.6=> pt2(prev_pt.x,prev_pt.y + 1.0),
    //     _ => pt2(prev_pt.x, prev_pt.y - 1.0)
    // };

    // exercise I.3: move toward mouse
    // let boundary = app.window_rect();
    // let x = map_range(model.mouse_pos.x, boundary.left(), boundary.right(), -1.0, 1.0);
    // let y = map_range(model.mouse_pos.y, boundary.bottom(), boundary.top(), -1.0, 1.0);
    // let new_pt = pt2(prev_pt.x + x + random_range(-1.0, 1.0),
    //                        prev_pt.y + y + random_range(-1.0, 1.0));

    // exercise I.5: gaussian walk
    // let normal = Normal::new(0.0, 1.0).unwrap();
    // let x_delta = normal.sample(&mut rand::thread_rng());
    // let y_delta = normal.sample(&mut rand::thread_rng());
    // let new_pt = pt2(prev_pt.x  + x_delta,
    //                        prev_pt.y  + y_delta);

    // exercise I.6: Monte Carlo walk
    // let step_size = monte_carlo(0.0, 10.0);
    // let new_pt = pt2(prev_pt.x + step_size * random_range(-1.0, 1.0),
    //                        prev_pt.y + step_size * random_range(-1.0, 1.0));

    // example I.5: noisy location
    let t = (app.elapsed_frames() as f32) * 0.03;
    // let size = app.main_window().inner_size_points();
    // let noisy_x = Perlin::new().get([prev_pt.x as f64, t as f64]);
    // let noisy_y = Perlin::new().get([prev_pt.y as f64, t as f64]);
    // let x = map_range(noisy_x, 0.0, 1.0, 0.0, size.0/2.0);
    // let y = map_range(noisy_y, 0.0, 1.0, 0.0, size.1/2.0);
    // let new_pt = pt2(x, y );

    // exercise I.7: noisy step size
    let noisy_step = Perlin::new().get([random_range(0.0, 10.0) as f64, t as f64]);
    let noisy_step_y = Perlin::new().get([random_range(0.0, 10.0) as f64, t as f64]);

    let new_pt = pt2(prev_pt.x + noisy_step as f32 * random_range(-1.0, 1.0),
                           prev_pt.y + noisy_step_y as f32 *random_range(-1.0, 1.0));
    

    model.pts.push(new_pt);
    
}

// fn monte_carlo(min: f32, max: f32) -> f32 {
//     loop {
//       let r1 = random_range(min, max);
//       let p = r1;
//       let r2 = random_range(min, max);
//       if r2 < p{
//         return r1;
//       }  
//     }
// }

