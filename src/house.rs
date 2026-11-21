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
// เฟอร์นิเจอร์ (v13.1)
pub const O_TABLE: u8 = 70;
pub const O_CHAIR: u8 = 71;
pub const O_RUG: u8 = 72;
pub const O_SHELF: u8 = 73;
pub const O_PLANT: u8 = 74;
pub const O_LAMP: u8 = 75;
pub const O_WINDOW: u8 = 76;
pub const O_WARDROBE: u8 = 77;
pub const O_TENTWALL: u8 = 78;
pub const O_LOG: u8 = 79;
// สิ่งก่อสร้างสำเร็จรูป (เลือกแล้ววางทีเดียว · ใช้เวลาสร้าง)
pub const OBJ_HOUSE_S: u8 = 252;
pub const OBJ_HOUSE_L: u8 = 251;
pub const OBJ_TENT: u8 = 250;
pub const OBJ_HUT: u8 = 249;
pub const OBJ_FIRERING: u8 = 248;

/// ผนังของห้อง: กำแพงไม้ · กำแพงหิน · กำแพงเมือง · ประตู (ปิด/เปิด) · หน้าต่าง · ผ้าเต็นท์
pub fn is_wall_obj(o: u8) -> bool { matches!(o, 10 | 11 | 38 | 12 | 13 | O_WINDOW | O_TENTWALL) }
pub fn is_furniture(o: u8) -> bool { matches!(o, O_TABLE | O_CHAIR | O_RUG | O_SHELF | O_PLANT | O_LAMP | O_WARDROBE | O_LOG) }
pub fn is_prefab(o: u8) -> bool { matches!(o, OBJ_HOUSE_S | OBJ_HOUSE_L | OBJ_TENT | OBJ_HUT | OBJ_FIRERING) }
pub fn is_seat(o: u8) -> bool { o == O_CHAIR || o == O_LOG }
/// เวลาสร้าง (วินาที ต้องอยู่ใกล้ๆ)
pub fn prefab_time(kind: u8) -> f32 {
    match kind { OBJ_TENT => 6., OBJ_FIRERING => 8., OBJ_HUT => 15., OBJ_HOUSE_S => 30., _ => 60. }
}
pub fn prefab_name(kind: u8) -> &'static str {
    match kind { OBJ_TENT => "เต็นท์", OBJ_FIRERING => "วงกองไฟ", OBJ_HUT => "กระท่อม", OBJ_HOUSE_S => "บ้านเล็ก", _ => "บ้านใหญ่" }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell { Wall(u8), Floor(u8, u8), Ground(u8) }

