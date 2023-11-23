use serde::{Deserialize, Serialize};
use tonic::transport::{ Identity, Certificate };

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientConfig {
    pub server_info: ServerInfo,
    pub certs_info: CertsInfo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfo {
    pub address: String,
    pub domain: String,
    pub port: u16
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertsInfo {
    certs_dir: String,
    leaf_cert: String,
    leaf_key: String,
    ca_cert: String
}

impl CertsInfo {
    pub fn get(&self) -> (Identity, Certificate) {
        let cert_path = self.certs_dir.clone() + "\\" + self.leaf_cert.as_str();
        let key_path = self.certs_dir.clone() + "\\" + self.leaf_key.as_str();
        let ca_cert_path = self.certs_dir.clone() + "\\" + self.ca_cert.as_str();

        println!("cert_path: {}", cert_path);
        println!("key_path: {}", key_path);
        println!("ca_cert_path: {}", ca_cert_path);
        
        (
            Identity::from_pem(cert_path, key_path),
            Certificate::from_pem(ca_cert_path)
        )
    }
}

impl ClientConfig {
    pub fn new (config_file_path: String) -> Self {
        let config_file = std::fs::File::open(config_file_path)
                                                .expect("Could not open file.");
        let config: ClientConfig = serde_yaml::from_reader(config_file)
                                                .expect("Could not read values.");
        println!("{:?}", config);

        config
    }
}

#[cfg(test)]
mod tests {
    use super::ClientConfig;

    #[test]
    fn test_server_config_new() {
        // Call the new method with the temporary config file
        let _config = ClientConfig::new(
            String::from(r"C:\Users\zeyadhabib\source\repos\generic_search\zombie_hunt\config\client_config.yaml"));
    }
}