// use base64::{Engine as _, engine::general_purpose};
// use rand::{Rng, rng};
// use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Default)]
pub struct TidalConfig {}

impl TidalConfig {
    pub fn new() -> TidalConfig {
        Self::default()
        // let mut rng = rand::rng();
        //
        // // Reconstructed initial token
        // let mut token = format!("{}.{}", std::env::consts::OS, std::any::type_name::<Self>());
        //
        // token = token[..8].to_owned() + &token[16..];
        //
        // let api_token = general_purpose::STANDARD
        //     .decode("d3RjaThkamFfbHlhQnBKaWQuMkMwb3puT2ZtaXhnMA==")
        //     .unwrap();
        // let tok: String = token[token.len() - 6..]
        //     .chars()
        //     .map(|x| ((x as u8) - 2) as char)
        //     .collect();
        // let mut token2 = token.clone();
        // token = token[..9].to_owned() + &tok;
        //
        // let tok2: String = token[..token.len() - 7]
        //     .chars()
        //     .map(|x| ((x as u8) - 2) as char)
        //     .collect();
        // token = token[8..].to_string();
        // token = tok2 + &token;
        //
        // let decoded = general_purpose::STANDARD
        //     .decode("enJVZzRiWF9IalZfVm5rZ2MuMkF0bURsUGRvZzRldA==")
        //     .unwrap();
        // let mut api_token_vec: Vec<char> = decoded.iter().map(|&b| b as char).collect();
        // for c in token.chars() {
        //     if let Some(pos) = api_token_vec.iter().position(|&x| x == c) {
        //         api_token_vec.remove(pos);
        //     }
        // }
        // let api_token_str: String = api_token_vec.iter().collect();
        //
        // // Handle encoding
        // let mut string = String::new();
        // let mut save = false;
        // if !token2.is_ascii() {
        //     save = true;
        //     string = "".to_string();
        //     token2 = token2.encode_utf16().map(|x| x as u8 as char).collect();
        // }
        //
        // let tok: String = token2[..token2.len() - 7]
        //     .chars()
        //     .map(|x| ((x as u8) + 24) as char)
        //     .collect();
        // token2 = token2[8..].to_string();
        // token2 = tok + &token2;
        //
        // let tok2: String = token2[token2.len() - 6..]
        //     .chars()
        //     .map(|x| ((x as u8) + 23) as char)
        //     .collect();
        // token2 = token2[..9].to_string() + &tok2;
        //
        // let mut client_id_vec: Vec<char> = general_purpose::STANDARD
        //     .decode(
        //         "VoxKgUt8aHlEhEZ5cYhKgVAucVp2hnOFUH1WgE5+QlY2dWtYVEptd2x2YnR0UDd3bE1scmM3MnNlND0=",
        //     )
        //     .unwrap()
        //     .iter()
        //     .map(|&b| b as char)
        //     .collect();
        //
        // if save {
        //     // simulate decode/encode chain
        //     client_id_vec = client_id_vec
        //         .iter()
        //         .map(|&x| {
        //             let code = x as u32;
        //             // clamp to u8
        //             let byte = (code & 0xFF) as u8;
        //             byte as char
        //         })
        //         .collect();
        // }
        //
        // for c in token2.chars() {
        //     if let Some(pos) = client_id_vec.iter().position(|&x| x == c) {
        //         client_id_vec.remove(pos);
        //     }
        // }
        // let client_id_str: String = client_id_vec.iter().collect();
        //
        // let client_unique_key = format!("{:02x}", rng.random::<u64>());
        // let code_verifier =
        //     base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(rng.random::<[u8; 32]>());
        // let code_challenge = base64::engine::general_purpose::URL_SAFE_NO_PAD
        //     .encode(Sha256::digest(code_verifier.as_bytes()));
        //
        // let client_id_pkce = general_purpose::STANDARD
        //     .decode(
        //         general_purpose::STANDARD
        //             .decode("TmtKRVUxSmtjRXM=")
        //             .unwrap()
        //             .iter()
        //             .chain(
        //                 general_purpose::STANDARD
        //                     .decode("NWFIRkZRbFJuVlE9PQ==")
        //                     .unwrap()
        //                     .iter(),
        //             )
        //             .cloned()
        //             .collect::<Vec<u8>>(),
        //     )
        //     .unwrap();
        // let client_secret_pkce = general_purpose::STANDARD
        //     .decode(
        //         general_purpose::STANDARD
        //             .decode("ZUdWMVVHMVpOMjVpY0ZvNVNVbGlURUZqVVQ=")
        //             .unwrap()
        //             .iter()
        //             .chain(
        //                 general_purpose::STANDARD
        //                     .decode("a3pjMmhyWVRGV1RtaGxWVUZ4VGpaSlkzTjZhbFJIT0QwPQ==")
        //                     .unwrap()
        //                     .iter(),
        //             )
        //             .cloned()
        //             .collect::<Vec<u8>>(),
        //     )
        //     .unwrap();
        //
        // Self {
        //     api_token: api_token_str.clone(),
        //     client_id: api_token_str.clone(),
        //     client_secret: client_id_str,
        //     client_unique_key,
        //     code_verifier,
        //     code_challenge,
        //     client_id_pkce: String::from_utf8(client_id_pkce).unwrap(),
        //     client_secret_pkce: String::from_utf8(client_secret_pkce).unwrap(),
        // }
    }
}
