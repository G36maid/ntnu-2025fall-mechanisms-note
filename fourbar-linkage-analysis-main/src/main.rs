//! Four-Bar Linkage Simulator
//!
//! Interactive simulator using egui for real-time mechanism visualization
//! and Newton-Raphson numerical analysis.
//!
//! Supports both native desktop and web (WASM) deployment.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fourbar;

use eframe::egui;
use fourbar::{FourBar, Point2D};

// =============================================================================
// App Creation (Shared between Native and WASM)
// =============================================================================

fn create_app(_cc: &eframe::CreationContext) -> Box<dyn eframe::App> {
    // You can load fonts or images here if needed
    Box::new(FourBarApp::new())
}

// =============================================================================
// Native Entry Point (Desktop: Windows, macOS, Linux)
// =============================================================================

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Four-Bar Linkage Simulator - Newton-Raphson Method"),
        ..Default::default()
    };

    eframe::run_native("four_bar_sim", options, Box::new(|cc| Ok(create_app(cc))))
}

// =============================================================================
// WASM Entry Point (Web Browser)
// =============================================================================

#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast;

    // Redirect panics to console.error for debugging in browser
    console_error_panic_hook::set_once();

    // Initialize tracing for web logging
    tracing_wasm::set_as_global_default();

    // Start the web app automatically
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find canvas with id 'the_canvas_id'")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Element is not a canvas");

        eframe::WebRunner::new()
            .start(canvas, web_options, Box::new(|cc| Ok(create_app(cc))))
            .await
            .expect("failed to start eframe");
    });
}

// =============================================================================
// Application State
// =============================================================================

struct FourBarApp {
    linkage: FourBar,
    theta2_deg: f64,
    auto_play: bool,
    animation_speed: f64,
    scale: f64,
    show_trace: bool,
    trace_points: Vec<Point2D>,
    max_trace_points: usize,
    error_message: Option<String>,
    show_angles: bool,
    show_grid: bool,
}

impl FourBarApp {
    fn new() -> Self {
        let mut app = Self {
            linkage: FourBar::new(),
            theta2_deg: 0.0,
            auto_play: false,
            animation_speed: 2.0,
            scale: 50.0,
            show_trace: false,
            trace_points: Vec::new(),
            max_trace_points: 360,
            error_message: None,
            show_angles: true,
            show_grid: true,
        };

        // Initialize with theta2 = 0
        let _ = app.linkage.set_theta2_degrees(0.0);

        app
    }

    fn update_mechanism(&mut self) {
        match self.linkage.set_theta2_degrees(self.theta2_deg) {
            Ok(_) => {
                self.error_message = None;

                // Add to trace if enabled
                if self.show_trace {
                    let positions = self.linkage.get_positions();
                    self.trace_points.push(positions.p3);

                    // Limit trace points
                    if self.trace_points.len() > self.max_trace_points {
                        self.trace_points.remove(0);
                    }
                }
            }
            Err(e) => {
                self.error_message = Some(e);
            }
        }
    }

