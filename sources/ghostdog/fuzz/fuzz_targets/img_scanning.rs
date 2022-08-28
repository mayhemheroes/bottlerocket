#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Prevent known panic in gptman-1.0.0 with inputs smaller than a sector size
    if data.len() < 512 {
        return;
    }

    let mut data = std::io::Cursor::new(data);
    let _ = ghostdog::find_device_type(&mut data);
});
