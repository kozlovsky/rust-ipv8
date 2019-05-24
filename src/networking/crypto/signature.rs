use serde::{Deserialize,Serialize};
use crate::networking::crypto::{verify_signature_ed25519, create_signature_ed25519, verify_signature_openssl, create_signature_openssl};
use std::error::Error;
use rust_sodium::crypto::sign::ed25519;
use crate::networking::payloads::Ipv8Payload;
use serde::ser::Serializer;
use serde::ser::SerializeTuple;
use crate::networking::serialization::rawend::RawEnd;
use openssl::bn::BigNum;
use crate::networking::crypto::keytypes::{PublicKey, PrivateKey, get_signature_length};
use std::fmt;

create_error!(KeyError, "Invalid Key");
create_error!(CurveError, "This curve is unknown");
create_error!(OpenSSLError, "OpenSSL had a rapid unscheduled disassembly (oops)");

#[derive(PartialEq, Debug)]
pub struct Signature{
  pub signature : Vec<u8>
}

impl Ipv8Payload for Signature{
  // this is just to allow it to be serialized to a packet. It isn't actually a "payload"
}

impl Serialize for Signature {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer{
    let mut state = serializer.serialize_tuple(self.signature.len())?;
    state.serialize_element(&self.signature)?;
    state.end()
  }
}

impl Signature{
  pub fn from_bytes(data: &[u8], skey: PrivateKey) -> Result<Self, Box<Error>>{
    match skey {
      PrivateKey::Ed25519(i) => {
        let signature: Vec<u8> = create_signature_ed25519(data,i)?.as_ref().to_owned();
        Ok(Self{
          signature
        })
      },
      PrivateKey::OpenSSLHigh(i) |
      PrivateKey::OpenSSLMedium(i) |
      PrivateKey::OpenSSLLow(i) |
      PrivateKey::OpenSSLVeryLow(i) => {
        // get the curve name. this has to go first as the create_signature function consumes i.
        let curvename = match i.group().curve_name(){
          Some(i) => i,
          None => return Err(Box::new(CurveError))
        };
        // from the name the signature length can be calcualted
        let half_signature_length = (match get_signature_length(curvename){
          Some(i) => i,
          None => return Err(Box::new(CurveError))
        } / 2u16) as usize;

        let signature = create_signature_openssl(data, i)?;

        let s = signature.s().to_vec();
        let r = signature.r().to_vec();

        let s_padding = half_signature_length - s.len() + s.len()%2;
        let r_padding = half_signature_length - r.len() + r.len()%2;

        let mut result = vec![0; r_padding as usize];
        result.extend(r);
        result.resize(result.len() + s_padding, 0); // resize to append n zeros faster
        result.extend(s);

        return Ok(Self{
          signature:result
        })
      }
      _ => Err(Box::new(KeyError))
    }
  }

  pub fn verify(&self, data: &[u8], pkey: PublicKey) -> bool{
    match pkey {
      PublicKey::Ed25519(i) => match verify_signature_ed25519(self.signature.to_owned(),data,i){
        Ok(i) => i,
        Err(_) => false, // if an error occurred, the signature is invalid and therefore did not match
      },
      PublicKey::OpenSSLHigh(i) |
      PublicKey::OpenSSLMedium(i) |
      PublicKey::OpenSSLLow(i) |
      PublicKey::OpenSSLVeryLow(i) => {
        let curvename = match i.group().curve_name(){
          Some(i) => i,
          None => return false
        };
        // from the name the signature length can be calcualted
        let half_signature_length = (match get_signature_length(curvename){
          Some(i) => i,
          None => return false
        } / 2u16) as usize;

        let signature = &*self.signature;
        let r = &signature[..half_signature_length];
        let s = &signature[half_signature_length..];

        match verify_signature_openssl((match BigNum::from_slice(s){
          Ok(i) => i,
          Err(_) => return false
        },match BigNum::from_slice(r){
          Ok(i) => i,
          Err(_) => return false
        }),data,i){
          Ok(i) => i,
          Err(_) => false, // if an error occurred, the signature is invalid and therefore did not match
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rust_sodium::crypto::kx::keypair_from_seed;
  use crate::networking::crypto::keytypes::PublicKey::OpenSSLVeryLow;
  use openssl;
  use core::mem;

  #[test]
  fn test_signature_ed25519() {
    let seed = ed25519::Seed::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,]).unwrap();
    let (pkey,skey) = ed25519::keypair_from_seed(&seed);
    let sig = Signature::from_bytes(&[42,43,44],PrivateKey::Ed25519(skey)).unwrap();
    assert_eq!(
      vec![31, 14, 50, 234, 129, 186, 124, 84, 223, 67, 233, 173, 116, 95, 218, 136, 149, 223, 171, 234, 13, 173, 164, 78, 74, 59, 106, 31, 252, 230, 79, 207, 199, 207, 134, 92, 252, 211, 142, 172, 183, 61, 17, 236, 208, 124, 206, 37, 204, 85, 62, 155, 171, 129, 153, 90, 3, 148, 202, 220, 53, 159, 172, 7],
      sig.signature
    );

    assert!(sig.verify(&[42,43,44], PublicKey::Ed25519(pkey)));
  }

  #[test]
  fn test_signature_SECT163K1() {
    // private key generated with SECT163K1 and is allways constant because it is directly pasted here
    //TODO: check if we can manually verify the sig
    let skey = openssl::ec::EcKey::private_key_from_pem("-----BEGIN EC PRIVATE KEY-----\nMFMCAQEEFQKu4aaDxyTSj92iquQP5CIdbagLP6AHBgUrgQQAAaEuAywABABQ76xopUysBuWInGkX+S4elFdpOQZphgLlc6ksoim+5DgUZEBPp+B2Dg==\n-----END EC PRIVATE KEY-----".as_bytes()).unwrap();
    //let pkey = openssl::ec::EcKey::public_key_from_pem("-----BEGIN PUBLIC KEY-----\nMEAwEAYHKoZIzj0CAQYFK4EEAAEDLAAEAFDvrGilTKwG5YicaRf5Lh6UV2k5BmmGAuVzqSyiKb7kOBRkQE+n4HYO\n-----END PUBLIC KEY-----".as_bytes()).unwrap();
    let pkey_tmp = openssl::pkey::PKey::public_key_from_pem("-----BEGIN PUBLIC KEY-----\nMEAwEAYHKoZIzj0CAQYFK4EEAAEDLAAEAFDvrGilTKwG5YicaRf5Lh6UV2k5BmmGAuVzqSyiKb7kOBRkQE+n4HYO\n-----END PUBLIC KEY-----".as_bytes()).unwrap();
    let pkey = pkey_tmp.ec_key().unwrap();


    let sig = Signature::from_bytes(&[42,43,44],PrivateKey::OpenSSLVeryLow(skey)).unwrap();
    println!("{:?}",sig);

    assert!(sig.verify(&[42,43,44], PublicKey::OpenSSLVeryLow(pkey)));
  }
}
