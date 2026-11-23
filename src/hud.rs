// hud.rs — v27 จัด HUD ให้โล่ง: แบ่งจอเป็น 3 โซนไม่ให้ซ้อนกัน
//  ซ้าย  = สถานะ + กล่องข้อความแจ้งเตือน (กว้างไม่เกิน ~28% จอ)
//  กลาง  = แถบบทเรียน/เป้าหมาย → แถบศึก/แถบเลือดบอส → ป๊อปอัปคำแนะนำ (อยู่ระหว่างกล่องข้อความกับปุ่มมุมขวาบน)
//  ขวา   = แผนที่เล็ก + ปุ่มเมนู/แผนที่ (มือถือ)
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

static TOUCH_ON: AtomicBool = AtomicBool::new(false);
pub fn set_touch(on: bool) { TOUCH_ON.store(on, Ordering::Relaxed); }
pub fn touch_on() -> bool { TOUCH_ON.load(Ordering::Relaxed) }

#[derive(Clone, Copy, Debug)]
pub struct Layout {
    /// กล่องข้อความแจ้งเตือนซ้าย: x และความกว้างสูงสุด
    pub feed_x: f32,
    pub feed_w: f32,
    /// โซนกลางบน (x ซ้าย, ความกว้าง)
    pub col_x: f32,
    pub col_w: f32,
    /// แผนที่เล็ก
    pub mm: Rect,
}
/// คำนวณโซน HUD จากขนาดจอ · u = ui_scale · t = หน่วยปุ่มทัช · touch = มีปุ่มเมนู/แผนที่มุมขวาบน
pub fn layout_for(sw: f32, u: f32, t: f32, touch: bool) -> Layout {
    let s = 120. * u;
    let mm = Rect::new(sw - s - 10. * u, 10. * u, s, s);
    let feed_x = 10. * u;
    let feed_w = (sw * 0.28).min(380. * u).max(200. * u);
    // ปุ่มเมนู/แผนที่ (มือถือ) อยู่ซ้ายของแผนที่เล็ก: ศูนย์กลาง mm.x-36t และ mm.x-100t รัศมี 26t
    let right = if touch { mm.x - (100. + 26. + 6.) * t } else { mm.x - 10. * u };
    let left = feed_x + feed_w + 10. * u;
    let (col_x, col_w) = if right - left >= 260. * u { (left, right - left) } else {
        // จอแคบมาก: ยอมให้โซนกลางกว้างอย่างน้อย 260u ตรงกลางระหว่างขอบ
        let w = (right - 20. * u).min(260. * u).max(180. * u);
        ((right - w).max(10. * u), w)
    };
    Layout { feed_x, feed_w, col_x, col_w, mm }
}
pub fn layout() -> Layout {
    let u = ui_scale();
    let t = (screen_height() / 450.).clamp(1., 3.2) * prefs::touch_mul();
    layout_for(screen_width(), u, t, touch_on())
}
/// กึ่งกลางโซนกลาง
pub fn col_cx() -> f32 { let l = layout(); l.col_x + l.col_w / 2. }
/// ความกว้างแถบในโซนกลาง (ไม่เกิน max)
pub fn col_wd(max: f32) -> f32 { layout().col_w.min(max) }

/// ความสูงที่แถบบทเรียน/เป้าหมายใช้ไป (0 = ไม่มี)
pub fn top_banner_h(g: &Game, u: f32) -> f32 {
    if g.tut_text().is_some() { 50. * u } else if !war::in_combat(g) && top_goal(g).is_some() { 42. * u } else { 0. }
}

impl Game {
    /// ปุ่มลัดเหนือหัวชาวบ้าน: โผล่เมื่อหยุดยืนใกล้ ~0.3 วินาที (เดินผ่านในเมืองไม่เด้ง) แล้วค้างไว้จนเดินออกนอกระยะ
    pub fn quick_tick(&mut self, dt: f32) {
        if self.p.moving { self.rs.still_t = 0.; } else { self.rs.still_t += dt; }
        let near = self.npc_near_r(v19::QUICK_R);
        if near.is_none() { self.rs.quick_id = None; }
        else if self.rs.quick_id.is_some() && self.rs.quick_id != near { self.rs.quick_id = None; }
        else if self.rs.quick_id.is_none() && self.rs.still_t >= 0.3 { self.rs.quick_id = near; }
    }
}

/// คำนำหน้าข้อความ (ก่อน ": ") ไว้รวมข้อความเรื่องเดียวกันที่มาติดๆ กัน
pub fn toast_prefix(t: &str) -> Option<&str> {
    let i = t.find(": ")?;
    let p = &t[..i];
    if p.chars().count() <= 22 { Some(p) } else { None }
}
pub const TOAST_MAX: usize = 3;
pub fn toast_life(t: &str) -> f32 { (2.2 + t.chars().count() as f32 / 55.).min(7.) }

