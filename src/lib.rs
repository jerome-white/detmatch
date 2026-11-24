type Coord = f64;

#[derive(Debug, Clone, Copy, PartialEq)]
struct BBox {
    xmin: Coord,
    ymin: Coord,
    xmax: Coord,
    ymax: Coord,
}

impl BBox {
    fn new(xmin: Coord, ymin: Coord, xmax: Coord, ymax: Coord) -> Self {
        Self {
            xmin,
            ymin,
            xmax,
            ymax,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_coordinates_are_equal_bboxes() {
        let b1 = BBox::new(0.0, 0.0, 1.0, 1.0);
        let b2 = BBox::new(b1.xmin, b1.ymin, b1.xmax, b1.ymax);
        assert_eq!(b1, b2);
    }

    #[test]
    fn swapped_coordinates_are_unequal_bboxes() {
        let b1 = BBox::new(0.0, 0.0, 1.0, 1.0);
        let b2 = BBox::new(b1.xmin, b1.xmax, b1.ymin, b1.ymax);
        assert_ne!(b1, b2);
    }
}
