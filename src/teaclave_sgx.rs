use sgx_trts::trts::rsgx_read_rand;

use crate::Error;

#[allow(deprecated)]
pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
  // sgx_read_rand cannot take len=0, but this function does
  if dest.is_empty() {
    return Ok(());
  }

  match rsgx_read_rand(dest) {
    Ok(()) => Ok(()),
    Err(_) => Err(Error::UNAVAILABLE),
  }
}
