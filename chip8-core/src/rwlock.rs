use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::error::Chip8Error;

pub trait CheckedRead<T> {
    fn checked_read(&self) -> Result<RwLockReadGuard<'_, T>, Chip8Error>;
}

impl<T> CheckedRead<T> for RwLock<T> {
    fn checked_read(&self) -> Result<RwLockReadGuard<'_, T>, Chip8Error> {
        self.read()
            .map_err(|e| Chip8Error::MutexReadError(e.to_string()))
    }
}

impl<T> CheckedRead<T> for Arc<RwLock<T>> {
    fn checked_read(&self) -> Result<RwLockReadGuard<'_, T>, Chip8Error> {
        self.read()
            .map_err(|e| Chip8Error::MutexReadError(e.to_string()))
    }
}

pub trait CheckedWrite<T> {
    fn checked_write(&self) -> Result<RwLockWriteGuard<'_, T>, Chip8Error>;
}

impl<T> CheckedWrite<T> for RwLock<T> {
    fn checked_write(&self) -> Result<RwLockWriteGuard<'_, T>, Chip8Error> {
        self.write()
            .map_err(|e| Chip8Error::MutexWriteError(e.to_string()))
    }
}

impl<T> CheckedWrite<T> for Arc<RwLock<T>> {
    fn checked_write(&self) -> Result<RwLockWriteGuard<'_, T>, Chip8Error> {
        self.write()
            .map_err(|e| Chip8Error::MutexWriteError(e.to_string()))
    }
}
