// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tiny_keccak::Keccak;
use std::hash::{BuildHasher, Hasher};
use tiny_keccak::Hasher as tiny_keccak_Hasher;
use std::marker::PhantomData;
use typenum::{Unsigned, U64};

pub(crate) fn is_sorted<A, I>(l: I) -> bool
where
    I: IntoIterator<Item = A>,
    A: Ord,
{
    let mut it = l.into_iter().peekable();
    loop {
        match (it.next(), it.peek()) {
            (_, None) => return true,
            (Some(ref a), Some(b)) if a > b => return false,
            _ => (),
        }
    }
}

pub(crate) struct LolHasher<N: Unsigned = U64> {
    state: u64,
    shift: usize,
    size: PhantomData<N>,
}

impl<N: Unsigned> LolHasher<N> {
    fn feed_me(&mut self, byte: u8) {
        self.state ^= u64::from(byte) << self.shift;
        self.shift += 8;
        if self.shift >= 64 {
            self.shift = 0;
        }
    }
}

impl<N: Unsigned> Hasher for LolHasher<N> {
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.feed_me(*byte)
        }
    }

    fn finish(&self) -> u64 {
        if N::USIZE == 64 {
            self.state
        } else {
            self.state & ((1 << N::USIZE) - 1)
        }
    }
}

impl<N: Unsigned> Default for LolHasher<N> {
    fn default() -> Self {
        LolHasher {
            state: 0,
            shift: 0,
            size: PhantomData,
        }
    }
}

pub(crate) struct KeccakHasher {
    k : Keccak,
}

impl KeccakHasher {
    fn with_seed(seed: u64) -> KeccakHasher {
        let mut k = Keccak::v256();
        let sb = seed.to_le_bytes();
        k.update(&sb);
        KeccakHasher { k }

    }
}

impl Hasher for KeccakHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.k.update(bytes);
    }

    fn finish(&self) -> u64 {
        let mut rb: [u8; 8] = [0;8];
        let k = self.k.clone();
        k.finalize(& mut rb);
        return u64::from_le_bytes(rb);
    }
}


pub(crate) struct MetroHashBuilder {
    seed: u64,
}

impl MetroHashBuilder {
    pub(crate) fn new(seed: u64) -> Self {
        MetroHashBuilder { seed }
    }

    pub(crate) fn seed(&self) -> u64 {
        self.seed
    }
}

impl BuildHasher for MetroHashBuilder {
    type Hasher = KeccakHasher;
    fn build_hasher(&self) -> Self::Hasher {
        KeccakHasher::with_seed(self.seed)
    }
}
