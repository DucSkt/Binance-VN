#[macro_export]
macro_rules! clg {
    ($title:expr, $expression:expr) => {{
        use colored::{Color, ColoredString, Colorize};
        use fxhash::FxHasher;
        use std::hash::{Hash, Hasher};
        // List of colors to choose from
        let colors = vec![
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::BrightRed,
            Color::BrightGreen,
            Color::BrightYellow,
            Color::BrightBlue,
            Color::BrightMagenta,
            Color::BrightCyan,
        ];

        // Closure to consistently map a title to a color
        let get_color_for_title = |title: &str| {
            // Hash the title to a usize value
            let mut hasher = FxHasher::default();
            title.hash(&mut hasher);
            let hash_value = hasher.finish();

            // Map the hash value to a color index
            colors[hash_value as usize % colors.len()]
        };

        // Apply the selected color consistently to the title
        let color = get_color_for_title(&$title.to_string());
        let colored_title: ColoredString = $title.to_string().color(color);

        // Print the title with consistent color and expression with bright yellow
        println!(
            "{}: {}",
            colored_title,
            format!("{:?}", $expression).bright_yellow()
        );
    }};
}

// #[macro_use]
// mod marco;
