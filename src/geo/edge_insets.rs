use crate::style_spec::util::number;
// use crate::util::clamp;
use nalgebra::{clamp, Point2};

#[derive(Default)]
pub(crate) struct EdgeInsets {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl EdgeInsets {
    pub fn interpolate(&mut self, start: PaddingOptions, target: PaddingOptions, t: f32) {
        if let Some(start_top) = start.top {
            if let Some(target_top) = target.top {
                self.top = number(start_top, target_top, t);
            }
        }
        if let Some(start_bottom) = start.bottom {
            if let Some(target_bottom) = target.bottom {
                self.bottom = number(start_bottom, target_bottom, t);
            }
        }
        if let Some(start_left) = start.left {
            if let Some(target_left) = target.left {
                self.left = number(start_left, target_left, t);
            }
        }
        if let Some(start_right) = start.right {
            if let Some(target_right) = target.right {
                self.right = number(start_right, target_right, t);
            }
        }
    }

    pub fn center(&self, width: f32, height: f32) -> Point2<f32> {
        let x = clamp((self.left + width - self.right) / 2.0, 0.0, width);
        let y = clamp((self.top + height - self.bottom) / 2.0, 0.0, height);

        Point2::new(x, y)
    }

    pub fn padding_options(&self) -> PaddingOptions {
        PaddingOptions {
            top: if self.top != 0.0 {
                Some(self.top)
            } else {
                None
            },
            bottom: if self.bottom != 0.0 {
                Some(self.bottom)
            } else {
                None
            },
            left: if self.left != 0.0 {
                Some(self.left)
            } else {
                None
            },
            right: if self.right != 0.0 {
                Some(self.right)
            } else {
                None
            },
        }
    }
}

#[derive(Default, PartialEq)]
pub(crate) struct PaddingOptions {
    pub top: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
    pub right: Option<f32>,
}
