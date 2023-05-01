// Given two crystal balls that will brake if dropped from high enough
// distance, determine the exact spot in which it will break in the most
// optimized way.

pub fn two_crystal_balls(breaks: &[bool]) -> i32 {
    let jmp_amount = (breaks.len() as f64).sqrt().floor() as usize;

    let mut i = jmp_amount;

    while i < breaks.len() {
        if breaks[i] {
            break;
        }
        i += jmp_amount;
    }

    i -= jmp_amount;

    let mut j = 0;
    while j <= jmp_amount && i < breaks.len() {
        if breaks[i] {
            return i as i32;
        }
        j += 1;
        i += 1;
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::two_crystal_balls::two_crystal_balls;

    #[test]
    fn test_two_crystal_balls() {
        let mut rng = rand::prelude::thread_rng();
        let idx: usize = rng.gen_range(0..10000);
        let mut data = [false; 10000];

        for floor in data[idx..].iter_mut() {
            *floor = true;
        }

        assert_eq!(two_crystal_balls(&data), idx as i32);
        assert_eq!(two_crystal_balls(&[false;821]), -1);
    }
}
