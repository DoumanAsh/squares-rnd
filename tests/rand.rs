use squares_rnd::{KEY, Rand};

#[test]
fn should_work() {
    let rand = Rand::new(KEY);
    assert_eq!(rand.next_u32(), 1393168711);
    assert_eq!(rand.next_u32(), 2635734307);

    let rand = rand.set_counter(0);
    let result64 = rand.next_u64();

    assert_eq!((result64 >> 32) as u32, 1393168711);
    assert_eq!(result64 as u32, 2635734307);
}
