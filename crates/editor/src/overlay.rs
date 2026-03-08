use std::ops::Range;

use crate::{DisplayPoint, DisplayRow};
use gpui::{AnyElement, HighlightStyle, IntoElement, StyledText};

#[derive(Debug, Clone, Default)]
pub struct Overlay {
    pub text: String,
    pub highlights: Vec<(Range<usize>, HighlightStyle)>,
    pub point: DisplayPoint,
}

impl Overlay {
    pub fn render(
        &self,
        visible_display_row_range: Range<DisplayRow>,
    ) -> Option<(DisplayPoint, AnyElement)> {
        if !visible_display_row_range.contains(&self.point.row()) {
            return None;
        }
        let iter = self.highlights.iter().cloned();

        let el = StyledText::new(self.text.clone())
            .with_highlights(iter)
            .into_any_element();
        Some((self.point, el))
    }
}
