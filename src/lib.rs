pub type Coord = f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BBoxError {
    NonFiniteCoordinate,
    InvalidCoordinateRange,
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
                return Err(BBoxError::InvalidCoordinateRange);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

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
