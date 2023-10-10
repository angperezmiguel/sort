use clap::Parser;
use sort;
use std::collections::HashSet;
use std::fs;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    filename: Option<String>,
    #[clap(short, long)]
    unique: bool,

    #[clap(long)]
    heapsort: bool,

    #[clap(long)]
    mergesort: bool,

    #[clap(long)]
    radixsort: bool,

    #[clap(long)]
    qsort: bool,

    #[clap(long)]
    random_sort: bool,
}

fn delete_duplicates(words: &mut Vec<&str>) {
    let mut occurrences = HashSet::new();
    let mut index = 0;
    while index < words.len() {
        let word = words[index];
        if occurrences.contains(&word) {
            words.remove(index);
        } else {
            occurrences.insert(word);
            index += 1; // Only increment if no removal was done
        }
    }
}

fn main() -> () {
    let cli = Cli::parse();

    // Match can read existing file
    if let Some(filename) = cli.filename {
        let file_contents = fs::read_to_string(filename).unwrap();
        let words: Vec<&str> = file_contents.lines().collect();

        // Apply sorting
        let mut end = sort::quicksort(words);

        if cli.qsort {
            end = sort::quicksort(end);
        }

        if cli.heapsort {
            unimplemented!("Heapsort is not implemented...yet");
        }

        if cli.mergesort {
            unimplemented!("Mergesort is not implemented...yet");
        }

        if cli.radixsort {
            unimplemented!("Radix sort is not implemented...yet");
        }

        if cli.random_sort {
            unimplemented!("Random sort is not implemented...yet");
        }

        // end.sort();

        if cli.unique {
            delete_duplicates(&mut end);
        }

        end.iter().for_each(|x| println!("{}", x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_partition() {
    //     let data = vec!["banana", "date", "apple", "fig", "cherry"];
    //     let (left, pivot, right) = partition(data);
    //     assert!(left.iter().all(|x| *x < pivot));
    //     assert!(right.iter().all(|x| *x > pivot));
    // }

    #[test]
    fn test_empty_vector() {
        let data: Vec<&str> = Vec::new();
        let data = sort::quicksort(data);
        assert_eq!(data, Vec::<&str>::new());
    }

    #[test]
    fn test_sorted_vector() {
        let data = vec!["apple", "banana", "cherry", "date", "fig"];
        let expected = vec!["apple", "banana", "cherry", "date", "fig"];
        let data = sort::quicksort(data);
        assert_eq!(data, expected);
    }

    #[test]
    fn test_reverse_sorted_vector() {
        let data = vec!["fig", "date", "cherry", "banana", "apple"];
        let expected = vec!["apple", "banana", "cherry", "date", "fig"];
        let data = sort::quicksort(data);
        assert_eq!(data, expected);
    }

    #[test]
    fn test_random_order_vector() {
        let data = vec!["banana", "date", "apple", "fig", "cherry"];
        let expected = vec!["apple", "banana", "cherry", "date", "fig"];
        let data = sort::quicksort(data);
        assert_eq!(data, expected);
    }
}
