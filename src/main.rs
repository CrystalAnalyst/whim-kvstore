#![allow(unused)]
use std::sync::{Arc, Mutex};
/*
    Design a In-momery Key-value storage,
    it can be accessed by multiple threads.
    Two basic APIs Providing:
        1. put(key,value).
        2. get(key) -> value.
*/

use std::thread;
use std::{collections::HashMap, str::Bytes};

#[derive(Clone)]
pub struct KvStore {
    inner: Arc<Mutex<HashMap<String, String>>>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn put(&self, key: String, value: String) {
        let mut store = self.inner.lock().unwrap();
        store.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        let mut store = self.inner.lock().unwrap();
        store.get(&key).cloned()
    }
}

/// 在主线程中New一个KvStore, 开一个线程放数据,
/// 在另一个子线程中接受这些个数据, 然后输出到屏幕.
fn main() {
    let store = KvStore::new();
    let store1 = store.clone();
    thread::spawn(move || {
        store1.put("Hello".into(), "World".into());
    })
    .join()
    .unwrap();
    let store2 = store.clone();
    let handle = thread::spawn(move || {
        if let Some(x) = store2.get("Hello".into()) {
            println!("Got value {:?}", x);
        } else {
            println!("Value not found!");
        }
    });
    handle.join().unwrap()
}
