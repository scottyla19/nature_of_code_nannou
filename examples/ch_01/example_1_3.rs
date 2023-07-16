use nannou::prelude::*;

struct Model {
    location: Vec2,
    velocity: Vec2
    // x: f32,
    // y: f32,
    // x_speed: f32,
    // y_speed: f32
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    
    Model {
        location: vec2(0.0, 0.0),
        velocity: vec2(1.0, 3.3)
        // x: 0.0,
        // y: 0.0,
        // x_speed: 1.0,
        // y_speed: 3.3
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {
    model.location += model.velocity;

    let (width, height) = app.main_window().inner_size_points();

    if (model.location.x > width/2.0) || (model.location.x < -width/2.0) {
        model.velocity.x *=  -1.0;
    }
    if (model.location.y > height/2.0) || (model.location.y < -height/2.0) {
        model.velocity.y *=  -1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.ellipse()
    .xy(model.location)
    .w(16.0)
    .h(16.0)
    .gray(0.5)
    .stroke(BLACK).stroke_weight(2.0);

    draw.to_frame(app, &frame).unwrap();

}