/// แบบสิ่งก่อสร้าง: ช่องที่เลือก = (0,0) (บ้าน = ประตู ตัวบ้านยื่นขึ้นไปด้านบน · วงกองไฟ = กองไฟตรงกลาง)
/// Floor(พื้น, ของบนพื้น) · Ground(ของบนดินเดิม 0 = เว้นว่าง)
pub fn prefab_layout(kind: u8) -> Vec<((i32, i32), Cell)> {
    if kind == OBJ_FIRERING {
        return vec![((0, 0), Cell::Ground(9)), ((-2, 0), Cell::Ground(O_LOG)), ((2, 0), Cell::Ground(O_LOG)), ((0, -2), Cell::Ground(O_LOG)), ((0, 2), Cell::Ground(O_LOG))];
    }
    let (hw, h, wall): (i32, i32, u8) = match kind { OBJ_TENT => (1, 3, O_TENTWALL), OBJ_HUT => (2, 3, 10), OBJ_HOUSE_L => (3, 5, 11), _ => (2, 4, 10) };
    let mut v = vec![];
    for dy in -h..=0 {
        for dx in -hw..=hw {
            let edge = dy == -h || dy == 0 || dx.abs() == hw;
            let c = if !edge {
                match kind {
                    OBJ_TENT => Cell::Ground(if (dx, dy) == (0, -2) { 33 } else { 0 }),
                    OBJ_HUT => Cell::Ground(match (dx, dy) { (-1, -2) => 33, (1, -2) => O_LOG, _ => 0 }),
                    OBJ_HOUSE_L => Cell::Floor(T_FLOORW, match (dx, dy) { (-2, -4) => 33, (2, -4) => O_LAMP, (-1, -4) => O_SHELF, (1, -3) => O_TABLE, (0, -3) => O_CHAIR, (2, -1) => O_PLANT, (-2, -1) => 14, (0, -2) => O_RUG, _ => 0 }),
                    _ => Cell::Floor(T_FLOORW, match (dx, dy) { (-1, -3) => 33, (1, -3) => O_LAMP, _ => 0 }),
                }
            } else if (dx, dy) == (0, 0) {
                Cell::Wall(12)
            } else if kind != OBJ_TENT && dy == -h && (if kind == OBJ_HOUSE_L { dx.abs() == 2 } else { dx == 0 }) {
                Cell::Wall(O_WINDOW)
            } else {
                Cell::Wall(wall)
            };
            v.push(((dx, dy), c));
        }
    }
    v
}
/// ขั้นตอนการสร้าง: พื้น (ล่าง→บน) → กำแพง (ล่าง→บน) → ของในบ้าน — ใช้ทั้งตอนสร้างทีละชิ้นและวาดนั่งร้าน
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Step { Tile(u8), Obj(u8) }
pub fn prefab_steps(kind: u8, k: (i32, i32)) -> Vec<((i32, i32), Step)> {
    let lay: Vec<((i32, i32), Cell)> = prefab_layout(kind).into_iter().map(|((dx, dy), c)| ((k.0 + dx, k.1 + dy), c)).collect();
    let mut tiles = vec![];
    let mut walls = vec![];
    let mut furn = vec![];
    for (p, c) in lay {
        match c {
            Cell::Floor(t, f) => { tiles.push((p, Step::Tile(t))); if f != 0 { furn.push((p, Step::Obj(f))); } }
            Cell::Wall(o) => walls.push((p, Step::Obj(o))),
            Cell::Ground(f) => if f != 0 { furn.push((p, Step::Obj(f))); },
        }
    }
    let ord = |v: &mut Vec<((i32, i32), Step)>| v.sort_by_key(|((x, y), _)| (-*y, *x));
    ord(&mut tiles);
    ord(&mut walls);
    tiles.extend(walls);
    tiles.extend(furn);
    tiles
}

/// ไซต์ก่อสร้างที่กำลังสร้างอยู่
#[derive(Clone, Debug)]
pub struct Site { pub kind: u8, pub k: (i32, i32), pub prog: f32, pub done: usize }

/// ความน่าอยู่ของห้อง 0–5 ดาว: เตียง · ปูพื้นเกือบเต็ม · หน้าต่าง · เฟอร์นิเจอร์คนละแบบ (2 ชิ้น = 1 ดาว) · แสงไฟ/กองไฟ
pub fn comfort_of(w: &World, r: &Room) -> u8 {
    let mut pts = 0f32;
    if r.has_bed { pts += 1.; }
    let floored = r.tiles.iter().filter(|&&(x, y)| matches!(w.tile_at(x, y), T_FLOORW | T_FLOORS)).count();
    if floored * 10 >= r.tiles.len() * 7 { pts += 1.; }
    if r.walls.iter().any(|&(x, y)| w.obj_at(x, y) == O_WINDOW) { pts += 0.5; }
    let mut kinds: HashSet<u8> = HashSet::new();
    for &(x, y) in &r.tiles { let o = w.obj_at(x, y); if is_furniture(o) || matches!(o, 14 | 26 | 9 | 18) { kinds.insert(o); } }
    if kinds.iter().any(|&o| matches!(o, O_LAMP | 9 | 18 | 26)) { pts += 0.5; }
    pts += kinds.len() as f32 * 0.5;
    (pts.floor() as u8).min(5)
}
/// ฟอนต์ไทยไม่มีรูปดาว → เขียนเป็นตัวเลข
pub fn stars(n: u8) -> String { format!("{}/5 ดาว", n) }

