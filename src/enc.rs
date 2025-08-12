use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};

pub fn calculate_block_i(is_up_link: bool, dev_addr: [u8; 4], f_count: u16, i: u8) -> [u8; 16] {
    let result: [u8; 16] = [
        0x01,
        0x00,
        0x00,
        0x00,
        0x00,
        if is_up_link { 0x00 } else { 0x01 },
        dev_addr[0],
        dev_addr[1],
        dev_addr[2],
        dev_addr[3],
        f_count as u8,
        (f_count >> 8) as u8,
        0x00,
        0x00,
        0x00,
        i,
    ];

    result
}

pub fn calculate_encrypted_block_i(
    is_up_link: bool,
    dev_addr: [u8; 4],
    f_count: u16,
    i: u8,
    key: [u8; 16],
) -> [u8; 16] {
    let a_i = calculate_block_i(is_up_link, dev_addr, f_count, i);

    let cipher = Aes128::new(&GenericArray::from(key));

    let mut block = GenericArray::from(a_i);

    cipher.encrypt_block(&mut block);

    return block.into();
}

pub fn encrypt_payload(
    is_up_link: bool,
    dev_addr: [u8; 4],
    f_count: u16,
    payload: &[u8],
    key: [u8; 16],
) -> Vec<u8> {
    let k = payload.len() as f32 / 16 as f32;
    let k = k.ceil() as usize;

    let mut result = Vec::new();

    for i in 0..k {
        let s_i = calculate_encrypted_block_i(is_up_link, dev_addr, f_count, i as u8 + 1, key);

        // Copy the payload data into the block by right padding with zeros
        let mut block = [0u8; 16];
        let start = i * 16;
        let end = start + 16;
        let payload_slice = &payload[start..end.min(payload.len())];
        block[..payload_slice.len()].copy_from_slice(payload_slice);

        // XOR the block with the encrypted block
        for j in 0..16 {
            block[j] ^= s_i[j];
        }

        // Append the block to the result
        result.extend_from_slice(&block);
    }

    // Truncate the result to the original payload length
    if result.len() > payload.len() {
        result.truncate(payload.len());
    }

    return result;
}
