#[macro_export]
macro_rules! spawn_task {
    ({ name: $name:expr, $($fields:tt)* }, $f:expr) => {{
        use std::ffi::CString;
        use esp_idf_svc::hal::task::thread::ThreadSpawnConfiguration;

        let c_string = CString::new($name).unwrap();
        let leaked_str: &'static _ = Box::leak(c_string.into_boxed_c_str());

        ThreadSpawnConfiguration {
            name: Some(leaked_str),
            $($fields)*
            ..Default::default()
        }
        .set()
        .unwrap();

        let thread_handle = std::thread::spawn($f);

        ThreadSpawnConfiguration::default().set().unwrap();

        thread_handle
    }};
}
