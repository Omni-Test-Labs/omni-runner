use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct ResourceLockManager {
    locked_devices: RwLock<HashMap<String, String>>,
}

impl ResourceLockManager {
    pub fn new() -> Self {
        Self {
            locked_devices: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for ResourceLockManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceLockManager {
    pub async fn acquire_lock(&self, device_id: &str, task_id: &str) -> Result<bool, String> {
        let mut locks = self.locked_devices.write().await;

        if let Some(owner) = locks.get(device_id) {
            if owner == task_id {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            locks.insert(device_id.to_string(), task_id.to_string());
            Ok(true)
        }
    }

    pub async fn release_lock(&self, device_id: &str, task_id: &str) -> Result<(), String> {
        let mut locks = self.locked_devices.write().await;

        if let Some(owner) = locks.get(device_id) {
            if owner == task_id {
                locks.remove(device_id);
                Ok(())
            } else {
                Err(format!(
                    "Cannot release lock: device {} is owned by {}, not {}",
                    device_id, owner, task_id
                ))
            }
        } else {
            Err(format!(
                "Cannot release lock: device {} is not locked",
                device_id
            ))
        }
    }

    pub async fn is_locked(&self, device_id: &str) -> bool {
        let locks = self.locked_devices.read().await;
        locks.contains_key(device_id)
    }

    pub async fn get_lock_owner(&self, device_id: &str) -> Option<String> {
        let locks = self.locked_devices.read().await;
        locks.get(device_id).cloned()
    }

    pub async fn clear_all(&self) {
        let mut locks = self.locked_devices.write().await;
        locks.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_acquire_lock_success() {
        let manager = ResourceLockManager::new();
        let result = manager.acquire_lock("device1", "task1").await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_acquire_lock_already_locked() {
        let manager = ResourceLockManager::new();
        manager.acquire_lock("device1", "task1").await.unwrap();

        let result = manager.acquire_lock("device1", "task2").await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_release_lock_success() {
        let manager = ResourceLockManager::new();
        manager.acquire_lock("device1", "task1").await.unwrap();

        let result = manager.release_lock("device1", "task1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_release_lock_not_owner() {
        let manager = ResourceLockManager::new();
        manager.acquire_lock("device1", "task1").await.unwrap();

        let result = manager.release_lock("device1", "task2").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_release_lock_not_locked() {
        let manager = ResourceLockManager::new();

        let result = manager.release_lock("device1", "task1").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_locked() {
        let manager = ResourceLockManager::new();
        assert!(!manager.is_locked("device1").await);

        manager.acquire_lock("device1", "task1").await.unwrap();
        assert!(manager.is_locked("device1").await);

        manager.release_lock("device1", "task1").await.unwrap();
        assert!(!manager.is_locked("device1").await);
    }

    #[tokio::test]
    async fn test_get_lock_owner() {
        let manager = ResourceLockManager::new();
        assert!(manager.get_lock_owner("device1").await.is_none());

        manager.acquire_lock("device1", "task1").await.unwrap();
        let owner = manager.get_lock_owner("device1").await;
        assert_eq!(owner, Some("task1".to_string()));

        manager.release_lock("device1", "task1").await.unwrap();
        assert!(manager.get_lock_owner("device1").await.is_none());
    }

    #[tokio::test]
    async fn test_clear_all() {
        let manager = ResourceLockManager::new();
        manager.acquire_lock("device1", "task1").await.unwrap();
        manager.acquire_lock("device2", "task2").await.unwrap();

        assert!(manager.is_locked("device1").await);
        assert!(manager.is_locked("device2").await);

        manager.clear_all().await;

        assert!(!manager.is_locked("device1").await);
        assert!(!manager.is_locked("device2").await);
    }
}
