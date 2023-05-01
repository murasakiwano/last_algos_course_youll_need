pub fn binary_search(haystack: &mut [i32], needle: i32) -> bool {
    let mut lo = 0;
    let mut hi = haystack.len();

    while lo < hi {
        let mid_point = lo + (hi - lo) / 2;
        let value = haystack[mid_point];
        if value == needle {
            return true;
        } else if value > needle {
            hi = mid_point;
        } else if value < needle {
            lo = mid_point + 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::binary_search::binary_search;

    #[test]
    fn test_binary_search() {
        let mut foo: [i32; 11] = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];

        assert_eq!(binary_search(&mut foo, 69), true);
        assert_eq!(binary_search(&mut foo, 1336), false);
        assert_eq!(binary_search(&mut foo, 69420), true);
        assert_eq!(binary_search(&mut foo, 69421), false);
        assert_eq!(binary_search(&mut foo, 1), true);
        assert_eq!(binary_search(&mut foo, 0), false);
    }
}
