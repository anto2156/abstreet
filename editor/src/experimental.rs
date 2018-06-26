use animation;
use ezgui::GfxCtx;
use ezgui::canvas::Canvas;
use ezgui::input::UserInput;
use graphics;
use graphics::math::Vec2d;
use graphics::types::Color;
use gui;
use map_model::{polygons_for_polyline, shift_polyline, Pt2D, geometry};
use piston::input::Key;
use piston::window::Size;
use std::f64;
use std::process;

const WHITE: Color = [1.0; 4];
const RED: Color = [1.0, 0.0, 0.0, 0.8];
const GREEN: Color = [0.0, 1.0, 0.0, 0.8];
const BLUE: Color = [0.0, 0.0, 1.0, 0.8];
const BLACK: Color = [0.0, 0.0, 0.0, 0.3];

pub struct UI {
    canvas: Canvas,
    p3_offset: (f64, f64),
    show_labels: bool,
}

impl UI {
    pub fn new() -> UI {
        UI {
            canvas: Canvas::new(),
            p3_offset: (200.0, 150.0),
            show_labels: true,
        }
    }
}

impl gui::GUI for UI {
    fn event(
        mut self,
        input: &mut UserInput,
        _window_size: &Size,
    ) -> (UI, animation::EventLoopMode) {
        if input.unimportant_key_pressed(Key::Escape, "Press escape to quit") {
            process::exit(0);
        }
        let speed = 5.0;
        if input.unimportant_key_pressed(Key::H, "left") {
            self.p3_offset.0 -= speed;
        }
        if input.unimportant_key_pressed(Key::J, "down") {
            self.p3_offset.1 += speed;
        }
        if input.unimportant_key_pressed(Key::K, "up") {
            self.p3_offset.1 -= speed;
        }
        if input.unimportant_key_pressed(Key::L, "right") {
            self.p3_offset.0 += speed;
        }
        if input.unimportant_key_pressed(Key::P, "toggle labels") {
            self.show_labels = !self.show_labels;
        }

        self.canvas.handle_event(input.use_event_directly());

        (self, animation::EventLoopMode::InputOnly)
    }

    fn draw(&self, g: &mut GfxCtx, _input: UserInput) {
        graphics::clear(WHITE, g.gfx);
        g.ctx = self.canvas.get_transformed_context(&g.orig_ctx);

        let mut labels: Vec<(Pt2D, String)> = Vec::new();

        macro_rules! point {
            ($pt_name:ident, $value:expr) => {
                let $pt_name = $value;
                labels.push(($pt_name, stringify!($pt_name).to_string()));
            };
        }
        /*macro_rules! points {
            ($pt1_name:ident, $pt2_name:ident, $value:expr) => {
                let ($pt1_name, $pt2_name) = $value;
                labels.push(($pt1_name, stringify!($pt1_name).to_string()));
                labels.push(($pt2_name, stringify!($pt2_name).to_string()));
            };
        }*/

        self.debug_polyline(g, &mut labels);

        let thin = 1.0;
        let thick = 5.0;
        let shift_away = 50.0;

        // TODO detect "breakages" by dist from p2 to p2_c beyond threshold
        // TODO still try the angle bisection method
        // TODO bezier curves could be ideal for both drawing and car paths, but no easy way to
        // try them out in piston

        /*point!(p1, Pt2D::new(100.0, 100.0));
        point!(p2, Pt2D::new(110.0, 200.0));
        point!(
            p3,
            Pt2D::new(p1.x() + self.p3_offset.0, p1.y() + self.p3_offset.1)
        );
        point!(p4, Pt2D::new(500.0, 120.0));

        /*println!("");
        println!("p1 -> p2 is {}", angle_degrees(p1, p2));
        println!("p2 -> p3 is {}", angle_degrees(p2, p3));*/

        draw_polyline(g, &vec![p1, p2, p3, p4], thick, RED);*/

        /*let polygon = polygon_for_polyline(&vec![p1, p2, p3, p4], shift_away);
        for (idx, pt) in polygon.iter().enumerate() {
            labels.push(((pt[0], pt[1]), format!("x{}", idx + 1)));
        }
        draw_polygon(g, polygon, BLACK);*/
        /*for p in polygons_for_polyline(&vec![p1, p2, p3, p4], shift_away) {
            draw_polygon(g, p, BLACK);
        }

        // Two lanes on one side of the road
        let l1_pts = shift_polyline(shift_away, &vec![p1, p2, p3, p4]);
        for (idx, pt) in l1_pts.iter().enumerate() {
            labels.push((*pt, format!("l1_p{}", idx + 1)));
        }
        draw_polyline(g, &l1_pts, thin, GREEN);

        let l2_pts = shift_polyline(shift_away * 2.0, &vec![p1, p2, p3, p4]);
        for (idx, pt) in l2_pts.iter().enumerate() {
            labels.push((*pt, format!("l2_p{}", idx + 1)));
        }
        draw_polyline(g, &l2_pts, thin, GREEN);

        // Other side
        let l3_pts = shift_polyline(shift_away, &vec![p4, p3, p2, p1]);
        for (idx, pt) in l3_pts.iter().enumerate() {
            labels.push((*pt, format!("l3_p{}", idx + 1)));
        }
        draw_polyline(g, &l3_pts, thin, BLUE);*/

        // Manual approach for more debugging
        /*points!(p1_e, p2_e, shift_line(shift_away, p3, p2));
        points!(p2_f, p3_f, shift_line(shift_away, p2, p1));
        point!(p2_g, line_intersection((p1_e, p2_e), (p2_f, p3_f)));

        draw_line(g, p1_e, p2_g, thin, BLUE);
        draw_line(g, p2_g, p3_f, thin, BLUE);*/

        if self.show_labels {
            for pair in &labels {
                self.label(g, pair.0, &pair.1);
            }
        }
    }
}

