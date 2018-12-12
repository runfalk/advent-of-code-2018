use std::iter::repeat;

fn extract_base10_digit(mut value: usize, index: u32) -> usize {
    value /= 10usize.pow(index);
    value % 10
}

fn grid_iterator((sx, sy): (usize, usize), (ex, ey): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    (sx..=ex).flat_map(move |x| repeat(x).zip(sy..=ey))
}

fn fuel_cell_value(serial: usize, x: usize, y: usize) -> isize {
    let rack_id = x + 10;
    extract_base10_digit((y * rack_id + serial) * rack_id, 2) as isize - 5
}

fn fuel_cell_group_sum(serial: usize, x: usize, y: usize) -> isize {
    grid_iterator((x, y), (x + 2, y +2))
        .map(|(x, y)| fuel_cell_value(serial, x,  y))
        .sum::<isize>()
}

fn part_a(serial: usize) -> (usize, usize) {
    // We end at x or y = 298 since that's the top left corner of the last fuel
    // cell group
    let (_, x, y) = grid_iterator((1, 1), (298, 298))
        .map(|(x, y)| (fuel_cell_group_sum(serial, x, y), x, y))
        .max().unwrap();
    (x, y)
}

fn main() {
    let (x, y) = part_a(5535);
    println!("Answer A: {},{}", x, y);
}

#[test]
fn test_extract_digit() {
    assert_eq!(extract_base10_digit(12, 2), 0);
    assert_eq!(extract_base10_digit(123, 2), 1);
    assert_eq!(extract_base10_digit(12345, 2), 3);
}

#[test]
fn test_fuel_cell_value() {
    assert_eq!(fuel_cell_value(8, 3, 5), 4);
    assert_eq!(fuel_cell_value(57, 122, 79), -5);
    assert_eq!(fuel_cell_value(39, 217, 196), 0);
    assert_eq!(fuel_cell_value(71, 101, 153), 4);
}

#[test]
fn test_fuel_cell_group_sum() {
    assert_eq!(fuel_cell_group_sum(18, 33, 45), 29);
    assert_eq!(fuel_cell_group_sum(42, 21, 61), 30);
}

#[test]
fn test_a() {
}
