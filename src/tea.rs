use std::num::Wrapping;

pub type Block = [u32; 2];
pub type Key = [u32; 4];

pub fn encrypt(block: Block, key: Key) -> Block {
    // Open up the input block
    let mut v0 = Wrapping(block[0]);
    let mut v1 = Wrapping(block[1]);

    // Open up the key
    let k0 = Wrapping(key[0]);
    let k1 = Wrapping(key[1]);
    let k2 = Wrapping(key[2]);
    let k3 = Wrapping(key[3]);

    // Mash things together
    let mut sum = Wrapping(0);
    let delta = Wrapping(0x9E3779B9);
    for _ in 0..32 {
        sum += delta;
        v0 += ((v1 << 4) + k0) ^ (v1 + sum) ^ ((v1 >> 5) + k1);
        v1 += ((v0 << 4) + k2) ^ (v0 + sum) ^ ((v0 >> 5) + k3);
    }

    [v0.0, v1.0]
}

pub fn decrypt(block: Block, key: Key) -> Block {
    // Open up the input block
    let mut v0 = Wrapping(block[0]);
    let mut v1 = Wrapping(block[1]);

    // Open up the key
    let k0 = Wrapping(key[0]);
    let k1 = Wrapping(key[1]);
    let k2 = Wrapping(key[2]);
    let k3 = Wrapping(key[3]);

    // Mash things together (sum init'd to delta << 5)
    let mut sum = Wrapping(0xC6EF3720);
    let delta = Wrapping(0x9E3779B9);

    for _ in 0..32 {
        v1 -= ((v0 << 4) + k2) ^ (v0 + sum) ^ ((v0 >> 5) + k3);
        v0 -= ((v1 << 4) + k0) ^ (v1 + sum) ^ ((v1 >> 5) + k1);
        sum -= delta;
    }

    [v0.0, v1.0]
}
