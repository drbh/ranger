#![allow(dead_code, unused_imports)]

#[cfg(test)]
mod tests {
    use polygon2::difference;
    use polygon2::intersection;

    #[test]
    fn new_chain() {
        let subject = [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [0.0, 0.0]];
        let clip = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5]];

        let difference_polygons = difference(&subject, &clip);
        let intersection_polygons = intersection(&subject, &clip);
        println!("{:?}", difference_polygons);
        println!("{:?}", intersection_polygons);

        assert_eq!(
            intersection_polygons,
            [[[0.0, 0.0], [0.5, 0.0], [0.5, 0.5], [0.0, 0.5]]]
        );
    }
}
