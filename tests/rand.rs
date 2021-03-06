use squares_rnd::{KEY, Rand};

#[test]
fn should_work() {
    let rand = Rand::new(KEY);
    assert_eq!(rand.next_u32(), 920159078);
    assert_eq!(rand.next_u32(), 2487686880);

    assert_eq!(rand.set_counter(0), 2);
    let result64 = rand.next_u64();

    assert_eq!((result64 >> 32) as u32, 920159078);
    assert_eq!(result64 as u32, 2487686880);

    for _ in 0..50000 {
        assert!(rand.next_u32_up(500) < 500);
    }

    for _ in 0..50000 {
        assert!(rand.next_u64_up(500) < 500);
    }
}
