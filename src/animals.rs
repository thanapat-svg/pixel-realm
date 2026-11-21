// animals.rs — v8d: สัตว์ป่า
//  เหยื่อ: กวาง (ป่า/ทุ่งหญ้า) · กระต่าย (ทุ่งหญ้า/สะวันนา/หิมะ — หน้าหนาวขนขาว) · แอนทิโลป (สะวันนา) — วิ่งหนีผู้เล่นและหมาป่า
//  ดุ: หมูป่า (ป่า/หนองน้ำ — หวงถิ่น เข้าใกล้หรือโดนตีจะพุ่งชน · เจ็บหนักจะหนี) · หมาป่า (ป่า/หิมะ ออกตอนกลางคืน/หน้าหนาว — ล่าเหยื่อและผู้เล่น · กินอิ่มแล้วไม่ล่า)
//       จระเข้ (หนองน้ำ — นิ่งเหมือนขอนไม้ เข้าใกล้แล้วงับ)
//  นก: ฝูงนกบนทุ่ง/หาด บินหนีเมื่อเข้าใกล้ (บรรยากาศ)
//  ล่าแล้วได้เนื้อดิบ (กินดิบเสี่ยงท้องเสีย — ทำอาหารก่อน)
use super::*;

pub const A_DEER: u8 = 0;
pub const A_RABBIT: u8 = 1;
pub const A_BOAR: u8 = 2;
pub const A_WOLF: u8 = 3;
pub const A_CROC: u8 = 4;
pub const A_BIRD: u8 = 5;
pub const A_ANTE: u8 = 6;

pub const AN_NAME: [&str; 7] = ["กวาง", "กระต่าย", "หมูป่า", "หมาป่า", "จระเข้", "นก", "แอนทิโลป"];
/// (เลือด, ความเร็วเดิน, ความเร็ววิ่ง, ดาเมจ, เนื้อที่ได้, XP)
pub const AN_STAT: [(f32, f32, f32, i32, i32, i32); 7] = [
    (4., 18., 78., 0, 2, 3),
    (1., 14., 88., 0, 1, 1),
    (8., 14., 82., 2, 3, 6),
    (6., 22., 74., 2, 1, 6),
    (10., 6., 120., 3, 3, 8),
    (1., 0., 90., 0, 0, 0),
    (4., 20., 84., 0, 2, 3),
];

/// สถานะ: เดินเล่น/กินหญ้า · หนี · ไล่ล่า · พุ่งชน (หมูป่า/จระเข้) · บินหนี (นก)
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AS { Idle, Wander, Flee, Chase, Charge, Fly }

#[derive(Clone, Debug)]
pub struct Animal {
    pub k: u8,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub hp: f32,
    pub st: AS,
    pub t: f32,
    pub anim: f32,
    pub flip: bool,
    pub flash: f32,
    /// กินอิ่ม (หมาป่าไม่ล่าจนกว่าจะหมด) / คูลดาวน์โจมตี
    pub fed: f32,
    pub cd: f32,
    /// ความกล้า 0..1 (สุ่มต่อตัว)
    pub bold: f32,
    pub winter: bool,
    pub h: f32,
    pub dead: bool,
    /// สัตว์เลี้ยง (เชื่องแล้ว)
    pub tame: bool,
}

pub fn predator(k: u8) -> bool { matches!(k, A_WOLF | A_CROC | A_BOAR) }
pub fn prey(k: u8) -> bool { matches!(k, A_DEER | A_RABBIT | A_ANTE) }

