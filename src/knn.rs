use crate::math::distance;
use crate::math::norm;
use std::cmp::Ordering;

/// KNN struct handles the computation and data for the K-Nearest Neighbors algorithm.
pub struct KNN {
    /// K-Nearest to analyze
    pub k: i32,
    /// Features
    x: Vec<Vec<f64>>,
    /// Class labels for each feature.
    y: Vec<i32>,
    /// Number of labels.
    num_labels: usize,
    /// Type of distance to use.
    pub distance: Option<distance::Distance>,
    /// The type of normalization, or None.
    pub normalize: Option<norm::Norm>,
}

/// (class value, distance)
#[derive(PartialEq, Debug)]
pub struct Point(i32, f64);

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Eq for Point {}

impl KNN {
    /// Create a new KNN with optional normalization.
    pub fn new(
        k: i32,
        x: Vec<Vec<f64>>,
        y: Vec<i32>,
        distance: Option<distance::Distance>,
        normalize: Option<norm::Norm>,
    ) -> KNN {
        let num_labels = KNN::get_num_labels(&y);
        let mut knn = KNN {
            k,
            x,
            y,
            num_labels,
            distance,
            normalize,
        };
        knn.normalize_data();
        knn
    }

    /// Gets the number of unique labels.
    fn get_num_labels(y: &[i32]) -> usize {
        let mut labels: Vec<i32> = Vec::new();

        for i in y {
            if !labels.contains(i) {
                labels.push(*i);
            }
        }

        labels.len()
    }

    /// Normalize the data given by the KNN's configured normalization setting.
    pub fn normalize_data(&mut self) {
        if let Some(n) = &self.normalize {
            self.x = self
                .x
                .iter()
                .map(|xi| norm::normalize_vector(xi, n))
                .collect();
        }
    }

    /// Borrow immutable reference to the data.
    pub fn data(&self) -> (&Vec<Vec<f64>>, &Vec<i32>) {
        (&self.x, &self.y)
    }

    /// Calculate the distance from `new_point` to all other points in the set.
    pub fn calculate_distances(&self, new_point: &[f64]) -> Vec<Point> {
        let distance_fn = match self.distance {
            Some(distance::Distance::Manhattan) => distance::manhattan_distance,
            _ => distance::euclidean_distance,
        };
        // let mut distances: Vec<Point> = Vec::new();

        // for i in 0..self.x.len() {
        //     distances.push(Point(self.y[i], distance_fn(&new_point, &self.x[i])));
        // }
        // distances

        self.x
            .iter()
            .zip(self.y.iter())
            .map(|(x, y)| Point(*y, distance_fn(new_point, x)))
            .collect()
    }

    /// Predict the class of a point `x`.
    pub fn predict(&self, x: Vec<f64>) -> i32 {
        let x = match &self.normalize {
            None => x,
            Some(n) => norm::normalize_vector(&x, n),
        };
        let mut points = self.calculate_distances(&x);
        points.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let mut predictions = vec![0; self.num_labels];

        for i in 0..(self.k) as usize {
            predictions[points[i].0 as usize] += 1;
        }
        KNN::get_max_value(&predictions)
    }

    /// Get the class of the highest index.
    fn get_max_value(predictions: &[i32]) -> i32 {
        predictions
            .iter()
            .enumerate() // add index to the iterated items [a, b, c] -> [(0, a), (1, b), (2, c)]
            .max_by_key(|(_, pred)| **pred) // take maximum by the actual item, not the index,
            // `pred` has type `&&i32`, because of all the combinators, so we have to dereference twice
            .map(|(idx, _)| idx) // Option::map - take tuple (idx, value) and transform it to just idx
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_data_test() {
        let p: Vec<Vec<f64>> = vec![vec![2.0, 2.0, 2.0]];
        let mut knn = KNN::new(5, p, vec![1], None, Some(norm::Norm::L2));
        knn.normalize_data();
        assert_eq!(
            knn.data().0.clone(),
            vec![vec![2.0 / f64::from(12).sqrt(); 3]]
        );
    }

    #[test]
    fn calculate_distances_test() {
        let p: Vec<Vec<f64>> = vec![vec![2.0, 2.0]];
        let knn = KNN::new(5, p, vec![1], None, None);

        let q = knn.calculate_distances(&(vec![0.0, 0.0] as Vec<f64>));
        assert_eq!(q[0].1, f64::from(8).sqrt());
    }
}
