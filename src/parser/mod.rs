
use serde::{de::Error, Deserialize, Serialize};
use std::result::Result;


#[derive(Debug,PartialEq, Eq, Serialize ,Deserialize,Clone)]
pub enum ConnectionType {
    SSH,
    SSH_KEY,

}

#[derive(Debug,PartialEq, Eq, Serialize, Deserialize,Clone)]
pub struct ServerDetails {
    pub name: String,
    config: ServerConfig,
    connect: ServerConnect,
    commands: Vec<ServerCommands>,
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
    ip_address: String,
    location: Option<String>,
    password: Option<String>,
}

impl Default for ConnectionType {
    fn default() -> Self {
        ConnectionType::SSH
    }
}
#[derive(Debug,PartialEq, Eq,Serialize, Deserialize,Clone,Default)]
pub struct ServerCommands {
    name: String,
    exec: Vec<String>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize,Clone)]
pub struct ConfigYaml {
    version: String,
    application: String,
    servers: Vec<ServerDetails>
}

impl ConfigYaml {
    pub fn new(path: &str) -> Result<ConfigYaml,serde_yaml_ng::Error > {

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

    pub fn get_info_server(&self,name_server: &str) -> Option<(ServerConfig,ServerConnect,Vec<ServerCommands>)> {

        self.servers.iter()
                    .find(| &item | item.name == name_server)
                    .map(| server | {
                        (server.config.clone(),server.connect.clone(),server.commands.clone())
                    })
    }

    pub fn get_quantity_servers(&self) -> usize {
        self.servers.len()
    }

}

impl ServerConfig {
    pub fn os(&self) -> &str {
        &self.os
    }

    pub fn memory(&self) -> &str {
        &self.memory
    }

    pub fn disk(&self) -> &str {
        &self.disk
    }
}

impl ServerCommands {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut str {
        &mut self.name
    }

    pub fn commands(&self) -> &Vec<String> {
        &self.exec
    }

    pub fn commands_mut(&mut self) -> &mut Vec<String> {
        &mut self.exec
    }
}

impl ServerConnect {
    pub fn type_connection(&self) -> &ConnectionType {
       &self.type_connection
    }

    pub fn user(&self) -> &String {
        &self.user
    }

    pub fn ip_address(&self) -> &String {
        &self.ip_address
    }

    pub fn ip_address_mut(&mut self) -> &mut String {
        &mut self.ip_address
    }

    pub fn password(&self) -> &Option<String> {
        &self.password
    }

    pub fn password_mut(&mut self) -> &mut Option<String> {
        &mut self.password
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
            assert_eq!(server1.commands.len(),2);

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
        os: String::from("Ubuntu"),
        memory: String::from("32GB"),
        disk: String::from("400GB"),
    };

    let expected_connect = ServerConnect{
        type_connection: ConnectionType::SSH,
        user: String::from(""),
        ip_address: String::from("123456"),
        location: None,
        password: Some(String::from(""))
    };

    let expected_commands = vec![
      ServerCommands{
        name: String::from("Atualizar Servidor"),
        exec: vec![
            String::from("mkdir {nome_pasta}"),
            String::from("git clone {url}"),
            String::from("touch {nome_arquivo}")
        ]
      },
      ServerCommands{
        name: String::from("Criar cliente"),
        exec: vec![
            String::from("git clone {url}"),
            String::from("cd {nome_pasta}"),
            String::from("composer install"),
            String::from("chmod 777 -R {nome_pasta}"),
            String::from("php index.php migrate"),
            String::from("chown -R gitlab-runner:gitlab-runner ."),
            String::from("git checkout .")
        ]
      }
    ];

    let configs = config.get_info_server("Servidor 1");
    let expected_tuple = Some((expected_config,expected_connect,expected_commands));

    if let Some((config,connect,commands)) = &configs {

        let command = &commands[0];

        assert_eq!(command.name,"Atualizar Servidor".to_string());
        assert_eq!(commands.len(),2);
    }
    assert_eq!(configs,expected_tuple);


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