// house.rs — v13: บ้านจริง
//  - พื้นไม้/พื้นหิน (เมนูสร้าง · วางซ้ำที่เดิม = รื้อคืนวัสดุ)
//  - ห้อง = พื้นที่ที่ล้อมด้วยกำแพง/ประตู (หรือหน้าผาภูเขา) ครบทุกด้าน ขนาด 1–120 ช่อง
//  - หลังคาอัตโนมัติคลุมห้องเมื่อมองจากข้างนอก · เดินเข้าบ้านแล้วหลังคาจางหาย
//  - อยู่ในบ้าน: ฝน/หิมะไม่โดน · อุ่น (หน้าหนาว/ในหิมะไม่หิวเร็ว · ทะเลทรายร่มเย็น) · มอนสเตอร์ไม่เกิดข้างใน · นอนในบ้านฟื้นดีกว่า
//  - ชาวบ้านที่เตียงอยู่ในห้องมีหลังคามีความสุขขึ้น · นอนกลางแจ้งความสุขลด
use super::*;

pub const T_FLOORW: u8 = 19;
pub const T_FLOORS: u8 = 20;
pub const OBJ_FLOOR_W: u8 = 254;
pub const OBJ_FLOOR_S: u8 = 253;
pub const ROOM_MAX: usize = 120;

/// ผนังของห้อง: กำแพงไม้ · กำแพงหิน · กำแพงเมือง · ประตู (ปิด/เปิด)
pub fn is_wall_obj(o: u8) -> bool { matches!(o, 10 | 11 | 38 | 12 | 13) }

#[derive(Clone, Debug, Default)]
pub struct Room {
    pub tiles: Vec<(i32, i32)>,
    /// ผนังรอบห้อง (ไม่รวมประตู) — ใช้วาดหลังคาคลุม
    pub walls: Vec<(i32, i32)>,
    pub doors: Vec<(i32, i32)>,
    pub stone: bool,
    pub has_bed: bool,
    pub fire: bool,
    pub alpha: f32,
}

impl World {
    /// ไบโอมจริง (พื้นที่ปูทับ = ดูดินเดิมข้างใต้)
    pub fn biome_at(&self, x: i32, y: i32) -> u8 {
        let t = self.tile_at(x, y);
        if t == T_FLOORW || t == T_FLOORS { self.gen_tile(x, y) } else { t }
    }
    fn boundary(&self, tx: i32, ty: i32) -> bool {
        let o = self.obj_at(tx, ty);
        is_wall_obj(o) || self.tile_at(tx, ty) == 6
    }
    /// ห้องที่ช่องนี้อยู่ (None = ไม่ได้อยู่ในห้องปิด)
    pub fn room_from(&self, start: (i32, i32)) -> Option<Room> {
        if self.dun.is_some() || self.boundary(start.0, start.1) { return None; }
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        let mut stack = vec![start];
        let mut r = Room::default();
        let mut walls: HashSet<(i32, i32)> = HashSet::new();
        seen.insert(start);
        while let Some((x, y)) = stack.pop() {
            r.tiles.push((x, y));
            if r.tiles.len() > ROOM_MAX { return None; }
            let o = self.obj_at(x, y);
            if o == 33 { r.has_bed = true; }
            if o == 9 || o == 26 { r.fire = true; }
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let n = (x + dx, y + dy);
                if seen.contains(&n) { continue; }
                if self.boundary(n.0, n.1) {
                    walls.insert(n);
                    continue;
                }
                // ทะลุออกไปน้ำ/ของธรรมชาติที่ไม่ใช่ผนัง = ห้องไม่ปิด
                if SOLIDT[self.tile_at(n.0, n.1) as usize] { return None; }
                seen.insert(n);
                stack.push(n);
            }
        }
        // มุมห้อง (เฉียง) ให้หลังคาคลุมด้วย
        let inside: HashSet<(i32, i32)> = r.tiles.iter().copied().collect();
        for &(x, y) in &r.tiles {
            for (dx, dy) in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                let n = (x + dx, y + dy);
                if !inside.contains(&n) && self.boundary(n.0, n.1) { walls.insert(n); }
            }
        }
        for w in walls {
            let o = self.obj_at(w.0, w.1);
            if o == 12 || o == 13 { r.doors.push(w); } else { r.walls.push(w); }
            if o == 11 || o == 38 { r.stone = true; }
        }
        Some(r)
    }
}

