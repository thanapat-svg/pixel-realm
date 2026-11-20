//! บทเรียนเริ่มเกม (6 ข้อ + ลูกศรชี้เป้า) และข้อความบนปุ่ม "ใช้" ตามบริบท
use super::*;

pub const TUT_N: u8 = 6;
pub const TUT_DONE: u8 = 99;
/// ตัวนับเหตุการณ์: 0 คราฟต์ · 1 วางกองไฟ · 2 ฆ่ามอนสเตอร์ · 3 วางสิ่งปลูกสร้าง
pub const EV_N: usize = 4;

impl Game {
    pub fn tut_text(&self) -> Option<(String, &'static str)> {
        let t = match self.tut {
            0 => ("ตีต้นไม้ 3 ครั้ง เก็บไม้ให้ได้ 3 ชิ้น", "แตะปุ่มดาบหันหน้าเข้าต้นไม้ (ลูกศรชี้ต้นที่ใกล้สุด)"),
            1 => ("ตีก้อนหินเทาให้ได้หิน 3 ก้อน", "หินเทาตามพื้นหญ้า/ภูเขา (ลูกศรชี้ให้)"),
            2 => ("คราฟต์ของสักชิ้น", "กดปุ่มเมนู → คราฟต์ → เลือกสิ่งที่ราคาพอ"),
            3 => ("วางกองไฟ 1 กอง", "คราฟต์กองไฟก่อน (ไม้+หิน) แล้วกดปุ่ม \"ไฟ\""),
            4 => ("ปราบมอนสเตอร์ 1 ตัว", "ตีสไลม์ในทุ่งหญ้า — ดาเมจเด้งเลขเหลือง"),
            5 => ("สร้างอะไรสักอย่างในฐานของคุณ", "เมนู → สร้าง → เลือกชิ้น → ปุ่มสร้างเพื่อวาง"),
            _ => return None,
        };
        Some((t.0.to_string(), t.1))
    }

    fn tut_set(&mut self, n: u8) {
        self.tut = n;
        self.tut_ev0 = self.ev;
        self.tut_flash = 2.5;
    }

    /// เช็กเงื่อนไขของข้อปัจจุบัน แล้วเลื่อนข้อ (เรียกทุกเฟรม ราคาถูก)
    pub fn tut_update(&mut self, dt: f32) {
        if self.tut_flash > 0. { self.tut_flash -= dt; }
        if self.tut >= TUT_N { return; }
        let done = match self.tut {
            0 => self.p.inv[WOOD] >= 3,
            1 => self.p.inv[STONE] >= 3,
            2 => self.ev[0] > self.tut_ev0[0],
            3 => self.ev[1] > self.tut_ev0[1],
            4 => self.ev[2] > self.tut_ev0[2],
            _ => self.ev[3] > self.tut_ev0[3],
        };
        if !done { return; }
        play(&self.sfx.quest);
        if self.tut + 1 >= TUT_N {
            self.tut = TUT_DONE;
            self.p.inv[COIN] += 30;
            self.toast("จบบทเรียน! ได้ 30 เหรียญ — ออกสำรวจอิสระได้เลย");
            self.save();
        } else {
            let n = self.tut + 1;
            self.tut_set(n);
            self.toast(format!("ผ่านข้อ {}/{} สำเร็จ!", n, TUT_N));
            self.save();
        }
    }

    /// ตำแหน่งเป้าของข้อที่ต้องหาในโลก (ต้นไม้/หินใกล้สุดในรัศมี 18 ช่อง)
    pub fn tut_target(&self) -> Option<(f32, f32)> {
        let want: &[u8] = match self.tut { 0 => &[1, 8, 40, 41, 42, 43, 45, 46], 1 => &[4], _ => return None };
        if self.w.dun.is_some() { return None; }
        let (ptx, pty) = (tfloor(self.p.x), tfloor(self.p.y));
        let mut best: Option<(i32, (i32, i32))> = None;
        for ty in pty - 18..=pty + 18 {
            for tx in ptx - 18..=ptx + 18 {
                let o = self.w.obj_at(tx, ty);
                if !want.contains(&o) { continue; }
                let d = (tx - ptx).abs() + (ty - pty).abs();
                if best.map_or(true, |b| d < b.0) { best = Some((d, (tx, ty))); }
            }
        }
        best.map(|b| (b.1 .0 as f32 * 16. + 8., b.1 .1 as f32 * 16. + 8.))
    }

