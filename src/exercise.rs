use std::collections::HashMap;


fn get_median(v: &mut Vec<i32>) -> Option<i32> {
    match v.capacity() < 3 {
        true => None,
        false => {
            v.sort_by(|a, b| a.cmp(b));
            let queue_capacity = v.len();

            let is_balanced_array = queue_capacity % 2 > 0;

            println!("sorted array {v:?}");
            match is_balanced_array {
                false => {
                    let last_index = queue_capacity / 2;
                    let median = v
                        .get(last_index)
                        .map(|n| n + v.get(last_index - 1).unwrap())
                        .map(|total| total / 2)
                        .unwrap_or(0);
                    Some(median)
                }
                true => Some(v[queue_capacity / 2]),
            }
        }
    }
}

pub(crate) fn get_mode(v: &mut Vec<i32>) -> Option<i32> {
    let mut map = HashMap::new();

    for num in v {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
    let mode = map
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(a, b)| *b)
        .unwrap();

    Some(mode)
}