    fn draw_mechanism(&self, _ui: &mut egui::Ui, painter: &egui::Painter, center: egui::Pos2) {
        let positions = self.linkage.get_positions();

        // Convert mechanism coordinates to screen coordinates
        let to_screen = |p: Point2D| -> egui::Pos2 {
            egui::pos2(
                center.x + p.x as f32 * self.scale as f32,
                center.y - p.y as f32 * self.scale as f32, // Flip Y axis
            )
        };

        let p1_screen = to_screen(positions.p1);
        let p2_screen = to_screen(positions.p2);
        let p3_screen = to_screen(positions.p3);
        let p4_screen = to_screen(positions.p4);

        // Draw grid if enabled
        if self.show_grid {
            self.draw_grid(painter, center);
        }

        // Draw trace if enabled
        if self.show_trace && self.trace_points.len() > 1 {
            let trace_screen: Vec<egui::Pos2> =
                self.trace_points.iter().map(|&p| to_screen(p)).collect();

            painter.add(egui::Shape::line(
                trace_screen,
                egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 165, 0)),
            ));
        }

        // Draw links
        let link_stroke = egui::Stroke::new(3.0, egui::Color32::from_rgb(50, 50, 50));

        // Ground link (r1)
        painter.line_segment([p1_screen, p4_screen], link_stroke);

        // Input crank (r2)
        painter.line_segment(
            [p1_screen, p2_screen],
            egui::Stroke::new(3.0, egui::Color32::from_rgb(200, 50, 50)),
        );

        // Coupler link (r3)
        painter.line_segment(
            [p2_screen, p3_screen],
            egui::Stroke::new(3.0, egui::Color32::from_rgb(50, 150, 200)),
        );

        // Output rocker (r4)
        painter.line_segment(
            [p3_screen, p4_screen],
            egui::Stroke::new(3.0, egui::Color32::from_rgb(50, 200, 50)),
        );

        // Draw joints (circles)
        let joint_radius = 6.0;
        let joint_color = egui::Color32::from_rgb(40, 40, 40);
        let joint_fill = egui::Color32::from_rgb(220, 220, 220);

        // Ground joints (fixed)
        painter.circle(
            p1_screen,
            joint_radius + 2.0,
            joint_fill,
            egui::Stroke::new(2.0, joint_color),
        );
        painter.circle(
            p4_screen,
            joint_radius + 2.0,
            joint_fill,
            egui::Stroke::new(2.0, joint_color),
        );

        // Moving joints
        painter.circle_filled(
            p2_screen,
            joint_radius,
            egui::Color32::from_rgb(255, 100, 100),
        );
        painter.circle(
            p2_screen,
            joint_radius,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(2.0, joint_color),
        );

        painter.circle_filled(
            p3_screen,
            joint_radius,
            egui::Color32::from_rgb(100, 180, 255),
        );
        painter.circle(
            p3_screen,
            joint_radius,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(2.0, joint_color),
        );

        // Draw angle labels if enabled
        if self.show_angles {
            let state = self.linkage.state;

            // θ2 label
            painter.text(
                egui::pos2(p1_screen.x + 20.0, p1_screen.y - 20.0),
                egui::Align2::LEFT_TOP,
                format!("θ₂ = {:.1}°", state.theta2.to_degrees()),
                egui::FontId::proportional(14.0),
                egui::Color32::from_rgb(200, 50, 50),
            );

            // θ3 label
            painter.text(
                egui::pos2(p2_screen.x, p2_screen.y - 25.0),
                egui::Align2::CENTER_TOP,
                format!("θ₃ = {:.1}°", state.theta3.to_degrees()),
                egui::FontId::proportional(14.0),
                egui::Color32::from_rgb(50, 150, 200),
            );

            // θ4 label
            painter.text(
                egui::pos2(p4_screen.x - 20.0, p4_screen.y - 20.0),
                egui::Align2::RIGHT_TOP,
                format!("θ₄ = {:.1}°", state.theta4.to_degrees()),
                egui::FontId::proportional(14.0),
                egui::Color32::from_rgb(50, 200, 50),
            );
        }

        // Draw link labels
        let label_color = egui::Color32::from_gray(100);

        painter.text(
            egui::pos2(
                (p1_screen.x + p4_screen.x) / 2.0,
                (p1_screen.y + p4_screen.y) / 2.0 + 15.0,
            ),
            egui::Align2::CENTER_CENTER,
            format!("r₁ = {:.1}", self.linkage.config.r1),
            egui::FontId::proportional(12.0),
            label_color,
        );
    }

    fn draw_grid(&self, painter: &egui::Painter, center: egui::Pos2) {
        let grid_color = egui::Color32::from_rgba_premultiplied(150, 150, 150, 30);
        let axis_color = egui::Color32::from_rgba_premultiplied(100, 100, 100, 100);
        let grid_spacing = self.scale as f32;

        let rect = painter.clip_rect();

        // Vertical grid lines
        let mut x = center.x;
        while x < rect.max.x {
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                egui::Stroke::new(1.0, grid_color),
            );
            x += grid_spacing;
        }

        x = center.x;
        while x > rect.min.x {
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                egui::Stroke::new(1.0, grid_color),
            );
            x -= grid_spacing;
        }

        // Horizontal grid lines
        let mut y = center.y;
        while y < rect.max.y {
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                egui::Stroke::new(1.0, grid_color),
            );
            y += grid_spacing;
        }

        y = center.y;
        while y > rect.min.y {
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                egui::Stroke::new(1.0, grid_color),
            );
            y -= grid_spacing;
        }

        // Draw axes
        painter.line_segment(
            [
                egui::pos2(rect.min.x, center.y),
                egui::pos2(rect.max.x, center.y),
            ],
            egui::Stroke::new(2.0, axis_color),
        );

        painter.line_segment(
            [
                egui::pos2(center.x, rect.min.y),
                egui::pos2(center.x, rect.max.y),
            ],
            egui::Stroke::new(2.0, axis_color),
        );
    }
}

