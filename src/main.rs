use std::ops::Add;

use std::process::exit;

use clap::Parser;

mod file_manager;
mod qbit_manager;
mod email_handler;
mod config;


#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Remove unused files from qbit", long_about = None)]
struct Args {
    #[arg(short = 'c', long = "config-file")]
    pub config_file: String, 

    #[arg(short = 'd', long = "data-file")]
    pub data_file: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let app_config = config::Config::parse(&args.config_file);

    if app_config.is_none(){
        eprintln!("Failed to parse config file!");
        exit(1);
    }

    let app_config = app_config.unwrap();

    let (username, password, url) = (app_config.qb_user, app_config.qb_password, app_config.qb_url);

    let cache_file = &args.data_file;

    let mut data_file = file_manager::DataFile::new();

    if !file_manager::does_file_exists(cache_file) {
        file_manager::write_json_file(cache_file, &data_file);
    }

    data_file = file_manager::read_file_json(cache_file);

    let qbit = qbit_manager::QbitWrapper::new(&username, &password, &url).await;
    qbit.auth().await;

    let email = email_handler::Email::new(&app_config.mail_server, &app_config.mail_recipient, &app_config.mail_user, &app_config.mail_pw);

    let file_paths = qbit_manager::return_paths(&qbit).await;

    let mut deleted_paths: Vec<String> = Vec::new();
    deleted_paths.reserve_exact(file_paths.len());

    let mut need_to_delete_on_next_run: Vec<String> = Vec::new();
    need_to_delete_on_next_run.reserve_exact(file_paths.len());

    let mut deleted_size: u64 = 0;

    for path in file_paths.iter() {
        let file_path = &path.0;
        let hash = &path.1;

        println!("Checking: {}", file_path);

        let can_remove = file_manager::handle_path(file_path);

        if can_remove {

            if data_file.data.contains(&file_path.clone()){
                deleted_size += file_manager::get_size(file_path);
                deleted_paths.push(file_path.clone());
                qbit_manager::remove_file_from_torrent(&hash, &qbit).await;

            } else {
                need_to_delete_on_next_run.push(file_path.clone());
            }
        }

    }

    let mut new_data_file = file_manager::DataFile::new();
    new_data_file.data.reserve_exact(need_to_delete_on_next_run.len());

    for path in &need_to_delete_on_next_run {
        new_data_file.data.push(path.clone());
    }

    file_manager::del_file(cache_file);
    file_manager::write_json_file(cache_file, &new_data_file);


    let mut string: String = String::new();

    string.push_str("Deleted following files and directories:\n");
    for path in &deleted_paths {
        string.push_str(path.clone().add("\n").as_str());
    }


    string.push_str("\n\n");
    string.push_str(format!("Cleaned up {} files ({}) | Need to delete {} files on the next run",
                            deleted_paths.len(),
                            file_manager::format_bytes(deleted_size),
                            need_to_delete_on_next_run.len())
        .as_str());

    email.send("Cleaned qbittorrent downloads", string.as_str());

    return Ok(())
}
