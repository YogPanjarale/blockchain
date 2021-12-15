use std::fmt::Debug;
use chrono::{Utc};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme, pkcs1::ToRsaPublicKey};
use rand::rngs::OsRng;
#[derive(Debug)]
struct Transaction {
    pub amount: i32,
    pub payer: String,
    pub payee: String,
}
impl Transaction {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
struct Block {
    pub prev_hash: String,
    pub transaction: Transaction,
    pub ts: i64,
}
impl Block {
    pub fn new(prev_hash:String,transaction:Transaction,ts:Option<i64>) -> Block {
        let ts = match ts {
            Some(ts) => ts,
            None => Utc::now().timestamp(),
        };
        Block {
            prev_hash,
            transaction,
            ts,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn get_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(&self.to_string());
        hasher.result_str()
    }
}

struct Chain{
    chain : Vec<Block>,

}
impl Chain{
    pub fn new() -> Chain {
        let genesis_transaction = Transaction {
            amount: 0,
            payer: "genesis".to_string(),
            payee: "yog".to_string(),
        };
        let genesis_block = Block::new("0".to_string(),genesis_transaction,None);
        Chain {
            chain: vec![genesis_block],
        }
    }
    pub fn get_last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
    pub fn add_block(&mut self,transaction:Transaction,sender_public_key:String,signature:String)  {
       let last_block = self.get_last_block();
       let new_block = Block::new(last_block.get_hash(),transaction,None);
       //INFO: verify the signature
       self.chain.push(new_block);
    }
}
#[derive(Debug)]
struct Wallet{
    pub public_key: String,
    pub private_key: String,
}
impl Wallet{
    pub fn new()->Wallet{
        let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    Wallet {
        public_key:match public_key.to_pkcs1_pem(){
            Ok(pkcs1) => pkcs1,
            Err(e) => panic!("{:?}",e),
        },
        private_key : match private_key.to_pkcs1_pem(){
            Ok(pkcs1) => pkcs1,
            Err(e) => panic!("{:?}",e),
        },
    }
    }
    pub fn sign(&self,message:&str)->String{
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::from(&self.private_key).expect("failed to generate a key");
        let signature = private_key.sign(&mut rng,message.as_bytes()).expect("failed to sign");
        signature.to_pkcs1_pem().expect("failed to convert signature to pem")
    }
    pub fn add_money(&mut self,amount:i32,payeePublicKey:String){
        let transaction = Transaction{
            amount,payer:self.public_key,
            payee:payeePublicKey
        };
        let sign  = self.sign(&transaction.to_string());
}
fn main() {
    // use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
   
    // let mut rng = OsRng;
    // let bits = 2048;
    // let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    // let public_key = RsaPublicKey::from(&private_key);
    // // println!("{:?}", public_key);
    // // println!("{:?}", private_key);
    // let wallr
    // // Encrypt
    // let data = b"hello world";
    // let padding = PaddingScheme::new_pkcs1v15_encrypt();
    // let enc_data = public_key.encrypt(&mut rng, padding, &data[..]).expect("failed to encrypt");

    // assert_ne!(&data[..], &enc_data[..]);

    let wallet = Wallet::new();
    println!("{:?}",wallet);

}
