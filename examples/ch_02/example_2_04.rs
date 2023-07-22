use nannou::prelude::*;

struct Model{
    movers: Vec<Mover>
}

struct Mover {
    location: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    top_speed: f32,
    mass: f32
}
impl Mover{
    fn new(location: Vec2, mass: f32) -> Self{

        let loc = location;
        let velo = vec2(0.0, 0.0);
        let acc =  vec2(0.0,0.0);
        let speed = 5.0;
        Mover { 
            location: loc, 
            velocity: velo, 
            acceleration: acc, 
            top_speed: speed,
            mass: mass}

    }

    fn display(&self, draw: &Draw) {
        draw.ellipse()
        .xy(self.location)
        .w(32.0*self.mass)
        .h(32.0* self.mass)
        .gray(0.5)
        .stroke(BLACK)
        .stroke_weight(2.0);
    }

    fn update(&mut self){
        self.velocity += self.acceleration; 
        self.velocity = self.velocity.clamp_length_max(self.top_speed);
        self.location += self.velocity;
        self.acceleration *= 0.0;
    }

    fn apply_force(&mut self, force: Vec2){
        self.acceleration += force/ self.mass;
    }

    fn check_edges(&mut self, rect: Rect) {
        if self.location.x > rect.right() {
            self.location.x = rect.right();
            self.velocity.x *= -1.0;
        } else if self.location.x < rect.left() {
            self.velocity.x *= -1.0;
            self.location.x = rect.left();
        }
        if self.location.y < rect.bottom() {
            self.velocity.y *= -1.0;
            self.location.y = rect.bottom();
        }
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
        let x = rect.left() + 30.0; //random_range(rect.left(), rect.right());
        let y =  rect.top() + 30.0;//random_range(rect.bottom(), rect.top());
        let m: f32 = random_range(0.1, 4.0);
        v.push(Mover::new(vec2(x,y), m));
    }

    Model {
        movers: v
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for mover  in &mut model.movers{

        let c = 0.05;
        let mut friction = mover.velocity;
        if friction.length() > 0.0 {
            friction *= -1.0;
            friction = friction.normalize();
            friction *= c;
            mover.apply_force(friction);
        }
        // wind
        mover.apply_force(vec2(0.01, 0.0));
        // gravity
        mover.apply_force(vec2(0.0, -0.1*mover.mass));
        mover.update();
        mover.check_edges(app.window_rect());
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

