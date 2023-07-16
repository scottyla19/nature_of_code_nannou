#![allow(non_snake_case)]
use nannou::prelude::*;

struct Model {
    location: Vec3,
    velocity: Vec3,
    container: Cuboid
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    
    Model {
        location: vec3(0.0, 0.0, 0.0),
        velocity: vec3(1.0, 3.3, 1.6),
        container: Cuboid::from_x_y_z_w_h_d(0.0, 0.0, 0.0, 400.0, 400.0, 400.0)

    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.location += model.velocity;

    // let (width, height) = app.main_window().inner_size_points();

    // let x_container = ;
    if (model.location.x > model.container.x.end) || (model.location.x < model.container.x.start) {
        model.velocity.x *=  -1.0;
    }
    if (model.location.y > model.container.y.end) || (model.location.y < model.container.y.start) {
        model.velocity.y *=  -1.0;
    }
    if (model.location.z > model.container.z.end) || (model.location.z < model.container.z.start) {
        model.velocity.z *=  -1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().radians(vec3(
        // global rotation
        (app.time * 0.1).sin() * 2.0,
        (app.time * 0.2).sin() * 2.0,
        (app.time * 0.3).sin() * 2.0,
      ));
    draw.background().color(WHITE);
    
    let cube = Cuboid::from_xyz_whd(model.location, vec3(16.0, 16.0, 16.0));
    // let container = Cuboid::from_x_y_z_w_h_d(0.0, 0.0, 0.0, 400.0, 400.0, 400.0);

    // let rot = vec3(
    //     // individual rotation
    //     (app.time as f32) * 1.11,
    //     (app.time as f32) * 1.22,
    //     (app.time as f32) * 1.33,
    //   );
    // let scl = (cube.z() + app.window_rect().h()) / app.window_rect().h();
    

    // let scl = cube.z() + app.window_rect()
    let cube_points = cube.triangles_iter().flat_map(geom::Tri::vertices);
    draw
    .mesh()
    .points(cube_points)
    .color(STEELBLUE);
    
    let container_points = model.container.triangles_iter().flat_map(geom::Tri::vertices);
    draw
    .mesh()
    .points(container_points)
    .color(rgba(1.0, 0.0, 0.0, 0.1));

    // draw.ellipse()
    // .color(GRAY)
    // .xyz(model.location)
    // .w(16.0)
    // .h(16.0);

    draw.to_frame(app, &frame).unwrap();

}

