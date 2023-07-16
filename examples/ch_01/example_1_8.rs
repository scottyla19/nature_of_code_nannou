use nannou::prelude::*;

struct Model {
    location: Vec2,
    velocity: Vec2,
    acceleration: Vec2
}
impl Model{
    fn limit(&mut self, max_speed: f32) -> (){
        if self.velocity.length() > max_speed{
            self.velocity = self.velocity.normalize() * max_speed;
        }
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    
    Model {
        location: vec2(0.0, 0.0),
        velocity: vec2(1.0, 3.3),
        acceleration: vec2(-0.001,0.01)
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {
    model.velocity += model.acceleration;
    model.limit(10.0);
    model.location += model.velocity;

    let (width, height) = app.main_window().inner_size_points();

    if model.location.x > width/2.0  {
        model.location.x = -width/2.0;
    } else if model.location.x < -width/2.0{
        model.location.x = width/2.0;
    }
    if model.location.y > height/2.0  {
        model.location.y = -height/2.0;
    } else if model.location.y < -height/2.0{
        model.location.x = height/2.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.ellipse()
    .xy(model.location)
    .w(32.0)
    .h(32.0)
    .gray(0.5)
    .stroke(BLACK)
    .stroke_weight(2.0);

    draw.to_frame(app, &frame).unwrap();

}

