/// A quick match between lexer's source characters and given expressions.
#[macro_export]
macro_rules! flash_match {
    {(
        $source:expr, $start:expr, $peek:ident) {
        $($head:tt => {
            $($tail:expr => $value:expr,)*
        })+
    }
    } => ({
        // Plus 1 to skip the peek character position.
        let pos = $start as usize + 1;
        match $peek {
            $($head => {
                match () {
                    $(
                        _ if &$source.as_str()[pos..pos + $tail.len()] == $tail => {
                            // Plus 1 to eat the peek character.
                            $source.advance_n($tail.len() as u8 + 1);
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
