use std::io::stdin;

fn main() {
    println!("Input a list of comma separated values to receives the median and mode");

    let mut csv = String::new();
    stdin()
        .read_line(&mut csv)
        .expect("failed to read line");
    
    let ints: Result<Vec<i32>, _> = csv.split(',')
                  .map(|s| s.trim().parse::<i32>())
                  .collect();
    let mut ints_result = ints.unwrap();

    println!("Parsed integers {:?}", ints_result);

    let median = sort_array_and_get_median(&mut ints_result).unwrap();

    println!("Median: {median}");

    let mode = get_mode(ints_result).unwrap();

    println!("Mode: {mode}");
}

fn sort_array_and_get_median(ints: &mut Vec<i32>) -> Option<f64> {
    ints.sort();

    let length = ints.len();

    if length == 0 {
        return None;
    }
    
    let median = if length % 2 == 1 {
        ints[length / 2] as f64
    } else {
        ((ints[length / 2 - 1] + ints[length / 2]) as f64) / 2.0
    };

    Some(median)
}

fn get_mode(sorted_ints: Vec<i32>) -> Option<i32> {
    if sorted_ints.len() == 0 {
        return None;
    };

    let mut current_val = sorted_ints[0];
    let mut current_count = 1;
    let mut max_count = 1;
    let mut mode_val = current_val;

    for num in sorted_ints {
        if num == current_val {
            current_count+=1;

            if current_count > max_count {
                max_count = current_count;

                if mode_val != current_val {
                    mode_val = current_val;
                }
            }
        } else {
            current_val = num;
            max_count = 1;
        }
    }

    Some(mode_val)
}
