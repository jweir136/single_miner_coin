use std::collections::HashMap;

pub type Username = String;
pub type PublicKey = [u8; 32];

fn add_user(key_map: &mut HashMap<Username, PublicKey>, username: Username, key: PublicKey) -> Option<()> {
    match key_map.insert(username, key) {
        Some(_) => { Some(()) },
        None   => { None }
    }
}

fn get_user<'a>(key_map: &'a HashMap<Username, PublicKey>, username: &'a Username) -> Option<&'a PublicKey> {
    key_map.get(username)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut map = std::collections::HashMap::<Username, PublicKey>::new();

        add_user(&mut map, "Jake".to_string(), [0 as u8; 32]);
        get_user(&map, &"Jake".to_string());
    }
}