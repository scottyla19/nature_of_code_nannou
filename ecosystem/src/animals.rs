use nannou::prelude::*;
use nannou::noise::NoiseFn;
use nannou::noise::Perlin;

pub trait UpdateDisplay{
    fn display(&self, draw: &Draw);
}


pub struct Frog{
    location: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    top_speed: f32,
    rect: Rect
}
impl Frog{
    pub fn from(location: Vec2, rect: &Rect) -> Self{
        let loc = location;
        let velo = vec2(0.0, 0.0);
        let acc =  vec2(0.0,0.0);
        let speed = 5.0;
        Frog { 
            location: loc, 
            velocity: velo, 
            acceleration: acc, 
            top_speed: speed,
            rect: *rect }

    }

    pub fn update(&mut self, t:f32){
        let x = abs(t.sin());
        // https://iquilezles.org/articles/smoothsteps/
        let step = x*x*(3.0-2.0*x);
        
        let noisy_x = random_range(-1.0, 1.0) ;
        let noisy_y = random_range(-1.0, 1.0); 
        self.acceleration = vec2(noisy_x * step, noisy_y * step);
        self.velocity += self.acceleration;  

        if self.location.x > self.rect.right() || self.location.x < self.rect.left(){
            self.velocity.x *= -1.0;
        }
        if self.location.y > self.rect.top() || self.location.y < self.rect.bottom(){
            self.velocity.y *= -1.0;
        }    

        self.velocity = self.velocity.clamp_length_max(self.top_speed);
        self.location += self.velocity;
    }
    pub fn check_edges(&mut self, rect: Rect) {
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
}
impl UpdateDisplay for Frog{
    fn display(&self, draw: &Draw){
        draw.ellipse()
        .xy(self.location)
        .w(32.0)
        .h(32.0)
        .color(GREEN)
        .stroke(BLACK)
        .stroke_weight(2.0);
    }
}

pub struct Fish{
    location: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    top_speed: f32,
    mass: f32,
    rect: Rect
}
impl Fish{
    pub fn from(location: Vec2, rect:&Rect) -> Self{
        let loc = location;
        let velo = vec2(0.0, 0.0);
        let acc =  vec2(0.0,0.0);
        let speed = 5.0;
        Fish { 
            location: loc, 
            velocity: velo, 
            acceleration: acc, 
            top_speed: speed ,
            mass: 10.0,
            rect: *rect}

    }

    pub fn update(&mut self, t:f64){
        let noisy_x = Perlin::new().get([random_range(0.0, 10.0) as f64, t as f64]) as f32;
        let noisy_y = Perlin::new().get([random_range(0.0, 10.0) as f64, t as f64]) as f32; 

        self.acceleration = vec2(noisy_x, noisy_y);
        self.velocity += self.acceleration; 

        if self.location.x > self.rect.right() || !self.is_in_pond(){
            self.velocity.x *= -1.0;
        }
        if self.location.y > self.rect.top() || !self.is_in_pond(){
            self.velocity.y *= -1.0;
        }      

        self.velocity = self.velocity.clamp_length_max(self.top_speed);
        self.location += self.velocity;
    }

    fn is_in_pond(&self) -> bool{
        // https://math.stackexchange.com/a/76463
        let x_diff = self.location.x - self.rect.top_right().x;
        let y_diff = self.location.y - self.rect.top_right().y;
        let x_radius = self.rect.w()/2.0;
        let y_radius = self.rect.h()/2.0;
        ((x_diff * x_diff)/(x_radius *x_radius)) + ((y_diff * y_diff)/(y_radius * y_radius)) <= 1.0
    }

