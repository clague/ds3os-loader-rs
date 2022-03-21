use lazy_static::lazy_static;
use bytes::{Bytes, BytesMut, Buf, BufMut};
    use std::{num::Wrapping, ptr, ffi::c_void};

lazy_static! {
    pub static ref TEA_BLOCK_SIZE: usize = 8;
}

extern "C" {
    fn set_key(k: *const u32);
    fn encrypt(v: *mut u8);
}
// All this is super inefficient, should be rewritten when time allows.

pub fn tea32_encrypt(data: &BytesMut, key: &[u32;4]) -> Bytes {
    let length_rounded_to_block_size: usize = ((data.len() + (*TEA_BLOCK_SIZE - 1)) / *TEA_BLOCK_SIZE) * *TEA_BLOCK_SIZE;

    let mut output: BytesMut = BytesMut::with_capacity(length_rounded_to_block_size);
    output.extend_from_slice(&data[..]);

    unsafe { set_key(key.as_ptr()); }

    let mut block_offset: usize = 0;
    while block_offset < output.len() {
        tea32_encrypt_block(&mut output, block_offset);
        block_offset += *TEA_BLOCK_SIZE;
    }

    output.freeze()
}

pub fn tea32_encrypt_block(input: &mut BytesMut, input_offset: usize)
{
    unsafe {
        encrypt(input.as_mut_ptr().offset(input_offset as isize));
    }
}

pub fn test() {
    let mut output = BytesMut::from_iter(&[3u8, 2u8, 1u8, 0u8, 7u8, 6u8, 5u8, 4u8]);
    unsafe { set_key([0u32;4].as_ptr()); }
    tea32_encrypt_block(&mut output, 0);
    println!("{:#?}", output.freeze().to_vec());
    
    use tea_soft::block_cipher::generic_array::GenericArray;
    use tea_soft::block_cipher::{BlockCipher, NewBlockCipher};

    let key = GenericArray::from_slice(&[0u8;16]);
    let mut block = GenericArray::clone_from_slice(&[0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8]);
    // Initialize cipher
    let cipher = tea_soft::Tea32::new(&key);

    // Encrypt block in-place
    cipher.encrypt_block(&mut block);
    println!("{:#?}", block.to_vec());
}

// pub byte[] Tea32Decrypt(byte[] data, uint[] key)
// {
//     byte[] output = new byte[data.Length];

//     for (int block_offset = 0; block_offset < data.Length; block_offset += TEA_BLOCK_SIZE)
//     {
//         Tea32DecryptBlock(data, block_offset, output, block_offset, key);
//     }

//     return output;
// }



// pub void Tea32DecryptBlock(byte[] input, int input_offset, byte[] output, int output_offset, uint[] key)
// {
//     uint[] v = { 
//         BitConverter.ToUInt32(input, input_offset), 
//         BitConverter.ToUInt32(input, input_offset + 4) 
//     };

//     uint v0 = v[0], v1 = v[1], sum = 0xC6EF3720, i;
//     uint delta = 0x9E3779B9;
//     uint k0 = key[0], k1 = key[1], k2 = key[2], k3 = key[3];  
//     for (i = 0; i < 32; i++)
//     {
//         v1 -= ((v0 << 4) + k2) ^ (v0 + sum) ^ ((v0 >> 5) + k3);
//         v0 -= ((v1 << 4) + k0) ^ (v1 + sum) ^ ((v1 >> 5) + k1);
//         sum -= delta;
//     }

//     ToBytes(v0, output, output_offset);
//     ToBytes(v1, output, output_offset + 4);
// }

// // Faster inplace version of BitCoverter.ToBytes
// static unsafe void ToBytes(uint value, byte[] array, int offset)
// {
//     fixed (byte* ptr = &array[offset])
//         *(uint*)ptr = value;
// }