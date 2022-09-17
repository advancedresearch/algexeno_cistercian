use piston_window::*;
use algexenotation::Algexeno;
use algexeno_cistercian::*;

fn main() {
    let title = "Algexenotation-Cistercian Viewer";
    let mut window: PistonWindow = WindowSettings::new(title, [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let mut n = 2022;
    let mut number = Algexeno::Orig(n).eval();
    let mut shape = Draw::from_algexeno(&number).unwrap();
    let mut strokes = vec![];
    let settings = Settings {
        circle_resolution: 4,
    };
    shape.to_strokes(false, &mut strokes, &settings);
    window.set_title(format!("{}: {}' = {}", title, n, number));

    let scale = [80.0, 80.0];
    let offset = [20.0, 20.0];
    let mut paused = false;
    let mut time = 0.0;

    while let Some(e) = window.next() {
        window.draw_2d(&e, |ref c, g, _| {
            clear([1.0, 1.0, 1.0, 1.0], g);

            let mut rt = time * 2.0;
            let mut old_y: Option<[f64; 2]> = None;
            for &(x, y, dt) in &strokes {
                if let Some(old_y) = old_y {
                    if old_y != x {
                        rt -= 0.5;
                    }
                }
                old_y = Some(y);

                let dt = dt * ((y[0] - x[0]).powi(2) + (y[1] - x[1]).powi(2)).sqrt();
                if rt <= 0.0 {break}
                let (x, y) = if rt < dt {
                    (x, [x[0] + (y[0] - x[0]) * rt / dt, x[1] + (y[1] - x[1]) * rt / dt])
                } else {(x, y)};
                rt -= dt;
                let x = [x[0] * scale[0] + offset[0], x[1] * scale[1] + offset[1]];
                let y = [y[0] * scale[0] + offset[0], y[1] * scale[1] + offset[1]];
                Line::new([0.0, 0.0, 0.0, 1.0], 1.0)
                    .shape(line::Shape::Bevel)
                    .draw_from_to(x, y, &c.draw_state, c.transform, g);
            }
        });
        if let Some(b) = e.press_args() {
            if b == Button::Keyboard(Key::P) {
                paused = !paused;
            } else if !paused {
                n = if b == Button::Keyboard(Key::K) {n-1} else {n+1};
                number = Algexeno::Orig(n).eval();
                shape = Draw::from_algexeno(&number).unwrap();
                strokes.clear();
                shape.to_strokes(false, &mut strokes, &settings);
                time = 0.0;
                window.set_title(format!("{}: {}' = {}", title, n, number));
            }
        }
        if let Some(args) = e.update_args() {
            time += args.dt;
        }
    }
}

pub fn interesting(number: &Algexeno) -> bool {
    use Algexeno::*;
    use algexenotation::Op::*;

    match number {
        Orig(_) | Const(_) => true,
        Bin(Mul, ab) => interesting(&ab.0) && interesting(&ab.1),
        Bin(Add, _) => false,
        Bin(Pow, ab) => interesting(&ab.0) && interesting(&ab.1),
    }
}