    pub fn repel(&self, s:&Snake) -> Vec2{
        let mut force = self.location - s.location; 
        let mut d = force.length(); 
        d = d.max(5.0).min(25.0); 
        force = force.normalize(); 
        let strength = -0.5 * ( self.mass * s.mass) / (d * d); 
        force * strength
    }
}
impl UpdateDisplay for Fish{
    fn display(&self, draw: &Draw){
        draw.ellipse()
        .xy(self.location)
        .w(32.0)
        .h(24.0)
        .color(BLUE)
        .stroke(BLACK)
        .stroke_weight(2.0);
    }
}

pub struct Fly{
    location: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32
}
impl Fly{
    pub fn from(location: Vec2) -> Self{
        let loc = location;
        let velo = vec2(0.0, 0.0);
        let acc =  vec2(0.0,0.0);
        Fly { 
            location: loc, 
            velocity: velo, 
            acceleration: acc, 
            mass: 5.0 }

    }

    // pub fn update(&mut self, mouse: Vec2, t:f64){
        // let noisy_step = map_range(Perlin::new().get([random_range(1.0, 10.0) as f64, t as f64]),1.0, 10.0,0.2,0.6);
        // self.acceleration = (mouse - self.location).normalize() * noisy_step as f32;
        // self.velocity += self.acceleration;       
        // self.velocity = self.velocity.clamp_length_max(self.top_speed);
        // self.location += self.velocity;
        
    // }
    pub fn update(&mut self){
        self.velocity += self.acceleration; 
        // self.velocity = self.velocity.clamp_length_max(self.top_speed);
        self.location += self.velocity;
        self.acceleration *= 0.0;
    }
    pub fn apply_force(&mut self, force: Vec2){
        self.acceleration += force/ self.mass;
    }

    pub fn check_edges(&mut self, rect: Rect) {
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
}
impl UpdateDisplay for Fly{
    fn display(&self, draw: &Draw){
        draw.ellipse()
        .xy(self.location)
        .w(10.0)
        .h(10.0)
        .color(BLACK)
        .stroke(GREY)
        .stroke_weight(1.0);
    }
    
}
pub struct Attractor{
    location: Vec2,
    mass: f32
}
impl Attractor{
    pub fn new(loc: Vec2) -> Self{
        Attractor { 
            location: loc, 
            mass: 10.0}

    }
    pub fn set_location(&mut self, loc: Vec2){
        self.location = loc;
    }

    pub fn attract(&self, fly:&Fly) -> Vec2{
        let mut force = self.location - fly.location; 
        let mut d = force.length(); 
        d = d.max(5.0).min(25.0); 
        force = force.normalize(); 
        let strength = ( self.mass * fly.mass) / (d * d); 
        force * strength
    }

}
pub struct Snake{
    pub location: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    mass: f32
}
impl Snake{
    pub fn from(location: Vec2) -> Self{
        let loc = location;
        let velo = vec2(0.0, 0.0);
        let acc =  vec2(0.0,0.0);
        Snake { 
            location: loc, 
            velocity: velo, 
            acceleration: acc, 
            mass: 15.0
        }

    }

    pub fn update(&mut self){
        // let noisy_x = Perlin::new().get([random_range(0.0, 10.0) as f64, t as f64]) as f32;
        // let noisy_y = Perlin::new().get([random_range(0.0, 10.0) as f64, t as f64]) as f32; 

        // self.apply_force(vec2(noisy_x, noisy_y));
        self.velocity += self.acceleration; 
        // self.velocity = self.velocity.clamp_length_max(self.top_speed);
        self.location += self.velocity;
        self.acceleration *= 0.0;
    }

    pub fn apply_force(&mut self, force: Vec2){
        self.acceleration += force/ self.mass;
    }

    pub fn check_edges(&mut self, rect: Rect) {
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
}

impl UpdateDisplay for Snake{
    fn display(&self, draw: &Draw){
        let points = (0..50).map(|i| {
            let x = i as f32 + self.location.x ;          
            let point = pt2(x, x.sin() + self.location.y) * 2.0; //scale sine wave by 20.0
            (point, YELLOWGREEN)
          });
          draw.polyline()
              .weight(5.0)
              .z_degrees(45.0)
              .points_colored(points);        
    }
}

