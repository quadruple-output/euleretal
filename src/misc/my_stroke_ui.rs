use eframe::egui;

pub fn my_stroke_ui(ui: &mut crate::Ui, stroke: &mut egui::Stroke, text: &str, tooltip: &str) {
    let egui::Stroke { width, color } = stroke;
    ui.horizontal(|ui| {
        ui.color_edit_button_srgba(color);
        ui.add(
            egui::DragValue::f32(width)
                .speed(0.1)
                .clamp_range(0.0..=5.0),
        )
        .on_hover_text("Width");

        // stroke preview:
        let (_id, stroke_rect) = ui.allocate_space(ui.spacing().interact_size);
        let left = stroke_rect.left_center();
        let right = stroke_rect.right_center();
        ui.painter().line_segment([left, right], (*width, *color));

        if tooltip.is_empty() {
            ui.label(text);
        } else {
            ui.label(text).on_hover_text(tooltip);
        }
    });
}
