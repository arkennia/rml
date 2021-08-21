pub enum Norm {
    L1,
    L2,
}

pub fn l2_norm(p: &Vec<f64>) -> f64 {
    let norm: f64 = p.iter().map(|x| x.powi(2)).sum();

    norm.sqrt()
}

pub fn l1_norm(p: &Vec<f64>) -> f64 {
    let norm: f64 = p.iter().map(|x| x.abs()).sum();

    norm
}

pub fn normalize_vector(p: &Vec<f64>, norm_type: &Norm) -> Vec<f64> {
    let norm = match norm_type {
        Norm::L1 => l1_norm(&p),
        Norm::L2 => l2_norm(&p),
    };
    if norm == 0.0 {
        Vec::with_capacity(p.len()) as Vec<f64>
    } else {
        p.iter().map(|xi| xi / norm).collect()
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
        p = normalize_vector(&p, &Norm::L2);
        assert_eq!(p, vec![2.0 / f64::from(12).sqrt(); 3]);
    }
}