impl eframe::App for FourBarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-play animation
        if self.auto_play {
            self.theta2_deg += self.animation_speed;
            if self.theta2_deg >= 360.0 {
                self.theta2_deg -= 360.0;
            }
            self.update_mechanism();
            ctx.request_repaint();
        }

        // Control Panel (Left Side)
        egui::SidePanel::left("control_panel")
            .min_width(280.0)
            .show(ctx, |ui| {
                ui.heading("⚙ Controls");
                ui.add_space(10.0);

                // Theta2 control
                ui.label("Input Angle (θ₂):");
                let slider = egui::Slider::new(&mut self.theta2_deg, 0.0..=360.0)
                    .suffix("°")
                    .step_by(1.0);

                if ui.add(slider).changed() && !self.auto_play {
                    self.update_mechanism();
                }

                ui.add_space(5.0);

                // Animation controls
                ui.separator();
                ui.label("Animation:");

                if ui.checkbox(&mut self.auto_play, "Auto Play").changed() && self.auto_play {
                    ctx.request_repaint();
                }

                ui.add(
                    egui::Slider::new(&mut self.animation_speed, 0.5..=10.0)
                        .text("Speed")
                        .suffix(" °/frame"),
                );

                ui.add_space(5.0);

                // Link length controls
                ui.separator();
                ui.label("Link Lengths:");

                let mut config_changed = false;

                config_changed |= ui
                    .add(
                        egui::Slider::new(&mut self.linkage.config.r1, 1.0..=10.0)
                            .text("r₁ (Ground)")
                            .step_by(0.1),
                    )
                    .changed();

                config_changed |= ui
                    .add(
                        egui::Slider::new(&mut self.linkage.config.r2, 0.5..=8.0)
                            .text("r₂ (Crank)")
                            .step_by(0.1),
                    )
                    .changed();

                config_changed |= ui
                    .add(
                        egui::Slider::new(&mut self.linkage.config.r3, 1.0..=10.0)
                            .text("r₃ (Coupler)")
                            .step_by(0.1),
                    )
                    .changed();

                config_changed |= ui
                    .add(
                        egui::Slider::new(&mut self.linkage.config.r4, 1.0..=10.0)
                            .text("r₄ (Rocker)")
                            .step_by(0.1),
                    )
                    .changed();

                if config_changed {
                    self.update_mechanism();
                    if self.show_trace {
                        self.trace_points.clear();
                    }
                }

                ui.add_space(5.0);

                // Display options
                ui.separator();
                ui.label("Display:");

                ui.checkbox(&mut self.show_angles, "Show Angles");
                ui.checkbox(&mut self.show_grid, "Show Grid");

                if ui
                    .checkbox(&mut self.show_trace, "Show Trace (Coupler Curve)")
                    .changed()
                    && !self.show_trace
                {
                    self.trace_points.clear();
                }

                if self.show_trace {
                    ui.add(
                        egui::Slider::new(&mut self.max_trace_points, 50..=1000)
                            .text("Trace Points"),
                    );

                    if ui.button("Clear Trace").clicked() {
                        self.trace_points.clear();
                    }
                }

                ui.add(
                    egui::Slider::new(&mut self.scale, 20.0..=100.0)
                        .text("Zoom")
                        .suffix("x"),
                );

                ui.add_space(10.0);

                // Mechanism info
                ui.separator();
                ui.label("Mechanism Info:");

                ui.horizontal(|ui| {
                    ui.label("Type:");
                    ui.label(egui::RichText::new(self.linkage.mechanism_type()).color(
                        if self.linkage.is_grashof() {
                            egui::Color32::from_rgb(50, 200, 50)
                        } else {
                            egui::Color32::from_rgb(200, 50, 50)
                        },
                    ));
                });

                ui.label(format!(
                    "Grashof: {}",
                    if self.linkage.is_grashof() {
                        "Yes ✓"
                    } else {
                        "No ✗"
                    }
                ));

                let state = self.linkage.state;
                ui.label(format!("θ₃ = {:.2}°", state.theta3.to_degrees()));
                ui.label(format!("θ₄ = {:.2}°", state.theta4.to_degrees()));

                // Error display
                if let Some(ref error) = self.error_message {
                    ui.add_space(10.0);
                    ui.separator();
                    ui.colored_label(egui::Color32::RED, "⚠ Error:");
                    ui.label(error);
                }

                ui.add_space(10.0);
                ui.separator();
                ui.label(
                    egui::RichText::new("NTNU Mechatronic Engineering")
                        .size(10.0)
                        .color(egui::Color32::GRAY),
                );
            });

        // Main drawing area
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let (response, painter) = ui.allocate_painter(available_size, egui::Sense::hover());

            let center = response.rect.center();

            // Draw mechanism
            self.draw_mechanism(ui, &painter, center);

            // Title overlay
            painter.text(
                egui::pos2(response.rect.min.x + 10.0, response.rect.min.y + 10.0),
                egui::Align2::LEFT_TOP,
                "Four-Bar Linkage Simulator",
                egui::FontId::proportional(20.0),
                egui::Color32::from_rgba_premultiplied(0, 0, 0, 150),
            );
        });
    }
}
