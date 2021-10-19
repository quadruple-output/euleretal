use super::{
    ui_import::{egui::DragValue, Color32, Stroke, Ui},
    PointFormat,
};

pub fn my_stroke_ui(ui: &mut Ui, stroke: &mut Stroke, text: &str, tooltip: &str) {
    let Stroke { width, color } = stroke;
    ui.horizontal(|ui| {
        ui.color_edit_button_srgba(color);
        ui.add(DragValue::new(width).speed(0.1).clamp_range(0.0..=5.0))
            .on_hover_text("Width");

        my_stroke_preview(ui, (*width, *color), None);

        if tooltip.is_empty() {
            ui.label(text);
        } else {
            ui.label(text).on_hover_text(tooltip);
        }
    });
}

pub fn my_stroke_preview(
    ui: &mut Ui,
    stroke: impl Into<Stroke>,
    point_format_or_none: Option<(&PointFormat, Color32)>,
) {
    let (_id, stroke_rect) = ui.allocate_space(ui.spacing().interact_size);
    let left = stroke_rect.left_center();
    let right = stroke_rect.right_center();
    ui.painter().line_segment([left, right], stroke);
    if let Some((format, dot_color)) = point_format_or_none {
        let mut adapted_format: PointFormat = (*format).clone();
        adapted_format.stroke.color = dot_color;
        adapted_format.draw_position_on(stroke_rect.center(), ui.painter());
    }
}
