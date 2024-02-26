pub fn default_vec3() -> Vec3 {
    [0; 3]
}

pub fn default_pair() -> Pair {
    (0, 0)
}

pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
    let mut c = default_vec3();
    for i in 0..3 {
        c[i] = a[i] + b[i];
    }
    c
}

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
    let mut c = 0;
    for i in 0..VEC3_LEN {
        c += a[i] + b[i];
    }
    c
}

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}
