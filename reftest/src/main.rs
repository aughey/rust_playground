fn create_int_vec() -> Vec<i32> {
  Vec::new()
}

fn append_int_vec(vec: &Vec<i32>, count: usize) -> Vec<i32> {
    let newvalues = (0..count).into_iter().map(|_| i32::default());
    let oldvalues = vec.iter();
    oldvalues.copied().chain(newvalues).collect()
}

fn append_generic_vec<T : Copy + Default>(vec: &Vec<T>, count: usize) -> Vec<T> {
    let newvalues = (0..count).into_iter().map(|_| T::default());
    let oldvalues = vec.iter();
    oldvalues.copied().chain(newvalues).collect()
}

fn int_use_of_generic(vec: &Vec<i32>, count: usize) -> Vec<i32> {
    append_generic_vec(vec, count)
}

fn test_parse(s : &str) -> Result<i32, std::num::ParseIntError> {
    let v = s.parse::<i32>()?;
    Ok(v * 2)
}

fn main() {
    println!("Hello, world!");
    println!("{}", test_parse("123").unwrap());
}
