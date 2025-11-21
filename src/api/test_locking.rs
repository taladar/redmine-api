//! A helper module for conditionally acquiring locks in tests.
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Conditionally acquires a read lock for synchronous tests.
///
/// The lock is only acquired if the `REDMINE_TEST_MODE` environment variable is
/// not set to "testcontainers".
pub fn read_lock<T>(lock: &'static RwLock<T>) -> Option<RwLockReadGuard<'static, T>> {
    if std::env::var("REDMINE_TEST_MODE").unwrap_or_default() != "testcontainers" {
        Some(lock.blocking_read())
    } else {
        None
    }
}

/// Conditionally acquires a write lock for synchronous tests.
///
/// The lock is only acquired if the `REDMINE_TEST_MODE` environment variable is
/// not set to "testcontainers".
pub fn write_lock<T>(lock: &'static RwLock<T>) -> Option<RwLockWriteGuard<'static, T>> {
    if std::env::var("REDMINE_TEST_MODE").unwrap_or_default() != "testcontainers" {
        Some(lock.blocking_write())
    } else {
        None
    }
}

/// Conditionally acquires a read lock for asynchronous tests.
///
/// The lock is only acquired if the `REDMINE_TEST_MODE` environment variable is
/// not set to "testcontainers".
pub async fn read_lock_async<T>(lock: &'static RwLock<T>) -> Option<RwLockReadGuard<'static, T>> {
    if std::env::var("REDMINE_TEST_MODE").unwrap_or_default() != "testcontainers" {
        Some(lock.read().await)
    } else {
        None
    }
}

/// Conditionally acquires a write lock for asynchronous tests.
///
/// The lock is only acquired if the `REDMINE_TEST_MODE` environment variable is
/// not set to "testcontainers".
pub async fn write_lock_async<T>(lock: &'static RwLock<T>) -> Option<RwLockWriteGuard<'static, T>> {
    if std::env::var("REDMINE_TEST_MODE").unwrap_or_default() != "testcontainers" {
        Some(lock.write().await)
    } else {
        None
    }
}
