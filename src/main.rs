fn main() {
    let rotations = 4;
    let arr = vec![1, 2, 3, 4, 5];
    println!("Initial array: {:?}", arr);
    println!(
        "Array after {} rotations: {:?}",
        rotations,
        left_rotation(rotations, arr)
    );
}

fn left_rotation(rotations: u32, arr: Vec<u32>) -> Vec<u32> {
    let mut arr = arr;
    let mut temp_arr = vec![0; arr.len()];
    for _ in 0..rotations {
        for (i, _) in arr.iter().enumerate() {
            match i {
                0 => temp_arr[arr.len() - 1] = arr[0],
                _ => {
                    temp_arr[i - 1] = arr[i];
                }
            }
        }
        arr = temp_arr.to_vec();
    }
    return arr;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2_rotations() {
        let rotations = 2;
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let result_arr = left_rotation(rotations, arr);
        let expected_arr = [3, 4, 5, 6, 7, 8, 1, 2];
        assert_eq!(result_arr, expected_arr);
    }
    #[test]
    fn test_4_rotations() {
        let rotations = 4;
        let arr = vec![1, 2, 3, 4, 5];
        let result_arr = left_rotation(rotations, arr);
        let expected_arr = [5, 1, 2, 3, 4];
        assert_eq!(result_arr, expected_arr);
    }
}
