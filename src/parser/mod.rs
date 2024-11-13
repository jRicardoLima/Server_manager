
use serde::{de::Error, Deserialize, Serialize};
use std::{fs::read_to_string, result::Result};


#[derive(Debug,PartialEq, Eq, Serialize ,Deserialize,Clone)]
pub enum ConnectionType {
    SSH,
    SSH_KEY,

}

#[derive(Debug,PartialEq, Eq, Serialize, Deserialize,Clone)]
pub struct ServerDetails {
    name: String,
    config: ServerConfig,
    connect: ServerConnect
}

#[derive(Debug,PartialEq, Eq, Serialize, Deserialize,Clone,Default)]
pub struct ServerConfig {
    os: String,
    memory: String,
    disk: String,
}

#[derive(Debug,PartialEq, Eq, Serialize, Deserialize,Clone,Default)]
pub struct ServerConnect {
    type_connection: ConnectionType,
    user: String,
    location: Option<String>,
    password: Option<String>,
}

impl Default for ConnectionType {
    fn default() -> Self {
        ConnectionType::SSH
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigYaml {
    version: String,
    application: String,
    servers: Vec<ServerDetails>
}

impl ConfigYaml {
    pub fn new(path: &str) -> Result<ConfigYaml,serde_yaml_ng::Error > {

//        let content_file = std::fs::read_to_string(path).unwrap();
//
//        let config: ConfigYaml = serde_yaml_ng::from_str(&content_file).unwrap();

//        let content_file = std::fs::read_to_string(path).map_err(|e| serde_yaml_ng::Error::custom(format!("Erro ao ler o arquivo")))?;
//
//        let config: ConfigYaml  = serde_yaml_ng::from_str(&content_file)
//            .map_err(|e| serde_yaml_ng::Error::custom(format!("Erro ao fazer parsing do arquivo config.yaml: {}",e)))?;

        //let content = std::fs::read_to_string("path/to/file").expect("Falha ao ler o arquivo!");

        let content_file = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => return Err(serde_yaml_ng::Error::custom(format!("Erro ao ler o arquivo: {}", e)))
        };

        let config: ConfigYaml = match serde_yaml_ng::from_str(&content_file) {
            Ok(parsed) => parsed,
            Err(e) => return Err(serde_yaml_ng::Error::custom(format!("Erro ao fazer parsing do arquivo Yaml: {}",e)))
        };
        Ok(config)

    }

    pub fn list_servers(&self) -> &Vec<ServerDetails> {
        &self.servers
    }

    pub fn get_info_server(&self,name_server: &str) -> Option<(ServerConfig,ServerConnect)> {

        self.servers.iter()
                    .find(| &item | item.name == name_server)
                    .map(| server | {
                        (server.config.clone(),server.connect.clone())
                    })
    }

    pub fn get_quantity_servers(&self) -> usize {
        self.servers.len()
    }
}

#[test]
fn test_parsing_yaml_file() {
    let path = "config.yaml";

    let config = ConfigYaml::new(&path);

    match config {
        Ok(config) => {
            assert_eq!(config.version, "1.0.0");
            assert_eq!(config.servers.len(),2);

            let server1 = &config.servers[0];
            assert_eq!(server1.config.memory,"32GB");
            assert_eq!(server1.config.os,"Ubuntu");
            assert_eq!(server1.connect.type_connection,ConnectionType::SSH);

            let server2 = &config.servers[1];
            assert_eq!(server2.config.memory,"100GB");
            assert_eq!(server2.config.os,"Red Hat");
            assert_eq!(server2.connect.type_connection,ConnectionType::SSH_KEY);

        },
        Err(e) => panic!("Erro ao ler o arquivo Yaml: {:?}",e)
    }
}

#[test]
fn test_quantity_servers() {
    let path = "config.yaml";

    let config = ConfigYaml::new(&path);

    match config {
        Ok(config) => {
            assert_eq!(config.get_quantity_servers(),2)
        },
        Err(e) => panic!("Erro ao executar método: {:?}",e)

    }
}

#[test]
fn test_info_server() {
    let path = "config.yaml";

    let config = ConfigYaml::new(&path).unwrap();

    let expected_config = ServerConfig{
        os: String::from("Red Hat"),
        memory: String::from("100GB"),
        disk: String::from("16TB"),
    };

    let expected_connect = ServerConnect{
        type_connection: ConnectionType::SSH_KEY,
        user: String::from(""),
        location: Some(String::from("")),
        password: None
    };

    let configs = config.get_info_server("Servidor 2");

    assert_eq!(configs, Some((expected_config,expected_connect)));
}

#[test]
fn test_info_server_configs() {
    let path = "config.yaml";

    let config = ConfigYaml::new(&path).unwrap();

    let result = config.get_info_server("Servidor 2").unwrap();

    assert_eq!(result.0.os, "Red Hat");
    assert_eq!(result.0.memory,"100GB");
    assert_eq!(result.0.disk, "16TB");

    assert_eq!(result.1.type_connection,ConnectionType::SSH_KEY);
    assert_eq!(result.1.user, String::from(""));
    assert_eq!(result.1.location, Some(String::from("")));
    assert_eq!(result.1.password, None);


}