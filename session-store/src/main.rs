// 4.3 — Mutex/RwLock, poisoning, deadlock patterns
// Exercise: Session Store

use std::{collections::HashMap, sync::Mutex, thread, time::Duration};

struct SessionStore {
    sessions: Mutex<HashMap<String, String>>,
    audit_log: Mutex<Vec<String>>,
}

impl SessionStore {
    fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            audit_log: Mutex::new(vec![]),
        }
    }
    // global lock-ordering: sessions -> audit_log
    // a mitigation to deadlock
    fn create_session(&self, id: &str, user: &str) {
        let mut inner_session = self.sessions.lock().unwrap();
        inner_session.insert(id.to_string(), user.to_string());

        // adding sleep to reproduce race condition reliably
        thread::sleep(Duration::from_millis(50));

        let mut inner_log = self.audit_log.lock().unwrap();
        inner_log.push(format!("created {id}"));
    }

    fn revoke_session_buggy(&self, id: &str) {
        let mut inner_log = self.audit_log.lock().unwrap();
        inner_log.push(format!("revoking {id}"));

        // adding sleep to reproduce race condition reliably
        thread::sleep(Duration::from_millis(50));

        let mut inner_session = self.sessions.lock().unwrap();
        inner_session.remove(id);
    }

    // global lock-ordering: sessions -> audit_log
    // a mitigation to deadlock
    fn revoke_session(&self, id: &str) {
        let mut inner_session = self.sessions.lock().unwrap();
        inner_session.remove(id);

        let mut inner_log = self.audit_log.lock().unwrap();
        inner_log.push(format!("revoking {id}"));
    }

    fn list_sessions(&self) -> Vec<String> {
        let session_guard = match self.sessions.lock() {
            Ok(sessions) => sessions,
            // recover poisoned data using PoisonError::into_inner()
            // JUSTIFICATION COMMENT TO RECOVER THE POISONED DATA:
            // It is safe to recover the poisoned data if there is no panic during mid-mutation and the
            // data is never half done.
            // the mutation methods on Sessions ie create_session() and revoke_session() are atomic as
            // there is no await points or a possible panic in between.
            // and so any usage of sessions that may panic will always atomically finish these impl
            // methods making the recovery fully safe.
            Err(e) => e.into_inner(),
        };

        session_guard.keys().cloned().collect()
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use std::sync::{Arc, mpsc};

    use super::*;
    #[test]
    fn test_deadlock() {
        let sessions = Arc::new(SessionStore::new());

        let sessions_clone1 = Arc::clone(&sessions);
        let sessions_clone2 = Arc::clone(&sessions);

        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        let _ = std::thread::spawn(move || {
            sessions_clone1.create_session("1", "alice");
            let _res = tx1.send(());
        });
        let _ = std::thread::spawn(move || {
            sessions_clone2.revoke_session_buggy("1");
            let _res = tx2.send(());
        });

        // join()-ing the 2 threads will deadlock the process indefinitely.
        // hence using channels recv_timeout as an alternative

        assert_eq!(
            rx1.recv_timeout(Duration::from_millis(400)),
            Err(mpsc::RecvTimeoutError::Timeout)
        );
        assert_eq!(
            rx2.recv_timeout(Duration::from_millis(400)),
            Err(mpsc::RecvTimeoutError::Timeout)
        );
    }
    #[test]
    fn test_no_deadlock() {
        let sessions = Arc::new(SessionStore::new());

        let sessions_clone1 = Arc::clone(&sessions);
        let sessions_clone2 = Arc::clone(&sessions);

        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        let _ = std::thread::spawn(move || {
            sessions_clone1.create_session("1", "alice");
            let _res = tx1.send(());
        });
        let _ = std::thread::spawn(move || {
            sessions_clone2.revoke_session("1");
            let _res = tx2.send(());
        });

        assert_eq!(rx1.recv_timeout(Duration::from_millis(400)), Ok(()));
        assert_eq!(rx2.recv_timeout(Duration::from_millis(400)), Ok(()));
    }

    #[test]
    fn test_panic_during_a_lock_poisons_the_data_and_its_recovery() {
        let sessions = Arc::new(SessionStore::new());

        let sessions_clone = Arc::clone(&sessions);

        let _ = std::thread::spawn(move || {
            let mut lock = sessions_clone.sessions.lock().unwrap();
            lock.insert(1.to_string(), "alice".to_string());
            panic!("this panic poisons the mutex lock");
        })
        .join();

        assert!(sessions.sessions.lock().is_err());

        // recover poisoned data
        let list = sessions.list_sessions();
        assert_eq!(list, [1.to_string()])
    }
}
