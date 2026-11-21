// saves.rs — v12: ช่องเซฟ 3 ช่อง · เซฟสำรองอัตโนมัติ · ส่งออก/นำเข้าเซฟเป็นไฟล์ · เขียนเซฟแบบปลอดภัย (atomic)
use super::*;

pub const SLOTS: u8 = 3;

/// ตำแหน่งเซฟของช่องที่เลือก (ช่อง 1 = ไฟล์เดิม เพื่อให้เซฟเก่าใช้ได้ต่อ)
pub fn slot_path(slot: u8) -> String {
    let base = save_base();
    if slot == 0 { return base; }
    match base.strip_suffix(".txt") { Some(b) => format!("{}_{}.txt", b, slot + 1), None => format!("{}_{}", base, slot + 1) }
}
/// เขียนไฟล์แบบปลอดภัย: เขียน .tmp ก่อนแล้วค่อยเปลี่ยนชื่อ (ไฟดับ/แอปถูกปิดกลางคันเซฟไม่พัง)
pub fn write_atomic(path: &str, data: &str) -> std::io::Result<()> {
    let tmp = format!("{}.tmp", path);
    std::fs::write(&tmp, data)?;
    std::fs::rename(&tmp, path)
}
/// เซฟสำรองอัตโนมัติ: หมุนเวียน 3 ชุด (.bak1 ใหม่สุด) — เรียกตอนเริ่มเล่นแต่ละครั้ง
pub fn rotate_backups(path: &str) {
    if std::fs::metadata(path).is_err() { return; }
    let b = |i: u8| format!("{}.bak{}", path, i);
    let _ = std::fs::rename(b(2), b(3));
    let _ = std::fs::rename(b(1), b(2));
    let _ = std::fs::copy(path, b(1));
}
pub fn backup_count(path: &str) -> usize { (1..=3).filter(|i| std::fs::metadata(format!("{}.bak{}", path, i)).is_ok()).count() }
/// กู้คืนเซฟสำรองล่าสุด (เซฟปัจจุบันเก็บไว้เป็น .before_restore)
pub fn restore_backup(path: &str) -> Result<(), String> {
    let b1 = format!("{}.bak1", path);
    let txt = std::fs::read_to_string(&b1).map_err(|_| "ไม่มีเซฟสำรอง".to_string())?;
    if !txt.lines().any(|l| l.starts_with("time ")) { return Err("เซฟสำรองเสีย".into()); }
    let _ = std::fs::copy(path, format!("{}.before_restore", path));
    write_atomic(path, &txt).map_err(|e| e.to_string())
}

/// โฟลเดอร์สำหรับส่งออก: มือถือ = Download/PixelRealm · คอมพิวเตอร์ = โฟลเดอร์ exports ข้างเกม
pub fn export_dir() -> String {
    if let Ok(p) = std::env::var("PIXEL_REALM_EXPORT") { return p; }
    #[cfg(target_os = "android")]
    { return "/storage/emulated/0/Download/PixelRealm".to_string(); }
    #[allow(unreachable_code)]
    "exports".to_string()
}
pub fn export_save(path: &str, slot: u8, stamp: &str) -> Result<String, String> {
    let txt = std::fs::read_to_string(path).map_err(|_| "ช่องนี้ยังไม่มีเซฟ".to_string())?;
    let dir = export_dir();
    std::fs::create_dir_all(&dir).map_err(|e| format!("สร้างโฟลเดอร์ไม่ได้: {}", e))?;
    let out = format!("{}/pixel_realm_slot{}_{}.txt", dir, slot + 1, stamp);
    std::fs::write(&out, txt).map_err(|e| format!("เขียนไฟล์ไม่ได้: {}", e))?;
    Ok(out)
}
/// นำเข้าไฟล์เซฟล่าสุดในโฟลเดอร์ส่งออก (เซฟเดิมของช่องนี้ถูกเก็บเป็นเซฟสำรองก่อน)
pub fn import_latest(path: &str) -> Result<String, String> {
    let dir = export_dir();
    let rd = std::fs::read_dir(&dir).map_err(|_| format!("ไม่พบโฟลเดอร์ {}", dir))?;
    let mut files: Vec<(std::time::SystemTime, std::path::PathBuf)> = rd.filter_map(|e| e.ok()).filter(|e| e.file_name().to_string_lossy().starts_with("pixel_realm_") && e.file_name().to_string_lossy().ends_with(".txt")).filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path()))).collect();
    files.sort();
    let (_, f) = files.pop().ok_or_else(|| "ไม่มีไฟล์เซฟในโฟลเดอร์".to_string())?;
    let txt = std::fs::read_to_string(&f).map_err(|_| "อ่านไฟล์ไม่ได้ (Android อาจไม่ให้อ่านไฟล์จากการติดตั้งครั้งก่อน)".to_string())?;
    if !txt.lines().any(|l| l.starts_with("time ")) { return Err("ไฟล์นี้ไม่ใช่เซฟของเกม".into()); }
    rotate_backups(path);
    write_atomic(path, &txt).map_err(|e| e.to_string())?;
    Ok(f.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default())
}
/// วันเวลาแบบสั้นสำหรับชื่อไฟล์ (UTC+7)
pub fn stamp_now() -> String {
    let s = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0) + 7 * 3600;
    let (days, rem) = (s.div_euclid(86400), s.rem_euclid(86400));
    // แปลงวันเป็นปี/เดือน/วัน (Howard Hinnant)
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    format!("{:04}{:02}{:02}-{:02}{:02}", y, m, d, rem / 3600, rem % 3600 / 60)
}
