# piston-rust-selectionsort

Visualize a Selectionsort.

```rust
for array_index in 0..unsorted_array.len() {
  let current = unsorted_array[array_index];
  let mut min_indx = array_index;

  for x in array_index..unsorted_array.len() {
      if unsorted_array[x] < unsorted_array[min_indx] {
          min_indx = x
      }
  }

  unsorted_array[array_index] = unsorted_array[min_indx];
  unsorted_array[min_indx] = current;
}
```

## Selectionsort needs ~N^2/2 compare- and N exchangeoperations to sort an Array with length of N
