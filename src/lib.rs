use core::ops::Range;
use oem_cp::{
    code_table::DECODING_TABLE_CP437,
    decode_string_complete_table
};
use rand::Rng;

type Error = Box<dyn std::error::Error>;

pub enum PasswordType {
    Ascii,
    ExtendedAscii,
}

pub fn generate(_length: usize, _passwordtype: PasswordType) -> Result<String, Error> { 
    let rng = &mut rand::thread_rng();

    let range: Range<u8> = match _passwordtype {
        PasswordType::ExtendedAscii => {
            33..255
        }
        PasswordType::Ascii => {
            33..127
        }
    };

    let random_bytes: Vec<u8> = (0.._length).map(|_| rng.gen_range(range.clone())).collect();
    let finished_string: String = decode_string_complete_table(random_bytes, &DECODING_TABLE_CP437);
    Ok(finished_string)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Error>{
        let ascii_string = generate(10, PasswordType::Ascii)?;
        println!("Ascii string: {}", ascii_string);
        Ok(())
    }
}
