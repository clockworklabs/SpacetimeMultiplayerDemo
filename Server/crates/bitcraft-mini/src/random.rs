use getrandom::register_custom_getrandom;
use getrandom::Error;

// Some application-specific error code
pub fn fake_random(buf: &mut [u8]) -> Result<(), Error> {
    for i in 0..buf.len() {
        let start = match i % 4 {
            0 => 0x64,
            1 => 0xe9,
            2 => 0x48,
            _ => 0xb5,
        };
        buf[i] = (start ^ i) as u8;
    }

    Result::Ok(())
}

pub(crate) fn register() {
    register_custom_getrandom!(fake_random);
}
