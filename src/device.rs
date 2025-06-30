use std::sync::{Arc, Mutex};

pub struct BlockDevice {
    storage: Arc<Mutex<Vec<u8>>>,
    size: usize,
}

impl BlockDevice {
    pub fn new(size: usize) -> Self {
        BlockDevice {
            storage: Arc::new(Mutex::new(vec![0u8; size])),
            size,
        }
    }

    pub fn read(&self, offset: usize, buf: &mut [u8]) -> Result<(), String> {
        let storage = self.storage.lock().unwrap();
        if offset + buf.len() > self.size {
            return Err("Read out of bounds".into());
        }
        buf.copy_from_slice(&storage[offset..offset + buf.len()]);
        Ok(())
    }

    pub fn write(&self, offset: usize, data: &[u8]) -> Result<(), String> {
        let mut storage = self.storage.lock().unwrap();
        if offset + data.len() > self.size {
            return Err("Write out of bound".into());
        }
        storage[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }
}
