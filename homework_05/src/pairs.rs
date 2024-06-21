pub const VEC3_LEN: usize = 3;
pub type Pair = (i32, i32);
pub type Vec3 = [i32; VEC3_LEN];

#[no_mangle]
pub fn default_vec3() -> Vec3 {
    [0; 3]
}

#[no_mangle]
pub fn default_pair() -> Pair {
    (0, 0)
}

#[no_mangle]
pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
    let mut c = default_vec3();
    for i in 0..3 {
        c[i] = a[i] + b[i];
    }
    c
}

#[no_mangle]
pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

#[no_mangle]
pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
    let mut c = 0;
    for i in 0..VEC3_LEN {
        c += a[i] + b[i];
    }
    c
}

#[no_mangle]
pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_vec3() {
        let result = default_vec3();
        let expected: Vec3 = [0; VEC3_LEN];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_default_pair() {
        let result = default_pair();
        let expected: Pair = (0, 0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vec3_vector_sum() {
        let vec_1: Vec3 = [2; VEC3_LEN];
        let vec_2: Vec3 = [3; VEC3_LEN];
        let result = vec3_vector_sum(vec_1, vec_2);
        let expected: Vec3 = [5; VEC3_LEN];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vec3_scalar_sum() {
        let vec_1: Vec3 = [2; VEC3_LEN];
        let vec_2: Vec3 = [3; VEC3_LEN];
        let result = vec3_scalar_sum(vec_1, vec_2);
        let expected = 15;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pair_vector_sum() {
        let pair_1: Pair = (2, 3);
        let pair_2: Pair = (4, 5);
        let result = pair_vector_sum(pair_1, pair_2);
        let expected: Pair = (6, 8);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pair_scalar_sum() {
        let pair_1: Pair = (2, 3);
        let pair_2: Pair = (4, 5);
        let result = pair_scalar_sum(pair_1, pair_2);
        let expected = 14;
        assert_eq!(result, expected);
    }
}
