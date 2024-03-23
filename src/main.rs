use enigo::*;
use std::f64::consts::PI;
use std::thread;
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new();
    let (x, y) = enigo.mouse_location();

    enigo.mouse_down(MouseButton::Left);
    thread::sleep(Duration::from_millis(100));
    enigo.mouse_up(MouseButton::Left);

    // Calculate the circle parameters based on the current mouse position
    let center_x = x;
    let center_y = y;
    let radius = 380.0; // Radius of the circle
    let num_points = 500; // Number of points to generate along the circumference
    let delay = Duration::from_millis(4); // Delay between each movement in milliseconds

    // Calculate the coordinates along the circumference and move the mouse to each point
    for i in 0..=num_points {
        if i == 1 {
            enigo.mouse_down(MouseButton::Left);
        }
        // Left mouse button press
        let angle = 2.0 * PI * (i as f64) / (num_points as f64);
        let x = center_x as f64 + radius * angle.cos();
        let y = center_y as f64 + radius * angle.sin();
        enigo.mouse_move_to(x as i32, y as i32);
        thread::sleep(delay);
    }

    // Left mouse button release
    enigo.mouse_up(MouseButton::Left);
}