impl Game {
    pub fn new_animal(&self, k: u8, x: f32, y: f32) -> Animal {
        let s = AN_STAT[k as usize];
        Animal { k, x, y, vx: 0., vy: 0., hp: s.0, st: AS::Idle, t: gen_range(0.5, 2.5), anim: 0., flip: gen_range(0., 1.) < 0.5, flash: 0., fed: 0., cd: 0., bold: gen_range(0., 1.), winter: season(self) == 3, h: 0., dead: false, tame: false }
    }
    /// สัตว์ชนิดไหนเกิดได้บนพื้นนี้ (ตามเวลา/ฤดู)
    pub fn animal_for(&self, t: u8, night: bool, roll: f32) -> Option<u8> {
        let winter = season(self) == 3;
        let table: &[(f32, u8)] = match (t, night) {
            (3, false) => &[(0.35, A_RABBIT), (0.6, A_DEER), (0.9, A_BIRD), (0.95, A_BOAR)],
            (3, true) => &[(0.25, A_RABBIT), (0.55, A_WOLF)],
            (4, false) => &[(0.4, A_DEER), (0.62, A_BOAR), (0.75, A_RABBIT), (0.85, A_BIRD), (if winter { 0.97 } else { 0.88 }, A_WOLF)],
            (4, true) => &[(0.2, A_DEER), (0.45, A_BOAR), (0.9, A_WOLF)],
            (7, _) => &[(0.45, A_RABBIT), (if night || winter { 0.95 } else { 0.7 }, A_WOLF)],
            (T_SAVANNA, false) => &[(0.45, A_ANTE), (0.7, A_RABBIT), (0.9, A_BIRD)],
            (T_SAVANNA, true) => &[(0.3, A_ANTE), (0.5, A_WOLF)],
            (T_SWAMP, _) => &[(0.4, A_CROC), (0.7, A_BOAR), (0.85, A_BIRD)],
            (2, false) => &[(0.6, A_BIRD)],
            _ => &[],
        };
        table.iter().find(|e| roll < e.0).map(|e| e.1)
    }
    pub fn try_spawn_animal(&mut self) {
        if self.w.dun.is_some() { return; }
        let night = self.darkv > 0.45;
        let cap = if season(self) == 3 { 6 } else { 10 };
        if self.animals.iter().filter(|a| a.k != A_BIRD && !a.tame).count() >= cap { return; }
        let a = gen_range(0., std::f32::consts::TAU);
        let r = gen_range(10., 17.) * 16.;
        let (x, y) = (self.p.x + a.cos() * r, self.p.y + a.sin() * r);
        let (tx, ty) = (tfloor(x), tfloor(y));
        let t = self.w.tile_at(tx, ty);
        if collide(&self.w, x, y) || self.w.camp_near(tx, ty, 8).is_some() || self.room_of.contains_key(&(tx, ty)) { return; }
        let Some(k) = self.animal_for(t, night, gen_range(0., 1.)) else { return };
        if predator(k) && (peaceful() || self.in_grace() || self.in_safe(x, y) || self.w.near_fire(tx, ty, 8).is_some()) { return; }
        // เหยื่อมาเป็นฝูงเล็ก · นกเป็นฝูง · หมาป่ามาเป็นคู่/ฝูง
        let n = match k { A_BIRD => rnd(3, 5), A_DEER | A_ANTE => rnd(1, 3), A_WOLF => if night { rnd(2, 3) } else { 1 }, A_RABBIT => rnd(1, 2), _ => 1 };
        for i in 0..n {
            let (ox, oy) = (gen_range(-20., 20.), gen_range(-20., 20.));
            if i > 0 && collide(&self.w, x + ox, y + oy) { continue; }
            let an = self.new_animal(k, x + if i > 0 { ox } else { 0. }, y + if i > 0 { oy } else { 0. });
            self.animals.push(an);
        }
    }

