const LEFT_ROTOR: (i32, i32, i32, i32) = (0, -1, 1, 0);
const RIGHT_ROTOR: (i32, i32, i32, i32) = (0, 1, -1, 0);

pub const fn rotate_left(x: i32, y: i32) -> (i32, i32) {
    let (a, b, c, d) = LEFT_ROTOR;
    (a * x + b * y, c * x + d * y)
}

pub const fn rotate_right(x: i32, y: i32) -> (i32, i32) {
    let (a, b, c, d) = RIGHT_ROTOR;
    (a * x + b * y, c * x + d * y)
}
