// nav.rs — v16: หาทางกลับบ้าน
//  - สัญลักษณ์บ้านบนแผนที่เล็ก/ใหญ่ = เตียงที่นอนล่าสุด (อยู่ในห้องที่ล้อมครบ = รูปบ้านสีส้ม · ไม่มีหลังคา = รูปเตียงสีเทา)
//  - บ้าน/หมุดอยู่นอกขอบแผนที่ → ลูกศรที่ขอบชี้ทิศ + ระยะ (ช่อง)
//  - แตะบ้านบนแผนที่ใหญ่ = วาร์ปกลับบ้าน (ฟรี รอใช้ซ้ำ 60 วิ · ในดันเจี้ยนใช้ไม่ได้)
//  - แตะที่ว่างบนแผนที่ใหญ่ = ปักหมุด (สูงสุด 8) + นำทางไปหมุดนั้น · แตะหมุดที่ไม่ได้นำทาง = นำทาง · แตะหมุดที่กำลังนำทาง = ลบ
//  - กำลังนำทาง: ลูกศรรอบตัวผู้เล่นชี้ไปหมุด + ระยะ · ถึงหมุดแล้วเลิกนำทางเอง
use super::*;

pub const PIN_MAX: usize = 8;
pub const HOME_CD: f32 = 60.;

impl Game {
    /// บ้าน = เตียงที่นอนล่าสุด (ถ้ายังอยู่) → (ช่อง, อยู่ในห้องปิด?)
    pub fn home_tile(&self) -> Option<((i32, i32), bool)> {
        let b = self.bed?;
        if self.w.placed.get(&b) != Some(&33) { return None; }
        Some((b, self.w.room_from(b).is_some()))
    }
    pub fn warp_home(&mut self) -> bool {
        if self.w.dun.is_some() { self.toast("วาร์ปในดันเจี้ยนไม่ได้"); return false; }
        if self.home_tile().is_none() { self.toast("ยังไม่มีบ้าน — วางเตียงแล้วนอนสักคืน เตียงนั้นจะเป็นบ้าน"); return false; }
        if self.home_cd > 0. { self.toast(format!("วาร์ปกลับบ้านได้อีกใน {} วินาที", self.home_cd.ceil() as i32)); return false; }
        let (nx, ny) = self.respawn_pt();
        let (ox, oy) = (self.p.x, self.p.y - 8.);
        self.burst(ox, oy, hx(0xffc060), 24, 90.);
        self.p.x = nx;
        self.p.y = ny;
        self.sit = None;
        self.en.clear();
        self.proj.clear();
        self.drops.clear();
        self.tp_comp();
        self.p.invul = self.p.invul.max(1.0);
        self.burst(nx, ny - 8., hx(0xffe0a0), 28, 110.);
        self.shake(0.6, 0.2);
        play(&self.sfx.upgrade);
        self.home_cd = HOME_CD;
        self.mini_t = 99.;
        self.toast("วาร์ปกลับบ้านแล้ว");
        true
    }
    pub fn nav_tick(&mut self, dt: f32) {
        if self.home_cd > 0. { self.home_cd -= dt; }
        if let Some(i) = self.pin_on {
            let Some(&(tx, ty)) = self.pins.get(i) else { self.pin_on = None; return };
            if self.w.dun.is_none() && (tx as f32 * 16. + 8. - self.p.x).hypot(ty as f32 * 16. + 8. - self.p.y) < 3. * 16. {
                self.pin_on = None;
                self.toast(format!("ถึงหมุด {} แล้ว", i + 1));
                play(&self.sfx.pickup);
            }
        }
    }
    /// แตะบนแผนที่ใหญ่ (หลังเช็กเสาวาร์ปแล้ว) → true ถ้าใช้การแตะนี้
    pub fn map_tap(&mut self, tp: Vec2, mr: Rect, ctr: (i32, i32), sc: i32, rr: f32) -> bool {
        if self.w.dun.is_some() || !mr.contains(tp) { return false; }
        if let Some((h, _)) = self.home_tile() {
            let (mx, my, ok) = map_pt(h.0, h.1, ctr, sc, mr);
            if ok && (tp.x - mx).hypot(tp.y - my) < rr * 1.5 {
                if self.warp_home() { self.big_map = false; }
                return true;
            }
        }
        for i in 0..self.pins.len() {
            let (mx, my, ok) = map_pt(self.pins[i].0, self.pins[i].1, ctr, sc, mr);
            if ok && (tp.x - mx).hypot(tp.y - my) < rr * 1.5 {
                if self.pin_on == Some(i) {
                    self.pins.remove(i);
                    self.pin_on = None;
                    self.toast("ลบหมุดแล้ว");
                } else {
                    self.pin_on = Some(i);
                    self.toast(format!("นำทางไปหมุด {} — ดูลูกศรรอบตัว", i + 1));
                }
                self.save();
                return true;
            }
        }
        // ที่ว่าง → ปักหมุดใหม่ (เต็มแล้วแทนอันเก่าสุด)
        let i = (tp.x - mr.x) / mr.w * 120.;
        let j = (tp.y - mr.y) / mr.h * 120.;
        let k = (ctr.0 + ((i - 60.) * sc as f32) as i32, ctr.1 + ((j - 60.) * sc as f32) as i32);
        if self.pins.len() >= PIN_MAX { self.pins.remove(0); }
        self.pins.push(k);
        self.pin_on = Some(self.pins.len() - 1);
        self.toast(format!("ปักหมุด {} แล้ว — กำลังนำทาง (แตะหมุดอีกครั้ง = ลบ)", self.pins.len()));
        play(&self.sfx.click);
        self.save();
        true
    }
    pub fn nav_save(&self) -> String {
        if self.pins.is_empty() { return String::new(); }
        let mut s = format!("pins {}", self.pin_on.map_or(-1, |i| i as i32));
        for (x, y) in &self.pins { s += &format!(" {},{}", x, y); }
        s + "\n"
    }
    pub fn nav_load(&mut self, r: &[&str]) {
        let Some(on) = r.first().and_then(|x| x.parse::<i32>().ok()) else { return };
        self.pins = r[1..].iter().filter_map(|p| { let mut it = p.split(','); Some((it.next()?.parse().ok()?, it.next()?.parse().ok()?)) }).take(PIN_MAX).collect();
        self.pin_on = if on >= 0 && (on as usize) < self.pins.len() { Some(on as usize) } else { None };
    }
}

