use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use rustc_serialize::hex::ToHex;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
//use openssl::rsa::{Rsa, Padding};
//use openssl::symm::Cipher;


pub fn hash(s:&str) -> String{
    let mut sh = Sha256::new();
    sh.input_str(s);
    sh.result_str().as_bytes().to_hex()

}

pub fn generateKeypair() -> (String, String){
    /*let pwd = "keyPair";

    let rsa = Rsa::generate(1024).unwrap();
    let priv_key = rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), pwd.as_bytes()).unwrap();
    let pub_key = rsa.public_key_to_pem().unwrap();
    (String::from_utf8(private_key).unwrap(), String::from_utf8(public_key).unwrap())*/

    
    let priv_key: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    let mut pub_key = priv_key.clone();
        pub_key.push('!');
    (priv_key, pub_key)

}
pub fn sign(priv_key: &str, msg: &str) -> String{
    /*let rsa = Rsa::private_key_from_pem_passphrase(priv_key.as_bytes(), "keyPair".as_bytes()).unwrap();
    let mut buf = vec![0; rsa.size() as usize];
    let  = rsa.private_encrypt(msg.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
    String::from_utf8(buf).unwrap()*/
    format!("{}{}", priv_key, msg)
}

pub fn verifySignature(pub_key: &str, msg: &str, sig: &str) -> bool{
    /*let rsa = Rsa::public_key_from_pem(pub_key.as_bytes()).unwrap();
    let mut buf = vec![0; rsa.size() as usize];
    let  = rsa.public_decrypt(msg.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
    String::from_utf8(buf).unwrap()*/
    let (priv_key, data) = sig.split_at(30);
    data == msg && priv_key == &pub_key[..pub_key.len() - 1]
}


pub fn calcAddress(key: &str) -> String{
    hash(&format!("addr{}", key))
}


pub fn addressMatchesKey(addr: &str, pub_key: &str) -> bool{
    addr == calcAddress(pub_key)
}

/*use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;
use rsa::{PublicKey, RsaPrivateKey, PaddingScheme};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

pub fn hash(s:&str) -> String{
    let mut hasher = Sha3::sha3_256();

    // write input message
    hasher.input_str(s);

    // read hash digest
    return hasher.result_str();

}

pub fn generateKeypair() -> (String, String){
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    (priv_key, pub_key)

}
// msg is a struct, not a string
pub fn sign(priv_key: RsaPrivateKey, msg: &str) -> String{
    let mut rng = OsRng;
    // convert msg to json
    pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15(), &msg[..]).expect("failed to encrypt")
}

// msg is a struct, not a string
pub fn verifySignature(pub_key: RsaPublicKey, msg: &str, sig: &str) -> bool{
    let mut rng = OsRng;
    // convert msg to json
    priv_key.decrypt(&mut rng, PaddingScheme::new_pkcs1v15(), &sig).expect("failed to decrypt") == msg
}


pub fn calcAddress(key: RsaPublicKey) -> String{
    let serialized = serde_json::to_string(&point).unwrap();
    hash(serialized)
}


pub fn addressMatchesKey(addr: &str, pub_key: RsaPublicKey) -> bool{
    addr == calcAddress(pub_key)
}
*/
