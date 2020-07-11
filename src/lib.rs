// Copyright 2020-Present (c) Raja Lehtihet & Wael El Oraiby
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors
// may be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(improper_ctypes)]

pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;

#[cfg(any(target_pointer_width = "32", windows))]
pub type c_long = i32;
#[cfg(any(target_pointer_width = "32", windows))]
pub type c_ulong = u32;

#[cfg(all(target_pointer_width = "64", not(windows)))]
pub type c_long = i64;
#[cfg(all(target_pointer_width = "64", not(windows)))]
pub type c_ulong = u64;

pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_float = f32;
pub type c_double = f64;

pub use core::ffi::c_void;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_sizes() {
        assert_eq!(::core::mem::size_of::<c_char>(), 1);
        assert_eq!(::core::mem::size_of::<c_schar>(), 1);
        assert_eq!(::core::mem::size_of::<c_uchar>(), 1);
        assert_eq!(::core::mem::size_of::<c_short>(), 2);
        assert_eq!(::core::mem::size_of::<c_ushort>(), 2);
        assert_eq!(::core::mem::size_of::<c_int>(), 4);
        assert_eq!(::core::mem::size_of::<c_uint>(), 4);
        assert_eq!(::core::mem::size_of::<c_longlong>(), 8);
        assert_eq!(::core::mem::size_of::<c_ulonglong>(), 8);
    }

    #[test]
    #[cfg(windows)]
    fn check_long_size() {
        assert_eq!(::core::mem::size_of::<c_long>(), 4);
        assert_eq!(::core::mem::size_of::<c_ulong>(), 4);
    }

    #[test]
    #[cfg(not(windows))]
    fn check_long_size() {
        assert_eq!(::core::mem::size_of::<c_long>(), 8);
        assert_eq!(::core::mem::size_of::<c_ulong>(), 8);
    }

}