#[derive(Clone, Debug, Default)]
pub struct Room {
    pub tiles: Vec<(i32, i32)>,
    /// ผนังรอบห้อง (ไม่รวมประตู) — ใช้วาดหลังคาคลุม
    pub walls: Vec<(i32, i32)>,
    pub doors: Vec<(i32, i32)>,
    pub stone: bool,
    pub tent: bool,
    pub has_bed: bool,
    pub fire: bool,
    pub alpha: f32,
    pub comfort: u8,
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
            if o == O_TENTWALL { r.tent = true; }
        }
        r.comfort = comfort_of(self, &r);
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
        let (c1, c2, ridge) = if r.tent { (0xd8c8a0, 0xbfae84, 0x9a8460) } else if r.stone { (0x5a6a88, 0x4a5874, 0x8a9ab8) } else { (0xb04a32, 0x8e3a26, 0xd87a52) };
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

impl Game {
    /// ช่องโลกของบ้านสำเร็จรูป เมื่อประตูอยู่ที่ k
    pub fn prefab_cells(&self, kind: u8, k: (i32, i32)) -> Vec<((i32, i32), Cell)> {
        prefab_layout(kind).into_iter().map(|((dx, dy), c)| ((k.0 + dx, k.1 + dy), c)).collect()
    }
    /// ช่องนี้ถูกจองโดยไซต์ก่อสร้างที่ยังสร้างไม่เสร็จ
    pub fn site_reserved(&self, k: (i32, i32)) -> bool {
        self.sites.iter().any(|st| prefab_layout(st.kind).iter().any(|((dx, dy), _)| (st.k.0 + dx, st.k.1 + dy) == k))
    }
    /// วางสิ่งก่อสร้างสำเร็จรูปตรงนี้ได้ไหม (None = ได้ · Some(เหตุผล))
    pub fn prefab_blocked(&self, kind: u8, k: (i32, i32)) -> Option<&'static str> {
        if self.w.dun.is_some() { return Some("สร้างในดันเจี้ยนไม่ได้"); }
        let (ptx, pty) = (tfloor(self.p.x), tfloor(self.p.y - 3.));
        for ((x, y), c) in self.prefab_cells(kind, k) {
            let t = self.w.tile_at(x, y);
            if SOLIDT[t as usize] { return Some("พื้นที่ติดน้ำ/ภูเขา — หาที่โล่งกว้างกว่านี้"); }
            if self.w.tmod.contains_key(&(x, y)) && t != T_FLOORW && t != T_FLOORS { return Some("ทับแปลงผัก/ดินที่ขุดไว้ — หาที่อื่น"); }
            if self.crops.contains_key(&(x, y)) { return Some("ทับแปลงผัก — หาที่อื่น"); }
            if self.site_reserved((x, y)) { return Some("ทับที่ที่กำลังก่อสร้างอยู่"); }
            let o = self.w.obj_at(x, y);
            if !(o == 0 || ((o == 2 || o == 3 || o == O_DRYGRASS) && !self.w.placed.contains_key(&(x, y)))) { return Some("มีต้นไม้/หิน/ของวางอยู่ในพื้นที่ — ตีออกก่อน หรือหาที่โล่ง"); }
            let solid_cell = match c { Cell::Wall(_) => true, Cell::Floor(_, f) | Cell::Ground(f) => f != 0 && OSOLID[f as usize] };
            if solid_cell && (self.player_in_tile(x, y) || (x, y) == (ptx, pty)) { return Some("ถอยออกมาหน่อย ตัวละครยืนขวางอยู่"); }
        }
        None
    }
    /// เริ่มก่อสร้าง: จ่ายของ เคลียร์หญ้า/ดอกไม้ จองพื้นที่ แล้วค่อยๆ สร้างขึ้นทีละชิ้นตอนผู้เล่นอยู่ใกล้
    pub fn place_prefab(&mut self, kind: u8, k: (i32, i32), cost: &[(usize, i32)]) -> bool {
        if let Some(why) = self.prefab_blocked(kind, k) { self.toast(why); return false; }
        if !can_pay(&self.p, cost) { self.toast("วัสดุไม่พอ"); return false; }
        for &(it, n) in cost { self.p.inv[it] -= n; }
        for ((x, y), _) in self.prefab_cells(kind, k) {
            if !self.w.placed.contains_key(&(x, y)) && self.w.nat_obj(x, y) != 0 { self.w.removed.insert((x, y)); }
            self.obj_dmg.remove(&(x, y));
            if gen_range(0., 1.) < 0.5 { self.burst(x as f32 * 16. + 8., y as f32 * 16. + 10., hx(0xc8a070), 3, 30.); }
        }
        self.sites.push(Site { kind, k, prog: 0., done: 0 });
        self.build_sel = None; // ออกจากโหมดสร้างเลย ไม่ต้องหาปุ่มยกเลิก
        play(&self.sfx.craft);
        self.toast(format!("เริ่มสร้าง{}! อยู่ใกล้ๆ แล้วจะค่อยๆ เสร็จ (~{} วินาที)", prefab_name(kind), prefab_time(kind) as i32));
        true
    }
    /// ก่อสร้างคืบหน้า: เฉพาะตอนผู้เล่นอยู่ในระยะ ~8 ช่อง · ชิ้นที่มีคน/มอนยืนขวางจะรอ
    pub fn sites_tick(&mut self, dt: f32) {
        if self.sites.is_empty() || self.w.dun.is_some() { return; }
        let mut sites = std::mem::take(&mut self.sites);
        let mut finished = vec![];
        for (si, st) in sites.iter_mut().enumerate() {
            let steps = prefab_steps(st.kind, st.k);
            let n = steps.len().max(1);
            let total = prefab_time(st.kind);
            let c = self.site_center(st);
            let near = (c.0 - self.p.x).hypot(c.1 - self.p.y) < 8. * 16.;
            if !near { continue; }
            st.prog = (st.prog + dt).min(total);
            let target = ((st.prog / total) * n as f32).floor() as usize;
            while st.done < target.min(n) {
                let (p, step) = steps[st.done];
                if let Step::Obj(o) = step {
                    let busy = OSOLID[o as usize] && (self.player_in_tile(p.0, p.1) || (p.0, p.1) == (tfloor(self.p.x), tfloor(self.p.y - 3.))
                        || self.npcs.iter().any(|q| (tfloor(q.x), tfloor(q.y - 3.)) == p) || self.en.iter().any(|e| !e.dead && (tfloor(e.x), tfloor(e.y - 3.)) == p));
                    if busy {
                        st.prog = st.prog.min((st.done as f32 + 0.99) / n as f32 * total);
                        if self.site_wait_t <= 0. { self.site_wait_t = 3.; self.toast("ถอยออกจากจุดที่กำลังสร้างหน่อย"); }
                        break;
                    }
                    self.w.placed.insert(p, o);
                    if o == 14 { self.chests.entry(p).or_insert([0; NI]); }
                } else if let Step::Tile(t) = step {
                    self.w.tmod.insert(p, t);
                }
                self.burst(p.0 as f32 * 16. + 8., p.1 as f32 * 16. + 8., hx(0xd8b080), 5, 45.);
                if st.done % 2 == 0 { play(&self.sfx.chop); }
                st.done += 1;
            }
            if st.done >= n { finished.push(si); }
        }
        if self.site_wait_t > 0. { self.site_wait_t -= dt; }
        for &si in finished.iter().rev() {
            let st = sites.remove(si);
            self.ev[3] += 1;
            self.room_t = 0.;
            self.refresh_torches();
            play(&self.sfx.lvl);
            let c = self.site_center(&st);
            for _ in 0..3 { self.burst(c.0, c.1, hx(0xffe080), 10, 70.); }
            self.toast(match st.kind {
                OBJ_FIRERING => "วงกองไฟเสร็จแล้ว! แตะท่อนไม้ = นั่ง · ตอนเย็นชาวบ้านจะมานั่งคุยกัน".to_string(),
                _ => format!("{}เสร็จแล้ว! เดินเข้าประตูได้เลย · แต่งบ้านเพิ่มดาวความน่าอยู่", prefab_name(st.kind)),
            });
            self.save();
        }
        self.sites = sites;
    }
    pub fn site_center(&self, st: &Site) -> (f32, f32) {
        let lay = prefab_layout(st.kind);
        let n = lay.len() as f32;
        let (sx, sy) = lay.iter().fold((0., 0.), |a, ((dx, dy), _)| (a.0 + *dx as f32, a.1 + *dy as f32));
        ((st.k.0 as f32 + sx / n) * 16. + 8., (st.k.1 as f32 + sy / n) * 16. + 8.)
    }
    pub fn sites_save(&self) -> String {
        self.sites.iter().map(|st| format!("site {} {} {} {:.2} {}\n", st.kind, st.k.0, st.k.1, st.prog, st.done)).collect()
    }
    pub fn sites_load(&mut self, r: &[&str]) {
        let v: Vec<f32> = r.iter().filter_map(|x| x.parse().ok()).collect();
        if v.len() >= 5 && is_prefab(v[0] as u8) { self.sites.push(Site { kind: v[0] as u8, k: (v[1] as i32, v[2] as i32), prog: v[3], done: v[4] as usize }); }
    }
    /// กดค้างปุ่มสร้างแล้วเดิน = วางกำแพงตามทางที่เดินผ่าน (วางที่ช่องที่เพิ่งเดินออกมา) · พื้น = ปูใต้เท้า
    pub fn paint_tick(&mut self, held: bool) {
        let cur = (tfloor(self.p.x), tfloor(self.p.y - 3.));
        let Some(sel) = self.build_sel.filter(|_| held && self.w.dun.is_none()) else { self.paint_last = None; return; };
        let pc = &PIECES[sel];
        let o = pc.obj;
        if o == OBJ_FLOOR_W || o == OBJ_FLOOR_S {
            let want = if o == OBJ_FLOOR_W { T_FLOORW } else { T_FLOORS };
            if self.paint_last != Some(cur) {
                self.paint_last = Some(cur);
                if self.w.tile_at(cur.0, cur.1) != want && self.place_floor_quiet(cur, o) { self.room_t = 0.; }
            }
            return;
        }
        if !matches!(o, 10 | 11 | 38 | O_WINDOW) { return; }
        match self.paint_last {
            Some(prev) if prev != cur => {
                if self.can_place_at(prev.0, prev.1) && can_pay(&self.p, pc.cost) {
                    for &(it, n) in pc.cost { self.p.inv[it] -= n; }
                    self.w.placed.insert(prev, o);
                    self.obj_dmg.remove(&prev);
                    self.room_t = 0.;
                    play(&self.sfx.door);
                    self.burst(prev.0 as f32 * 16. + 8., prev.1 as f32 * 16. + 10., hx(0xc8a070), 4, 30.);
                }
                self.paint_last = Some(cur);
            }
            None => self.paint_last = Some(cur),
            _ => {}
        }
    }
    fn place_floor_quiet(&mut self, k: (i32, i32), obj: u8) -> bool {
        let t = self.w.tile_at(k.0, k.1);
        let (it, want) = if obj == OBJ_FLOOR_W { (WOOD, T_FLOORW) } else { (STONE, T_FLOORS) };
        if SOLIDT[t as usize] || self.p.inv[it] < 1 || self.crops.contains_key(&k) { return false; }
        if self.w.tmod.contains_key(&k) && t != T_FLOORW && t != T_FLOORS { return false; }
        let o = self.w.obj_at(k.0, k.1);
        if o != 0 && o != 3 && !self.w.placed.contains_key(&k) { return false; }
        self.p.inv[it] -= 1;
        if t == T_FLOORW || t == T_FLOORS { self.p.inv[if t == T_FLOORW { WOOD } else { STONE }] += 1; }
        self.w.tmod.insert(k, want);
        true
    }
    /// ดาวความน่าอยู่ของห้องที่ผู้เล่นอยู่
    pub fn home_comfort(&self) -> Option<u8> {
        self.room_of.get(&(tfloor(self.p.x), tfloor(self.p.y - 3.))).map(|&i| self.rooms[i].comfort)
    }
}