    pub fn animals_tick(&mut self, dt: f32) {
        if self.w.dun.is_some() { self.animals.retain(|a| a.tame); self.pets_tick(dt); return; }
        if peaceful() { self.animals.retain(|a| a.tame || !predator(a.k)); }
        self.an_spawn_t -= dt;
        if self.an_spawn_t <= 0. { self.an_spawn_t = 1.6; self.try_spawn_animal(); }
        let (px, py) = (self.p.x, self.p.y);
        let mut list = std::mem::take(&mut self.animals);
        let mut hurt: Option<(i32, f32, f32, u8)> = None;
        let mut eaten: Vec<usize> = vec![];
        let n = list.len();
        for i in 0..n {
            let (k, ax, ay) = (list[i].k, list[i].x, list[i].y);
            if list[i].dead || list[i].tame { continue; }
            let a = &mut list[i];
            a.t -= dt;
            a.anim += dt;
            if a.flash > 0. { a.flash -= dt; }
            if a.fed > 0. { a.fed -= dt; }
            if a.cd > 0. { a.cd -= dt; }
            let s = AN_STAT[k as usize];
            let dp = (px - ax).hypot(py - ay);
            let _ = &a;
            // ---- ตัดสินใจ
            // ภัยใกล้สุดสำหรับเหยื่อ: ผู้เล่นหรือนักล่า
            let mut threat: Option<(f32, f32, f32)> = None;
            if prey(k) || k == A_BIRD {
                let fear = match k { A_RABBIT => 72., A_DEER | A_ANTE => 92., _ => 60. } * (1.2 - list[i].bold * 0.4);
                if dp < fear && !self.p_sneak() { threat = Some((px, py, dp)); }
                for (j, o) in list.iter().enumerate() {
                    if j == i || o.dead || o.k != A_WOLF || o.tame { continue; }
                    let d = (o.x - ax).hypot(o.y - ay);
                    if d < 110. && threat.map_or(true, |t| d < t.2) { threat = Some((o.x, o.y, d)); }
                }
            }
            // เป้าหมายของนักล่า
            let mut target: Option<(f32, f32, Option<usize>)> = None;
            if k == A_WOLF && list[i].fed <= 0. {
                let night = self.darkv > 0.45;
                let aggro = if night { 120. } else { 70. } * aggro_mul();
                if dp < aggro && self.p.hp > 0 { target = Some((px, py, None)); }
                for (j, o) in list.iter().enumerate() {
                    if j == i || o.dead || !prey(o.k) || o.tame { continue; }
                    let d = (o.x - ax).hypot(o.y - ay);
                    let cur = target.map_or(f32::MAX, |t| (t.0 - ax).hypot(t.1 - ay));
                    if d < 130. && d < cur * 0.8 { target = Some((o.x, o.y, Some(j))); }
                }
            }
            let a = &mut list[i];
            if k == A_BOAR {
                let hurt_bad = a.hp <= s.0 * 0.3;
                if hurt_bad && dp < 100. { threat = Some((px, py, dp)); }
                else if (dp < 36. * aggro_mul() || a.st == AS::Charge) && dp < 140. { target = Some((px, py, None)); }
            }
            if k == A_CROC && (dp < 30. || (a.st == AS::Charge && dp < 70.)) { target = Some((px, py, None)); }

            if k == A_BIRD {
                if a.st != AS::Fly && threat.is_some() {
                    a.st = AS::Fly;
                    let (tx, ty, _) = threat.unwrap();
                    let l = (ax - tx).hypot(ay - ty).max(1.);
                    a.vx = (ax - tx) / l * s.2 + gen_range(-20., 20.);
                    a.vy = (ay - ty) / l * s.2 - 30.;
                    a.t = 3.;
                }
                if a.st == AS::Fly {
                    a.x += a.vx * dt;
                    a.y += a.vy * dt;
                    a.h = (a.h + 40. * dt).min(30.);
                    if a.t <= 0. { a.dead = true; }
                } else if a.t <= 0. {
                    a.t = gen_range(0.6, 2.);
                    a.x += gen_range(-4., 4.);
                    a.flip = gen_range(0., 1.) < 0.5;
                }
                a.flip = if a.st == AS::Fly { a.vx < 0. } else { a.flip };
                continue;
            }

            let (mut vx, mut vy) = (0., 0.);
            if let Some((tx, ty, _)) = threat {
                a.st = AS::Flee;
                let l = (ax - tx).hypot(ay - ty).max(1.);
                vx = (ax - tx) / l * s.2;
                vy = (ay - ty) / l * s.2;
                a.t = 0.8;
            } else if let Some((tx, ty, tj)) = target {
                let l = (tx - ax).hypot(ty - ay).max(1.);
                let sp = if k == A_CROC { if l < 30. || a.st == AS::Charge { s.2 } else { 0. } } else { s.2 };
                a.st = if k == A_WOLF { AS::Chase } else { AS::Charge };
                vx = (tx - ax) / l * sp;
                vy = (ty - ay) / l * sp;
                if l < 11. && a.cd <= 0. {
                    a.cd = if k == A_CROC { 1.6 } else { 1.1 };
                    match tj {
                        None => { if self.p.invul <= 0. { hurt = Some((s.3, (px - ax) / l, (py - ay) / l, k)); } }
                        Some(j) => { eaten.push(j); a.fed = 90.; a.st = AS::Wander; }
                    }
                    if k == A_BOAR || k == A_CROC { a.st = AS::Wander; a.t = 1.5; }
                }
            } else {
                if matches!(a.st, AS::Flee | AS::Chase | AS::Charge) { a.st = AS::Idle; a.t = gen_range(0.5, 1.5); }
                if a.t <= 0. {
                    if a.st == AS::Idle {
                        a.st = AS::Wander;
                        let ang = gen_range(0., std::f32::consts::TAU);
                        a.vx = ang.cos() * s.1;
                        a.vy = ang.sin() * s.1;
                        a.t = gen_range(1., 2.5);
                    } else {
                        a.st = AS::Idle;
                        a.t = gen_range(1., 3.5);
                    }
                }
                if a.st == AS::Wander { vx = a.vx; vy = a.vy; }
            }
            // จระเข้อยู่ได้แค่หนองน้ำ/ริมน้ำ
            let (nx, ny) = (ax + vx * dt, ay + vy * dt);
            let ok_tile = |g: &Game, x: f32, y: f32| k != A_CROC || matches!(g.w.tile_at(tfloor(x), tfloor(y)), T_SWAMP | 2);
            if !collide(&self.w, nx, ay) && ok_tile(self, nx, ay) { a.x = nx; } else if a.st == AS::Wander { a.t = 0.; }
            if !collide(&self.w, a.x, ny) && ok_tile(self, a.x, ny) { a.y = ny; } else if a.st == AS::Wander { a.t = 0.; }
            if vx.abs() > 1. { a.flip = vx < 0.; }
            a.vx = if a.st == AS::Wander { a.vx } else { vx };
            a.vy = if a.st == AS::Wander { a.vy } else { vy };
            // ไกลเกิน → หายไป
            if dp > 40. * 16. { a.dead = true; }
        }
        for j in eaten {
            if let Some(v) = list.get_mut(j) {
                if !v.dead {
                    v.dead = true;
                    let (x, y) = (v.x, v.y);
                    self.burst(x, y - 4., hx(0xb02020), 10, 50.);
                }
            }
        }
        list.retain(|a| !a.dead);
        list.extend(std::mem::take(&mut self.animals));
        self.animals = list;
        if let Some((d, nx, ny, k)) = hurt {
            self.hurt(d, nx, ny);
            if !self.an_told.contains(&k) {
                self.an_told.insert(k);
                self.toast(match k { A_WOLF => "หมาป่าโจมตี! พวกมันออกล่าตอนกลางคืนและหน้าหนาว", A_BOAR => "หมูป่าพุ่งชน! อย่าเข้าใกล้ถ้าไม่พร้อมสู้", _ => "จระเข้งับ! ระวังขอนไม้ในหนองน้ำ" });
            }
        }
    }
    /// ย่องเบา (ตอนนี้ยังไม่มีปุ่มย่อง — เผื่ออนาคต)
    pub fn p_sneak(&self) -> bool { false }

