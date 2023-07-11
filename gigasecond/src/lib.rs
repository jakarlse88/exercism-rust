use time::{PrimitiveDateTime as DateTime, Duration};


// Returns a DateTime one billion seconds after start.
pub fn after( start : DateTime ) -> DateTime {
    const GIGASECOND : i64 = 1_000_000_000;
    start + Duration::seconds( GIGASECOND )
}
