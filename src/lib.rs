pub mod day_eight;
pub mod day_eleven;
pub mod day_five;
pub mod day_four;
pub mod day_nine;
pub mod day_one;
pub mod day_seven;
pub mod day_six;
pub mod day_ten;
pub mod day_three;
pub mod day_twelve;
pub mod day_two;
pub mod question;

use num_traits::int::PrimInt;
use num_traits::{abs_sub, Signed};

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct Point2D<T> {
    x: T,
    y: T,
}

impl<T: PrimInt> Point2D<T> {
    fn is_moore(&self, other: &Point2D<T>) -> bool {
        self.x <= T::saturating_add(other.x, T::one())
            && self.x >= T::saturating_sub(other.x, T::one())
            && self.y <= T::saturating_add(other.y, T::one())
            && self.y >= T::saturating_sub(other.y, T::one())
    }
}

impl<T: PrimInt + Signed> Point2D<T> {
    fn manhattan(&self, other: &Point2D<T>) -> T {
        abs_sub(self.x, other.x) + abs_sub(self.y, other.y)
    }
}

impl<T: PrimInt> Default for Point2D<T> {
    fn default() -> Self {
        Point2D {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Point2D;

    #[test]
    fn point_moore_signed() {
        let a: Point2D<i32> = Point2D { x: 0, y: 0 };
        let b: Point2D<i32> = Point2D { x: 1, y: 0 };
        let c: Point2D<i32> = Point2D { x: -1, y: -1 };
        let d: Point2D<i32> = Point2D { x: 0, y: 1 };

        assert!(a.is_moore(&b));
        assert!(a.is_moore(&c));
        assert!(!b.is_moore(&c));
        assert!(b.is_moore(&d));

        // symmetric
        assert!(b.is_moore(&a));
        assert!(c.is_moore(&a));
        assert!(!c.is_moore(&b));
        assert!(d.is_moore(&b));
    }

    #[test]
    fn point_moore_unsigned() {
        let a: Point2D<u32> = Point2D { x: 0, y: 0 };
        let b: Point2D<u32> = Point2D { x: 1, y: 0 };
        let c: Point2D<u32> = Point2D { x: 2, y: 2 };
        let d: Point2D<u32> = Point2D { x: 0, y: 1 };

        assert!(a.is_moore(&b));
        assert!(!a.is_moore(&c));
        assert!(!b.is_moore(&c));
        assert!(b.is_moore(&d));

        // symmetric
        assert!(b.is_moore(&a));
        assert!(!c.is_moore(&a));
        assert!(!c.is_moore(&b));
        assert!(d.is_moore(&b));
    }

    #[test]
    fn point_default() {
        let a: Point2D<i32> = Point2D::default();
        assert_eq!(a, Point2D { x: 0, y: 0 });

        let a: Point2D<u8> = Point2D::default();
        assert_eq!(a, Point2D { x: 0, y: 0 });

        let a: Point2D<usize> = Point2D::default();
        assert_eq!(a, Point2D { x: 0, y: 0 });
    }
}
