use configparser::ini::Ini;
use std::fs::File;
use std::io::Read;
use std::process::exit;

pub struct Config {
    pub qb_url: String,
    pub qb_user: String,
    pub qb_password: String,

    pub mail_server: String,
    pub mail_user: String,
    pub mail_pw: String,
    pub mail_recipient: String,
}

impl Config {
    pub fn parse(path: &str) -> Option<Config> {
        let mut file = match File::open(path){
            Err(e) => panic!("Failed to open config file '{}' because: {}", path, e),
            Ok(file) => file
        };

        let mut file_content = String::new();

        match file.read_to_string(&mut file_content){
            Err(e) => panic!("Could not config file contents because {}", e),
            Ok(_) => {

                let mut config = Ini::new();
                config.set_comment_symbols(&[]); // disable comment-symbols '#' and ';' for
                                                 // password reasons
                match config.read(file_content) {
                    Err(e) => panic!("Failed to parse config file because {}", e),
                    Ok(_) => {}
                };
                
                return Some(Config {
                    qb_url: Self::get_config("QBIT", "url", &config),
                    qb_user: Self::get_config("QBIT", "user", &config),
                    qb_password: Self::get_config("QBIT", "password", &config),
                    
                    mail_server: Self::get_config("EMAIL", "server", &config),
                    mail_user: Self::get_config("EMAIL", "user", &config),
                    mail_pw: Self::get_config("EMAIL", "password", &config),
                    mail_recipient: Self::get_config("EMAIL", "recipient", &config)
                });
            }
        }
    }

    fn get_config(section: &str, key: &str, config: &Ini) -> String{
        let value = config.get(section, key);
        
        if value.is_none() {
            eprintln!("Could not find key '{}' in section '{}'!", key, section);
            exit(1);
        }
        
        return value.unwrap();
    }
    
    
}
