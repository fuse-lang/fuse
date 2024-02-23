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
        let pos = $start as usize + 1;
        match $peek {
            $($head => {
                // Eat the peek character.
                $source.advance();
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
