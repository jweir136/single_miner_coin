use std::collections::HashMap;

pub type Username = String;
pub type PublicKey = [u8; 32];

fn add_user(key_map: &mut HashMap<Username, PublicKey>, username: Username, key: PublicKey) -> Option<()> {
    match key_map.insert(username, key) {
        Some(_) => { Some(()) },
        None   => { None }
    }
}