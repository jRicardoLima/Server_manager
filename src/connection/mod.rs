use std::{io::Read, net::TcpStream, path::Path};

use ssh2::{Channel, Session};

use crate::parser::{ConfigYaml, ConnectionType, ServerCommands, ServerConnect};

#[derive(Debug,PartialEq, Eq,Clone)]
pub struct SSH {
    type_connection: ConnectionType,
    ip_address: String,
    user_name: String,
    password: Option<String>,
    location: Option<String>
}

impl SSH {

    pub fn new(server_connect:  &ServerConnect) -> SSH {

        let ssh = SSH {
            type_connection: server_connect.type_connection().clone(),
            ip_address: server_connect.ip_address().clone(),
            user_name: server_connect.user().clone(),
            password: server_connect.password().clone(),
            location: server_connect.location().clone()
        };
        ssh
    }

    pub fn connect(&self) -> Result<Session,ssh2::Error> {

        let tcp = match TcpStream::connect(&self.ip_address) {
            Ok(res) => res,
            Err(err) => panic!("Não foi possivel inicializar a conexão TCP: {:?}",err)
        };

        let mut sess = match Session::new() {
          Ok(ses) => ses,
          Err(err) => panic!("Não foi possivel inicializar a sessão: {:?}",err)
        };

        sess.set_tcp_stream(tcp);
        sess.handshake()?;

        if let Some(password) = &self.password {
            sess.userauth_password(&self.user_name, password)?;
        } else {
            sess.userauth_agent(&self.user_name)?;
        }

        Ok(sess)

    }

    pub fn connect_with_private_key(&self) -> Result<Session,ssh2::Error> {

        let tcp = match TcpStream::connect(&self.ip_address){
             Ok(res) => res,
             Err(err) => panic!("Não foi possivle inicializar a conexão TCP: {:?}",err)
        };

        let mut sess = match Session::new() {
            Ok(ses) => ses,
            Err(err) => panic!("Não foi possivel inicalizar a sessa: {:?}",err)
        };

        sess.set_tcp_stream(tcp);
        sess.handshake()?;

        let localKey = match &self.location {
            Some(local) => {
                Path::new(local)
            },
            None => panic!("Chave não encontrada")
        };

        if let Some(password) = &self.password {
            sess.userauth_pubkey_file(&self.user_name, None,localKey,Some(password))?;
        } else {
            sess.userauth_pubkey_file(&self.user_name, None,localKey,None)?;
        }


        Ok(sess)
    }

    pub fn manager_commands<F>(exec_commands: &Vec<String>, concat_fn: F) -> String
    where
        F: Fn(&Vec<String>) -> String,
    {
        let concatenated_commands = concat_fn(exec_commands);

        concatenated_commands
    }

    pub fn execute_commands(&self,server_commands: &ServerCommands, session: Session) -> String {

        let commands_list = server_commands.commands();

        let mut formated_commands: String = String::new();

        if *&self.type_connection == ConnectionType::SSH {
            formated_commands = SSH::manager_commands(commands_list, |commands| commands.join(" && "));
        } else {
           let prefix = commands_list[0].clone();

           let remaining_commands: Vec<String> = commands_list.iter()
                                                              .skip(1)
                                                              .cloned()
                                                              .collect();
           let format_commands = vec![format!(
               "{} '{}'",
               prefix,
               remaining_commands.join(" && ")
               )];

           formated_commands = SSH::manager_commands(&format_commands, |commands| commands.join(""));

        }
        //let formated_commands = SSH::manager_commands(commands_list,|commands| commands.join(" && "));

        let mut channel = session.channel_session().unwrap();

        match channel.exec(formated_commands.as_str()) {
            Ok(_) => {
                let mut output = String::new();

                match channel.read_to_string(&mut output) {
                    Ok(_) => {
                        //println!("Saida do comando: {:?}",s);
                        channel.wait_close().unwrap();
                        channel.exit_status().unwrap();

                        return output;
                    },
                    Err(e) => {
                        return format!("Erro ao executar comando: {:?}",e)
                        //panic!("Erro ao ler a saida do comando: {:?}",e)
                    }
                }
            },
            Err(e) => format!("Erro desconhecido: {:?}",e)
        }

    }
}

#[test]
fn test_connect_server() {
    let path = "config.yaml";

    let config = ConfigYaml::new(path).unwrap();

    let server = config.get_info_server("Servidor 1").unwrap();

    let ssh = SSH::new(&server.1);

    let result = ssh.connect();

    assert!(result.is_ok(),"Falha na conexão SSH: {:?}",result.err());

}

#[test]
fn test_command_server() {
    let path = "config.yaml";

    let config = ConfigYaml::new(path).unwrap();

    let server = config.get_info_server("Servidor 1").unwrap();

    let ssh = SSH::new(&server.1);

    let session = match ssh.connect() {
        Ok(sess) => sess,
        Err(e) => panic!("Não foi possivel abrir a sessão: {:?}",e)
    };

    let commands = server.2[0].clone();

    ssh.execute_commands(&commands, session);
}

#[test]
fn test_manager_commands_ssh_key() {
    let commands_list = vec![
        String::from("sudo su -c"),
        String::from("cd /home/ubuntu"),
        String::from("touch testando_commands.txt")
    ];

    let prefix = commands_list[0].clone();

    let remaining_commands: Vec<String> = commands_list.iter()
                                         .skip(1)
                                         .cloned()
                                         .collect();
    let formatted_commands = vec![format!(
        "{} '{}'",
        prefix,
        remaining_commands.join(" && ")
        )];

    let result = SSH::manager_commands(&formatted_commands, |commands| commands.join(""));

    let expected ="sudo su -c 'cd /home/ubuntu && touch testando_commands.txt'";

    assert_eq!(result,expected,"Comands concatenados não correspondem ao formato esperado");
}