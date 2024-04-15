use sorting_library::{quick_sort, selection_sort, insertion_sort, merge_sort};

fn main() {
    
    let mut integers = vec![3, 2, 5, 1, 4];
    quick_sort(&mut integers);
    println!("Quick sort for integers: {:?}", integers);

    let mut integers = vec![3, 2, 5, 1, 4];
    selection_sort(&mut integers);
    println!("Selection sort for integers: {:?}", integers);

    let mut integers = vec![3, 2, 5, 1, 4];
    insertion_sort(&mut integers);
    println!("Insertion sort for integers: {:?}", integers);

    let mut integers = vec![3, 2, 5, 1, 4];
    merge_sort(&mut integers);
    println!("Merge sort for integers: {:?}", integers);

    let mut strings = vec!["apple", "banana", "cherry", "date", "fig"];
    quick_sort(&mut strings);
    println!("Quick sort for strings: {:?}", strings);

    let mut strings = vec!["apple", "banana", "cherry", "date", "fig"];
    selection_sort(&mut strings);
    println!("Selection sort for strings: {:?}", strings);

    let mut strings = vec!["apple", "banana", "cherry", "date", "fig"];
    insertion_sort(&mut strings);
    println!("Insertion sort for strings: {:?}", strings);

    let mut strings = vec!["apple", "banana", "cherry", "date", "fig"];
    merge_sort(&mut strings);
    println!("Merge sort for strings: {:?}", strings);
}
