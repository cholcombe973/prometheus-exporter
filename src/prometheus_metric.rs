use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PrometheusMetric {
    name: String,
    // value: PrometheusValue,
    callback: Option<Box<fn() -> PrometheusValue>>,
    features: HashMap<String, String>,
}

impl PrometheusMetric {
    pub fn new<T: Into<String>>(name: T) -> Self {
        PrometheusMetric {
            name: name.into(),
            callback: None,
            features: HashMap::new(),
        }
    }

    pub fn with_callback(mut self, callback: fn() -> PrometheusValue) -> Self {
        self.callback = Some(Box::new(callback));
        self
    }

    pub fn with_feature<T1: Into<String>, T2: Into<String>>(mut self, name: T1, value: T2) -> Self {
        self.features.insert(name.into(), value.into());
        self
    }

    pub fn to_string(&self) -> String {
        let mut s = self.name.clone();
        if let Some(ref callback) = self.callback {
            let mut features = String::new();
            let mut written = false;
            for (ref key, ref value) in &self.features {
                if written {
                    features.push_str(",");
                }
                features.push_str(&format!("{key}={value}", key=key, value=value));
                written = true;
            }
            if written {
                s.push_str(&format!("{{{}}}", features));
            }
            s.push_str(" ");

            s.push_str( &callback().to_string() );
        }
        s
    }
}

#[derive(Clone, Debug)]
pub enum PrometheusValue {
    String(String),
    Float(f64),
}

impl PrometheusValue {
    pub fn to_string(&self) -> String {
        match self {
            &PrometheusValue::String(ref s) => s.clone(),
            &PrometheusValue::Float(f) => format!("{:e}", f),
        }
    }
}