    /// ข้อความสั้นบนปุ่ม "ใช้" ตามสิ่งที่อยู่ตรงหน้า (None = ไม่มีอะไรให้ใช้)
    pub fn use_label(&self) -> Option<String> {
        if let Some(f) = &self.fish { return Some(if f.bite_t > 0. { "ดึง!" } else { "รอ..." }.to_string()); }
        if self.lost_near().is_some() { return Some("ช่วย".to_string()); }
        if self.npc_near().is_some() { return Some("คุย".to_string()); }
        if let Some((_, o)) = self.interact_target() {
            return Some(match o {
                12 => "เปิด", 13 => "ปิด", 14 => "กล่อง", 17 => "ตีบวก", 25 => "วาร์ป", 26 => "ทำอาหาร",
                19 | 36 => "เข้า", 20 => "ออก", 33 => "นอน", 39 => "ดื่ม", 41 => "เก็บผล", 42 => "ดื่มมะพร้าว", 34 => "โต๊ะ", 24 => "เปิดหีบ", O_ALTAR => "ขอพร", O_GIANT => "เก็บของ", _ => "คุย",
            }.to_string());
        }
        if let Some(h) = self.farm_hint(true) {
            let s = h.split(": ").nth(1).unwrap_or("ใช้");
            return Some(if s.starts_with("ปลูก") { "ปลูก".to_string() } else if s.starts_with("เก็บ") { "เก็บ".to_string() } else if s.starts_with("เติม") { "เติมน้ำ".to_string() } else { s.split(|c| c == ' ' || c == '(').next().unwrap_or("ใช้").to_string() });
        }
        if self.water_ahead().is_some() { return Some(if self.survival_mode() > 0 && self.p.thirst < 90. { "ดื่มน้ำ" } else { "ตกปลา" }.to_string()); }
        None
    }
}

/// แผงบทเรียนด้านบนกลางจอ + ลูกศรชี้เป้าในโลก
pub fn draw_tut(g: &Game, font: &Font, v: &View, u: f32) {
    let Some((title, sub)) = g.tut_text() else { return };
    let sw = screen_width();
    let w = (440. * u).min(sw * 0.38);
    let h = 50. * u;
    let x = sw / 2. - w / 2.;
    let y = 8. * u;
    draw_rectangle(x, y, w, h, Color::new(0.05, 0.07, 0.12, 0.8));
    draw_rectangle_lines(x, y, w, h, 2., if g.tut_flash > 0. { hx(0xffee66) } else { hx(0x66aaff) });
    text(font, &format!("บทเรียน {}/{}", g.tut as u32 + 1, TUT_N), x + 8. * u, y + 14. * u, 11. * u, hx(0x88bbff));
    text_c(font, &title, sw / 2., y + 30. * u, 15. * u, hx(0xffffff));
    text_c(font, sub, sw / 2., y + 44. * u, 10. * u, hx(0xaaaaaa));
    if let Some((tx, ty)) = g.tut_target() {
        let (sx, sy) = ((tx - v.cx) * v.s, (ty - v.cy) * v.s);
        let (sw_, sh_) = (screen_width(), screen_height());
        let b = (get_time() as f32 * 6.).sin() * 5. * u;
        let col = Color::new(1., 0.9, 0.2, 0.95);
        if sx > 20. && sx < sw_ - 20. && sy > 20. && sy < sh_ - 20. {
            let (ax, ay) = (sx, sy - 28. * u + b);
            draw_triangle(vec2(ax, ay + 12. * u), vec2(ax - 10. * u, ay - 4. * u), vec2(ax + 10. * u, ay - 4. * u), col);
        } else {
            let c = vec2(sw_ / 2., sh_ / 2.);
            let d = (vec2(sx, sy) - c).normalize_or_zero();
            let k = ((sw_ / 2. - 40. * u) / d.x.abs().max(0.001)).min((sh_ / 2. - 40. * u) / d.y.abs().max(0.001));
            let p = c + d * k;
            let n = vec2(-d.y, d.x);
            draw_triangle(p + d * 16. * u, p - d * 8. * u + n * 11. * u, p - d * 8. * u - n * 11. * u, col);
        }
    }
}
