pub type Coord = f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BBox {
    xmin: Coord,
    ymin: Coord,
    xmax: Coord,
    ymax: Coord,
}

impl BBox {
    pub fn new(xmin: Coord, ymin: Coord, xmax: Coord, ymax: Coord) -> Self {
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
    fn new_stores_coordinates_in_order() {
        let b1 = BBox::new(1.0, 2.0, 3.0, 4.0);
        let b2 = BBox {
            xmin: 1.0,
            ymin: 2.0,
            xmax: 3.0,
            ymax: 4.0,
        };
        assert_eq!(b1, b2);
    }
}
