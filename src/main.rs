use eframe::{egui, wgpu};
use egui_wgpu::{WgpuConfiguration, WgpuSetup, WgpuSetupCreateNew};
use std::sync::Arc;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).



    let wgpu_setup: WgpuSetup = WgpuSetup::CreateNew(WgpuSetupCreateNew {
        device_descriptor: Arc::new(|adapter| {
            wgpu::DeviceDescriptor {
                label: Some("egui wgpu device"),
                required_limits: wgpu::Limits {
                    ..wgpu::Limits::downlevel_defaults()
                }
                .using_resolution(adapter.limits()),
                ..Default::default()
            }
        }),
        ..WgpuSetupCreateNew::without_display_handle()
    });

    let wgpu_options = WgpuConfiguration {
        wgpu_setup: wgpu_setup,
        ..WgpuConfiguration::default()
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        wgpu_options: wgpu_options,
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    eframe::run_ui_native("My egui App", options, move |ui, _frame| {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    })
}
