use frunk::Semigroup;
use frunk::semigroup::All;

pub fn example() {
    assert_eq!(Some(1).combine(&Some(2)), Some(3));

    assert_eq!(All(3).combine(&All(5)), All(1)); // bit-wise &&
    assert_eq!(All(true).combine(&All(false)), All(false));
}