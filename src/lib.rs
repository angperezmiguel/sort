use rand::Rng;

fn partition(words: Vec<&str>) -> (Vec<&str>, &str, Vec<&str>) {
    let mut result = words.clone();
    let mut rng = rand::thread_rng();
    let pivot_pos = rng.gen_range(0..result.len());
    let pivot = result[pivot_pos];
    let mut left = 0;
    let mut right = result.len() - 1;
    while left < right {
        while result[left] < pivot {
            left += 1;
        }
        while result[right] > pivot {
            right -= 1;
        }

        if left < right {
            result.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    (
        result[0..left].to_vec(),
        result[left],
        result[left + 1..].to_vec(),
    )
}

pub fn quicksort(words: Vec<&str>) -> Vec<&str> {
    let length = words.len();

    let mut end = vec![];
    if length <= 1 {
        return words;
    } else {
        let (left, pivot, right) = partition(words);
        let mut q_left = quicksort(left);
        let mut q_right = quicksort(right);
        let mut q_pivot = vec![pivot];
        end.append(&mut q_left);
        end.append(&mut q_pivot);
        end.append(&mut q_right);
        end
    }
}

pub enum SortingAlgorithms {
    QuickSort,
}
