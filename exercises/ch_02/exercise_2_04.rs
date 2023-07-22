use nannou::prelude::*;

struct Model{
    movers: Vec<Mover>,
    friction_area: FrictionArea
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

    fn apply_friction(&mut self, area: &FrictionArea){
        if self.is_inside(area){
            let mut friction = self.velocity;
            if friction.length() > 0.0 {
                friction *= -1.0;
                friction = friction.normalize();
                friction *= area.friction_coefficient;
                self.apply_force(friction);
            }
        }
        
    }

    fn is_inside(&self, area: &FrictionArea) -> bool{
        // let x_diff = self.location.x - area.rect.x();
        // let y_diff = self.location.y - area.rect.y();
        self.location.x>area.rect.left() && self.location.x<area.rect.right() && self.location.y<area.rect.top() && self.location.y>area.rect.bottom()
    }

}

struct FrictionArea{
    // location: Vec2,
    rect: Rect,
    friction_coefficient: f32
}
impl FrictionArea{
    fn new(rect: Rect, c: f32) -> Self{
        FrictionArea{
            rect: rect,
            friction_coefficient: c
        }
    }

    fn display(&self, draw: &Draw) {
        draw.rect()
        .xy(self.rect.xy())
        .w(self.rect.w())
        .h(self.rect.h())
        .color(BLUE)
        .stroke(BLACK)
        .stroke_weight(2.0);
    }

    // fn apply_friction(&self, m: &mut Mover){
    //     if self.is_mover_inside(m){
    //         let mut friction = m.velocity;
    //         if friction.length() > 0.0 {
    //             friction *= -1.0;
    //             friction = friction.normalize();
    //             friction *= self.friction_coefficient;
    //             m.apply_force(friction);
    //         }
    //     }
        
    // }

    // fn is_mover_inside(&self, m: &mut Mover) -> bool{
    //     let x_diff = m.location.x - self.location.x;
    //     let y_diff = m.location.y - self.location.y;
    //     (x_diff * x_diff) + (y_diff * y_diff) <= (self.radius * self.radius)
    // }
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
        movers: v,
        friction_area: FrictionArea::new(Rect::from_xy_wh(vec2(-200.0, -200.0),vec2(100.0, 300.0)), -0.05)
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    
    for mover  in &mut model.movers{
        mover.apply_friction(&model.friction_area);
       
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
    model.friction_area.display(&draw);
    for mover  in &model.movers{
        mover.display(&draw);
    }
    draw.to_frame(app, &frame).unwrap();

}

