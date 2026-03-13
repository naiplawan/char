use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, Instant, SystemTime};

use crate::{DetectCallback, DetectEvent};

pub struct SleepDetector {
    running: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
}

impl Default for SleepDetector {
    fn default() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }
}

impl crate::Observer for SleepDetector {
    fn start(&mut self, f: DetectCallback) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }

        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();

        self.thread_handle = Some(std::thread::spawn(move || {
            platform_sleep_watch(running, f);
        }));
    }

    fn stop(&mut self) {
        if !self.running.load(Ordering::SeqCst) {
            return;
        }

        self.running.store(false, Ordering::SeqCst);

        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

#[cfg(target_os = "macos")]
fn platform_sleep_watch(running: Arc<AtomicBool>, f: DetectCallback) {
    use block2::RcBlock;
    use objc2::{msg_send, rc::Retained};
    use objc2_app_kit::NSWorkspace;
    use objc2_foundation::{NSNotification, NSNotificationCenter, NSObject, NSString};

    struct SleepObserver {
        center: Retained<NSNotificationCenter>,
        will_sleep_observer: Retained<NSObject>,
        did_wake_observer: Retained<NSObject>,
    }

    impl Drop for SleepObserver {
        fn drop(&mut self) {
            unsafe {
                let _: () = msg_send![&*self.center, removeObserver: &*self.will_sleep_observer];
                let _: () = msg_send![&*self.center, removeObserver: &*self.did_wake_observer];
            }
        }
    }

    let will_sleep_callback = f.clone();
    let will_sleep_block = RcBlock::new(move |_notification: *const NSNotification| {
        will_sleep_callback(DetectEvent::SleepStateChanged { value: true });
    });

    let did_wake_callback = f.clone();
    let did_wake_block = RcBlock::new(move |_notification: *const NSNotification| {
        did_wake_callback(DetectEvent::SleepStateChanged { value: false });
    });

    let _observer = unsafe {
        let workspace = NSWorkspace::sharedWorkspace();
        let center = workspace.notificationCenter();

        let will_sleep_name = NSString::from_str("NSWorkspaceWillSleepNotification");
        let did_wake_name = NSString::from_str("NSWorkspaceDidWakeNotification");

        let will_sleep_observer: Retained<NSObject> = msg_send![
            &*center,
            addObserverForName: &*will_sleep_name,
            object: std::ptr::null::<NSObject>(),
            queue: std::ptr::null::<NSObject>(),
            usingBlock: &*will_sleep_block
        ];

        let did_wake_observer: Retained<NSObject> = msg_send![
            &*center,
            addObserverForName: &*did_wake_name,
            object: std::ptr::null::<NSObject>(),
            queue: std::ptr::null::<NSObject>(),
            usingBlock: &*did_wake_block
        ];

        SleepObserver {
            center,
            will_sleep_observer,
            did_wake_observer,
        }
    };

    while running.load(Ordering::SeqCst) {
        std::thread::sleep(Duration::from_millis(500));
    }
}

// Linux and Windows: detect sleep/wake by comparing monotonic vs wall clock.
// When a machine sleeps, Instant (monotonic) pauses while SystemTime advances.
// A gap larger than 2× the poll interval indicates a wake from sleep.
#[cfg(not(target_os = "macos"))]
fn platform_sleep_watch(running: Arc<AtomicBool>, f: DetectCallback) {
    const POLL: Duration = Duration::from_secs(2);
    const WAKE_THRESHOLD: Duration = Duration::from_secs(5);

    let mut last_instant = Instant::now();
    let mut last_sys = SystemTime::now();
    let mut asleep = false;

    while running.load(Ordering::SeqCst) {
        std::thread::sleep(POLL);

        let now_instant = Instant::now();
        let now_sys = SystemTime::now();

        let mono_elapsed = now_instant.duration_since(last_instant);
        let wall_elapsed = now_sys.duration_since(last_sys).unwrap_or(Duration::ZERO);

        // Wall clock advanced significantly more than monotonic → woke from sleep
        if wall_elapsed > mono_elapsed + WAKE_THRESHOLD {
            if asleep {
                asleep = false;
                f(DetectEvent::SleepStateChanged { value: false });
            }
        } else if mono_elapsed < POLL / 2 && !asleep {
            // Monotonic barely advanced → about to sleep (rare to catch, best-effort)
            asleep = true;
            f(DetectEvent::SleepStateChanged { value: true });
        }

        last_instant = now_instant;
        last_sys = now_sys;
    }
}
