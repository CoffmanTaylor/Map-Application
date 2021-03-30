use std::collections::BTreeMap;

use ds_libs::Application;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command<Key, Value> {
    Get(Key),
    Store(Key, Value),
    Clear(Key),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CommandResponse<Value> {
    Ok(),
    Empty(),
    Value(Value),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapApplication<Key, Value> {
    map: BTreeMap<Key, Value>,
}

impl<Key, Value> MapApplication<Key, Value>
where
    Key: Ord,
{
    pub fn new() -> Self {
        MapApplication {
            map: BTreeMap::new(),
        }
    }

    pub fn from_map(map: BTreeMap<Key, Value>) -> Self {
        MapApplication { map }
    }

    /// *WARNING* this should only be used for testing
    pub fn get_map(&self) -> &BTreeMap<Key, Value> {
        &self.map
    }
}

impl<Key, Value> Default for MapApplication<Key, Value>
where
    Key: Ord,
{
    fn default() -> Self {
        MapApplication::new()
    }
}

impl<Key, Value> Application for MapApplication<Key, Value>
where
    Key: Ord + Clone,
    Value: Clone,
{
    type Command = Command<Key, Value>;
    type Res = CommandResponse<Value>;

    fn process(&mut self, request: Command<Key, Value>) -> CommandResponse<Value> {
        match request {
            Command::Get(key) => {
                if let Some(value) = self.map.get(&key).cloned() {
                    CommandResponse::Value(value)
                } else {
                    CommandResponse::Empty()
                }
            }
            Command::Store(key, new_value) => {
                self.map.insert(key, new_value);
                CommandResponse::Ok()
            }
            Command::Clear(key) => {
                self.map.remove(&key);
                CommandResponse::Ok()
            }
        }
    }
}