    /// สัตว์ที่อยู่ตรงจุดนี้ (ใช้กับลูกธนู/เวท)
    pub fn animal_at(&self, x: f32, y: f32, r: f32) -> Option<usize> {
        self.animals.iter().position(|a| !a.dead && !a.tame && a.k != A_BIRD && (a.x - x).hypot(a.y - 6. - y) < r + 6.)
    }
    pub fn hit_animal(&mut self, i: usize, dmg: i32, nx: f32, ny: f32) {
        let Some(a) = self.animals.get_mut(i) else { return };
        a.hp -= dmg as f32;
        a.flash = 0.15;
        a.x += nx * 6.;
        a.y += ny * 6.;
        // โดนตี: หมูป่าโกรธ · จระเข้งับกลับ · ตัวอื่นหนี
        match a.k { A_BOAR | A_CROC => { a.st = AS::Charge; } A_WOLF => {} _ => { a.st = AS::Flee; } }
        let (x, y, k) = (a.x, a.y, a.k);
        self.burst(x, y - 6., hx(0xb02020), 5, 40.);
        if self.animals[i].hp <= 0. {
            self.animals[i].dead = true;
            let s = AN_STAT[k as usize];
            if s.4 > 0 { self.gain(MEAT, s.4, x, y - 4.); }
            if k == A_WOLF && gen_range(0., 1.) < 0.3 { self.gain(COIN, rnd(2, 5), x, y - 10.); }
            self.quest_hunt(k);
            self.an_hunted |= 1 << k;
            if k != A_BIRD { self.add_hide(1); }
            self.p.xp += s.5;
            self.check_lv();
            self.burst(x, y - 6., hx(0xb02020), 12, 60.);
            self.animals.retain(|a| !a.dead);
        }
    }
    /// ฟันโดนสัตว์ (คืน true ถ้าโดน)
    pub fn melee_animals(&mut self, fx: f32, fy: f32, dmg: i32, dx: f32, dy: f32) -> bool {
        let mut hit = false;
        let mut i = 0;
        while i < self.animals.len() {
            let a = &self.animals[i];
            let n = self.animals.len();
            if a.k != A_BIRD && !a.dead && !a.tame && (a.x - fx).hypot(a.y - 5. - fy) < 15. {
                hit = true;
                self.hit_animal(i, dmg, dx, dy);
                if self.animals.len() < n { continue; }
            }
            i += 1;
        }
        hit
    }
}

