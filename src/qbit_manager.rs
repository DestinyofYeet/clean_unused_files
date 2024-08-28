use qbit_rs::Qbit;
use qbit_rs::model::{Credential, GetTorrentListArg};
use qbit_rs::model::Hashes::Hashes;

pub struct QbitWrapper {
    client: Qbit,
}

impl QbitWrapper {
    pub async fn new(username: &str, password: &str, url: &str) -> QbitWrapper {
        // let credential = Credential::new(username, password);

        QbitWrapper {
            client: Qbit::new(url, Credential::new(username, password))
        }
    }

    pub async fn auth(&self){
        self.client.login(true).await.unwrap();
    }
}

pub async fn return_paths(qbit: &QbitWrapper) -> Vec<(String, String)> {
    let mut paths: Vec<(String, String)> = Vec::new();

    let torrent_list = qbit.client.get_torrent_list(GetTorrentListArg::default()).await.unwrap();

    paths.reserve_exact(torrent_list.len());

    for torrent in torrent_list {
        // let tc = torrent.clone();
        let full_path: String = torrent.content_path.unwrap();

        // println!("{:?} {:?}\n", full_path, tc);

        let tuple = (full_path, String::from(torrent.hash.unwrap()));


        paths.append(&mut vec![tuple]);
    }

    return paths;
}

pub async fn remove_file_from_torrent(hash: &str, qbit: &QbitWrapper){
    qbit.client.delete_torrents(Hashes(String::from(hash).parse().unwrap()), true).await.unwrap();
}
