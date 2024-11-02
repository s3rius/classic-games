/// Wrap something around min and max values.
///
/// If should_wrap is false, do nothing. Otherwise,
/// if it's greater than max, then the min value is returned,
/// if it's less than min, max is returned.
pub fn wrap_val<T: Ord>(should_wrap: bool, value: T, min: T, max: T) -> T {
    if !should_wrap {
        value
    } else if value < min {
        max
    } else if value > max {
        min
    } else {
        value
    }
}