/// รูปบ้านเล็ก (หลังคาสามเหลี่ยม + ตัวบ้าน + ประตู) หรือรูปเตียง ถ้ายังไม่มีหลังคา
fn home_icon(x: f32, y: f32, r: f32, roofed: bool) {
    draw_circle(x, y, r, hx(0x1d1f2b));
    if roofed {
        let c = hx(0xffb040);
        draw_circle_lines(x, y, r, 2., c);
        draw_triangle(vec2(x - r * 0.62, y - r * 0.05), vec2(x + r * 0.62, y - r * 0.05), vec2(x, y - r * 0.62), c);
        draw_rectangle(x - r * 0.42, y - r * 0.05, r * 0.84, r * 0.55, c);
        draw_rectangle(x - r * 0.12, y + r * 0.15, r * 0.24, r * 0.35, hx(0x1d1f2b));
    } else {
        let c = hx(0xd8d8e0);
        draw_circle_lines(x, y, r, 2., c);
        draw_rectangle(x - r * 0.55, y - r * 0.1, r * 1.1, r * 0.35, c);
        draw_rectangle(x - r * 0.55, y - r * 0.35, r * 0.3, r * 0.3, hx(0xff6070));
    }
}
fn pin_icon(x: f32, y: f32, r: f32, n: usize, active: bool, font: &Font) {
    let c = if active { hx(0x60f0ff) } else { hx(0x9fb0c8) };
    draw_triangle(vec2(x - r * 0.45, y - r * 0.6), vec2(x + r * 0.45, y - r * 0.6), vec2(x, y + r * 0.5), c);
    draw_circle(x, y - r * 0.75, r * 0.6, c);
    text_c(font, &format!("{}", n + 1), x, y - r * 0.5, r * 0.95, hx(0x10202b));
}
/// ลูกศรที่ขอบแผนที่ชี้ไปเป้าที่อยู่นอกแผนที่ + ระยะ
fn edge_arrow(mr: Rect, ctr: (i32, i32), sc: i32, t: (i32, i32), c: Color, label: &str, font: &Font, u: f32, big: bool) {
    let (dx, dy) = ((t.0 - ctr.0) as f32, (t.1 - ctr.1) as f32);
    let m = dx.abs().max(dy.abs()).max(1.);
    let k = 54. / (m / sc as f32);
    let (i, j) = (60. + dx / sc as f32 * k, 60. + dy / sc as f32 * k);
    let (x, y) = (mr.x + i / 120. * mr.w, mr.y + j / 120. * mr.h);
    let a = dy.atan2(dx);
    let s = if big { 14. * u } else { 7. * u };
    let (ca, sa) = (a.cos(), a.sin());
    let tip = vec2(x + ca * s, y + sa * s);
    let l = vec2(x - ca * s * 0.6 - sa * s * 0.7, y - sa * s * 0.6 + ca * s * 0.7);
    let r = vec2(x - ca * s * 0.6 + sa * s * 0.7, y - sa * s * 0.6 - ca * s * 0.7);
    draw_triangle(tip, l, r, c);
    draw_triangle_lines(tip, l, r, 1.5, hx(0x10202b));
    let dist = (dx.hypot(dy)).round() as i32;
    let txt = format!("{} {}", label, dist);
    let fs = if big { 13. * u } else { 9. * u };
    // ป้ายระยะอยู่ด้านในแผนที่ (ไม่ล้นขอบ)
    let (lx, ly) = (x - ca * s * 2.6, y - sa * s * 2.2);
    let d = measure_text(&txt, Some(font), fs as u16, 1.);
    draw_rectangle(lx - d.width / 2. - 3., ly - fs * 0.85, d.width + 6., fs * 1.15, Color::new(0., 0., 0., 0.6));
    text_c(font, &txt, lx, ly, fs, c);
}

