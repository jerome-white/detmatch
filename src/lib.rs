type Coord = f64;

struct BBox {
    xmin: Coord,
    xmax: Coord,
    ymin: Coord,
    ymax: Coord,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bbox = BBox {
            xmin: 0.0,
            xmax: 1.0,
            ymin: 0.0,
            ymax: 1.0,
        };
        assert_eq!(bbox.xmin, 0.0);
    }
}
