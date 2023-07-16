use nannou::prelude::*;

struct Model {
    pos: Vec2,
    mouse_pos: Vec2
}


fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    
    Model {
        pos: vec2(0.0, 0.0),
        mouse_pos: vec2(0.0, 0.0)
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {
    model.mouse_pos = app.mouse.position();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);
    let mouse = model.mouse_pos - model.pos;

    draw.line()
    .start(model.pos)
    .end(mouse.normalize()*150.0)
    .stroke_weight(2.0);

    draw.to_frame(app, &frame).unwrap();

}

