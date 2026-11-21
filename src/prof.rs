// ตัววัดเวลาต่อเฟรม (เปิดด้วย env PIXEL_PROF=1) — พิมพ์เฉพาะเฟรมที่ช้าเกิน 20 ms พร้อมส่วนที่กินเวลา
use std::cell::RefCell;
use std::time::Instant;
thread_local! { static FR: RefCell<Vec<(&'static str, f32)>> = RefCell::new(Vec::new()); }
pub fn on() -> bool { std::env::var("PIXEL_PROF").is_ok() }
pub fn t<R>(name: &'static str, f: impl FnOnce() -> R) -> R {
    let s = Instant::now();
    let r = f();
    let ms = s.elapsed().as_secs_f32() * 1000.;
    if ms > 0.5 { FR.with(|v| v.borrow_mut().push((name, ms))); }
    r
}
pub fn end_frame(total_ms: f32) {
    FR.with(|v| {
        let mut v = v.borrow_mut();
        if total_ms > 20. && on() {
            v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            let s: Vec<String> = v.iter().take(8).map(|(n, m)| format!("{}={:.1}", n, m)).collect();
            eprintln!("[prof] เฟรม {:.1} ms: {}", total_ms, s.join(" "));
        }
        v.clear();
    });
}
thread_local! { static LAST: std::cell::Cell<Option<Instant>> = std::cell::Cell::new(None); }
pub fn begin() { LAST.with(|l| l.set(Some(Instant::now()))); }
/// เวลาตั้งแต่ lap ก่อนหน้า
pub fn lap(name: &'static str) {
    LAST.with(|l| {
        let now = Instant::now();
        if let Some(p) = l.get() { let ms = (now - p).as_secs_f32() * 1000.; if ms > 0.5 { FR.with(|v| v.borrow_mut().push((name, ms))); } }
        l.set(Some(now));
    });
}
