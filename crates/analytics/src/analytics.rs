use chrono::{DateTime, Duration, Utc};
use derive_builder::Builder;
// use anyhow::Result;
// use http::{Method, StatusCode};
// use spin_sdk::{
//     http::{Request, Response},
//     http_component,
//     key_value::{Error, Store},
// };

/// Represents a Record
#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
pub struct Record {
    #[builder(default)]
    pub trigger_type: String,

    #[builder(default)]
    pub component_id: String,

    #[builder(default)]
    pub path: String,

    #[builder(default)]
    pub execution_status: String,

    #[builder(default)]
    pub http_status_code: u16,

    #[builder(default)]
    pub start_time: DateTime<Utc>,

    #[builder(default = "Duration::zero()")]
    execution_time: Duration,
}

impl Record {
    pub fn set_component_id(&mut self, s: String) -> &mut Record {
        self.component_id = s;
        self
    }

    pub fn set_trigger_type(&mut self, s: String) -> &mut Record {
        self.trigger_type = s;
        self
    }

    pub fn set_path(&mut self, s: String) -> &mut Record {
        self.path = s;
        self
    }

    pub fn start_recording(&mut self) -> &mut Record {
        self.start_time = Utc::now();
        self
    }

    pub fn set_http_status_code(&mut self, c: u16) -> &mut Record {
        self.http_status_code = c;
        self
    }

    pub fn set_execution_status(&mut self, s: String) -> &mut Record {
        self.execution_status = s;
        self
    }

    fn set_execution_time(&mut self, s: Duration) -> &mut Record {
        self.execution_time = s;
        self
    }
}

impl Drop for Record {
    fn drop(&mut self) {
        self.set_execution_time(Utc::now() - self.start_time);
        println!("inside drop {:?}", self);
        // let store = Store::open_default()?;
        // store.set("first-key", "first-value")?;
        // println!("inside drop")
    }
}
