use macroquad::prelude::*;
use macroquad::rand::srand;

static VEIN_RADIUS: f32 = 4.0;
static VEIN_INNER_COLOR: Color = WHITE;
static VEIN_OUTER_COLOR: Color = BLACK;
static VEIN_GROWTH_RATE: f32 = 8.5;

static AUXIN_RADIUS: f32 = 4.0;
static AUXIN_COLOR: Color = RED;
static AUXIN_NUMBER: i32 = 750;

/// The distance at which an auxin is "consumed" by a vein.
static PROXIMITY_THRESHOLD: f32 = VEIN_RADIUS + AUXIN_RADIUS;
static SHOW_PROXIMITY_THRESHOLD: bool = false;

/// The maximum distance an auxin will attract a vein.
static DIST_MAX: f32 = 100.0;


type Point = Vec2;

struct Vein {
    position: Point,

    /// Vector pointing from the vein to the closest auxin.
    direction: Point,
}




//////////////////////////////////////////////
//////////////////////////////////////////////




#[macroquad::main("VenationPatternGenerator")]
async fn main() {

    next_frame().await;

    srand(miniquad::date::now() as u64);

    let mut veins: Vec<Vein> = Vec::new();
    let mut auxins: Vec<Point> = Vec::new();

    // Generate random auxins.
    for _ in 0..AUXIN_NUMBER {
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
        auxins.push(Point {x, y});
    }

    // Initialize a root vein.
    let root = Point { x: screen_width()/2.0, y: screen_height()/1.5 };
    veins.push(Vein {position:root, direction:Point { x: 0.0, y: 0.0 }});

    loop {

        clear_background(WHITE);

        // Calculate vein growth.
        grow_veins_step(&auxins, &mut veins);

        // Remove auxins that have been reached.
        remove_auxins_in_proximity(&mut auxins, &veins);

        // Render.
        draw(&auxins, &veins);

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
            let new_position = vein.position + (vein.direction.normalize() * VEIN_GROWTH_RATE);

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

fn remove_auxins_in_proximity(auxins: &mut Vec<Point>, veins: &[Vein]) {

    // Remove auxin if there is no vein in proximity.
    auxins.retain(|auxin| {
        !veins.iter().any(|vein| {
            (vein.position - *auxin).length() < PROXIMITY_THRESHOLD
        })
    });
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



