pub fn position_before<T, P>(slice: &[T], end: usize, predicate: P) -> Option<usize>
where
    P: Fn(&T) -> bool,
{
    for i in (0..end).rev() {
        if predicate(&slice[i]) {
            return Some(i);
        }
    }
    return None;
}

pub fn position_after<T, P>(slice: &[T], start: usize, predicate: P) -> Option<usize>
where
    P: Fn(&T) -> bool,
{
    dbg!(&start);
    for i in (start + 1)..slice.len() {
        dbg!(&i);
        if dbg!(predicate(&slice[i])) {
            return Some(i);
        }
    }
    return None;
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
