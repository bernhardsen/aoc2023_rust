

pub(crate) fn rotate_vec_of_vec<T:Copy>(input: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..input[0].len()).map(|col| {
        input.iter().rev().map(|r| *r.get(col).unwrap()).collect::<Vec<T>>()
    }).collect()
}
