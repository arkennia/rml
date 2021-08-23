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

//! Contains functions for normalizing vectors.

/// Describes the types of normalizations that are possible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Norm {
    L1,
    L2,
}

/// Produces an L2 norm from the given data.
/// # Example
/// ```rust
/// let p: Vec<f64> = vec![2.0, 2.0, 2.0];
/// println!("{}", l2_norm(&p));
/// ```
pub fn l2_norm(p: &[f64]) -> f64 {
    let norm: f64 = p.iter().map(|x| x.powi(2)).sum();

    norm.sqrt()
}

/// Produces an L1 norm from the given data.
/// # Example
/// ```rust
/// let p: Vec<f64> = vec![2.0, 2.0, 2.0];
/// println!("{}", l1_norm(&p));
/// ```
pub fn l1_norm(p: &[f64]) -> f64 {
    let norm: f64 = p.iter().map(|x| x.abs()).sum();

    norm
}

/// Produces a normalizard from the given data.
/// # Example
/// ```rust
/// let mut p: Vec<f64> = vec![2.0, 2.0, 2.0];
/// println!("{:?}", normalize_vector(&mut p, &Norm::L2));
/// ```
pub fn normalize_vector(p: &mut [f64], norm_type: &Norm) {
    let norm = match norm_type {
        Norm::L1 => l1_norm(p),
        Norm::L2 => l2_norm(p),
    };
    if norm != 0.0 {
        p.iter_mut().for_each(|xi| *xi /= norm);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l2_norm_test() {
        let p: Vec<f64> = vec![2.0, 2.0, 2.0];

        assert_eq!(l2_norm(&p), f64::from(12).sqrt());
    }

    #[test]
    fn l1_norm_test() {
        let p: Vec<f64> = vec![2.0, -2.0, 2.0];

        assert_eq!(l1_norm(&p), f64::from(6));
    }

    #[test]
    fn normalize_vector_test() {
        let mut p: Vec<f64> = vec![2.0, 2.0, 2.0];
        normalize_vector(&mut p, &Norm::L2);
        assert_eq!(p, vec![2.0 / f64::from(12).sqrt(); 3]);
    }
}
