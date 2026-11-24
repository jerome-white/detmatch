type Coord = f64;

#[derive(Debug, Clone, Copy, PartialEq)]
struct BBox {
    xmin: Coord,
    xmax: Coord,
    ymin: Coord,
    ymax: Coord,
}

impl BBox {
    fn new(xmin: Coord, xmax: Coord, ymin: Coord, ymax: Coord) -> Self {
        Self { xmin, xmax, ymin, ymax }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bbox = BBox::new(0.0, 1.0, 0.0, 1.0);
        assert_eq!(bbox.xmin, 0.0);
    }
}
