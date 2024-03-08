use magic_crypt::MagicCryptTrait;
use magic_crypt::new_magic_crypt;


pub fn encrypt_string(data: &str, key: &str) -> String {
  let mcrypt = new_magic_crypt!(key, 256); //Creates an instance of the magic crypt library/crate.
  return mcrypt.encrypt_str_to_base64(data); //Encrypts the string and saves it to the 'encrypted_string' variable.
}

pub fn decrypt_string(data: &str, key: &str) -> String {
  let mcrypt = new_magic_crypt!(key, 256); //Creates an instance of the magic crypt library/crate.
  return mcrypt.decrypt_base64_to_string(data).unwrap(); //Decrypts the string so we can read it.
}