// ======================= ภาพ =======================
const DEER: [&str; 12] = [
    "............a.a.",
    "...........a.a..",
    "...........bb...",
    "..........bbbe..",
    "..........bbbb..",
    "..bbbbbbbbbbb...",
    ".bbbbbbbbbbbb...",
    ".Wbbbbbbbbbbb...",
    "..bdddddddbb....",
    "..b.b....b.b....",
    "..b.b....b.b....",
    "..k.k....k.k....",
];
const DEER_L: [&str; 3] = ["...b.b..b.b.....", "...b.b..b.b.....", "...k.k..k.k....."];
const RABBIT: [&str; 7] = ["......pe..", "......pe..", "....bbbb..", "..bbbbbbkb", ".Wbbbbbbb.", "..bbbbbb..", "..b.b..b.."];
const RABBIT_L: [&str; 1] = ["...bb.b..."];
const BOAR: [&str; 9] = [
    "...hhhh.........",
    "..hbbbbbbbbb....",
    ".bbbbbbbbbbbb...",
    "tbbbbbbbbbbbbbb.",
    ".bbbbbbbbbbbbkbs",
    ".bbbbbbbbbbbbbbW",
    "..bbbbbbbbbbbb..",
    "..b.b.....b.b...",
    "..k.k.....k.k...",
];
const BOAR_L: [&str; 2] = ["...b.b...b.b....", "...k.k...k.k...."];
const WOLF: [&str; 9] = [
    "............e...",
    "...........gge..",
    "t.........ggggkn",
    ".tgggggggggggg..",
    "..ggggggggggg...",
    "..gggggggggg....",
    "..LLLLLLLLL.....",
    "..g.g....g.g....",
    "..g.g....g.g....",
];
const WOLF_L: [&str; 2] = ["...g.g..g.g.....", "...g.g..g.g....."];
const CROC: [&str; 5] = [
    "......s.s.s.s.........",
    "..gggggggggggggggggk..",
    "tggggggggggggggggggggg",
    "..gDDDDDDDDDDDDDDDDgWW",
    "...g.g......g.g.......",
];
const BIRD: [[&str; 3]; 2] = [["k....k", ".k..k.", "..kk.."], ["......", "kkkkkk", "..kk.."]];

fn an_img(rows: &[&str], legs: Option<&[&str]>, pal: &[(char, u32)]) -> Image {
    let mut r: Vec<&str> = rows.to_vec();
    if let Some(l) = legs { let n = r.len(); for (i, row) in l.iter().enumerate() { r[n - l.len() + i] = row; } }
    spr(&r, pal)
}

