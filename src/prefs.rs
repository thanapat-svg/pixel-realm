//! การตั้งค่าส่วนกลาง (ใช้ทั้งหน้าเมนูหลักและในเกม) เก็บในไฟล์ <save>.cfg แยกจากเซฟเกม
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering::Relaxed};

/// 0 = 30 FPS · 1 = 60 FPS · 2 = ไม่จำกัด (ตามรีเฟรชเรตจอ)
pub static FPS_MODE: AtomicU8 = AtomicU8::new(1);
pub static WEATHER_FX: AtomicBool = AtomicBool::new(true);
/// 0 = ปิด (มืดเรียบ) · 1 = ต่ำ (เซลล์ใหญ่ ไม่กระพริบ) · 2 = สูง
pub static LIGHT_Q: AtomicU8 = AtomicU8::new(2);
/// ขนาดปุ่มทัช 0 เล็ก · 1 ปกติ · 2 ใหญ่ · 3 ใหญ่มาก
pub static TOUCH_SZ: AtomicU8 = AtomicU8::new(1);
pub static SHOW_FPS: AtomicBool = AtomicBool::new(false);
pub static SHAKE: AtomicBool = AtomicBool::new(true);
/// ระบบเอาตัวรอด (หิว/กระหาย/เหนื่อย): 0 = ปิด · 1 = ปกติ (ไม่ตายเพราะหิว) · 2 = ยาก (หมดสติได้)
pub static SURVIVAL: AtomicU8 = AtomicU8::new(1);
/// ระดับความยาก 0 ง่าย · 1 ปกติ · 2 ยาก
pub static DIFF: AtomicU8 = AtomicU8::new(1);

pub fn fps_mode() -> u8 { FPS_MODE.load(Relaxed) }
pub fn weather_fx() -> bool { WEATHER_FX.load(Relaxed) }
pub fn light_q() -> u8 { LIGHT_Q.load(Relaxed) }
pub fn show_fps() -> bool { SHOW_FPS.load(Relaxed) }
pub fn shake() -> bool { SHAKE.load(Relaxed) }
pub fn survival() -> u8 { SURVIVAL.load(Relaxed) }
pub fn survival_name() -> &'static str { ["ปิด", "ปกติ", "ยาก"][survival().min(2) as usize] }
pub fn cycle_diff() { DIFF.store((DIFF.load(Relaxed) + 1) % 3, Relaxed); save(); }
pub fn cycle_survival() { SURVIVAL.store((survival() + 1) % 3, Relaxed); save(); }
pub fn touch_mul() -> f32 { [0.8, 1.0, 1.2, 1.4][(TOUCH_SZ.load(Relaxed) as usize).min(3)] }
/// เวลาต่อเฟรมเป้าหมาย (วินาที) หรือ None = ไม่จำกัด
pub fn frame_target() -> Option<f64> {
    match fps_mode() { 0 => Some(1. / 30.), 1 => Some(1. / 60.), _ => None }
}

fn path() -> String { format!("{}.cfg", crate::save_path()) }

pub fn save() {
    let s = format!(
        "fps {}\nweather {}\nlight {}\ntouch {}\nshowfps {}\nshake {}\nsurvival {}\ndiff {}\n",
        fps_mode(), weather_fx() as u8, light_q(), TOUCH_SZ.load(Relaxed), show_fps() as u8, shake() as u8, survival(), DIFF.load(Relaxed)
    );
    let _ = std::fs::write(path(), s);
}

pub fn load() {
    let Ok(txt) = std::fs::read_to_string(path()) else { return };
    for l in txt.lines() {
        let mut it = l.split_whitespace();
        let (Some(k), Some(v)) = (it.next(), it.next().and_then(|v| v.parse::<u8>().ok())) else { continue };
        match k {
            "fps" => FPS_MODE.store(v.min(2), Relaxed),
            "weather" => WEATHER_FX.store(v > 0, Relaxed),
            "light" => LIGHT_Q.store(v.min(2), Relaxed),
            "touch" => TOUCH_SZ.store(v.min(3), Relaxed),
            "showfps" => SHOW_FPS.store(v > 0, Relaxed),
            "shake" => SHAKE.store(v > 0, Relaxed),
            "survival" => SURVIVAL.store(v.min(2), Relaxed),
            "diff" => DIFF.store(v.min(2), Relaxed),
            _ => {}
        }
    }
}

pub fn fps_name() -> &'static str { ["30 FPS", "60 FPS", "ไม่จำกัด"][fps_mode().min(2) as usize] }
pub fn light_name() -> &'static str { ["ปิด", "ต่ำ", "สูง"][light_q().min(2) as usize] }
pub fn touch_name() -> &'static str { ["เล็ก", "ปกติ", "ใหญ่", "ใหญ่มาก"][(TOUCH_SZ.load(Relaxed) as usize).min(3)] }
pub fn cycle_fps() { FPS_MODE.store((fps_mode() + 1) % 3, Relaxed); save(); }
pub fn cycle_light() { LIGHT_Q.store((light_q() + 2) % 3, Relaxed); save(); } // 2→1→0→2
pub fn cycle_touch() { TOUCH_SZ.store((TOUCH_SZ.load(Relaxed) + 1) % 4, Relaxed); save(); }
pub fn toggle(a: &AtomicBool) { a.store(!a.load(Relaxed), Relaxed); save(); }

/// สรุปเซฟสำหรับหน้าเมนู: (วันที่, เลเวล)
pub fn save_summary(day_len: f32) -> Option<(i32, i32)> {
    let txt = std::fs::read_to_string(crate::save_path()).ok()?;
    let (mut day, mut lvl, mut ok) = (1, 1, false);
    for l in txt.lines() {
        let mut it = l.split_whitespace();
        match it.next() {
            Some("time") => { day = (it.next()?.parse::<f32>().ok()? / day_len) as i32 + 1; ok = true; }
            Some("p") => { lvl = it.nth(5).and_then(|v| v.parse().ok()).unwrap_or(1); }
            _ => {}
        }
    }
    if ok { Some((day, lvl)) } else { None }
}
