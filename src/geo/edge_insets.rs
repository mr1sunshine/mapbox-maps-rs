use crate::style_spec::util::number;
use nalgebra::{clamp, Point2};
use num::traits::Float;

#[derive(Default, Debug)]
pub(crate) struct EdgeInsets<T: Float + std::fmt::Debug> {
    top: T,
    bottom: T,
    left: T,
    right: T,
}

impl<T: 'static + Float + std::fmt::Debug> EdgeInsets<T> {
    pub fn interpolate(&mut self, start: PaddingOptions<T>, target: PaddingOptions<T>, t: T) {
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

    pub fn center(&self, width: T, height: T) -> Point2<T> {
        let x = clamp(
            (self.left + width - self.right) / T::from(2.0).unwrap(),
            T::from(0.0).unwrap(),
            width,
        );
        let y = clamp(
            (self.top + height - self.bottom) / T::from(2.0).unwrap(),
            T::from(0.0).unwrap(),
            height,
        );

        Point2::new(x, y)
    }

    pub fn padding_options(&self) -> PaddingOptions<T> {
        PaddingOptions {
            top: if self.top != T::from(0.0).unwrap() {
                Some(self.top)
            } else {
                None
            },
            bottom: if self.bottom != T::from(0.0).unwrap() {
                Some(self.bottom)
            } else {
                None
            },
            left: if self.left != T::from(0.0).unwrap() {
                Some(self.left)
            } else {
                None
            },
            right: if self.right != T::from(0.0).unwrap() {
                Some(self.right)
            } else {
                None
            },
        }
    }
}

#[derive(Default, PartialEq)]
pub(crate) struct PaddingOptions<T: Float> {
    pub top: Option<T>,
    pub bottom: Option<T>,
    pub left: Option<T>,
    pub right: Option<T>,
}
