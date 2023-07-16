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
    let win = app.window_rect();

    draw.background().color(WHITE);


    draw.line()
    .start(model.pos)
    .end(model.mouse_pos)
    .stroke_weight(2.0);

    let mag = model.mouse_pos.length();
    let r = Rect::from_w_h(mag, 10.0).top_left_of(win);
    draw.rect().xy(r.xy()).wh(r.wh()).color(BLACK);

    draw.to_frame(app, &frame).unwrap();

}

