use nannou::prelude::*;

struct Model{
    movers: Vec<Mover>
}

struct Mover {
    location: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    top_speed: f32
}
impl Mover{
    fn new(location: Vec2) -> Self{

        let loc = location;
        let velo = vec2(0.0, 0.0);
        let acc =  vec2(0.0,0.0);
        let speed = 5.0;
        Mover { 
            location: loc, 
            velocity: velo, 
            acceleration: acc, 
            top_speed: speed }

    }

    fn display(&self, draw: &Draw) {
        draw.ellipse()
        .xy(self.location)
        .w(32.0)
        .h(32.0)
        .gray(0.5)
        .stroke(BLACK)
        .stroke_weight(2.0);
    }

    fn update(&mut self, mouse: Vec2){
        self.acceleration = (mouse - self.location).normalize() * 0.2;
        self.velocity += self.acceleration;       
        self.velocity = self.velocity.clamp_length_max(self.top_speed);
        self.location += self.velocity;
    }

}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    let rect = app.window_rect();
    
    let mut v: Vec<Mover> = Vec::new();
    for _ in 0..10 {
        let x = random_range(rect.left(), rect.right());
        let y = random_range(rect.bottom(), rect.top());
        v.push(Mover::new(vec2(x,y)));
    }
    
    Model {
        movers: v
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse = app.mouse.position();
    for mover  in &mut model.movers{
        mover.update(mouse);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for mover  in &model.movers{
        mover.display(&draw);
    }
    draw.to_frame(app, &frame).unwrap();

}

