pub fn obfuscate(data: &[u8], key: &[u8]) -> Vec<u8> {
    if key.is_empty() {
        return data.to_vec();
    }

    let mut result = Vec::with_capacity(data.len());

    for (i, &byte) in data.iter().enumerate() {
        let xored = byte ^ key[i % key.len()];

        let rotated = (xored << 3) | (xored >> 5);

        result.push(rotated);
    }

    result
}

pub fn deobfuscate(data: &[u8], key: &[u8]) -> Vec<u8> {
    if key.is_empty() {
        return data.to_vec();
    }

    let mut result = Vec::with_capacity(data.len());

    for (i, &byte) in data.iter().enumerate() {
        let rotated = (byte >> 3) | (byte << 5);

        let xored = rotated ^ key[i % key.len()];

        result.push(xored);
    }

    result
}
