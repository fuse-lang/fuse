/// A quick match between lexer's source characters and given expressions.
#[macro_export]
macro_rules! flash_match {
    {(
        $source:expr, $start:expr, $first:ident) {
        $($head:tt => {
            $($tail:expr => $value:expr,)*
        })+
    }
    } => ({
        // Plus 1 to skip the first character position.
        let pos = $start as usize + 1;
        match $first {
            $($head => {
                match () {
                    $(
                        _ if &$source.as_str()[pos..pos + $tail.len()] == $tail => {
                            $source.advance_n($tail.len() as u8);
                            Some($value)
                        },
                    )+
                        _ => None,
                }}
                ,)+
                _ => None,
        }
    });
}
