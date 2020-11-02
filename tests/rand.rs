use squares_rnd::{KEY, Rand};

#[test]
fn should_work() {
    let rand = Rand::new(KEY);
    assert_eq!(rand.next_u32(), 1393168711);
    assert_eq!(rand.next_u32(), 2635734307);
}
