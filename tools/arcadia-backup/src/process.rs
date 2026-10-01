use std::ops::{Deref, DerefMut};
use std::process::Child;

/// A child process killed and reaped when dropped, whatever the exit path
pub struct KillOnDrop(Child);

impl KillOnDrop {
    pub fn new(child: Child) -> Self {
        KillOnDrop(child)
    }
}

impl Deref for KillOnDrop {
    type Target = Child;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KillOnDrop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for KillOnDrop {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn kill_on_drop_reaps_the_process() {
        let child = Command::new("sleep")
            .arg("30")
            .spawn()
            .expect("cannot spawn sleep");
        let pid = child.id();
        let guard = KillOnDrop::new(child);

        // Process should be running
        assert!(
            Command::new("kill")
                .arg("-0")
                .arg(pid.to_string())
                .status()
                .map(|s| s.success())
                .unwrap_or(false),
            "process {pid} should be running"
        );

        drop(guard);

        // Process should be dead
        let is_running = Command::new("kill")
            .arg("-0")
            .arg(pid.to_string())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        assert!(!is_running, "process {pid} should be dead after drop");
    }
}
