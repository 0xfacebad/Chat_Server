pub struct RcFour{
       plain_text:String,
}
impl RcFour{
       pub fn new(plain_text:String)->Self{
           RcFour{plain_text:plain_text,}
       }

       pub fn rc4_key_scheduling(&self)->[u8;256]{
              let key:&[u8] = b"YourSecretKey123456789012fdsfdfdf34"; // bad coding prcatice to hard code  values 
              let key_length = key.len();
              let mut s:[u8;256] =[0;256];
              for i in 0..s.len(){s[i] = i as u8;}
              let mut j:usize= 0;
              for i in 0..256{
                   j = (j+ s[i as usize] as usize + key[i%key_length] as usize)%256;
                   s.swap(i,j);
              }
              s

       }
       pub fn stream_gen(&self)->Vec<u8>{
           let s_array:[u8;256] = self.rc4_key_scheduling();
           let output_length = self.plain_text.len();
           let mut new_key:Vec<u8> = Vec::with_capacity(output_length);
           let mut i = 0;
           let mut j = 0;
           let mut s = s_array.clone();

            for _ in 0..output_length {
                i = (i + 1) % 256;
                j = (j + s[i as usize] as usize) % 256;
                s.swap(i, j);

                // Generate the keystream byte
                let k_index = (s[i as usize] as usize + s[j as usize] as usize) % 256;
                let k = s[k_index];
                new_key.push(k);
           }

    new_key 
       }

       pub fn encryption(&self)->Vec<u8>{
            let new_key:Vec<u8> = self.stream_gen();
            let plain_text = self.plain_text.len();
            if new_key.len() != plain_text {
            panic!("The new_key and plain_text must have the same length.");
    }

    let mut encrypted_bytes = Vec::with_capacity(plain_text);

    for i in 0..plain_text {
        // XOR the bytes
        let plain_text_bytes = self.plain_text.as_bytes();
        let encrypted_byte = new_key[i] ^ plain_text_bytes[i];
        encrypted_bytes.push(encrypted_byte);
    }
     encrypted_bytes
    
       }
     pub fn decrypt(&self)->Vec<u8>{
         let new_key = self.stream_gen();
         let cipher_text = self.encryption();
         if new_key.len() != cipher_text.len() {
        panic!("The new_key and cipher_text must have the same length.");
    }

    let mut decrypted_bytes = Vec::with_capacity(cipher_text.len());

    for (key_byte, cipher_byte) in new_key.iter().zip(cipher_text.iter()) {
        // XOR the bytes
        let decrypted_byte = key_byte ^ cipher_byte;
        decrypted_bytes.push(decrypted_byte);
    }

    decrypted_bytes
     }
}
