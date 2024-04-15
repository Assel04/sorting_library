# Sorting Library

A Rust library providing implementations of various sorting algorithms.

## Usage

To use this library, simply import the sorting algorithms in your Rust code:

## Sorting Algorithms

### Quick Sort

Quick sort is a divide-and-conquer algorithm that partitions an array into smaller sub-arrays and recursively sorts them.

<img src="https://github.com/AsselK2023/sorting_library/blob/main/111.JPG" width="400">

### Selection Sort

Selection sort repeatedly selects the minimum element from the unsorted portion of the array and swaps it with the first unsorted element.

<img src="https://github.com/AsselK2023/sorting_library/blob/main/222.JPG" width="400">

### Insertion Sort

Insertion sort builds the sorted array one element at a time by repeatedly taking the next element and inserting it into the correct position in the sorted portion of the array.

<img src="https://github.com/AsselK2023/sorting_library/blob/main/333.JPG" width="400">

### Merge Sort

Merge sort divides the array into smaller sub-arrays, sorts them individually, and then merges them back together to form the final sorted array.

<img src="https://github.com/AsselK2023/sorting_library/blob/main/444.JPG" width="400">


## Examples

Here are some examples of sorting algorithms in action:

### Quick Sort

```rust
fn main() {

  let mut integers = vec![3, 2, 5, 1, 4];
    quick_sort(&mut integers);
    println!("Quick sort for integers: {:?}", integers);

  let mut strings = vec!["apple", "banana", "cherry", "date", "fig"];
    quick_sort(&mut strings);
    println!("Quick sort for strings: {:?}", strings);

}
```

### Selection Sort

```rust
fn main() {

  let mut integers = vec![3, 2, 5, 1, 4];
    selection_sort(&mut integers);
    println!("Selection sort for integers: {:?}", integers);

  let mut strings = vec!["apple", "banana", "cherry", "date", "fig"];
    selection_sort(&mut strings);
    println!("Selection sort for strings: {:?}", strings);

}
```

### Insertion Sort

```rust
fn main() {

  let mut integers = vec![3, 2, 5, 1, 4];
    insertion_sort(&mut integers);
    println!("Insertion sort for integers: {:?}", integers);

  let mut strings = vec!["apple", "banana", "cherry", "date", "fig"];
    insertion_sort(&mut strings);
    println!("Insertion sort for strings: {:?}", strings);

}
```

### Merge Sort

```rust
fn main() {

  let mut integers = vec![3, 2, 5, 1, 4];
    merge_sort(&mut integers);
    println!("Merge sort for integers: {:?}", integers);

  let mut strings = vec!["apple", "banana", "cherry", "date", "fig"];
    merge_sort(&mut strings);
    println!("Merge sort for strings: {:?}", strings);

}
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

- **Title**: Clearly states the title of the project.
- **Usage**: Provides instructions on how to use the library in Rust projects.
- **Sorting Algorithms**: Describes each sorting algorithm provided by the library.
- **Examples**: Includes demo screenshots or images showcasing the sorting algorithms in action.
- **License**: Mentions the license under which the project is distributed. In this case, it's the MIT License.
