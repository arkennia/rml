// Copyright 2021 Jonathan Manly.

// This file is part of rustml.

// rustml is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// rustml is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with rustml.  If not, see <https://www.gnu.org/licenses/>.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
