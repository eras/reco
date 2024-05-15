mod digit;
mod digits;
mod find;
mod numpad;
mod rules;

use digits::Digits;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

struct AppMutState {
    reco_info: Arc<Mutex<find::Info>>,
    thread: Option<std::thread::JoinHandle<()>>,
}

struct ReCoApp {
    pixels_per_point: Option<f32>,
    mut_state: RefCell<AppMutState>,
    digits_input: String,
}

impl ReCoApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let reco_info = Arc::new(Mutex::new(Default::default()));

        let reco_info = RefCell::new(AppMutState {
            reco_info,
            thread: None,
        });

        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self {
            mut_state: reco_info,
            pixels_per_point: None,
            digits_input: String::from(""),
        }
    }

    fn start(
        reco_info: Arc<Mutex<find::Info>>,
        digits_input: String,
    ) -> std::thread::JoinHandle<()> {
        let info_to_app = InfoToApp::new(reco_info);
        let digits = if digits_input.is_empty() {
            None
        } else {
            Some(Digits::from(&find::parse_digits(&digits_input)[..]))
        };
        let thread = std::thread::spawn(move || {
            find::find(digits, info_to_app);
        });
        thread
    }
}

impl eframe::App for ReCoApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let ppp = match self.pixels_per_point {
                Some(x) => x,
                None => {
                    let x = ctx.pixels_per_point() * 3.0;
                    self.pixels_per_point = Some(x);
                    x
                }
            };
            ctx.set_pixels_per_point(ppp);
            ui.heading("Remember Code");

            {
                let info = self.mut_state.borrow();
                let info = info.reco_info.lock().unwrap();
                // // Display total and matching counts
                ui.label(format!("Total: {}", info.total));
                ui.label(format!("Matching: {}", info.matches));
                let ratio = info.matches as f64 / info.total as f64 * 100.0;
                ui.label(format!("Ratio: {:.2}%", ratio));
            }
            // Display digits input field
            ui.horizontal(|ui| {
                ui.label("Digits:");
                ui.text_edit_singleline(&mut self.digits_input);
            });

            // Display buttons to perform actions
            ui.horizontal(|ui| {
                if ui.button("Calculate").clicked() {
                    let mut reco_info = self.mut_state.borrow_mut();
                    reco_info.thread = Some(Self::start(
                        reco_info.reco_info.clone(),
                        self.digits_input.clone(),
                    ));
                }
                if ui.button("Reset").clicked() {
                    // Reset input and results
                    // (code to reset state and results)
                }
            });

            {
                let info = self.mut_state.borrow();
                let info = info.reco_info.lock().unwrap();

                let long_text = &info.message;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(long_text);
                });
            }
        });
    }
}

struct InfoToApp {
    info: Arc<Mutex<find::Info>>,
}

impl InfoToApp {
    fn new(info: Arc<Mutex<find::Info>>) -> Self {
        InfoToApp { info }
    }
}

impl find::InfoSignal for InfoToApp {
    fn update(&self, new_info: find::Info) {
        let mut info = self.info.lock().unwrap();
        *info = new_info
    }
}

fn main() {
    // let matches = App::new("Remember Code")
    //     .arg(
    //         Arg::with_name("digits")
    //             .takes_value(true)
    //             .index(1)
    //             .help("Digits to query"),
    //     )
    //     .get_matches();

    // let digits = matches.value_of("digits").unwrap_or("");
    // let digits = if !digits.is_empty() {
    //     Some(Digits::from(&find::parse_digits(digits)[..]))
    // } else {
    //     None
    // };

    // GUI setup and rendering
    let options = eframe::NativeOptions {
        //initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    let title = "Remember Code";

    eframe::run_native("ReCo", options, Box::new(|cc| Box::new(ReCoApp::new(cc)))).unwrap();
}

#[derive(Default)]
struct AppState {
    digits_input: String,
}
