pub type Coord = f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BBoxError {
    NonFiniteCoordinate,
    InvertedCoordinateRange,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(approx_derive::AbsDiffEq))]
pub struct BBox {
    xmin: Coord,
    ymin: Coord,
    xmax: Coord,
    ymax: Coord,
}

impl BBox {
    pub fn new(
        xmin: Coord,
        ymin: Coord,
        xmax: Coord,
        ymax: Coord,
    ) -> Result<Self, BBoxError> {
        for (min, max) in [(xmin, xmax), (ymin, ymax)] {
            if !min.is_finite() || !max.is_finite() {
                return Err(BBoxError::NonFiniteCoordinate);
            }
            if max < min {
                return Err(BBoxError::InvertedCoordinateRange);
            }
        }

        let bbox = Self {
            xmin,
            ymin,
            xmax,
            ymax,
        };

        return Ok(bbox);
    }

    pub fn width(&self) -> Coord {
        return self.xmax - self.xmin;
    }

    pub fn height(&self) -> Coord {
        return self.ymax - self.ymin;
    }

    pub fn area(&self) -> Coord {
        return self.width() * self.height();
    }

    pub fn intersection(&self, other: &BBox) -> Option<BBox> {
        let xmin = self.xmin.max(other.xmin);
        let ymin = self.ymin.max(other.ymin);
        let xmax = self.xmax.min(other.xmax);
        let ymax = self.ymax.min(other.ymax);

        let result = BBox::new(xmin, ymin, xmax, ymax);

        return result.ok();
    }

