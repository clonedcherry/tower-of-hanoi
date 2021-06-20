#![windows_subsystem = "windows"]
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate rand;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

mod hanoi_game;
use hanoi_game::hanoi_game::Hanoi;

fn main() {
    let opengl = OpenGL::V3_2;

    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = 600;

    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

    let mut game_window: GlutinWindow = WindowSettings::new("Tower of Hanoi", [WIDTH, HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new()).ups(1);

    let mut mouse_location: [f64; 2] = [0.0, 0.0];
    let mut selected_tower: Option<usize> = None;
    let mut game_state = Hanoi::new(5, 3);

    loop {
        if let Some(e) = events.next(&mut game_window) {
            if let Some(e) = e.mouse_cursor_args() {
                mouse_location = e;
            }

            if let Some(e) = e.button_args() {
                if e.state == piston::input::ButtonState::Press
                    && e.button
                        == piston::input::Button::Mouse(piston::input::mouse::MouseButton::Left)
                {
                    let currently_clicked_tower = (mouse_location[0]
                        / (WIDTH as f64 / game_state.get_num_towers() as f64) as f64)
                        .floor() as usize;

                    if let Some(sel_tower) = selected_tower {
                        if sel_tower == currently_clicked_tower {
                            selected_tower = None;
                            //println!("deselected tower");
                        } else {
                            game_state.do_command(sel_tower, currently_clicked_tower);
                            // println!("{}", game_state);
                            selected_tower = None;
                            if game_state.is_won() {
                                break;
                            }
                        }
                    } else {
                        selected_tower = Some(currently_clicked_tower);
                        // println!("selected tower {}", currently_clicked_tower);
                    }
                }
            }

            if let Some(r) = e.render_args() {
                gl.draw(r.viewport(), |_c, gl| {
                    graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
                });

                //==================================================================
                // Tower Rendering
                //==================================================================
                const BOTTOM_OFFSET: f64 = 0.95;
                const TOP_OFFSET: f64 = 0.6;
                const SIDE_OFFSET: f64 = 0.95;
                const TOWER_SPACING: f64 = 0.99;

                let y_baseline = HEIGHT as f64 * BOTTOM_OFFSET;
                let y_step = -(HEIGHT as f64 * TOP_OFFSET) / game_state.get_tower_height() as f64;
                let mut tower_bases_x_coords: Vec<f64> =
                    Vec::with_capacity(game_state.get_num_towers());
                let tower_width = (WIDTH as f64 * SIDE_OFFSET) / game_state.get_num_towers() as f64;
                let width_unit =
                    (tower_width / game_state.get_tower_height() as f64) * TOWER_SPACING;

                tower_bases_x_coords
                    .push(WIDTH as f64 * ((1 as f64 - SIDE_OFFSET) / 2.0) + tower_width / 2.0);

                for i in 1..game_state.get_num_towers() {
                    tower_bases_x_coords.push(tower_bases_x_coords[i - 1] + tower_width);
                }

                // gl.draw(r.viewport(), |c, gl|{
                //     let transform = c.transform;

                //     graphics::rectangle(
                //         GREEN,
                //         graphics::rectangle::centered()
                //     )
                // });
                for i in 0..game_state.get_num_towers() {
                    for (j, floor_width) in game_state.get_tower(i).iter().enumerate() {
                        gl.draw(r.viewport(), |c, gl| {
                            let transform = c.transform;

                            let mut color = GREEN;
                            // dbg!(&selected_tower);
                            if j == game_state.get_tower(i).len() - 1 && selected_tower != None && selected_tower.unwrap() == i {
                                color = [1.0, 0.25, 0.25, 1.0];
                            }

                            // dbg!(&tower_bases_x_coords, y_baseline + (y_step * j as f64));
                            graphics::rectangle(
                                color,
                                graphics::rectangle::centered([
                                    tower_bases_x_coords[i],
                                    y_baseline + (y_step * j as f64),
                                    width_unit * (floor_width).clone() as f64 / 2.0,
                                    y_step / 2.0,
                                ]),
                                transform,
                                gl,
                            );
                        });
                    }
                }
            }
        } else {
            break;
        }
    }
}
