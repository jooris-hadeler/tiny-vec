use crate::TinyVec;

#[test]
fn test_push_and_pop() {
    let mut tv = TinyVec::<_, 4>::new();

    tv.push(5);
    assert_eq!(tv.pop(), Some(5));
    assert_eq!(tv.pop(), None);

    tv.push(12);
    tv.push(-33);
    tv.push(1346);
    tv.push(-9994);
    assert!(!tv.has_spilled());

    tv.push(99);
    assert!(tv.has_spilled());

    assert_eq!(tv.pop(), Some(99));
    assert_eq!(tv.pop(), Some(-9994));
    assert!(tv.has_spilled());

    assert_eq!(tv.pop(), Some(1346));
    assert_eq!(tv.pop(), Some(-33));
    assert_eq!(tv.pop(), Some(12));
    assert_eq!(tv.pop(), None);
}

#[test]
fn test_iter() {
    let mut tv = TinyVec::<_, 8>::new();

    for i in 0..6 {
        tv.push(i);
    }

    for (idx, elm) in tv.iter().enumerate() {
        assert_eq!(idx, *elm);
    }

    tv.push(6);
    tv.push(7);
    tv.push(8);
    tv.push(9);

    for (idx, elm) in tv.iter().enumerate() {
        assert_eq!(idx, *elm);
    }
}

#[test]
fn test_partial_eq() {
    let mut ta = TinyVec::<_, 4>::new();
    let mut tb = TinyVec::<_, 12>::new();

    for i in 0..8 {
        ta.push(i);
        tb.push(i);
    }

    assert!(ta.has_spilled());
    assert!(!tb.has_spilled());

    assert_eq!(ta, tb);
}

#[test]
fn test_get() {
    let mut tv = TinyVec::<_, 4>::new();

    tv.push(12);
    assert_eq!(tv.get(0), Some(&12));

    *(tv.get_mut(0).unwrap()) = 55;
    assert_eq!(tv.get(0), Some(&55));
}

#[test]
fn test_extend() {
    let mut tv = TinyVec::<_, 4>::new();
    tv.extend(0..12);

    for (idx, elm) in tv.iter().enumerate() {
        assert_eq!(idx, *elm);
    }
}

#[test]
fn test_from() {
    let tv = TinyVec::<_, 4>::from(0..8);

    assert_eq!(tv.len(), 8);

    for (idx, elm) in tv.iter().enumerate() {
        assert_eq!(idx, *elm);
    }
}

#[test]
fn test_into_iter() {
    let tv = TinyVec::<_, 16>::from(0..12);

    for (idx, elm) in tv.into_iter().enumerate() {
        assert_eq!(idx, elm);
    }
    
    let tv = TinyVec::<_, 8>::from(0..12);

    for (idx, elm) in tv.into_iter().enumerate() {
        assert_eq!(idx, elm);
    }
}
