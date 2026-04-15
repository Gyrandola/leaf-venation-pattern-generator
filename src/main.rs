use macroquad::prelude::*;
use macroquad::rand::srand;

static VEIN_RADIUS: f32 = 4.0;
static VEIN_INNER_COLOR: Color = WHITE;
static VEIN_OUTER_COLOR: Color = BLACK;
static VEIN_GROWTH_RATE: f32 = 8.5;

static AUXIN_RADIUS: f32 = 4.0;
static AUXIN_COLOR: Color = RED;
static AUXIN_STARTING_NUMBER: i32 = 800;

/// Number of auxins to generate when pressing space.
static AUXIN_GENERATE_NUMBER: i32 = 100;

/// The distance at which an auxin is "consumed" by a vein.
static PROXIMITY_THRESHOLD: f32 = VEIN_RADIUS + AUXIN_RADIUS + 5.0;
static SHOW_PROXIMITY_THRESHOLD: bool = false;

/// The maximum distance an auxin will attract a vein.
static DIST_MAX: f32 = 75.0;


type Point = Vec2;

struct Vein {
    position: Point,

    /// Vector to the closest auxin(s).
    direction: Point,
}




//////////////////////////////////////////////
//////////////////////////////////////////////

fn window_conf() -> Conf {
    Conf {
        window_title: "Venation Pattern Generator".to_owned(),
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    srand(miniquad::date::now() as u64);

    let mut veins: Vec<Vein> = Vec::new();
    let mut auxins: Vec<Point> = generate_auxins(AUXIN_STARTING_NUMBER);

    // Initialize a root vein.
    let root = Point { x: screen_width()/2.0, y: screen_height()/1.5 };
    veins.push(Vein {position:root, direction:Point { x: 0.0, y: 0.0 }});

    // Control variables
    let mut is_paused = true;
    let mut last_update = get_time();
    let update_speed = 0.05;           // Step every 0.05 seconds (20 FPS)


    loop {
        if is_key_pressed(KeyCode::Escape) { return; }

        if is_key_pressed(KeyCode::Space) {
            is_paused = !is_paused;
        }

        if is_key_pressed(KeyCode::A) {
            auxins.append(&mut generate_auxins(AUXIN_GENERATE_NUMBER));
        }

        let current_time = get_time();
        if !is_paused && (current_time - last_update >= update_speed) {
            grow_veins_step(&auxins, &mut veins);
            remove_auxins_in_proximity(&mut auxins, &veins);
            last_update = current_time;
        }

        clear_background(WHITE);

        draw(&auxins, &veins);

        let status_text = if is_paused { "PAUSED" } else { "RUNNING" };
        let status_color = if is_paused { RED } else { GREEN };

        draw_text(&format!("Status: {}", status_text), 10.0, 30.0, 40.0, status_color);
        draw_text("Press SPACE to Play/Pause", 10.0, 60.0, 30.0, BLACK);
        draw_text("Press A to add more auxins", 10.0, 90.0, 30.0, BLACK);

        next_frame().await
    }
}




//////////////////////////////////////////////
//////////////////////////////////////////////




fn grow_veins_step(auxins: &Vec<Point>, veins: &mut Vec<Vein>){

    for vein in veins.iter_mut() {
        vein.direction = Point::ZERO;
    }

    for auxin in auxins {
        let mut closest_vein = None;
        let mut closest_distance = DIST_MAX;

        // Find closest vein.
        for vein in veins.iter_mut() {
            let distance = (vein.position - *auxin).length();
            if distance < closest_distance {
                closest_vein = Some(vein);
                closest_distance = distance;
            }
        }

        // Update direction of closest vein.
        if let Some(vein) = closest_vein {
            vein.direction += *auxin - vein.position;
        }
    }

    let mut new_veins: Vec<Vein> = Vec::new();

    for vein in veins.iter_mut() {

        // Veins with a non-zero direction are being attracted to an auxin, so they grow.
        if vein.direction != Point::ZERO {

            // Without a bit of noise, if a vein finds itself inbetween two auxins,
            // the resulting direction vector points to the middle, and gets "stuck".
            let growth_direction = vein.direction.normalize() + noise();
            let new_position = vein.position + (growth_direction * VEIN_GROWTH_RATE);

            new_veins.push(Vein {
                position: new_position,
                direction: Point::ZERO,
            });

            // After growing in a direction, the "father" vein is once again static.
            vein.direction = Point::ZERO;
        }
    }

    veins.append(&mut new_veins);
}

fn noise() -> Vec2 {
    let x = rand::gen_range(-0.15, 0.15);
    let y = rand::gen_range(-0.15, 0.15);
    Vec2::new(x, y)
}

fn remove_auxins_in_proximity(auxins: &mut Vec<Point>, veins: &[Vein]) {

    // Remove auxin if there is no vein in proximity.
    auxins.retain(|auxin| {
        !veins.iter().any(|vein| {
            (vein.position - *auxin).length() < PROXIMITY_THRESHOLD
        })
    });
}

fn generate_auxins(n: i32) -> Vec<Point> {
    let mut auxins: Vec<Point> = Vec::new();
    for _ in 0..n {
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
        auxins.push(Point {x, y});
    }
    auxins
}

fn draw(auxins: &[Vec2], veins: &[Vein]) {

    for vein in veins {
        draw_circle(vein.position.x, vein.position.y, VEIN_RADIUS, VEIN_OUTER_COLOR);
        draw_circle(vein.position.x, vein.position.y, VEIN_RADIUS / 2.0, VEIN_INNER_COLOR);
    }

    for auxin in auxins {
        draw_circle(auxin.x, auxin.y, AUXIN_RADIUS, AUXIN_COLOR);
        if SHOW_PROXIMITY_THRESHOLD {
            draw_circle_lines(auxin.x, auxin.y, PROXIMITY_THRESHOLD, 1.0, AUXIN_COLOR);
        }
    }
}



