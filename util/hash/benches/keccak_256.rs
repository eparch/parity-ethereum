// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

#![feature(test)]

extern crate test;
extern crate ethereum_types;
extern crate keccak_hash;

use keccak_hash::{keccak, write_keccak};
use test::Bencher;

#[bench]
fn bench_keccak_256_with_empty_input(b: &mut Bencher) {
    let empty = [0u8;0];
    b.bytes = empty.len() as u64;
    b.iter(|| {
        let _out = keccak(empty);
    })
}

#[bench]
fn bench_keccak_256_with_typical_input(b: &mut Bencher) {
    let data: Vec<u8> = From::from("some medum length string with important information");
    b.bytes = data.len() as u64;
    b.iter(|| {
        let _out = keccak(&data);
    })
}

#[bench]
fn bench_keccak_256_with_large_input(b: &mut Bencher) {
    // 4096 chars
    let data: Vec<u8> = From::from("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");
    b.bytes = data.len() as u64;
    b.iter(|| {
        let _out = keccak(&data);
    })
}
