use securestore::{KeySource, SecretsManager};
use std::path::Path;

pub struct Secrets {
    sman: SecretsManager,
}

impl Secrets {
    pub fn new() -> Secrets {
        let secret_path_str: &str = "secrets.key";
        let secret_path = Path::new(secret_path_str);
        let json_path_str: &str = "secrets.json";
        let json_path = Path::new(json_path_str);
        let loaded_secrets = SecretsManager::load(json_path, KeySource::Path(secret_path))
            .expect("Failed to load secrets store!");
        Secrets {
            sman: loaded_secrets,
        }
    }
    pub fn get_secret(self: &Self, key: &str) -> String {
        self.sman
            .get(key)
            .expect("Couldn't get db_password from vault!")
    }
}

//pub trait SecretsImpl {
//    fn new() -> Secrets;
//    fn get_secret(self: &Self, key: &str) -> String;
//}
//
//impl SecretsImpl for Secrets {
//    fn new() -> Secrets {
//        let p: String = "../secrets.key";
//        let kp = Path::new(p);
//        let loaded_secrets = SecretsManager::load("secrets.json", KeySource::Path(kp))
//            .expect("Failed to load secrets store!");
//        Secrets {
//            path: p,
//            key_path: kp,
//            sman: loaded_secrets,
//        }
//    }
//    fn get_secret(self: &Self, key: &str) -> String {
//        self.sman
//            .get(key)
//            .expect("Couldn't get db_password from vault!")
//    }
//}
