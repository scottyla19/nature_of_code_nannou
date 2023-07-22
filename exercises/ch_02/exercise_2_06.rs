use nannou::prelude::*;

struct Model{
    movers: Vec<Mover>,
    friction_area: FrictionArea
}

struct Mover {
    location: Vec2,
    rect: Rect,
    rotation: f32,
    velocity: Vec2,
    acceleration: Vec2,
    top_speed: f32,
    mass: f32
}
impl Mover{
    fn new(rect: Rect, mass: f32, rot: f32) -> Self{

        let loc = rect.xy();
        let velo = vec2(0.0, 0.0);
        let acc =  vec2(0.0,0.0);
        let speed = 5.0;
        Mover { 
            location: loc, 
            rect: rect,
            rotation: rot,
            velocity: velo, 
            acceleration: acc, 
            top_speed: speed,
            mass: mass}

    }

    fn display(&self, draw: &Draw) {
        draw.rect()
        .xy(self.location)
        .w(self.rect.w()*self.mass)
        .h(self.rect.h()* self.mass)
        .z_degrees(self.rotation)
        .gray(0.5)
        .stroke(BLACK)
        .stroke_weight(2.0);

        draw.text(&self.rotation.cos().to_string())
        .xy(self.location)
        .color(BLACK);
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

    // fn apply_friction(&mut self, area: &FrictionArea){
    //     if self.is_inside(area){
    //         let mut friction = self.velocity;
    //         if friction.length() > 0.0 {
    //             friction *= -1.0;
    //             friction = friction.normalize();
    //             friction *= area.friction_coefficient;
    //             self.apply_force(friction);
    //         }
    //     }
    // }

    fn apply_drag(&mut self, area: &FrictionArea){
        let speed = self.velocity.length();
        let frontal_area: f32 = self.rect.w() * abs(self.rotation.cos());
        let dragnitude = area.friction_coefficient * speed * speed * frontal_area * 0.5;
        let mut drag = self.velocity;
        if drag.length() > 0.0 {
            drag *= -1.0;
            drag = drag.normalize();
            drag *= dragnitude;
            self.apply_force(drag);
        }
        
    }


    fn is_inside(&self, area: &FrictionArea) -> bool{
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
        let y =  rect.top();//random_range(rect.y(), rect.top());
        let rot =  random_range(0.0, 90.0);
        let m: f32 = 3.0;//random_range(0.5, 4.0);
        v.push(Mover::new(Rect::from_xy_wh(vec2(x,y), vec2(32.0, 32.0)), m, rot));
    }


    Model {
        movers: v,
        friction_area: FrictionArea::new(Rect::from_corners(rect.mid_left(), rect.bottom_right()), 0.03)
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    
    for mover  in &mut model.movers{
        if mover.is_inside(&model.friction_area){
            mover.apply_drag(&model.friction_area);
        } else{
            // wind
            // mover.apply_force(vec2(0.01, 0.0));
        }

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

