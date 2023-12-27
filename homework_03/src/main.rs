fn main() {
    let int_i_32: i32 = 5;
    let float_32: f32 = 5.;

    fn double_int32(x: i32) -> i32 {
        x * 2
    }
    println!("double_int32: {:?}", double_int32(int_i_32));

    fn double_int64(x: i32) -> i64 {
        x as i64 * 2
    }
    println!("double_int64: {:?}", double_int64(int_i_32));

    fn double_float32(x: f32) -> f32 {
        x * 2.
    }
    println!("double_float32: {:?}", double_float32(float_32));

    fn double_float64(x: f32) -> f64 {
        x as f64 * 2.
    }
    println!("double_float64: {:?}", double_float64(float_32));

    fn int_plus_float_to_float(x: i32, y: f32) -> f64 {
        x as f64 + y as f64
    }
    println!(
        "int_plus_float_to_float: {:?}",
        int_plus_float_to_float(int_i_32, float_32)
    );

    fn int_plus_float_to_int(x: i32, y: f32) -> i64 {
        x as i64 + y as i64
    }
    println!(
        "int_plus_float_to_int: {:?}",
        int_plus_float_to_int(int_i_32, float_32)
    );

    fn tuple_sum(tuple: (i32, i32)) -> i32 {
        let (x, y) = tuple;
        x + y
    }
    println!("tuple_sum: {:?}", tuple_sum((int_i_32, int_i_32)));

    fn array_sum(array: [i32; 3]) -> i32 {
        array[0] + array[1] + array[2]
    }
    println!("array_sum: {:?}", array_sum([int_i_32, int_i_32, int_i_32]));
}