/// วาดบ้าน + หมุด บนแผนที่ (เล็กหรือใหญ่)
pub fn draw_nav_map(g: &Game, font: &Font, mr: Rect, ctr: (i32, i32), sc: i32, big: bool, u: f32) {
    if g.w.dun.is_some() { return; }
    let rr = if big { 11. * u } else { 6.5 * u };
    for (i, &p) in g.pins.iter().enumerate() {
        let active = g.pin_on == Some(i);
        let (mx, my, ok) = map_pt(p.0, p.1, ctr, sc, mr);
        if ok { pin_icon(mx, my, rr, i, active, font); } else if active || big { edge_arrow(mr, ctr, sc, p, if active { hx(0x60f0ff) } else { hx(0x9fb0c8) }, &format!("หมุด{}", i + 1), font, u, big); }
    }
    if let Some((h, roofed)) = g.home_tile() {
        let (mx, my, ok) = map_pt(h.0, h.1, ctr, sc, mr);
        if ok { home_icon(mx, my, rr, roofed); } else { edge_arrow(mr, ctr, sc, h, if roofed { hx(0xffb040) } else { hx(0xd8d8e0) }, "บ้าน", font, u, big); }
        if big && g.home_cd > 0. {
            text_c(font, &format!("วาร์ปกลับบ้านได้อีกใน {} วิ", g.home_cd.ceil() as i32), mr.x + mr.w / 2., mr.y + 18. * u, 12. * u, hx(0xffd090));
        }
    }
}

/// ลูกศรรอบตัวผู้เล่นชี้ไปหมุดที่กำลังนำทาง
pub fn draw_nav_world(g: &Game, v: &View, font: &Font, t_now: f32) {
    let Some(i) = g.pin_on else { return };
    let Some(&(tx, ty)) = g.pins.get(i) else { return };
    if g.w.dun.is_some() { return; }
    let (dx, dy) = (tx as f32 * 16. + 8. - g.p.x, ty as f32 * 16. + 8. - g.p.y);
    let d = dx.hypot(dy);
    if d < 4. * 16. { return; }
    let a = dy.atan2(dx);
    let (ca, sa) = (a.cos(), a.sin());
    let r = 30. + (t_now * 4.).sin() * 2.;
    let (cx, cy) = ((g.p.x - v.cx) * v.s, (g.p.y - 10. - v.cy) * v.s);
    let (x, y) = (cx + ca * r * v.s, cy + sa * r * v.s);
    let s = 6. * v.s;
    let tip = vec2(x + ca * s, y + sa * s);
    let l = vec2(x - ca * s * 0.5 - sa * s * 0.6, y - sa * s * 0.5 + ca * s * 0.6);
    let rr = vec2(x - ca * s * 0.5 + sa * s * 0.6, y - sa * s * 0.5 - ca * s * 0.6);
    draw_triangle(tip, l, rr, Color::new(0.38, 0.94, 1., 0.9));
    draw_triangle_lines(tip, l, rr, 2., Color::new(0.06, 0.12, 0.17, 0.9));
    let txt = format!("หมุด{} {} ช่อง", i + 1, (d / 16.).round() as i32);
    let fs = (4.5 * v.s).max(12.);
    let (lx, ly) = (x + ca * s * 3.2, y + sa * s * 3.2);
    let dm = measure_text(&txt, Some(font), fs as u16, 1.);
    draw_rectangle(lx - dm.width / 2. - 3., ly - fs * 0.85, dm.width + 6., fs * 1.15, Color::new(0., 0., 0., 0.55));
    text_c(font, &txt, lx, ly, fs, hx(0x90f4ff));
}