impl Game {
    /// คำนวณห้องรอบตัวผู้เล่นใหม่ (เรียกทุก ~1 วินาที และทันทีหลังสร้าง/รื้อ)
    pub fn rooms_refresh(&mut self) {
        let old: HashMap<(i32, i32), f32> = self.rooms.iter().filter_map(|r| r.tiles.iter().min().map(|&k| (k, r.alpha))).collect();
        self.rooms.clear();
        self.room_of.clear();
        if self.w.dun.is_some() { return; }
        let (px, py) = (tfloor(self.p.x), tfloor(self.p.y));
        let walls: Vec<(i32, i32)> = self.w.placed.iter().filter(|(k, &o)| is_wall_obj(o) && (k.0 - px).abs() <= 24 && (k.1 - py).abs() <= 18).map(|(k, _)| *k).collect();
        for w in walls {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let s = (w.0 + dx, w.1 + dy);
                if self.room_of.contains_key(&s) { continue; }
                if let Some(mut r) = self.w.room_from(s) {
                    let id = self.rooms.len();
                    let key = *r.tiles.iter().min().unwrap();
                    r.alpha = old.get(&key).copied().unwrap_or(1.);
                    for &t in &r.tiles { self.room_of.insert(t, id); }
                    self.rooms.push(r);
                }
            }
        }
        if !self.rooms.is_empty() && self.tips_seen & (1 << 8) == 0 {
            self.tips_seen |= 1 << 8;
            self.toast("สร้างบ้านสำเร็จ! ห้องที่ล้อมกำแพงครบมีหลังคาแล้ว — ข้างในฝนไม่โดน อุ่น และมอนสเตอร์ไม่เกิด");
        }
    }
    /// ผู้เล่นอยู่ในบ้าน (ห้องปิดที่มีหลังคา)
    pub fn indoors(&self) -> bool { self.w.dun.is_none() && self.room_of.contains_key(&(tfloor(self.p.x), tfloor(self.p.y - 3.))) }
    pub fn house_tick(&mut self, dt: f32) {
        self.room_t -= dt;
        if self.room_t <= 0. { self.room_t = 1.; self.rooms_refresh(); }
        let me = self.room_of.get(&(tfloor(self.p.x), tfloor(self.p.y - 3.))).copied();
        for (i, r) in self.rooms.iter_mut().enumerate() {
            let target = if Some(i) == me { 0. } else { 1. };
            r.alpha += (target - r.alpha) * (dt * 8.).min(1.);
        }
    }
    /// วาง/รื้อพื้น (คืน true ถ้าทำแล้ว)
    pub fn place_floor(&mut self, k: (i32, i32), obj: u8) -> bool {
        let want = if obj == OBJ_FLOOR_W { T_FLOORW } else { T_FLOORS };
        let (it, n) = if obj == OBJ_FLOOR_W { (WOOD, 1) } else { (STONE, 1) };
        let t = self.w.tile_at(k.0, k.1);
        if t == want {
            self.w.tmod.remove(&k);
            self.p.inv[it] += n;
            self.toast("รื้อพื้นแล้ว (ได้วัสดุคืน)");
            return true;
        }
        if SOLIDT[t as usize] { self.toast("ปูพื้นบนน้ำ/ภูเขาไม่ได้"); return false; }
        if t == T_SOIL || t == 12 || self.crops.contains_key(&k) { self.toast("รื้อแปลงผักก่อน (ใช้ไถดินวางซ้ำ)"); return false; }
        // ช่องที่ถูกดัดแปลงไว้แล้ว (ลาวาที่เย็นเป็นหิน ฯลฯ) ห้ามปูทับ — รื้อพื้นจะได้คืนดินเดิมเสมอ
        if self.w.tmod.contains_key(&k) && t != T_FLOORW && t != T_FLOORS { self.toast("ปูพื้นตรงนี้ไม่ได้"); return false; }
        let o = self.w.obj_at(k.0, k.1);
        if o != 0 && o != 3 && !self.w.placed.contains_key(&k) { self.toast("มีของขวางอยู่"); return false; }
        if self.p.inv[it] < n { self.toast("วัสดุไม่พอ"); return false; }
        if o == 3 && !self.w.placed.contains_key(&k) { self.w.removed.insert(k); }
        if (27..=32).contains(&o) { self.toast("รื้อแปลงผักก่อน"); return false; }
        self.p.inv[it] -= n;
        if t == T_FLOORW || t == T_FLOORS { self.p.inv[if t == T_FLOORW { WOOD } else { STONE }] += 1; }
        self.w.tmod.insert(k, want);
        true
    }
    /// ชาวบ้าน: เตียงอยู่ในห้องที่มีหลังคาไหม (ตรวจตรงๆ ไม่พึ่งแคชรอบตัวผู้เล่น)
    pub fn npc_has_house(&self, bed: (i32, i32)) -> bool { self.w.room_from(bed).is_some() }
}

