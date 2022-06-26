/// # Bubble Sort
///
/// Worst case complexity
/// Time: O(n^2)
/// Space: O(n)
///
/// ## Additional Learning Topics
///
/// * The Ord Trait - Trait for types that form a total order
///     i.e. allows for comparisons like "greater than", "equal to" etc.

pub fn bubble_sort<T: Ord>(arr: &mut [T], ascending: bool) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if is_out_of_order(&arr[j], &arr[j + 1], ascending) {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn is_out_of_order<T: Ord>(arr_item: &T, next_arr_item: &T, ascending: bool) -> bool {
    if ascending {
        return arr_item > next_arr_item;
    } else {
        return arr_item < next_arr_item;
    }
}

#[test]
fn descending_bubble_sort() {
    // Arrange
    let mut test_subject = [1, 3, 9, 22, 99, 7];

    // Act
    bubble_sort(&mut test_subject, false);

    // Assert
    let expecting = [99, 22, 9, 7, 3, 1];
    assert_eq!(test_subject, expecting)
}

#[test]
fn ascending_bubble_sort() {
    // Arrange
    let mut test_subject = [1, 3, 9, 22, 99, 7];

    // Act
    bubble_sort(&mut test_subject, true);

    // Assert
    let expecting = [1, 3, 7, 9, 22, 99];
    assert_eq!(test_subject, expecting)
}
