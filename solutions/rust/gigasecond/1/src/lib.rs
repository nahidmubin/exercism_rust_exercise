use std::ops::Add;

use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let giga_second = time::Duration::seconds(1_000_000_000);
    let finish = start.add(giga_second);
    finish
    // todo!("What time is a gigasecond later than {start}");
}
