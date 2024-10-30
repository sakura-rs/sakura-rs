#![allow(unused)]

use super::magic::{
    LOOKUP_G11, LOOKUP_G13, LOOKUP_G14, LOOKUP_G2, LOOKUP_G3, LOOKUP_G9, LOOKUP_RCON, LOOKUP_SBOX,
    LOOKUP_SBOX_INV, SHIFT_ROWS_TABLE, SHIFT_ROWS_TABLE_INV,
};

fn xorr(a: &mut [u8], b: &[u8], n: usize) {
    (0..n).for_each(|i| a[i] ^= b[i]);
}

fn xor_round_key(state: &mut [u8], keys: &[u8], round: usize) {
    xorr(state, &keys[round * 16..], 16);
}

fn sub_bytes(a: &mut [u8], n: usize) {
    (0..n).for_each(|i| a[i] = LOOKUP_SBOX[a[i] as usize]);
}

fn sub_bytes_inv(a: &mut [u8], n: usize) {
    (0..n).for_each(|i| a[i] = LOOKUP_SBOX_INV[a[i] as usize]);
}

fn key_schedule_core(a: &mut [u8], i: usize) {
    let temp = a[0];
    a[0] = a[1];
    a[1] = a[2];
    a[2] = a[3];
    a[3] = temp;
    sub_bytes(a, 4);
    a[0] ^= LOOKUP_RCON[i];
}

fn oqs_aes128_load_schedule_c(key: &[u8]) -> [u8; 176] {
    let mut schedule = [0u8; 176];

    let mut bytes = 16;
    let mut i = 1;
    let mut t = [0u8; 4];

    schedule[0..16].copy_from_slice(key);

    while bytes < 176 {
        t.copy_from_slice(&schedule[bytes - 4..]);
        key_schedule_core(&mut t, i);
        i += 1;
        xorr(&mut schedule[bytes..], &t, 4);
        schedule[bytes..].copy_from_slice(&t);
        bytes += 4;

        for _ in 0..3 {
            t.copy_from_slice(&schedule[bytes - 4..]);
            xorr(&mut t, &schedule[bytes - 16..], 4);
            schedule[bytes..].copy_from_slice(&t);
            bytes += 4;
        }
    }

    schedule
}

fn shift_rows(state: &mut [u8]) {
    let temp = state.to_vec();
    (0..16).for_each(|i| state[i] = temp[SHIFT_ROWS_TABLE[i] as usize]);
}

fn shift_rows_inv(state: &mut [u8]) {
    let temp = state.to_vec();
    (0..16).for_each(|i| state[i] = temp[SHIFT_ROWS_TABLE_INV[i] as usize]);
}

fn mix_col(state: &mut [u8]) {
    let (a0, a1, a2, a3) = (state[0], state[1], state[2], state[3]);

    state[0] = LOOKUP_G2[a0 as usize] ^ LOOKUP_G3[a1 as usize] ^ a2 ^ a3;
    state[1] = LOOKUP_G2[a1 as usize] ^ LOOKUP_G3[a2 as usize] ^ a3 ^ a0;
    state[2] = LOOKUP_G2[a2 as usize] ^ LOOKUP_G3[a3 as usize] ^ a0 ^ a1;
    state[3] = LOOKUP_G2[a3 as usize] ^ LOOKUP_G3[a0 as usize] ^ a1 ^ a2;
}

fn mix_cols(state: &mut [u8]) {
    mix_col(&mut state[0..4]);
    mix_col(&mut state[4..8]);
    mix_col(&mut state[8..12]);
    mix_col(&mut state[12..16]);
}

fn mix_col_inv(state: &mut [u8]) {
    let (a0, a1, a2, a3) = (state[0], state[1], state[2], state[3]);

    state[0] = LOOKUP_G14[a0 as usize]
        ^ LOOKUP_G9[a3 as usize]
        ^ LOOKUP_G13[a2 as usize]
        ^ LOOKUP_G11[a1 as usize];
    state[1] = LOOKUP_G14[a1 as usize]
        ^ LOOKUP_G9[a0 as usize]
        ^ LOOKUP_G13[a3 as usize]
        ^ LOOKUP_G11[a2 as usize];
    state[2] = LOOKUP_G14[a2 as usize]
        ^ LOOKUP_G9[a1 as usize]
        ^ LOOKUP_G13[a0 as usize]
        ^ LOOKUP_G11[a3 as usize];
    state[3] = LOOKUP_G14[a3 as usize]
        ^ LOOKUP_G9[a2 as usize]
        ^ LOOKUP_G13[a1 as usize]
        ^ LOOKUP_G11[a0 as usize];
}

fn mix_cols_inv(state: &mut [u8]) {
    mix_col_inv(&mut state[0..4]);
    mix_col_inv(&mut state[4..8]);
    mix_col_inv(&mut state[8..12]);
    mix_col_inv(&mut state[12..16]);
}

fn oqs_aes128_enc_c(plaintext: &[u8], schedule: &[u8], ciphertext: &mut [u8]) {
    ciphertext.copy_from_slice(&plaintext[..16]);
    xor_round_key(ciphertext, schedule, 0);

    for i in 1..10 {
        sub_bytes(ciphertext, 16);
        shift_rows(ciphertext);
        mix_cols(ciphertext);
        xor_round_key(ciphertext, schedule, i);
    }

    sub_bytes(ciphertext, 16);
    shift_rows(ciphertext);
    xor_round_key(ciphertext, schedule, 10);
}

pub fn oqs_mhy128_enc_c(plaintext: &[u8], schedule: &[u8], ciphertext: &mut [u8]) {
    ciphertext.copy_from_slice(&plaintext[..16]);
    xor_round_key(ciphertext, schedule, 0);

    for i in 1..10 {
        sub_bytes_inv(ciphertext, 16);
        shift_rows_inv(ciphertext);
        mix_cols_inv(ciphertext);
        xor_round_key(ciphertext, schedule, i);
    }

    sub_bytes_inv(ciphertext, 16);
    shift_rows_inv(ciphertext);
    xor_round_key(ciphertext, schedule, 10);
}

fn oqs_aes128_dec_c(ciphertext: &[u8], schedule: &[u8], plaintext: &mut [u8]) {
    plaintext.copy_from_slice(&ciphertext[..16]);
    xor_round_key(plaintext, schedule, 10);
    shift_rows_inv(plaintext);
    sub_bytes_inv(plaintext, 16);

    for i in 0..9 {
        xor_round_key(plaintext, schedule, 9 - i);
        mix_cols_inv(plaintext);
        shift_rows_inv(plaintext);
        sub_bytes_inv(plaintext, 16);
    }

    xor_round_key(plaintext, schedule, 0);
}

fn oqs_mhy128_dec_c(ciphertext: &[u8], schedule: &[u8], plaintext: &mut [u8]) {
    plaintext.copy_from_slice(&ciphertext[..16]);
    xor_round_key(plaintext, schedule, 10);
    shift_rows(plaintext);
    sub_bytes(plaintext, 16);

    for i in 0..9 {
        xor_round_key(plaintext, schedule, 9 - i);
        mix_cols(plaintext);
        shift_rows(plaintext);
        sub_bytes(plaintext, 16);
    }

    xor_round_key(plaintext, schedule, 0);
}
