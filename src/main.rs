fn main() {
    println!("Hello, world!");

    let m = [3, 1, 2, 4];
    println!("{:?}", m);

    let water = calc_water(&m);
    println!("water is {}", water);
}

fn calc_water(m: &[i32]) -> i32 {
    let mut water = 0;
    for (i, x) in m.iter().enumerate() {
        if let Some(max_left) = m[0..i].iter().max() {
            if let Some(max_right) = m[i + 1..].iter().max() {
                let diff = max_left.min(max_right) - x;
                if diff > 0 {
                    water += diff;
                }
            }
        }
    }
    water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, calc_water(&[]));
        assert_eq!(0, calc_water(&[1]));
        assert_eq!(0, calc_water(&[1, 2]));
        assert_eq!(0, calc_water(&[1, 2, 3]));
        assert_eq!(0, calc_water(&[3, 2, 1]));
        assert_eq!(1, calc_water(&[2, 1, 3]));
        assert_eq!(1, calc_water(&[1, 0, 1]));
        assert_eq!(3, calc_water(&[3, 1, 2, 4]));

        assert_eq!(6, calc_water(&[3, 5, 2, 3, 4, 6, 3]));

        assert_eq!(8, calc_water(&[2, 5, 1, 2, 0, 3, 1, 3]));

        assert_eq!(4, calc_water(&[1, 6, 3, 3, 4, 1, 1, 2, 1]));
    }
}