/// สไปรต์สัตว์: [ชนิด*2 + เฟรม] · กระต่ายขาว (หน้าหนาว) ต่อท้ายที่ 14,15
pub fn build_animals(at: &mut Atlas) -> Vec<Rect> {
    let deer: &[(char, u32)] = &[('a', 0x6a4a2a), ('b', 0xa0683a), ('d', 0xc89a6a), ('W', 0xf0e8d8), ('e', 0x1a1a1a), ('k', 0x3a2a1a)];
    let rab: &[(char, u32)] = &[('b', 0x9a7a5a), ('p', 0xe8a0a0), ('e', 0x7a5a3a), ('W', 0xf4f0e8), ('k', 0x1a1a1a)];
    let rabw: &[(char, u32)] = &[('b', 0xf0f2f6), ('p', 0xe8a0a0), ('e', 0xd8dce4), ('W', 0xffffff), ('k', 0x1a1a1a)];
    let boar: &[(char, u32)] = &[('h', 0x2a1a10), ('b', 0x5a3c26), ('t', 0x3a2618), ('k', 0x0a0a0a), ('s', 0x8a5a4a), ('W', 0xf0ead8)];
    let wolf: &[(char, u32)] = &[('g', 0x8a8e96), ('L', 0xc8ccd2), ('t', 0x6a6e76), ('e', 0x5a5e66), ('k', 0xffd040), ('n', 0x1a1a1a)];
    let croc: &[(char, u32)] = &[('g', 0x4a6a32), ('s', 0x3a5426), ('D', 0x8aa06a), ('t', 0x3a5426), ('k', 0xd8c040), ('W', 0xf0f0e0)];
    let ante: &[(char, u32)] = &[('a', 0x2a2018), ('b', 0xc8904a), ('d', 0xf0e0c0), ('W', 0xf8f0e0), ('e', 0x1a1a1a), ('k', 0x3a2a1a)];
    let bird: &[(char, u32)] = &[('k', 0x3a3a46)];
    let mut v = vec![];
    v.push(at.add(&an_img(&DEER, None, deer)));
    v.push(at.add(&an_img(&DEER, Some(&DEER_L), deer)));
    v.push(at.add(&an_img(&RABBIT, None, rab)));
    v.push(at.add(&an_img(&RABBIT, Some(&RABBIT_L), rab)));
    v.push(at.add(&an_img(&BOAR, None, boar)));
    v.push(at.add(&an_img(&BOAR, Some(&BOAR_L), boar)));
    v.push(at.add(&an_img(&WOLF, None, wolf)));
    v.push(at.add(&an_img(&WOLF, Some(&WOLF_L), wolf)));
    v.push(at.add(&an_img(&CROC, None, croc)));
    v.push(at.add(&an_img(&CROC, None, croc)));
    v.push(at.add(&spr(&BIRD[0], bird)));
    v.push(at.add(&spr(&BIRD[1], bird)));
    v.push(at.add(&an_img(&DEER, None, ante)));
    v.push(at.add(&an_img(&DEER, Some(&DEER_L), ante)));
    v.push(at.add(&an_img(&RABBIT, None, rabw)));
    v.push(at.add(&an_img(&RABBIT, Some(&RABBIT_L), rabw)));
    v
}

pub fn draw_animal(g: &Game, sp: &Sprites, v: &View, i: usize) {
    let a = &g.animals[i];
    let (sx, sy) = ((a.x - v.cx).round(), (a.y - v.cy).round());
    let moving = a.vx.abs() + a.vy.abs() > 1. && a.st != AS::Idle;
    let fr = if a.k == A_BIRD { if a.st == AS::Fly { (a.anim * 10.) as usize % 2 } else { 1 } } else if moving { (a.anim * if a.st == AS::Wander { 5. } else { 10. }) as usize % 2 } else { 0 };
    let base = if a.k == A_RABBIT && a.winter { 14 } else { a.k as usize * 2 };
    let r = sp.animals[base + fr];
    let shadow = Color::new(0., 0., 0., 0.22);
    let _ = shadow;
    if a.k != A_BIRD || a.h < 4. { vfx::char_shadow(v, sx, sy, r.w - 4.); }
    vfx::blit_outline(sp, v, r, sx - (r.w / 2.).floor(), sy - r.h - a.h, r.w, r.h, a.flip, 0.8);
    let c = if a.flash > 0. { Color::new(1., 0.5, 0.5, 1.) } else { WHITE };
    blit(sp, v, r, sx - (r.w / 2.).floor(), sy - r.h - a.h, a.flip, c);
    if a.tame {
        // ปลอกคอสีแดง + หัวใจเล็กลอยเป็นระยะ
        let nx = if a.flip { sx - r.w / 2. + 3. } else { sx + r.w / 2. - 6. };
        prect(v, nx, sy - r.h + if a.k == A_WOLF { 4. } else { 3. }, 3., 1., hx(0xe02020));
        if (a.anim * 0.5).fract() < 0.25 { prect(v, sx - 1., sy - r.h - 5., 1., 1., hx(0xff6aa0)); prect(v, sx + 1., sy - r.h - 5., 1., 1., hx(0xff6aa0)); prect(v, sx - 1., sy - r.h - 4., 3., 1., hx(0xff6aa0)); prect(v, sx, sy - r.h - 3., 1., 1., hx(0xff6aa0)); }
    }
}
