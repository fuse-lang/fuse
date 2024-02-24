use std::{marker::PhantomData, slice};

macro_rules! debug_assert_char_byte {
    ($byte:expr) => {
        debug_assert_char_byte!($byte, true);
    };
    ($byte:expr, $extra_check:expr) => {
        debug_assert!(
            !is_utf8_continuation_byte($byte) || $extra_check,
            "Invalid UTF-8 character boundary, Lexer is in an invalid state."
        );
    };
}

pub struct Source<'a> {
    start: SourcePointer,
    end: SourcePointer,
    ptr: *const u8,
    marker: PhantomData<&'a str>,
}

impl<'a> Source<'a> {
    pub(super) fn new(src: &'a str) -> Self {
        assert!(
            src.len() <= crate::MAX_SOURCE_SIZE,
            "Input source code can not be larger than {} bytes.",
            src.len()
        );
        let start = src.as_ptr();
        // SAFETY: Start of source + length of it gives us the end pointer.
        let end = unsafe { start.add(src.len()) };
        Self {
            start,
            end,
            ptr: start,
            marker: PhantomData,
        }
    }

    pub(super) fn as_str(&self) -> &'a str {
        unsafe {
            let len = self.end as usize - self.start as usize;
            let slice = slice::from_raw_parts(self.start, len);
            std::str::from_utf8_unchecked(slice)
        }
    }

    /// Get the remaining source.
    pub(super) fn remaining(&self) -> &'a str {
        // SAFETY: The `ptr` is always >= `start` and <=`end` so a slice
        // spanning from `ptr` to `end` is a section of source `&str`.
        // Also since `ptr` can never point to a UTF-8 continuation byte,
        // We know the slice from `ptr` to `end` is a valid string.
        unsafe {
            let len = self.end as usize - self.ptr as usize;
            let slice = slice::from_raw_parts(self.ptr, len);
            std::str::from_utf8_unchecked(slice)
        }
    }

    /// Get current position.
    #[inline]
    pub(super) fn position(&self) -> SourcePosition<'a> {
        SourcePosition {
            ptr: self.ptr,
            marker: PhantomData,
        }
    }

    /// Set current position.
    ///  
    ///  # SAFETY
    ///  `pos` must be created from this `Source`, Since position is
    ///  is just a raw pointer we can only use a `SourcePosition` only with
    ///  its own `Source`.
    #[inline]
    pub(super) unsafe fn set_position(&mut self, pos: SourcePosition) {
        debug_assert_char_byte!(pos.ptr.read_u8(), pos.ptr == self.end);
        debug_assert!(
            pos.ptr >= self.start && pos.ptr <= self.end,
            "Position out of bound."
        );
        self.ptr = pos.ptr;
    }

    /// Get offset from start of source.
    #[inline]
    pub(super) fn offset(&self) -> u32 {
        (self.ptr as usize - self.start as usize) as u32
    }

    /// Advance to the next character.
    #[inline]
    pub(super) fn advance(&mut self) {
        self.next_char().unwrap();
    }

    /// Advance for the next n characters
    #[inline]
    pub(super) fn advance_n(&mut self, n: u8) {
        for _ in 0..n {
            self.advance();
        }
    }

    /// Advance if the next character is accepted by `predicate`.
    #[inline]
    pub(super) fn advance_if<F: FnMut(char) -> bool>(&mut self, mut predicate: F) -> bool {
        match self.peek_char() {
            Some(peek) if predicate(peek) => {
                self.advance();
                true
            }
            _ => false,
        }
    }

    #[inline]
    pub(super) fn next_char(&mut self) -> Option<char> {
        let byte = self.peek_byte()?;

        if byte.is_ascii() {
            // SAFETY: We already peeked the next byte and know that `ptr < end`.
            // and since this byte is ASCII, advancing by 1 would result in a valid UTF-8 boundary.
            unsafe { self.ptr = self.ptr.add(1) };
            return Some(byte as char);
        }

        debug_assert_char_byte!(byte);

        // Create an iterator for remaining of source code.
        let mut chars = self.remaining().chars();
        // Get the current character and iterate to the next character.
        let current = unsafe { chars.next().unwrap_unchecked() };
        // Point to the next character.
        self.ptr = chars.as_str().as_ptr();
        Some(current)
    }

    /// Peek the next char without consuming it.
    #[inline]
    pub(super) fn peek_char(&self) -> Option<char> {
        let byte = self.peek_byte()?;
        if byte.is_ascii() {
            return Some(byte as char);
        }

        debug_assert_char_byte!(byte);

        // Create an iterator for remaining of source code.
        let mut chars = self.remaining().chars();
        // Get the current character.
        let current = unsafe { chars.next().unwrap_unchecked() };
        Some(current)
    }

    /// Peek the second next char without consuming it.
    #[inline]
    pub(super) fn peek_char2(&self) -> Option<char> {
        if self.is_eof() {
            return None;
        }

        debug_assert_char_byte!(self.peek_byte().unwrap());

        // Create an iterator for remaining of source code.
        let mut chars = self.remaining().chars();
        // SAFETY: Since we are not at EOF there should be a next char.
        unsafe { chars.next().unwrap_unchecked() };
        chars.next()
    }

    /// Peek the next byte without consuming it.
    /// It would return `None` if pointer is at EOF.
    #[inline]
    pub(super) fn peek_byte(&self) -> Option<u8> {
        if self.is_eof() {
            None
        } else {
            Some(unsafe { self.peek_byte_unchecked() })
        }
    }

    /// Peek the next byte of the source without consuming it.
    /// caller should do the EOF bounds-check.
    #[inline]
    pub(super) unsafe fn peek_byte_unchecked(&self) -> u8 {
        debug_assert!(self.ptr >= self.start && self.ptr < self.end);
        self.ptr.read_u8()
    }

    #[inline]
    pub(super) fn is_eof(&self) -> bool {
        self.ptr == self.end
    }

    // pub(crate) fn consume_if(&mut self, character: char) -> bool {
    //     if self.current() == Some(character) {
    //         self.next();
    //         true
    //     } else {
    //         false
    //     }
    // }
}

pub type SourcePointer = *const u8;

trait SourceView {
    /// View pointer as `u8`
    unsafe fn read_u8(self) -> u8;
}

impl SourceView for SourcePointer {
    /// Read `u8` from `SourcePointer`.
    ///
    /// It is copied from `oxc` toolchain.
    /// This is about 7% faster than `*ptr` or `ptr.read()`, presumably because it tells the compiler
    /// it can rely on the memory being immutable, because if a `&mut` reference existed, that would
    /// violate Rust's aliasing rules.
    ///
    /// # SAFETY
    /// Caller must ensure pointer is non-null, and points to allocated, initialized memory.
    /// Pointer must point to within an object for which no `&mut` references are currently held.
    #[inline]
    unsafe fn read_u8(self) -> u8 {
        debug_assert!(!self.is_null());
        *self.as_ref().unwrap_unchecked()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SourcePosition<'a> {
    ptr: SourcePointer,
    marker: PhantomData<&'a u8>,
}

/// Reaturns true if byte have a valid boundary.
/// Would fail when we are pointing to a UTF-8 continuation byte.
#[inline]
const fn is_utf8_continuation_byte(byte: u8) -> bool {
    byte & 0xC0 == 0x80
}
