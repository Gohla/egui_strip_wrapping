use egui::*;
use egui_extras::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {}

impl Default for TemplateApp {
  fn default() -> Self { Self {} }
}

impl TemplateApp {
  /// Called once before the first frame.
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    if let Some(storage) = cc.storage {
      return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
    }
    Default::default()
  }
}

impl eframe::App for TemplateApp {
  /// Called by the frame work to save state before shutdown.
  fn save(&mut self, storage: &mut dyn eframe::Storage) {
    eframe::set_value(storage, eframe::APP_KEY, self);
  }

  /// Called each time the UI needs repainting, which may be many times per second.
  /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // CentralPanel::default().show(ctx, |ui| {
    //   ui.with_layout(Layout::left_to_right().with_main_wrap(true).with_cross_align(Align::TOP), |ui| {
    //     for i in 0..50 { ui.button(format!("Test {}", i)); }
    //   });
    // });
    
    // CentralPanel::default().show(ctx, |ui| {
    //   ui.with_layout(Layout::left_to_right().with_main_wrap(true), |ui| {
    //     CollapsingHeader::new("Test Test Test Test Test").default_open(true).show(ui, |ui| {
    //       for i in 0..3 { ui.label(format!("Test {}", i)); }
    //     });
    //     CollapsingHeader::new("Test Test Test Test Test Test Test Test Test Test").default_open(true).show(ui, |ui| {
    //       for i in 3..9 { ui.label(format!("Test {}", i)); }
    //     });
    //   });
    // });
    
    CentralPanel::default().show(ctx, |ui| {
      StripBuilder::new(ui)
        .cell_layout(Layout::left_to_right().with_main_wrap(true))
        .size(Size::remainder())
        .horizontal(|mut strip| {
          strip.cell(|ui| {
            CollapsingHeader::new("Test Test Test Test Test").default_open(true).show(ui, |ui| {
              for i in 0..3 { ui.label(format!("Test {}", i)); }
            });
            CollapsingHeader::new("Test Test Test Test Test Test Test Test Test Test").default_open(true).show(ui, |ui| {
              for i in 3..9 { ui.label(format!("Test {}", i)); }
            });
          });
        });
    });
  }
}

