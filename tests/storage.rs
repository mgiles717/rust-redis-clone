use redis_clone::storage::Storage;

fn setup() -> Storage {
    return Storage::new()
}

#[test]
fn test_get_key() {
    let mut storage = setup();
    storage.set("test_key", "test_value");

    assert_eq!(storage.get("test_key"), Some(&"test_value".to_string()));
}

#[test]
fn test_get_key_empty() {
    let storage = setup();
    assert_eq!(storage.get("test_key"), None);
}

#[test]
fn test_set_key() {
    let mut storage = setup();
    assert_eq!(storage.get("test_key"), None);

    storage.set("test_key", "test_value");

    assert_eq!(storage.get("test_key"), Some(&"test_value".to_string()));
}

#[test]
fn test_set_key_override() {
    let mut storage = setup();
    assert_eq!(storage.get("test_key"), None);

    storage.set("test_key", "test_value");

    assert_eq!(storage.get("test_key"), Some(&"test_value".to_string()));
    
    storage.set("test_key", "overriden_value");

    assert_eq!(storage.get("test_key"), Some(&"overriden_value".to_string()));
}


