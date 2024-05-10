#[macro_export]
macro_rules! multiply {
    // Edge case
    ( $last:expr ) => { $last };

    ( $head:expr, $($tail:expr), +) => {
        // Recursive call
        $head * multiply!($($tail),+)
    };
 }

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let val = multiply!(2, 4, 8);
        assert_eq!(2*4*8,val);
    }
}
