use std::convert::TryFrom;
use std::convert::TryInto;

pub fn main() {
    assert_eq!(EventNumber::try_from(8), Ok(EventNumber(8)));
    assert_eq!(EventNumber::try_from(5), Err(()));

    let result:Result<EventNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EventNumber(8)));
    let result:Result<EventNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

#[derive(Debug, PartialEq)]
struct EventNumber(i32);

impl TryFrom<i32> for EventNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value %2 == 0 {
            return Ok(EventNumber(value));
        } else {
            return Err(());
        }
    }
}