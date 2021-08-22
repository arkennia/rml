#[derive(Debug)]
pub enum Distance {
    Euclidean,
    Manhattan,
}

/// Calculate the euclidean distance between two points.
pub fn euclidean_distance(p: &[f64], q: &[f64]) -> f64 {
    let distance: f64 = q.iter().zip(p).map(|(&q, &p)| (f64::powi(q - p, 2))).sum();

    if distance == 0.0 {
        0.0
    } else {
        distance.sqrt()
    }
}

/// Calculate the manhattan distance between two points.
pub fn manhattan_distance(p: &[f64], q: &[f64]) -> f64 {
    let distance: f64 = p.iter().zip(q).map(|(&p, &q)| (p - q).abs()).sum();

    distance
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn euclidean_distance_test() {
        let x: Vec<f64> = vec![5.0, 6.0];
        let y: Vec<f64> = vec![-7.0, 11.0];

        assert_eq!(euclidean_distance(&x, &y), 13.0);

        let x: Vec<f64> = vec![0.0, 0.0, 0.0];
        let y: Vec<f64> = vec![1.0, 1.0, 1.0];

        assert_eq!(euclidean_distance(&x, &y), f64::from(3).sqrt());
    }

    #[test]
    fn manhattan_distance_test() {
        let x: Vec<f64> = vec![0.0, 0.0];
        let y: Vec<f64> = vec![1.0, 1.0];

        assert_eq!(manhattan_distance(&x, &y), 2.0);

        let x: Vec<f64> = vec![0.0, 0.0];
        let y: Vec<f64> = vec![-1.0, 1.0];

        assert_eq!(manhattan_distance(&x, &y), 2.0);

        let x: Vec<f64> = vec![0.0, 0.0, 0.0];
        let y: Vec<f64> = vec![1.0, 1.0, 1.0];

        assert_eq!(manhattan_distance(&x, &y), 3.0);
    }
}