/// ตอนอยู่ในบ้าน: ข้างนอกมืดลง (โฟกัสในห้อง) · ผนังด้านในเป็นวอลเปเปอร์ · หน้าต่างเห็นท้องฟ้า+แสงลอดเข้ามา
pub fn cur_room(g: &Game) -> Option<&Room> {
    g.room_of.get(&(tfloor(g.p.x), tfloor(g.p.y - 3.))).map(|&i| &g.rooms[i])
}
/// ผนังห้องที่ต้องซ่อน sprite ปกติ (วาดเป็นผนังด้านในแทน)
pub fn interior_hidden(g: &Game) -> HashSet<(i32, i32)> {
    match cur_room(g) { Some(r) if r.alpha < 0.5 => r.walls.iter().copied().collect(), _ => HashSet::new() }
}
/// phase 0 = ก่อนวาดตัวละคร (ผนังด้านใน แสง) · phase 1 = หลังวาด (ข้างนอกมืด)
pub fn draw_interior(g: &Game, v: &View, t_now: f32, phase: u8) {
    let Some(r) = cur_room(g) else { return };
    let k = (1. - r.alpha).clamp(0., 1.);
    if k < 0.02 { return; }
    let inside: HashSet<(i32, i32)> = r.tiles.iter().chain(r.walls.iter()).chain(r.doors.iter()).copied().collect();
    let room: HashSet<(i32, i32)> = r.tiles.iter().copied().collect();
    if phase == 1 {
    // 1) ข้างนอกมืด
    let (tx0, ty0) = (tfloor(v.cx) - 1, tfloor(v.cy) - 1);
    let (tx1, ty1) = (tfloor(v.cx + v.vw) + 1, tfloor(v.cy + v.vh) + 2);
    for ty in ty0..=ty1 {
        let mut run: Option<i32> = None;
        for tx in tx0..=tx1 + 1 {
            let out = tx <= tx1 && !inside.contains(&(tx, ty));
            match (out, run) {
                (true, None) => run = Some(tx),
                (false, Some(s)) => { prect(v, s as f32 * 16. - v.cx, ty as f32 * 16. - v.cy, (tx - s) as f32 * 16., 16., Color::new(0.04, 0.03, 0.06, 0.9 * k)); run = None; }
                _ => {}
            }
        }
    }
    return;
    }
    // 2) ผนัง: ผนังด้านหลัง (เหนือห้อง) = วอลเปเปอร์ · ผนังข้าง/ล่าง = สันกำแพงเข้ม
    let day = (1. - g.darkv).clamp(0., 1.);
    let (wp1, wp2, base) = if r.tent { (0xe0d0a8, 0xccbc94, 0x9a8460) } else if r.stone { (0xb8b4a8, 0xa8a498, 0x6a6660) } else { (0xe8d8b0, 0xd8c498, 0x7a5028) };
    let c = |h: u32, a: f32| Color::new(((h >> 16) & 255) as f32 / 255., ((h >> 8) & 255) as f32 / 255., (h & 255) as f32 / 255., a);
    let kw = if r.alpha < 0.5 { 1. } else { k };
    for &(x, y) in r.walls.iter().chain(r.doors.iter()) {
        let (px, py) = (x as f32 * 16. - v.cx, y as f32 * 16. - v.cy);
        let back = room.contains(&(x, y + 1));
        let o = g.w.obj_at(x, y);
        if back && o != 12 && o != 13 {
            prect(v, px, py, 16., 16., c(wp1, kw));
            for s in 0..4 { prect(v, px + s as f32 * 4. + 1., py, 2., 13., c(wp2, kw)); }
            prect(v, px, py, 16., 2., c(0x00000, 0.35 * kw));
            prect(v, px, py + 13., 16., 3., c(base, kw));
            if o == O_WINDOW {
                // ท้องฟ้าตามเวลา + กรอบหน้าต่าง
                let sky = Color::new(0.05 + 0.45 * day, 0.07 + 0.6 * day, 0.18 + 0.72 * day, kw);
                prect(v, px + 3., py + 3., 10., 8., c(0x5a3418, kw));
                prect(v, px + 4., py + 4., 8., 6., sky);
                prect(v, px + 7.5, py + 4., 1., 6., c(0x5a3418, kw));
                prect(v, px + 4., py + 6.5, 8., 1., c(0x5a3418, kw));
                if day < 0.4 { prect(v, px + 5., py + 5., 1., 1., c(0xffffff, kw * 0.8)); prect(v, px + 10., py + 8., 1., 1., c(0xffffff, kw * 0.6)); }
                // แสงแดดลอดลงพื้น
                if day > 0.3 {
                    let a = 0.16 * day * k;
                    for j in 0..3 { prect(v, px + 2. - j as f32, py + 16. + j as f32 * 8., 12. + j as f32 * 2., 8., Color::new(1., 0.95, 0.7, a * (1. - j as f32 * 0.3))); }
                }
            }
        } else if o != 12 && o != 13 {
            // ผนังข้าง/ล่าง: มองจากบนลงเห็นสันกำแพง (ยังเห็น sprite เดิมจางๆ ตอนกำลังเฟด)
            prect(v, px, py, 16., 16., c(base, kw));
            prect(v, px + 1., py + 1., 14., 14., c(if r.stone { 0x5a5650 } else { 0x6a4220 }, kw));
            prect(v, px + 1., py + 1., 14., 2., c(if r.stone { 0x7a766e } else { 0x8a5a2c }, kw));
        }
    }
    // 3) ไฟอุ่นในห้องเบาๆ (มีโคม/กองไฟ)
    let lit = r.tiles.iter().any(|&(x, y)| matches!(g.w.obj_at(x, y), O_LAMP | 9 | 18 | 26));
    if lit {
        let fl = 0.04 + (t_now * 2.3).sin() * 0.01;
        for &(x, y) in &r.tiles { prect(v, x as f32 * 16. - v.cx, y as f32 * 16. - v.cy, 16., 16., Color::new(1., 0.7, 0.3, fl * k)); }
    }
}

