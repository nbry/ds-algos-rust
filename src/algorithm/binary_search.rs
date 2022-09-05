// Binary Search
//
// ## Worst case complexity
// Time: O(log n)
// Space: O(n)
//
// ## Additional Learning Topics
//
// "flooring" - getting the average of two usizes will automatically "floor"

/// Determine if a target value is within a list sorted in ascending order
pub fn binary_search<T: Ord>(sorted_list: &[T], target: T) -> bool {
    let mut l: usize = 0;
    let mut r: usize = sorted_list.len() - 1;

    while l <= r {
        let m = (l + r) / 2; // Conveniently, this will be floored

        if target < sorted_list[m] {
            r = m - 1;
        } else if target > sorted_list[m] {
            l = m + 1
        } else {
            return true;
        }
    }

    false
}

#[test]
fn target_is_in_array() {
    // Arrange
    let target = 22;
    let sorted_list = [1, 3, 7, 9, 22, 99];

    // Act
    let result = binary_search(&sorted_list, target);

    // Assert
    assert_eq!(result, true);
}

#[test]
fn target_is_in_array_with_one_item() {
    // Arrange
    let target = 22;
    let sorted_list = [22];

    // Act
    let result = binary_search(&sorted_list, target);

    // Assert
    assert_eq!(result, true);
}

#[test]
fn target_isnt_in_array_with_one_item() {
    // Arrange
    let target = 99;
    let sorted_list = [22];

    // Act
    let result = binary_search(&sorted_list, target);

    // Assert
    assert_eq!(result, false);
}

#[test]
fn target_is_in_array_on_the_left_edge() {
    // Arrange
    let target = 1;
    let sorted_list = [1, 3, 7, 9, 22, 99];

    // Act
    let result = binary_search(&sorted_list, target);

    // Assert
    assert_eq!(result, true);
}

#[test]
fn target_is_in_array_on_the_right_edge() {
    // Arrange
    let target = 99;
    let sorted_list = [1, 3, 7, 9, 22, 99];

    // Act
    let result = binary_search(&sorted_list, target);

    // Assert
    assert_eq!(result, true);
}

#[test]
fn target_is_not_in_array() {
    // Arrange
    let target = 300;
    let sorted_list = [1, 3, 7, 9, 22, 99];

    // Act
    let result = binary_search(&sorted_list, target);

    // Assert
    assert_eq!(result, false);
}
