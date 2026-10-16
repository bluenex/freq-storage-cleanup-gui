use eframe::egui::Color32;

const COLORS: [Color32; 8] = [
    Color32::from_rgb(72, 160, 150),
    Color32::from_rgb(112, 122, 214),
    Color32::from_rgb(214, 150, 62),
    Color32::from_rgb(206, 96, 122),
    Color32::from_rgb(112, 170, 92),
    Color32::from_rgb(162, 112, 202),
    Color32::from_rgb(82, 152, 212),
    Color32::from_rgb(220, 122, 72),
];

/// Color for the group at `index` in size order.
pub fn group_color(index: usize) -> Color32 {
    COLORS[index % COLORS.len()]
}

/// Unselected items are a faint wash of their group color; selected ones are solid.
pub fn item_fill(color: Color32, selected: f32) -> Color32 {
    color.gamma_multiply(0.28).lerp_to_gamma(color, selected)
}