impl Game {
    /// ประตูอัตโนมัติ: เดินชนประตูที่ปิด = เปิดเอง · เดินห่างออกไป = ปิดเอง (กันมอนเข้า) · ประตูที่เปิดเองด้วย Q ไม่ปิดให้
    pub fn auto_door(&mut self, mv: Vec2) {
        if self.w.dun.is_some() { return; }
        if mv.length() > 0.3 {
            let (px, py) = (self.p.x + mv.x.signum() * if mv.x.abs() > 0.3 { 10. } else { 0. }, if mv.y < -0.3 { self.p.y - 10. } else if mv.y > 0.3 { self.p.y + 6. } else { self.p.y - 2. });
            let k = (tfloor(px), tfloor(py));
            if self.w.placed.get(&k) == Some(&12) {
                self.w.placed.insert(k, 13);
                self.auto_doors.insert(k);
                self.room_t = self.room_t.min(0.2);
                play(&self.sfx.door);
            }
        }
        let close: Vec<(i32, i32)> = self.auto_doors.iter().copied().filter(|k| (k.0 as f32 * 16. + 8. - self.p.x).hypot(k.1 as f32 * 16. + 8. - (self.p.y - 3.)) > 30. && !self.player_in_tile(k.0, k.1)).collect();
        for k in close {
            self.auto_doors.remove(&k);
            let blocked = self.en.iter().any(|e| !e.dead && tfloor(e.x) == k.0 && tfloor(e.y) == k.1) || self.npcs.iter().any(|n| tfloor(n.x) == k.0 && tfloor(n.y - 3.) == k.1);
            if self.w.placed.get(&k) == Some(&13) && !blocked { self.w.placed.insert(k, 12); play(&self.sfx.door); }
        }
    }
}

