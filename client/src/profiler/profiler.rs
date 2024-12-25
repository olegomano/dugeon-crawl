#![feature(lazy_get)]
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::ops::DerefMut;
use std::sync::{LazyLock, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Scope {
    timestamp: SystemTime,
    name: String,
}

#[derive(Debug)]
struct ScopeStats {
    call_count: u64,
    total_time_ms: u64,
    min_per_call_time_ms: u64,
    max_per_call_time_ms: u64,
}

impl ScopeStats {
    fn default() -> Self {
        return Self {
            call_count: 0,
            total_time_ms: 0,
            min_per_call_time_ms: 0xFFFFFFFFFFFFFFFF,
            max_per_call_time_ms: 0,
        };
    }
}

pub struct Profiler {
    scope_stack: Vec<Scope>,
    stats: HashMap<String, ScopeStats>,
}

impl Profiler {
    pub fn Instance<T>(f: T)
    where
        T: FnOnce(&mut Profiler),
    {
        static mut INSTANCE: *mut Profiler = std::ptr::null_mut();
        unsafe {
            if INSTANCE.is_null() {
                INSTANCE = Box::into_raw(Box::new(Profiler {
                    scope_stack: Vec::new(),
                    stats: HashMap::new(),
                }));
            }
            f(&mut *INSTANCE)
        }
    }

    pub fn OpenScope(&mut self, name: &str) {
        let scope = Scope {
            name: name.to_string(),
            timestamp: SystemTime::now(),
        };
        self.scope_stack.push(scope);
        self.stats
            .entry(name.to_string())
            .or_insert(ScopeStats::default());
    }

    pub fn CloseScope(&mut self) {
        if let Some(scope) = self.scope_stack.pop() {
            let delta = SystemTime::now().duration_since(scope.timestamp).expect("");
            let delta_ms = delta.as_millis() as u64;
            let mut stats: &mut ScopeStats = self.stats.get_mut(&scope.name).expect("");
            stats.call_count += 1;
            stats.total_time_ms += delta_ms;
            if delta_ms > stats.max_per_call_time_ms {
                stats.max_per_call_time_ms = delta_ms;
            }
            if delta_ms < stats.min_per_call_time_ms {
                stats.min_per_call_time_ms = delta_ms;
            }
        }
    }

    pub fn DumpStats(&self) -> String {
        return self
            .stats
            .iter()
            .map(|(key, value)| format!("{}:{:?}", key, value))
            .collect::<Vec<_>>()
            .join("\n");
    }
}

#[macro_export]
macro_rules! open_scope {
    ($name:expr) => {
        profiler::Profiler::Instance(|instance| {
            instance.OpenScope($name);
        });
    };
}

#[macro_export]
macro_rules! close_scope {
    () => {
        profiler::Profiler::Instance(|instance| {
            instance.CloseScope();
        });
    };
}

#[macro_export]
macro_rules! print_perf {
    () => {
        profiler::Profiler::Instance(|instance| {
            println!("{}", instance.DumpStats());
        });
    };
}
