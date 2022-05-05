use eframe::{
    egui::{self, plot::{Points, Plot, Values, Value, Line}, Layout},
    epi::{App}, run_native,
};
use functions::function_value;
use laguere::{laguere_approx, laguere_approx_value};

mod functions;
mod laguere;
mod integral;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Nodes,
    AproxError,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Function {
    Poly1,
    Poly2,
    Linear,
    Sinusoidal,
    Absolute,
    Mixed,
}

struct AppState {
    function: Function,
    left: f64,
    right: f64,
    no_of_nodes: usize,
    mode: Mode,
    chosen_function_values: Vec<Value>,
    approx_values: Vec<Value>,
    approx_coefficients: Vec<f64>,
    center_plot: bool,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            function: Function::Poly1,
            left: 0.,
            right: 2.,
            no_of_nodes: 2,
            mode: Mode::Nodes,
            chosen_function_values: Vec::new(),
            approx_values: Vec::new(),
            approx_coefficients: Vec::new(),
            center_plot: false,
        }
    }
}

impl App for AppState {
    fn name(&self) -> &str {
        "Laguere Polynomial Approximation"
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &eframe::epi::Frame) {
        ctx.set_pixels_per_point(1.5);
        egui::SidePanel::left("left_panel").min_width(150.).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.group(|ui| {
                    ui.heading("Function");
                    ui.add_space(5.);
    
                    ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
                        ui.radio_value(&mut self.function, Function::Poly1, "Polynomial 2nd Power");
                        ui.radio_value(&mut self.function, Function::Poly2, "Polynomial 4th power");
                        ui.radio_value(&mut self.function, Function::Linear, "Linear");
                        ui.radio_value(&mut self.function, Function::Sinusoidal, "Sinusoidal");
                        ui.radio_value(&mut self.function, Function::Absolute, "Absolute");
                        ui.radio_value(&mut self.function, Function::Mixed, "Mixed");
                    });
                });
                ui.group(|ui| {
                    ui.heading("Approx. Range");
                    ui.add_space(10.);
                    ui.horizontal(|ui| {
                        ui.add_space(2.);
                        ui.label("L :");
                        ui.add(egui::DragValue::new(&mut self.left));
                        ui.add_space(5.);
                        ui.label("R :");
                        ui.add(egui::DragValue::new(&mut self.right));
                        ui.add_space(1.);
                    });
                    
                });
                ui.label("Mode");
                ui.radio_value(&mut self.mode, Mode::Nodes, "Nodes");
                ui.radio_value(&mut self.mode, Mode::AproxError, "Approx. Error");
                match self.mode {
                    Mode::Nodes => {
                        ui.group(|ui| {
                            ui.label("No. Of Nodes");
                            ui.add(egui::Slider::new(&mut self.no_of_nodes, 2..=10));
                        });
                    }
                    Mode::AproxError => {
                        ui.label("todo");
                    }
                }
                if ui.button("Calculate").clicked() {

                    // default some parameters
                    self.chosen_function_values = Vec::new();
                    self.approx_values = Vec::new();
                    self.approx_coefficients = Vec::new();

                    let mut min = 0.;
                    if self.left < 3. {
                        self.left = 0.;
                        min = 0.;
                    } else {
                        min = self.left - 3.;
                    }
                    let max = self.right + 10.;
                    // generating values of chosen function for the plot
                    self.chosen_function_values = (0..10000)
                    .map(|i| {
                        let x = min + i as f64 *
                        ((max) - (min)) / 10000.;
                        Value::new(x, function_value(x, self.function, false))
                    })
                    .collect();

                    // generating values of approximated function for the plot
                    self.approx_coefficients = laguere_approx(self.function, self.no_of_nodes, self.left, self.right);
                    for i in 0..10000 {
                        let x = min + i as f64 * ((max) - (min)) / 10000.;
                        self.approx_values.push(Value::new(x, laguere_approx_value(&self.approx_coefficients, x)));
                    }
                }
                let mut text: String = String::new();
                for i in &self.approx_coefficients {
                    text.push_str(&format!("{:.5}, ", i));
                }
                ui.text_edit_multiline(
                    &mut text
                )
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            //chosen function
            let chosen_values = Values::from_values(self.chosen_function_values.clone());
            let approx_values = Values::from_values(self.approx_values.clone());
            let chosen_plot = Line::new(chosen_values).name("Chosen Function");
            let approx_plot = Line::new(approx_values).name("Approx. Function");

            ui.checkbox(&mut self.center_plot, "Center Plot");
            let mut plot = Plot::new("my_plot")
                .show_x(true)
                .show_y(true)
                .legend(egui::widgets::plot::Legend::default());
            plot = plot.data_aspect(1.0);
            if self.center_plot {
                plot = plot.center_x_axis(true).center_y_axis(true)
            }
            plot.show(ui, |plot_ui| {
                plot_ui.line(chosen_plot);
                plot_ui.line(approx_plot);
            });
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000., 660.0)),
        ..eframe::NativeOptions::default()
    };
    run_native(Box::new(AppState::new()), native_options);
}
