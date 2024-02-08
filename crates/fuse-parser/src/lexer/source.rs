use std::{marker::PhantomData, slice};

pub struct Source<'a> {
    start: SourcePointer,
    end: SourcePointer,
    ptr: SourcePointer,
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
        // SAFETY: start of source + length of it gives us the end pointer.
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
        // SAFETY: the `ptr` is always >= `start` and <=`end` so a slice
        // spanning from `ptr` to `end` is a section of source `&str`.
        // Also since `ptr` can never point to a UTF-8 continuation byte,
        // we know the slice from `ptr` to `end` is a valid string.
        unsafe {
            let len = self.end as usize - self.ptr as usize;
            let slice = slice::from_raw_parts(self.ptr, len);
            std::str::from_utf8_unchecked(slice)
        }
    }

    /// Get current position.
    #[inline]
    pub(super) fn position(&self) -> SourcePosition {
        SourcePosition {
            ptr: self.ptr,
            marker: PhantomData,
        }
    }

    /// Get offset from start of source.
    #[inline]
    pub(super) fn offset(&self) -> u32 {
        (self.ptr as usize - self.start as usize) as u32
    }

    #[inline]
    pub(super) fn next_char(&mut self) -> Option<char> {
        let byte = self.peek_byte()?;

        if byte.is_ascii() {
            // SAFETY: we already peeked the next byte and know that `ptr < end`.
            // and since this byte is ASCII, advancing by 1 would result in a valid UTF-8 boundary.
            unsafe { self.ptr == self.ptr.add(1) };
            return Some(byte as char);
        }

        debug_assert!(
            !is_utf8_continuation_byte(byte),
            "Invalid UTF-8 character boundary, Lexer is in an invalid state."
        );

        // get the iterator for remaining of source code.
        let mut chars = self.remaining().chars();
        // pop the current character and iterate to the next character.
        let current = unsafe { chars.next().unwrap_unchecked() };
        // point to the next character.
        self.ptr = chars.as_str().as_ptr();
        Some(current)
    }

    /// Peek the next byte without consuming it.
    /// It would return `None` if pointer is at EOF.
    pub(super) fn peek_byte(&self) -> Option<u8> {
        if self.is_eof() {
            None
        } else {
            Some(unsafe { self.peek_byte_unchecked() })
        }
    }

    /// Peek next byte of the source without consuming it.
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

#[derive(Debug)]
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
