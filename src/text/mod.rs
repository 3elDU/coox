pub mod render;

use taffy::{AvailableSpace, Size};
use unicode_segmentation::UnicodeSegmentation;

/// Size of each character.
pub struct FontMetrics {
    /// Character width in pixels
    pub width: f32,
    /// Character height in pixels
    pub height: f32,
}

/// Fit text into the given space, optionally with width/height provided by
/// parent.
///
/// Handles line wrapping.
pub fn measure_text(
    text: &str,
    metrics: &FontMetrics,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
) -> Size<f32> {
    if text.is_empty() {
        return Size::zero();
    }

    let min_line_length: usize = 1; // scale down to just 1 character in width
    let max_line_length = text.graphemes(true).count();

    let width = known_dimensions
        .width
        .unwrap_or_else(|| match available_space.width {
            AvailableSpace::MinContent => metrics.width * min_line_length as f32,
            AvailableSpace::MaxContent => metrics.width * max_line_length as f32,
            // round down to character width
            AvailableSpace::Definite(width) => f32::trunc(width / metrics.width) * metrics.width,
        });
    let height = known_dimensions.height.unwrap_or_else(|| {
        let lines = f32::ceil(max_line_length as f32 * metrics.width / width);
        return lines * metrics.height;
    });

    Size { width, height }
}

#[cfg(test)]
mod tests {
    use crate::text::{FontMetrics, measure_text};
    use taffy::{AvailableSpace, Size};

    const EMPTY_TEXT: &'static str = "";
    const TEXT: &'static str = "The quick brown fox jumps over the lazy dog";
    const METRICS: FontMetrics = FontMetrics {
        width: 12.0,
        height: 24.0,
    };

    #[test]
    fn empty_text() {
        assert!(matches!(
            measure_text(EMPTY_TEXT, &METRICS, Size::NONE, Size::max_content(),),
            Size {
                width: 0.0,
                height: 0.0
            }
        ));
    }

    #[test]
    fn max_content() {
        let size = measure_text(TEXT, &METRICS, Size::NONE, Size::max_content());

        assert_eq!(size.width, TEXT.len() as f32 * METRICS.width);
        assert_eq!(size.height, METRICS.height);
    }

    #[test]
    fn min_content() {
        let size = measure_text(TEXT, &METRICS, Size::NONE, Size::min_content());

        // The text will scale to 1 character in width, but will wrap on every
        // next character
        assert_eq!(size.width, METRICS.width);
        assert_eq!(size.height, TEXT.len() as f32 * METRICS.height);
    }

    #[test]
    fn wrap() {
        let width = TEXT.len() as f32 / 2.0 * METRICS.width;
        let size = measure_text(
            TEXT,
            &METRICS,
            Size::NONE,
            Size {
                width: AvailableSpace::Definite(width),
                height: AvailableSpace::MaxContent,
            },
        );

        assert_eq!(size.width, (TEXT.len() / 2) as f32 * METRICS.width);
        // Text wraps to 3 lines because the number of characters is uneven
        assert_eq!(size.height, METRICS.height * 3.0);
    }

    /// The available space is only one pixel smaller to fit text in one line,
    /// so one character should wrap to the next line
    #[test]
    fn wrap_one_char() {
        let width = TEXT.len() as f32 * METRICS.width - 1.0;
        let size = measure_text(
            TEXT,
            &METRICS,
            Size::NONE,
            Size {
                width: AvailableSpace::Definite(width),
                height: AvailableSpace::MaxContent,
            },
        );

        assert_eq!(size.width, (TEXT.len() - 1) as f32 * METRICS.width);
        assert_eq!(size.height, METRICS.height * 2.0);
    }
}
