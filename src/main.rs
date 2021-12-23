extern crate hyper;
extern crate multipart;

mod shared;
mod deploy_client;
mod deploy_server;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "deploy", about = "Deployment toolsuite for leadform")]
enum Command {
    GenerateKeys,
    Serve {
        public_key: String        
    },
    Deploy {
        #[structopt(long)]
        private_key: String,

        url: String,

        #[structopt(parse(from_os_str))]
        files: Vec<std::path::PathBuf>
    }
}

fn make_private_key(s: &'_ str) -> Result<minisign::SecretKey, ()> {
    let private_key = base64::decode(s).or(Err(()))?;
    minisign::SecretKey::from_bytes(&private_key).or(Err(()))
}

fn main() {
    let command = Command::from_args();

    match command {
        Command::GenerateKeys => {
            let key_pair = match minisign::KeyPair::generate_unencrypted_keypair() {
                Err(_) => {
                    eprintln!("Key generation failed");
                    std::process::exit(1);
                },
                Ok(key_pair) => key_pair
            };

            let pk = key_pair.pk.to_base64();
            let sk = base64::encode(&key_pair.sk.to_bytes());
            println!("Public Key:  {}", pk);
            println!("Private Key: {}", sk);
        },
        Command::Serve { public_key } => {
            let public_key = match minisign::PublicKey::from_base64(&public_key) {
                Ok(key) => key,
                Err(_) => {
                    eprintln!("The provided public key is not valid");
                    std::process::exit(1);
                }
            };
            deploy_server::main(public_key);
        },
        Command::Deploy { private_key, url, files } => {
            let private_key = match make_private_key(&private_key) {
                Ok(key) => key,
                Err(_) => {
                    eprintln!("The provided private key is not valid");
                    std::process::exit(1);
                }
            };

            deploy_client::main(private_key, files, &url);
        }
    }
}