#[cfg(test)]
mod tests {
    use super::*;
    fn overlap(a: (f32, f32), b: (f32, f32)) -> bool { a.0 < b.0 + b.1 && b.0 < a.0 + a.1 }
    #[test]
    fn zones_never_overlap_on_common_screens() {
        // (กว้าง, สูง): เดสก์ท็อป · มือถือแนวนอนหลายขนาด · แท็บเล็ต
        for (sw, sh) in [(1280., 720.), (2400., 1080.), (1600., 720.), (2340., 1080.), (1920., 1200.), (1024., 600.), (2800., 1260.)] {
            for touch in [false, true] {
                let u = (f32::min(sw, sh) / 520.).clamp(1., 3.);
                let t = (sh / 450.).clamp(1., 3.2);
                let l = layout_for(sw, u, t, touch);
                let feed = (l.feed_x, l.feed_w);
                let col = (l.col_x, l.col_w);
                let mm = (l.mm.x, l.mm.w);
                assert!(!overlap(feed, col), "{}x{} touch={} กล่องข้อความทับโซนกลาง {:?} {:?}", sw, sh, touch, feed, col);
                assert!(!overlap(col, mm), "{}x{} โซนกลางทับแผนที่เล็ก", sw, sh);
                if touch {
                    // ปุ่มแผนที่ (ซ้ายสุดของกลุ่มปุ่มมุมขวาบน)
                    let map_btn = (l.mm.x - 100. * t - 26. * t, 52. * t);
                    assert!(!overlap(col, map_btn), "{}x{} โซนกลางทับปุ่มแผนที่/เมนู", sw, sh);
                }
                assert!(l.col_w >= 180. * u, "{}x{} โซนกลางกว้างพอ {}", sw, sh, l.col_w);
                assert!(l.feed_w >= 200. * u);
            }
        }
    }
    #[test]
    fn toast_merging_and_limits() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.toasts.clear();
        g.toast("ภารกิจสำเร็จ: ประชากร 4 คน (+15 เหรียญ)");
        g.toast("ภารกิจสำเร็จ: ทุกคนมีน้ำดื่ม (+10 เหรียญ)");
        g.toast("ภารกิจสำเร็จ: มีอาชีพ 3 แบบ (+10 เหรียญ)");
        assert_eq!(g.toasts.len(), 1, "เรื่องเดียวกันรวมเป็นกล่องเดียว");
        assert!(g.toasts[0].0.contains("ประชากร") && g.toasts[0].0.contains("อาชีพ 3 แบบ"));
        for i in 0..6 { g.toast(format!("ข้อความที่ {}", i)); }
        assert_eq!(g.toasts.len(), TOAST_MAX, "ค้างบนจอไม่เกิน {} กล่อง", TOAST_MAX);
        assert!(g.toasts.last().unwrap().0.contains("ที่ 5"), "เก็บอันล่าสุด");
        // ซ้ำเดิม = ต่อเวลา ไม่ซ้อน
        g.toast("ข้อความที่ 5");
        assert_eq!(g.toasts.len(), TOAST_MAX);
        assert!(toast_life("สั้น") < 3. && toast_life(&"ก".repeat(500)) <= 7.);
    }
    #[test]
    fn quick_buttons_wait_until_player_stops() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let b = (tfloor(g.p.x) + 1, tfloor(g.p.y));
        g.w.placed.insert(b, 33);
        g.npcs.push(Npc { id: 1, name: 0, look: 0, x: g.p.x + 14., y: g.p.y, state: ST_HOME, hearts: 3, mood: 60, married: false, child: false, role: 0, bed: b, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(1, 0) });
        g.p.moving = true;
        for _ in 0..30 { g.quick_tick(1. / 30.); }
        assert!(g.rs.quick_id.is_none(), "เดินผ่าน = ไม่เด้ง");
        g.p.moving = false;
        g.quick_tick(0.1);
        assert!(g.rs.quick_id.is_none());
        for _ in 0..10 { g.quick_tick(0.05); }
        assert_eq!(g.rs.quick_id, Some(1), "หยุดยืนแล้วเด้ง");
        g.p.moving = true;
        g.quick_tick(0.05);
        assert_eq!(g.rs.quick_id, Some(1), "ขยับนิดหน่อยยังค้างอยู่");
        g.p.x += 200.;
        g.quick_tick(0.05);
        assert!(g.rs.quick_id.is_none(), "ออกนอกระยะ = หาย");
    }
}