pub fn floor_tex(stone: bool, i: i32) -> Image {
    let mut im = Image::gen_image_color(16, 16, hx(if stone { 0x8e8a84 } else { 0xb07a44 }));
    if stone {
        for (y0, off) in [(0, 0), (8, 4)] {
            for bx in -1..3 {
                let x0 = bx * 8 + off;
                for y in y0..y0 + 8 { for x in x0..x0 + 8 {
                    if x < 0 || x >= 16 { continue; }
                    let edge = y == y0 || x == x0;
                    let n = hash(x + i * 16, y, 830);
                    let c = if edge { 0x6a6660 } else if n < 0.12 { 0x7e7a74 } else if n > 0.9 { 0xa29e98 } else { 0x8e8a84 };
                    im.set_pixel(x as u32, y as u32, hx(c));
                } }
            }
        }
    } else {
        for y in 0..16 {
            let plank = y / 4;
            for x in 0..16 {
                let seam = y % 4 == 3 || (x + plank * 5 + i * 3) % 16 == 0;
                let n = hash(x + i * 16, y, 831);
                let c = if seam { 0x7a5028 } else if n < 0.1 { 0x9a6838 } else if n > 0.92 { 0xc89058 } else { 0xb07a44 };
                im.set_pixel(x as u32, y as u32, hx(c));
            }
        }
    }
    im
}

/// หลังคา: ปูกระเบื้อง (ไม้ = แดงอิฐ · หิน = เทาน้ำเงิน) มีสันกลางและเงาชายคา
pub fn draw_roofs(g: &Game, v: &View) {
    for r in &g.rooms {
        if r.alpha < 0.03 { continue; }
        let all: Vec<(i32, i32)> = r.tiles.iter().chain(r.walls.iter()).copied().collect();
        let (mut y0, mut y1) = (i32::MAX, i32::MIN);
        for &(_, y) in &all { y0 = y0.min(y); y1 = y1.max(y); }
        let mid = (y0 + y1) as f32 / 2.;
        let (c1, c2, ridge) = if r.stone { (0x5a6a88, 0x4a5874, 0x8a9ab8) } else { (0xb04a32, 0x8e3a26, 0xd87a52) };
        let a = r.alpha;
        for &(tx, ty) in &all {
            let (px, py) = (tx as f32 * 16. - v.cx, ty as f32 * 16. - 8. - v.cy);
            let top = (ty as f32) < mid;
            let base = if top { c1 } else { c2 };
            prect(v, px, py, 16., 16., Color::new(((base >> 16) & 255) as f32 / 255., ((base >> 8) & 255) as f32 / 255., (base & 255) as f32 / 255., a));
            // แถวกระเบื้อง
            for k in 0..4 {
                let yy = py + k as f32 * 4. + 3.;
                prect(v, px, yy, 16., 1., Color::new(0., 0., 0., 0.22 * a));
                let off = if k % 2 == 0 { 0. } else { 4. };
                for s in 0..2 { prect(v, px + off + s as f32 * 8., yy - 3., 1., 3., Color::new(0., 0., 0., 0.15 * a)); }
            }
            if (ty as f32 - mid).abs() < 0.6 {
                prect(v, px, py + 6., 16., 3., Color::new(((ridge >> 16) & 255) as f32 / 255., ((ridge >> 8) & 255) as f32 / 255., (ridge & 255) as f32 / 255., a));
            }
            if ty == y1 { prect(v, px, py + 15., 16., 2., Color::new(0., 0., 0., 0.35 * a)); }
        }
        // ปล่องไฟ + ควัน ถ้าในบ้านมีกองไฟ/หม้อ
        if r.fire {
            if let Some(&(tx, ty)) = r.tiles.iter().min_by_key(|t| (t.1, -t.0)) {
                let (px, py) = (tx as f32 * 16. + 9. - v.cx, ty as f32 * 16. - 18. - v.cy);
                prect(v, px, py, 5., 8., Color::new(0.35, 0.3, 0.28, a));
                prect(v, px - 1., py, 7., 2., Color::new(0.25, 0.2, 0.18, a));
                let t = get_time() as f32;
                for k in 0..3 {
                    let ph = (t * 0.6 + k as f32 / 3.).fract();
                    prect(v, px + 1. + (ph * 6. + k as f32).sin() * 2., py - ph * 14., 3., 3., Color::new(0.85, 0.85, 0.88, 0.5 * (1. - ph) * a));
                }
            }
        }
    }
}
