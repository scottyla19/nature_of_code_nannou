use nannou::prelude::*;

struct Model{
    movers: Vec<Mover>,
    attractor: Attractor
}

struct Mover {
    location: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32
}
impl Mover{
    fn new(loc: Vec2, mass: f32) -> Self{
        let velo = vec2(1.0, 0.0);
        let acc =  vec2(0.0,0.0);
        Mover { 
            location: loc, 
            velocity: velo, 
            acceleration: acc, 
            mass: mass}

    }

    fn display(&self, draw: &Draw) {
        draw.ellipse()
        .xy(self.location)
        .w(24.0*self.mass)
        .h(24.0* self.mass)
        .gray(0.5)
        .stroke(BLACK)
        .stroke_weight(2.0);
    }

    fn update(&mut self){
        self.velocity += self.acceleration; 
        // self.velocity = self.velocity.clamp_length_max(self.top_speed);
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
        } else if self.location.y > rect.top() {
            self.velocity.y *= -1.0;
            self.location.y = rect.top();
        } 
    }
    fn repel(&self, m:&Mover) -> Vec2{
        let mut force = self.location - m.location; 
        let mut d = force.length(); 
        d = d.max(5.0).min(25.0); 
        force = force.normalize(); 
        let strength = -0.5 * ( self.mass * m.mass) / (d * d); 
        force * strength
    }


}

struct Attractor{
    location: Vec2,
    mass: f32
}
impl Attractor{
    fn new(loc: Vec2, m: f32) -> Self{

        Attractor { 
            location: loc, 
            mass: m}

    }

    fn attract(&self, m:&Mover) -> Vec2{
        let mut force = self.location - m.location; 
        let mut d = force.length(); 
        d = d.max(5.0).min(25.0); 
        force = force.normalize(); 
        let strength = ( self.mass * m.mass) / (d * d); 
        force * strength
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
        let y =  random_range(rect.y(), rect.top());
        let m: f32 = random_range(0.5, 3.0);
        v.push(Mover::new(vec2(x,y), m));
    }


    Model {
        movers: v,
        attractor: Attractor::new(app.mouse.position(), 90.0)
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.attractor.location = app.mouse.position();
    for i in 0..model.movers.len() {
        for j in 0..model.movers.len() {
            if i != j {
                let force = model.movers[j].repel(&model.movers[i]);
                model.movers[i].apply_force(force);
            }
        }
        let force =  model.attractor.attract(&model.movers[i]);
        model.movers[i].apply_force(force);
        model.movers[i].update();
        model.movers[i].check_edges(app.window_rect());
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