    pub fn iou(&self, other: &BBox) -> Option<BBox> {
        intersection = self.intersection(other);
        union = self.area() + other.area() - intersection;

        return intersection / union;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod construction {
        use super::*;

        #[test]
        fn new_stores_coordinates_in_order() {
            let b1 = BBox::new(1.0, 2.0, 3.0, 4.0).unwrap();
            let b2 = BBox {
                xmin: 1.0,
                ymin: 2.0,
                xmax: 3.0,
                ymax: 4.0,
            };
            assert_eq!(b1, b2);
        }

        #[test]
        fn negative_coordinates_okay() {
            let result = BBox::new(-5.0, -3.0, 2.0, 1.0);
            assert!(result.is_ok());
        }

        #[test]
        fn inverted_x_is_invalid() {
            let result = BBox::new(1.0, 0.0, 0.0, 0.0);
            assert_eq!(result, Err(BBoxError::InvertedCoordinateRange));
        }

        #[test]
        fn inverted_y_is_invalid() {
            let result = BBox::new(0.0, 1.0, 0.0, 0.0);
            assert_eq!(result, Err(BBoxError::InvertedCoordinateRange));
        }

        fn coordinates_cannot_be(bad: Coord) {
            let mut coords = [0.0; 4];
            for i in 0..coords.len() {
                coords[i] = bad;
                let result =
                    BBox::new(coords[0], coords[1], coords[2], coords[3]);
                assert_eq!(result, Err(BBoxError::NonFiniteCoordinate));
                coords[i] = 0.0
            }
        }

        #[test]
        fn coordinates_must_be_numeric() {
            coordinates_cannot_be(Coord::NAN);
        }

        #[test]
        fn coordinates_must_be_finite() {
            coordinates_cannot_be(Coord::INFINITY);
        }
    }

    mod size {
        use super::*;
        use approx::assert_abs_diff_eq;

        #[test]
        fn width_is_xrange() {
            let xmin = 0.0;
            let xmax = 1.0;
            let bx = BBox::new(xmin, 2.0, xmax, 4.0).unwrap();
            assert_eq!(bx.width(), xmax - xmin);
        }

        #[test]
        fn height_is_yrange() {
            let ymin = 0.0;
            let ymax = 1.0;
            let bx = BBox::new(1.0, ymin, 3.0, ymax).unwrap();
            assert_eq!(bx.height(), ymax - ymin);
        }

        #[test]
        fn area_is_width_times_height() {
            let bx = BBox::new(1.0, 2.0, 3.0, 5.0).unwrap();
            assert_abs_diff_eq!(bx.area(), 6.0);
        }

        #[test]
        fn flat_box_has_zero_area() {
            let bx = BBox::new(0.0, 0.0, 1.0, 0.0).unwrap();
            assert_abs_diff_eq!(bx.area(), 0.0);
        }
    }

    mod intersection {
        use super::*;

        macro_rules! assert_intersection {
            ($lhs:expr, $rhs:expr, $target:expr) => {
                for (l, r) in [(&$lhs, &$rhs), (&$rhs, &$lhs)] {
                    let source = l.intersection(r);
                    assert_eq!(source, $target, "{:?} ^ {:?}", l, r);
                }
            };
        }

        #[test]
        fn complete() {
            let bx1 = BBox::new(0.0, 0.0, 3.0, 3.0).unwrap();
            let bx2 = BBox::new(1.0, 1.0, 2.0, 2.0).unwrap();
            assert_intersection!(bx1, bx2, Some(bx2));
        }

        #[test]
        fn partial() {
            let bx0 = BBox::new(1.0, 1.0, 2.0, 2.0).unwrap(); // expected
            let (xmin, ymin) = (bx0.xmin - 1.0, bx0.ymin - 1.0);
            let (xmax, ymax) = (bx0.xmax + 1.0, bx0.ymax + 1.0);

            let bx1 = BBox::new(xmin, ymin, bx0.xmax, bx0.ymax).unwrap();
            let bx2 = BBox::new(bx0.xmin, bx0.ymin, xmax, ymax).unwrap();

            assert_intersection!(bx1, bx2, Some(bx0));
        }

        #[test]
        fn exact() {
            let bx1 = BBox::new(0.0, 0.0, 1.0, 1.0).unwrap();
            assert_intersection!(bx1, bx1, Some(bx1));
        }

        #[test]
        fn disjoint() {
            let bx1 = BBox::new(0.0, 0.0, 1.0, 1.0).unwrap();
            let bx2 = BBox::new(2.0, 2.0, 3.0, 3.0).unwrap();
            assert_intersection!(bx1, bx2, None);
        }

        #[test]
        fn separated_on_x_only_is_none() {
            let bx1 = BBox::new(0.0, 0.0, 1.0, 1.0).unwrap();
            let bx2 = BBox::new(2.0, 0.0, 3.0, 1.0).unwrap();
            assert_intersection!(bx1, bx2, None);
        }

        #[test]
        fn separated_on_y_only_is_none() {
            let bx1 = BBox::new(0.0, 0.0, 1.0, 1.0).unwrap();
            let bx2 = BBox::new(0.0, 2.0, 1.0, 3.0).unwrap();
            assert_intersection!(bx1, bx2, None);
        }

        #[test]
        fn shared_horizontal_edge_is_flat() {
            let bx0 = BBox::new(0.0, 1.0, 1.0, 1.0).unwrap(); // edge
            let (ymin, ymax) = (bx0.ymin - 1.0, bx0.ymax + 1.0);

            let bx1 = BBox::new(bx0.xmin, ymin, bx0.xmax, bx0.ymax).unwrap();
            let bx2 = BBox::new(bx0.xmin, bx0.ymin, bx0.xmax, ymax).unwrap();

            assert_intersection!(bx1, bx2, Some(bx0));
        }

        #[test]
        fn shared_edge_is_flat() {
            let bx0 = BBox::new(1.0, 0.0, 1.0, 1.0).unwrap(); // edge
            let (xmin, xmax) = (bx0.xmin - 1.0, bx0.xmax + 1.0);

            let bx1 = BBox::new(xmin, bx0.ymin, bx0.xmax, bx0.ymax).unwrap();
            let bx2 = BBox::new(bx0.xmin, bx0.ymin, xmax, bx0.ymax).unwrap();

            assert_intersection!(bx1, bx2, Some(bx0));
        }

        #[test]
        fn shared_corner_is_point() {
            let bx0 = BBox::new(1.0, 1.0, 1.0, 1.0).unwrap(); // corner

            let bx1 = BBox::new(0.0, 0.0, bx0.xmax, bx0.ymax).unwrap();
            let bx2 = BBox::new(bx0.xmin, bx0.ymin, 2.0, 2.0).unwrap();

            assert_intersection!(bx1, bx2, Some(bx0));
        }
    }
}