impl UI {
    fn label(&self, g: &mut GfxCtx, pt: Pt2D, text: &str) {
        self.canvas
            .draw_text_at(g, &vec![text.to_string()], pt.x(), pt.y());
    }

    fn debug_polyline(&self, g: &mut GfxCtx, labels: &mut Vec<(Pt2D, String)>) {
        let thin = 1.0;
        let width = 50.0;

        // TODO retain this as a regression test
        let center_pts: Vec<Pt2D> = vec![
          //Pt2D::new(2623.117354164207, 1156.9671270455774),
          //Pt2D::new(2623.0950086610856, 1162.8272397294127),
          //Pt2D::new(2623.0956685132396, 1162.7341864981956),
          // One problem happens starting here -- some overlap
          Pt2D::new(2622.8995366939575, 1163.2433695162579),
          Pt2D::new(2620.4658232463926, 1163.9861244298272),
          Pt2D::new(2610.979416102837, 1164.2392149291984),
          //Pt2D::new(2572.5481805300115, 1164.2059309889344),
        ].iter().map(|pt| Pt2D::new(pt.x() - 2500.0, pt.y() - 1000.0)).collect();
        draw_polyline(g, &center_pts, thin, RED);
        for (idx, pt) in center_pts.iter().enumerate() {
            labels.push((*pt, format!("p{}", idx + 1)));
        }

        for p in polygons_for_polyline(&center_pts, width) {
            draw_polygon(g, p, BLACK);
        }

        // TODO colored labels!
        let side1 = shift_polyline(width / 2.0, &center_pts);
        //draw_polyline(g, &side1, thin, BLUE);
        for (idx, pt) in side1.iter().enumerate() {
            labels.push((*pt, format!("L{}", idx + 1)));
        }

        let mut reversed_center_pts = center_pts.clone();
        reversed_center_pts.reverse();
        let mut side2 = shift_polyline(width / 2.0, &reversed_center_pts);
        side2.reverse();
        //draw_polyline(g, &side2, thin, GREEN);
        for (idx, pt) in side2.iter().enumerate() {
            labels.push((*pt, format!("R{}", idx + 1)));
        }
    }
}

fn draw_line(g: &mut GfxCtx, pt1: Pt2D, pt2: Pt2D, thickness: f64, color: Color) {
    let l = graphics::Line::new(color, thickness);
    l.draw(
        [pt1.x(), pt1.y(), pt2.x(), pt2.y()],
        &g.ctx.draw_state,
        g.ctx.transform,
        g.gfx,
    );
}

fn draw_polyline(g: &mut GfxCtx, pts: &Vec<Pt2D>, thickness: f64, color: Color) {
    assert!(pts.len() >= 2);
    for pair in pts.windows(2) {
        draw_line(g, pair[0], pair[1], thickness, color);
    }
    let circle = graphics::Ellipse::new(BLUE);
    let radius = 0.5;
    for pt in pts {
        circle.draw(geometry::circle(pt.x(), pt.y(), radius), &g.ctx.draw_state, g.ctx.transform, g.gfx);
    }
}

fn draw_polygon(g: &mut GfxCtx, pts: Vec<Vec2d>, color: Color) {
    graphics::Polygon::new(color).draw(&pts, &g.ctx.draw_state, g.ctx.transform, g.gfx);
}

fn angle_degrees(from: Pt2D, to: Pt2D) -> f64 {
    // Y inversion necessary because of drawing
    let theta_rads = (from.y() - to.y()).atan2(to.x() - from.x());
    let theta_degs = theta_rads * 360.0 / (2.0 * f64::consts::PI);
    // Normalize
    if theta_degs < 0.0 {
        theta_degs + 360.0
    } else {
        theta_degs
    }
}
