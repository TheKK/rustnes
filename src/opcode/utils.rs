#[macro_export]
macro_rules! assert_field_eq (
    ($left: expr, $right: expr, [$($field: ident), *]) => {
        $(
            assert_eq!($left.$field, $right.$field);
        )*
    }
);

#[inline]
pub fn compose_addr(addr_high: u8, addr_low: u8) -> u16 {
    ((addr_high as u16) << 8) + addr_low as u16
}

#[inline]
pub fn compose_indexed_addr(addr_high: u8, addr_low: u8, index: u8) -> (u16, bool) {
    let (addr, page_crossed) = match addr_low.overflowing_add(index) {
        (addr_low, true) => {
            let (addr_high, _overflowed) = addr_high.overflowing_add(1);

            (compose_addr(addr_high, addr_low), true)
        }
        (addr_low, false) => (compose_addr(addr_high, addr_low), false),
    };

    (addr, page_crossed)
}
