use rand::rng;
use rand::seq::SliceRandom;

#[cfg(target_arch = "wasm32")]
use web_time::{Duration, Instant};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};

const GRID_SIZE: usize = 5;
const NUM_CELLS: usize = GRID_SIZE * GRID_SIZE;

const FS_FONT: &[u8] = include_bytes!("../assets/fs.ttf");

pub struct SchulteGridApp {
    numbers: Vec<usize>,
    current_target: usize,
    game_over: bool,
    start_time: Instant,
    end_time: Option<Instant>,
}

impl Default for SchulteGridApp {
    fn default() -> Self {
        let mut numbers: Vec<usize> = (1..=NUM_CELLS).collect();
        numbers.shuffle(&mut rng());
        Self {
            numbers,
            current_target: 1,
            game_over: false,
            start_time: Instant::now(),
            end_time: None,
        }
    }
}

impl SchulteGridApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .font_data
            .insert("fs".to_owned(), egui::FontData::from_static(FS_FONT).into());
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "fs".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        Default::default()
    }
}

impl eframe::App for SchulteGridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.game_over {
            ctx.request_repaint_after(Duration::from_millis(10));
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("文件", |ui| {
                        if ui.button("退出").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });

                    ui.add_space(16.0);
                }

                if ui.button("再来一次").clicked() {
                    *self = Default::default();
                };
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(egui::RichText::new("舒尔特方格(5x5)").size(30.0));

            if self.game_over {
                if let Some(end) = self.end_time {
                    let duration = end.duration_since(self.start_time);
                    let seconds = duration.as_secs_f64();

                    ui.label(
                        egui::RichText::new(format!("完成! 用时 {:.2} 秒!", seconds))
                            .color(egui::Color32::GREEN)
                            .size(20.0),
                    );
                }
                if ui.button("再来一次").clicked() {
                    *self = Self::default();
                }
                return;
            }

            let elapsed = self.start_time.elapsed().as_secs_f32();
            ui.label(format!("你已用时 {:.2} 秒", elapsed));
            ui.label(egui::RichText::new(format!("目标数字: {}", self.current_target)).size(20.0));

            egui::Grid::new("schulte_grid")
                .striped(true)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    for (i, &number) in self.numbers.iter().enumerate() {
                        if ui
                            .add_sized(
                                egui::vec2(40.0, 40.0),
                                egui::Button::new(
                                    egui::RichText::new(number.to_string()).size(25.0),
                                ),
                            )
                            .clicked()
                        {
                            if number == self.current_target {
                                self.current_target += 1;
                                if self.current_target > NUM_CELLS {
                                    self.game_over = true;
                                    self.end_time = Some(Instant::now());
                                }
                            }
                        }

                        if (i + 1) % GRID_SIZE == 0 {
                            ui.end_row();
                        }
                    }
                });

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                about(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn about(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("@ 2025   |   Arco <arco@aoi.sh>");
    });
}
