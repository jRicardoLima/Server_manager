use std::{io::Read, net::TcpStream};

use ssh2::{Channel, Session};

use crate::parser::{ConfigYaml, ServerCommands, ServerConnect};

#[derive(Debug,PartialEq, Eq,Clone)]
pub struct SSH {
    ip_address: String,
    user_name: String,
    password: Option<String>
}

impl SSH {

    pub fn new(server_connect:  &ServerConnect) -> SSH {

        let ssh = SSH {
            ip_address: server_connect.ip_address().clone(),
            user_name: server_connect.user().clone(),
            password: server_connect.password().clone(),
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
    pub fn manager_commands<F>(exec_commands: &Vec<String>, concat_fn: F) -> String
    where
        F: Fn(&Vec<String>) -> String,
    {
        let concatenated_commands = concat_fn(exec_commands);

        concatenated_commands
    }

    pub fn execute_commands(server_commands: &ServerCommands, session: Session) -> String {

        let commands_list = server_commands.commands();

        let formated_commands = SSH::manager_commands(commands_list,|commands| commands.join(" && "));

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

    SSH::execute_commands(&commands, session);
}