/// ไซต์ก่อสร้าง: เงาสิ่งที่จะสร้าง + นั่งร้าน + ค้อนเคาะ + แถบความคืบหน้า
pub fn draw_sites(g: &Game, sp: &Sprites, v: &View, font: &Font, t_now: f32) {
    for st in &g.sites {
        let steps = prefab_steps(st.kind, st.k);
        let n = steps.len().max(1);
        let near = { let c = g.site_center(st); (c.0 - g.p.x).hypot(c.1 - g.p.y) < 8. * 16. };
        for (i, &((x, y), step)) in steps.iter().enumerate().skip(st.done) {
            let (px, py) = (x as f32 * 16. - v.cx, y as f32 * 16. - v.cy);
            match step {
                Step::Tile(t) => blit(sp, v, sp.tiles[t as usize][0], px, py, false, Color::new(1., 1., 1., 0.18)),
                Step::Obj(o) => {
                    let r = sp.obj[o as usize];
                    blit(sp, v, r, px + 8. - r.w / 2., py + 16. - r.h, false, Color::new(0.7, 0.85, 1., 0.22));
                    if is_wall_obj(o) {
                        // นั่งร้านไม้: เสา 2 ต้น + คานขวาง
                        prect(v, px + 1., py - 4., 2., 20., hx(0x8a6034));
                        prect(v, px + 13., py - 4., 2., 20., hx(0x8a6034));
                        prect(v, px, py + 3., 16., 2., hx(0xa87844));
                        if (x + y) % 2 == 0 { prect(v, px + 2., py + 10., 12., 1., hx(0x6a4420)); }
                    }
                }
            }
            if i == st.done && near {
                // ค้อนเคาะขึ้นลง + ฝุ่น
                let sw = ((t_now * 9.).sin() * 0.5 + 0.5) * 5.;
                let (hx0, hy0) = (px + 10., py - 10. + sw);
                prect(v, hx0, hy0, 2., 8., hx(0x8a5a2a));
                prect(v, hx0 - 2., hy0 - 2., 6., 3., hx(0x9aa0aa));
            }
        }
        // แถบความคืบหน้าเหนือไซต์
        let lay = prefab_layout(st.kind);
        let (x0, y0) = (lay.iter().map(|c| c.0 .0).min().unwrap(), lay.iter().map(|c| c.0 .1).min().unwrap());
        let x1 = lay.iter().map(|c| c.0 .0).max().unwrap();
        let (bx, by) = (((st.k.0 + x0) as f32 * 16. - v.cx) * v.s, ((st.k.1 + y0) as f32 * 16. - 14. - v.cy) * v.s);
        let bw = (x1 - x0 + 1) as f32 * 16. * v.s;
        let f = st.done as f32 / n as f32;
        draw_rectangle(bx, by, bw, 8. * v.s / 2., Color::new(0., 0., 0., 0.6));
        draw_rectangle(bx + 1., by + 1., (bw - 2.) * f, 8. * v.s / 2. - 2., hx(0x6ad04a));
        let label = if near { format!("กำลังสร้าง{} {}%", prefab_name(st.kind), (f * 100.) as i32) } else { format!("{} {}% — หยุดอยู่ เข้าไปใกล้ๆ เพื่อสร้างต่อ", prefab_name(st.kind), (f * 100.) as i32) };
        let fs = (5.5 * v.s) as u16;
        let d = measure_text(&label, Some(font), fs, 1.);
        draw_rectangle(bx + bw / 2. - d.width / 2. - 4., by - fs as f32 - 6., d.width + 8., fs as f32 + 6., Color::new(0., 0., 0., 0.55));
        draw_text_ex(&label, bx + bw / 2. - d.width / 2., by - 5., TextParams { font: Some(font), font_size: fs, color: if near { hx(0xd8ffb0) } else { hx(0xffd080) }, ..Default::default() });
    }
}
