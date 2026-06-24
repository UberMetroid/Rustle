// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Rustle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Rustle.  If not, see <https://www.gnu.org/licenses/>.

use base64::{engine::general_purpose::STANDARD, Engine as _};
use blowfish::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use blowfish::Blowfish;

pub fn encrypt(data: &str) -> Result<String, String> {
    let key_bytes = crate::constants::config::BLOWFISH_KEY.as_bytes();
    let bf =
        Blowfish::<byteorder::BigEndian>::new_from_slice(key_bytes).map_err(|e| e.to_string())?;

    let mut bytes = Vec::with_capacity(data.len() + 8);
    bytes.extend_from_slice(data.as_bytes());
    let pad_len = (8 - (bytes.len() % 8)) % 8;
    bytes.resize(bytes.len() + pad_len, 0);

    for chunk in bytes.chunks_mut(8) {
        let block = blowfish::cipher::generic_array::GenericArray::from_mut_slice(chunk);
        bf.encrypt_block(block);
    }

    Ok(STANDARD.encode(&bytes))
}

pub fn decrypt(encoded: &str) -> Option<String> {
    let key_bytes = crate::constants::config::BLOWFISH_KEY.as_bytes();
    let bf = Blowfish::<byteorder::BigEndian>::new_from_slice(key_bytes).ok()?;

    let mut bytes = STANDARD.decode(encoded).ok()?;
    if bytes.len() % 8 != 0 {
        return None;
    }

    for chunk in bytes.chunks_mut(8) {
        let block = blowfish::cipher::generic_array::GenericArray::from_mut_slice(chunk);
        bf.decrypt_block(block);
    }

    while let Some(&0) = bytes.last() {
        bytes.pop();
    }

    String::from_utf8(bytes).ok()
}
