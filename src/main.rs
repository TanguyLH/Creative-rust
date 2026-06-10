use nannou::prelude::*;

#[derive(Debug)]
struct Electron {
    angle: f32,
    base_dist: f32,
    phase: f32,
    wobble_speed: f32,
    wobble_amount: f32,
    wobble_phase: f32,
}

struct Model {
    core_radius: f32,
    electrons: Vec<Electron>,
    offset: Vec2,
    last_window_pos: Option<Vec2>,
}

const ELECTRON_MAX_OFFSET: f32 = 300.0;

const RETURN_RATE: f32 = 4.0;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).moved(window_moved).build();

    let core_radius = 30.0;

    let electrons = (0..10000)
        .map(|_| Electron {
            angle: random_range(0.0, 2.0 * PI),
            base_dist: core_radius + 2.0 + random_f32().powf(0.2) * (ELECTRON_MAX_OFFSET - 2.0),
            phase: random_range(0.0, 2.0 * PI),
            wobble_speed: random_range(0.3, 1.2),
            wobble_amount: random_range(5.0, 25.0),
            wobble_phase: random_range(0.0, 2.0 * PI),
        })
        .collect();

    Model {
        core_radius,
        electrons,
        offset: vec2(0.0, 0.0),
        last_window_pos: None,
    }
}

fn window_moved(_app: &App, model: &mut Model, pos: Vec2) {
    if let Some(last) = model.last_window_pos {
        let delta = pos - last;
        model.offset -= delta;
    }
    model.last_window_pos = Some(pos);
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last.as_secs_f32();

    // Retour exponentiel vers le centre, sans rebond.
    model.offset *= (1.0 - RETURN_RATE * dt).max(0.0);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    // Toute la scene est decalee par l'inertie accumulee lors des
    // deplacements de la fenetre.
    let draw = draw.translate(model.offset.extend(0.0));

    let t = app.time;
    let core_radius = model.core_radius;

    // Aura : beaucoup de couches fines et rapprochees pour simuler un vrai degrade,
    // l'opacite decroit en carre a mesure qu'on s'eloigne du noyau
    let aura_layers = 50;
    let aura_span: f32 = 50.0;
    for i in (0..aura_layers).rev() {
        let f = i as f32 / aura_layers as f32; // 0 = bord du noyau, 1 = bord exterieur
        let r = core_radius + f * aura_span;
        let alpha = (1.0 - f).powi(2) * 0.15;
        draw.ellipse().radius(r).color(rgba(0.6, 0.3, 1.0, alpha));
    }

   let electron_points = model.electrons.iter().map(|e| {
        let base_x = e.angle.cos() * e.base_dist;
        let base_y = e.angle.sin() * e.base_dist;

        // Deux oscillations independantes (frequence/phase propres a chaque electron)
        // pour deplacer le point en x et en y, sans direction privilegiee
        let wobble_x = (t * e.wobble_speed + e.wobble_phase).sin() * e.wobble_amount;
        let wobble_y = (t * e.wobble_speed * 1.3 + e.phase).cos() * e.wobble_amount;

        let x = base_x + wobble_x;
        let y = base_y + wobble_y;

        // Plus on est proche du noyau, plus l'electron est discret
        let proximity = ((e.base_dist - core_radius) / ELECTRON_MAX_OFFSET).clamp(0.0, 1.0);
        let alpha = 0.05 + proximity * 0.55;

        (pt3(x, y, 0.0), rgba(0.6, 0.8, 1.0, alpha))
    });

    draw.point_mode().mesh().points_colored(electron_points);

    draw.to_frame(app, &frame).unwrap();
}
