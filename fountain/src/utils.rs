pub fn position_before<T, P>(slice: &[T], end: usize, predicate: P) -> Option<usize>
where
    P: Fn(&T) -> bool,
{
    (0..end).rev().find(|&i| predicate(&slice[i]))
}

pub fn position_after<T, P>(slice: &[T], start: usize, predicate: P) -> Option<usize>
where
    P: Fn(&T) -> bool,
{
    for (i, item) in slice.iter().enumerate().skip(start + 1) {
        if predicate(item) {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_before() {
        let v: Vec<_> = (0..10).collect();
        assert_eq!(Some(3), position_before(&v, 5, |x| x % 2 == 1));
    }

    #[test]
    fn test_position_after() {
        let v: Vec<_> = (0..10).collect();
        assert_eq!(Some(8), position_after(&v, 6, |x| x % 2 == 0));
    }
}
