// Copyright 2021 Jonathan Manly.

// This file is part of rml.

// rml is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// rml is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with rml.  If not, see <https://www.gnu.org/licenses/>.

//! Implementation for K-Nearest Neighbors.

/*!
Allows for predicting data based on a KNN search.
A full, working example is contained in the `examples/knn` directory.

# Example
```rust
// Collect and parse data to a format consistent with:

use rml::knn;
use rml::math;
type CSVOutput = (Vec<Vec<f64>>, Vec<i32>);

// THESE ARE PLACEHOLDERS.
// You must load your own data first.

let training_data: CSVOutput = (Vec::new(), Vec::new());
let testing_data: CSVOutput = (Vec::new(), Vec::new());

// Create a new KNN struct.
let knn = knn::KNN::new(
5, // 5-nearest
training_data.0, // x
training_data.1, // y
None, // Default distance(euclidean)
Some(math::norm::Norm::L2), // L2 Normalization
);

// Get a prediction for each point of the testing data.
let pred: Vec<i32> = testing_data.0.iter().map(|x| knn.predict(x)).collect();

// Count the number that were predicted correctly.
let num_correct = pred
    .iter()
    .cloned()
    .zip(&testing_data.1)
    .filter(|(a, b)| *a == **b)
    .count();

println!(
    "Accuracy: {}",
    (num_correct as f64) / (pred.len() as f64)
);

```
!*/

use crate::math::distance;
use crate::math::norm;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashSet;

/// KNN struct handles the computation and data for the K-Nearest Neighbors algorithm.
/// It is *highly recommended* to not change values inside of this struct manually. Always
/// create a new one using ::new.
#[derive(Debug, Clone)]
pub struct KNN {
    /// K-Nearest to analyze
    pub k: i32,
    /// Features
    pub x: Vec<Vec<f64>>,
    /// Class labels for each feature.
    pub y: Vec<i32>,
    /// Number of labels.
    pub num_labels: usize,
    /// Type of distance to use.
    pub distance: Option<distance::Distance>,
    /// The type of normalization, or None.
    pub normalize: Option<norm::Norm>,
}

/// A data point.
#[derive(PartialEq, Debug)]
pub struct Point {
    /// The class label for the point.
    pub class: i32,
    /// The distance from the test point.
    pub distance: f64,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
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
    /// This function is called when ::new is called. You can access the value using
    /// the value contained in the KNN struct.
    pub fn get_num_labels(y: &[i32]) -> usize {
        let set: HashSet<i32> = y.iter().cloned().collect::<HashSet<_>>();
        set.len()
    }

    /// Helper function to convert data that has integer based labels to floating point labels.
    /// # Example
    /// ```rust
    /// use rml::knn::KNN;
    /// let q: Vec<Vec<i32>> = vec![vec![2, 2]];
    /// KNN::convert_to_f64(&q); // This will return the vector <2.0, 2.0>.
    ///
    pub fn convert_to_f64(xi32: &Vec<Vec<i32>>) -> Vec<Vec<f64>> {
        xi32.iter()
            .map(|x| x.iter().map(|val| *val as f64).collect())
            .collect()
    }

    /// Normalize the data contain in `self` given by the KNN's configured normalization setting.
    pub fn normalize_data(&mut self) {
        if let Some(n) = &self.normalize {
            self.x
                .iter_mut()
                .for_each(|xi| norm::normalize_vector(xi, n));
        }
    }

    /// Borrow immutable reference to the data.
    pub fn data(&self) -> (&Vec<Vec<f64>>, &Vec<i32>) {
        (&self.x, &self.y)
    }

    /// Calculate the distance from `new_point` to all other points in the set.
    /// Note: new_point must be the same dimensions as the data passed into ::new.
    pub fn calculate_distances(&self, new_point: &[f64]) -> Vec<Point> {
        let distance_fn = match self.distance {
            Some(distance::Distance::Manhattan) => distance::manhattan_distance,
            _ => distance::euclidean_distance,
        };

        self.x
            .par_iter()
            .zip(self.y.par_iter())
            .map(|(x, y)| Point {
                class: *y,
                distance: distance_fn(new_point, x),
            })
            .collect()
    }

    /// Predict the class of a point `x`.
    pub fn predict(&self, x: &[f64]) -> i32 {
        let mut norm_x: Vec<f64> = x.to_owned();
        if let Some(n) = &self.normalize {
            norm::normalize_vector(&mut norm_x, n);
        }
        let mut points = self.calculate_distances(x);
        // points.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        points.par_sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let mut predictions = vec![0; self.num_labels];

        for i in &points[0..self.k as usize] {
            predictions[i.class as usize] += 1;
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
        assert!((q[0].distance - f64::from(8).sqrt()).abs() < f64::EPSILON);
    }

    #[test]
    fn convert_to_f64_test() {
        let p: Vec<Vec<f64>> = vec![vec![2.0, 2.0]];
        let q: Vec<Vec<i32>> = vec![vec![2, 2]];

        assert_eq!(KNN::convert_to_f64(&q), p);
    }
}
