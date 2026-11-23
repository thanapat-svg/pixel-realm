//! อาวุธระยะไกล (ธนู/คทา) · ชุดเกราะ · สูตรคราฟต์

use super::*;

pub const AR_RED: [f32; 5] = [0., 0.07, 0.12, 0.18, 0.25];
pub const AR_SPD: [f32; 5] = [0., 0.02, 0.04, 0.05, 0.06];
pub const AR_SPD_BOOT: [f32; 5] = [0., 0.05, 0.08, 0.12, 0.16];
pub const ATN: [&str; 5] = ["ไม่มี", "หิน", "อัญมณี", "ราชา", "ดาวตก"];
pub const ATC: [u32; 5] = [0x555566, 0x9aa0b4, 0x4ef0ff, 0xffc040, 0xc880ff];
pub const SLOT_N: [&str; 3] = ["หมวก", "เกราะอก", "รองเท้า"];
pub const SLOT_ICON: [usize; 3] = [IC_HELM, IC_CHEST, IC_BOOT];
pub const STAFF_N: [&str; 4] = ["-", "คทาเวทมนตร์", "คทาราชาสไลม์", "คทาดวงดาว"];

impl Player {
    pub fn reduction(&self) -> f32 {
        let base: f32 = self.armor.iter().map(|&t| AR_RED[t as usize]).sum();
        let plus: f32 = (0..3).filter(|&i| self.armor[i] > 0).map(|i| 0.015 * self.aplus[i] as f32).sum();
        (base + plus).min(0.75)
    }
    pub fn speed_mul(&self) -> f32 {
        (1. + AR_SPD[self.armor[0] as usize] + AR_SPD[self.armor[1] as usize] + AR_SPD_BOOT[self.armor[2] as usize] + 0.03 * self.st[2] as f32 + if self.buff_spd > 0. { 0.25 } else { 0. }) * self.body.speed_mul()
    }
    pub fn has_weapon(&self, w: u8) -> bool {
        match w {
            0 => true,
            1 => self.bow,
            _ => self.staff > 0,
        }
    }
    pub fn weapon_name(&self) -> String {
        match self.weapon {
            0 if self.bare => "มือเปล่า".to_string(),
            0 => SWN[self.sword].to_string(),
            1 => "ธนู".to_string(),
            _ => format!("{} ({})", STAFF_N[self.staff as usize], ELEM_N[self.elem as usize]),
        }
    }
}

// ---------------- โปรเจกไทล์ ----------------
/// kind: 0 ลูกธนู · 1 เวทมนตร์ · 2 เมือกราชาสไลม์ · 3 ก้อนหินโกเลม · 4 (สำรอง) · 5 เศษหิน
pub struct Proj {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub kind: u8,
    pub dmg: i32,
    pub life: f32,
    pub mine: bool,
    pub dead: bool,
}
pub const PR: [f32; 10] = [3., 3.5, 3., 5., 4., 2.5, 3.5, 3.5, 3.5, 3.5];
pub const ELEM_N: [&str; 3] = ["ไฟ", "น้ำแข็ง", "สายฟ้า"];
pub const ELEM_C: [u32; 3] = [0xff8a30, 0x7ad0ff, 0xfff060];

impl Game {
    /// หันตัวไปตามทิศที่เล็ง
    pub fn face(&mut self, a: Vec2) {
        if a.x.abs() > a.y.abs() {
            self.p.dir = 2;
            self.p.flip = a.x < 0.;
        } else {
            self.p.dir = if a.y > 0. { 0 } else { 1 };
        }
    }
    /// รายการของที่ถือได้ตามลำดับสลับ: มือเปล่า · ดาบ · ธนู · คทา(แต่ละธาตุ)
    pub fn hand_options(&self) -> Vec<(u8, bool, u8)> {
        let mut v = vec![(0u8, true, 0u8)];
        if self.p.sword > 0 { v.push((0, false, 0)); }
        if self.p.bow { v.push((1, false, 0)); }
        if self.p.staff > 0 { for e in 0..3u8 { if self.p.elems & (1 << e) != 0 { v.push((2, false, e)); } } }
        v
    }
    pub fn hand_cur(&self) -> (u8, bool, u8) {
        let bare = self.p.bare || (self.p.weapon == 0 && self.p.sword == 0);
        (self.p.weapon, bare && self.p.weapon == 0, if self.p.weapon == 2 { self.p.elem } else { 0 })
    }
    pub fn set_hand(&mut self, o: (u8, bool, u8)) {
        self.p.weapon = o.0;
        self.p.bare = o.1;
        self.p.elem = if o.0 == 2 { o.2 } else { 0 };
        let n = self.p.weapon_name();
        self.toast(format!("ถือ: {}", n));
    }
    /// สลับของในมือ (วนรอบ รวมมือเปล่า)
    pub fn swap_weapon(&mut self) {
        let opts = self.hand_options();
        if opts.len() <= 1 { self.toast("ยังไม่มีอาวุธ — คราฟต์ดาบ ธนู หรือคทาได้ที่เมนู (C)"); return; }
        let cur = self.hand_cur();
        let i = opts.iter().position(|&o| o == cur).unwrap_or(0);
        self.set_hand(opts[(i + 1) % opts.len()]);
    }
    /// ยิงโปรเจกไทล์ตามอาวุธที่ถืออยู่ (aim = ทิศเล็ง, None = ทิศที่หันหน้า)
    pub fn shoot(&mut self, aim: Option<Vec2>) {
        let a = match aim {
            Some(v) if v.length() > 0.01 => v.normalize(),
            _ => {
                let f = self.p.facing();
                vec2(f.0, f.1)
            }
        };
        self.face(a);
        let (ox, oy) = (self.p.x + a.x * 8., self.p.y - 6. + a.y * 8.);
        let bonus = self.p.lvl / 5;
        if self.p.weapon == 1 {
            if self.p.inv[ARROW] <= 0 {
                self.p.atk_cd = 0.4;
                self.toast("ลูกธนูหมด — คราฟต์ (C) หรือซื้อจากพ่อค้า");
                return;
            }
            self.p.inv[ARROW] -= 1;
            self.p.atk_cd = 0.42;
            self.p.atk_t = 0.1;
            self.proj.push(Proj { x: ox, y: oy, vx: a.x * 240., vy: a.y * 240., kind: 0, dmg: 3 + bonus, life: 0.9, mine: true, dead: false });
            play(&self.sfx.shoot);
        } else {
            self.p.atk_cd = 0.55;
            self.p.atk_t = 0.1;
            let (n, dmg) = if self.p.staff >= 3 { (5, 8 + bonus) } else if self.p.staff >= 2 { (3, 6 + bonus) } else { (1, 4 + bonus) };
            let mk = [7u8, 8, 9][self.p.elem as usize];
            let base = a.y.atan2(a.x);
            for i in 0..n {
                let ang = base + (i as f32 - (n as f32 - 1.) / 2.) * 0.26;
                self.proj.push(Proj { x: ox, y: oy, vx: ang.cos() * 170., vy: ang.sin() * 170., kind: mk, dmg, life: 1.1, mine: true, dead: false });
            }
            play(&self.sfx.shoot);
        }
    }
    pub fn hit_enemy(&mut self, i: usize, dmg: i32, dx: f32, dy: f32) {
        let (d, crit) = self.roll_dmg(dmg);
        self.damage_enemy(i, d, crit, dx, dy);
    }
    pub fn upd_proj(&mut self, dt: f32) {
        let mut ps = std::mem::take(&mut self.proj);
        for p in ps.iter_mut() {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.life -= dt;
            if p.life <= 0. {
                p.dead = true;
                continue;
            }
            let (tx, ty) = (tfloor(p.x), tfloor(p.y + 5.));
            if OSOLID[self.w.obj_at(tx, ty) as usize] || self.w.tile_at(tx, ty) == 6 {
                p.dead = true;
                let c = if p.kind == 6 { hx(0xff9a30) } else if matches!(p.kind, 1 | 7 | 8 | 9) { hx(match p.kind { 7 => 0xff8a30, 9 => 0xfff060, _ => 0x7ae8ff }) } else if p.kind == 2 { hx(0x6cd06c) } else if p.kind >= 3 { hx(0x9a9aa6) } else { hx(0xd8d8e0) };
                self.burst(p.x, p.y, c, 5, 40.);
                continue;
            }
            if matches!(p.kind, 1 | 7 | 8 | 9) && gen_range(0., 1.) < 0.6 {
                self.parts.push(Particle { x: p.x, y: p.y, vx: 0., vy: -6., l: 0.3, c: hx(match p.kind { 7 => 0xff8a30, 9 => 0xfff060, _ => 0x7ae8ff }) });
            }
            let r = PR[p.kind as usize];
            if p.mine {
                let mut hit: Option<usize> = None;
                for (i, e) in self.en.iter().enumerate() {
                    if e.dead {
                        continue;
                    }
                    let ey = e.y - if e.k == EK::Bat { 14. } else { 6. };
                    if (e.x - p.x).hypot(ey - p.y) < r + 6. {
                        hit = Some(i);
                        break;
                    }
                }
                if let Some(i) = hit {
                    p.dead = true;
                    let l = p.vx.hypot(p.vy).max(0.001);
                    self.hit_enemy(i, p.dmg, p.vx / l, p.vy / l);
                    let (ex, ey) = (self.en[i].x, self.en[i].y - 8.);
                    if !self.en[i].dead {
                        match p.kind {
                            7 => { self.en[i].burn = 3.2; self.burst(ex, ey, hx(0xff8a30), 6, 50.); }
                            8 => { self.en[i].slow = 3.5; self.burst(ex, ey, hx(0xa0e8ff), 6, 50.); }
                            9 => {
                                if gen_range(0., 1.) < 0.3 {
                                    self.en[i].stun = 1.2;
                                    self.add_float(ex, ey - 14., "สตัน!", hx(0xfff060), None);
                                    self.burst(ex, ey, hx(0xfff060), 10, 70.);
                                }
                            }
                            _ => {}
                        }
                    }
                    continue;
                }
                if let Some(i) = self.animal_at(p.x, p.y, r) {
                    p.dead = true;
                    let l = p.vx.hypot(p.vy).max(0.001);
                    self.hit_animal(i, p.dmg, p.vx / l, p.vy / l);
                    continue;
                }
                let mut bh: Option<usize> = None;
                for (i, b) in self.bosses.iter().enumerate() {
                    if !b.dead && (b.x - p.x).hypot(b.y - 12. - p.y) < r + BOSS_R {
                        bh = Some(i);
                        break;
                    }
                }
                if let Some(i) = bh {
                    p.dead = true;
                    self.hit_boss(i, p.dmg);
                    match p.kind {
                        7 => { if let Some(b) = self.bosses.get_mut(i) { b.burn = 3.2; } }
                        8 => { if let Some(b) = self.bosses.get_mut(i) { b.slow = 3.; } }
                        9 => { if gen_range(0., 1.) < 0.3 { self.hit_boss(i, (p.dmg / 2).max(1)); self.burst(p.x, p.y, hx(0xfff060), 8, 60.); } }
                        _ => {}
                    }
                }
            } else {
                let (dx, dy) = (self.p.x - p.x, self.p.y - 6. - p.y);
                let d = dx.hypot(dy).max(0.001);
                if d < r + 5. && self.p.invul <= 0. {
                    p.dead = true;
                    self.hurt(p.dmg, dx / d, dy / d);
                }
            }
        }
        ps.retain(|p| !p.dead);
        let added = std::mem::take(&mut self.proj);
        ps.extend(added);
        self.proj = ps;
    }
}

pub fn draw_proj(g: &Game, sp: &Sprites, v: &View) {
    for p in &g.proj {
        let r = sp.ex.proj[p.kind as usize];
        let (sx, sy) = (p.x - v.cx, p.y - v.cy);
        // เงา
        prect(v, (sx - 2.).round(), (sy + 5.).round(), 4., 1., Color::new(0., 0., 0., 0.25));
        let (w, h) = (r.w * v.s, r.h * v.s);
        let (px, py) = (sx * v.s - w / 2., sy * v.s - h / 2.);
        let rot = if p.kind == 0 { p.vy.atan2(p.vx) } else { 0. };
        if p.mine && g.p.wplus[g.p.weapon as usize] > 0 {
            let mut ac = hx(AURA[g.p.wplus[g.p.weapon as usize] as usize]);
            ac.a = 0.35;
            draw_circle(sx * v.s, sy * v.s, 5. * v.s, ac);
        }
        draw_texture_ex(&sp.tex, px, py, WHITE, DrawTextureParams { source: Some(r), dest_size: Some(vec2(w, h)), rotation: rot, pivot: Some(vec2(px + w / 2., py + h / 2.)), ..Default::default() });
    }
}

// ---------------- สูตรคราฟต์ ----------------
#[derive(Clone, Copy)]
pub enum Eff {
    Sword(usize),
    Pick,
    Fire,
    Heart,
    Bow,
    Staff(u8),
    Arrows(i32),
    Elem(u8),
    Rod,
    Armor(usize, u8),
    Can,
    Outfit(u8),
    Coat,
    Boat,
    /// v33 หุ่นเฝ้าบ้าน (0 หุ่นไม้ · 1 โกเลมหิน)
    Guard(u8),
}
pub struct Recipe {
    pub name: &'static str,
    pub desc: &'static str,
    pub cost: &'static [(usize, i32)],
    /// ต้องมีไอเทมนี้ (เช่น พิมพ์เขียว) แต่ไม่ถูกใช้หมด
    pub need: Option<usize>,
    pub eff: Eff,
}
pub const RECIPES: [Recipe; 30] = [
    Recipe { name: "ดาบไม้", desc: "พลังโจมตี 2", cost: &[(WOOD, 5)], need: None, eff: Eff::Sword(1) },
    Recipe { name: "อีเต้อหิน", desc: "ขุดภูเขาเปิดทางได้", cost: &[(WOOD, 4), (STONE, 4)], need: None, eff: Eff::Pick },
    Recipe { name: "ดาบหิน", desc: "พลังโจมตี 3 (ต้องมีดาบไม้)", cost: &[(WOOD, 3), (STONE, 6)], need: None, eff: Eff::Sword(2) },
    Recipe { name: "ดาบอัญมณี", desc: "พลังโจมตี 5 (ต้องมีดาบหิน)", cost: &[(GEM, 3), (STONE, 5)], need: None, eff: Eff::Sword(3) },
    Recipe { name: "ดาบแกนโกเลม", desc: "พลังโจมตี 8 · ต้องมีดาบอัญมณี + พิมพ์เขียวอาวุธ", cost: &[(CORE, 3), (STONE, 8)], need: Some(BP_WPN), eff: Eff::Sword(4) },
    Recipe { name: "ธนู", desc: "ยิงลูกธนูไกลๆ (ใช้ลูกธนูเป็นกระสุน)", cost: &[(WOOD, 10)], need: None, eff: Eff::Bow },
    Recipe { name: "ลูกธนู x10", desc: "กระสุนสำหรับธนู", cost: &[(WOOD, 3), (STONE, 1)], need: None, eff: Eff::Arrows(10) },
    Recipe { name: "คทาเวทมนตร์", desc: "ยิงลูกแก้วเวทไม่ต้องใช้กระสุน", cost: &[(GEM, 5), (WOOD, 4)], need: None, eff: Eff::Staff(1) },
    Recipe { name: "คทาราชาสไลม์", desc: "ยิงเวท 3 ทิศ · ต้องมีคทา + พิมพ์เขียวอาวุธ", cost: &[(GEL, 4), (GEM, 4)], need: Some(BP_WPN), eff: Eff::Staff(2) },
    Recipe { name: "กองไฟ", desc: "ให้แสง ไล่ศัตรู ฟื้นเลือดเมื่ออยู่ใกล้", cost: &[(WOOD, 3), (STONE, 2)], need: None, eff: Eff::Fire },
    Recipe { name: "หัวใจคริสตัล", desc: "เลือดสูงสุด +4", cost: &[(GEM, 2), (COIN, 15)], need: None, eff: Eff::Heart },
    Recipe { name: "หมวกหิน", desc: "ลดดาเมจ 7% · เดินเร็ว +2%", cost: &[(STONE, 6), (WOOD, 2)], need: None, eff: Eff::Armor(0, 1) },
    Recipe { name: "เกราะอกหิน", desc: "ลดดาเมจ 7% · เดินเร็ว +2%", cost: &[(STONE, 10), (WOOD, 3)], need: None, eff: Eff::Armor(1, 1) },
    Recipe { name: "รองเท้าหิน", desc: "ลดดาเมจ 7% · เดินเร็ว +5%", cost: &[(STONE, 5), (WOOD, 2)], need: None, eff: Eff::Armor(2, 1) },
    Recipe { name: "หมวกอัญมณี", desc: "ลดดาเมจ 12% · เดินเร็ว +4%", cost: &[(GEM, 3), (STONE, 4)], need: None, eff: Eff::Armor(0, 2) },
    Recipe { name: "เกราะอกอัญมณี", desc: "ลดดาเมจ 12% · เดินเร็ว +4%", cost: &[(GEM, 5), (STONE, 6)], need: None, eff: Eff::Armor(1, 2) },
    Recipe { name: "รองเท้าอัญมณี", desc: "ลดดาเมจ 12% · เดินเร็ว +8%", cost: &[(GEM, 3), (STONE, 3)], need: None, eff: Eff::Armor(2, 2) },
    Recipe { name: "หมวกราชาสไลม์", desc: "ลดดาเมจ 18% · เดินเร็ว +5% · ต้องมีพิมพ์เขียวเกราะ", cost: &[(GEL, 3), (CORE, 1)], need: Some(BP_ARM), eff: Eff::Armor(0, 3) },
    Recipe { name: "เกราะอกโกเลม", desc: "ลดดาเมจ 18% · เดินเร็ว +5% · ต้องมีพิมพ์เขียวเกราะ", cost: &[(CORE, 3), (GEL, 2)], need: Some(BP_ARM), eff: Eff::Armor(1, 3) },
    Recipe { name: "รองเท้าราชาสไลม์", desc: "ลดดาเมจ 18% · เดินเร็ว +12% · ต้องมีพิมพ์เขียวเกราะ", cost: &[(GEL, 3), (CORE, 1)], need: Some(BP_ARM), eff: Eff::Armor(2, 3) },
    Recipe { name: "ธาตุน้ำแข็ง (คทา)", desc: "เวทน้ำแข็ง ศัตรูเดินช้าลง · ต้องมีคทา", cost: &[(GEM, 4), (GEL, 2)], need: None, eff: Eff::Elem(1) },
    Recipe { name: "ธาตุสายฟ้า (คทา)", desc: "เวทสายฟ้า โอกาสทำให้สตัน · ต้องมีคทา", cost: &[(GEM, 4), (SHARD, 3)], need: None, eff: Eff::Elem(2) },
    Recipe { name: "คันเบ็ด", desc: "ตกปลาที่แหล่งน้ำ (ใช้ Q / ปุ่ม \"ใช้\") — ได้ปลาไว้ทำอาหารหรือขาย", cost: &[(WOOD, 8), (STONE, 2)], need: None, eff: Eff::Rod },
    Recipe { name: "บัวรดน้ำ", desc: "รดน้ำพืชผล (เติมที่แหล่งน้ำ จุ 8 ครั้ง) — ใช้ Q หันหน้าหาแปลง", cost: &[(WOOD, 3), (STONE, 3)], need: None, eff: Eff::Can },
    Recipe { name: "ชุดชาวไร่", desc: "หมวกฟาง+เสื้อเอี๊ยม เปลี่ยนรูปตัวละคร (เกราะทับได้)", cost: &[(WOOD, 4), (CORN, 3)], need: None, eff: Eff::Outfit(1) },
    Recipe { name: "เรือ", desc: "เดินลงน้ำได้ ล่องข้ามทะเลไปเกาะอื่น (เร็วกว่าเดิน · มอนสเตอร์ตามไม่ได้)", cost: &[(WOOD, 12)], need: None, eff: Eff::Boat },
    Recipe { name: "เสื้อขนสัตว์", desc: "ใช้หนังสัตว์ 5 ผืน (ล่าสัตว์ได้) + ไม้ 2 · หน้าหนาว/ในหิมะไม่หิวเร็วขึ้น", cost: &[(WOOD, 2)], need: None, eff: Eff::Coat },
    Recipe { name: "ชุดนักล่า", desc: "หมวกขนนก+เสื้อหนังสีเขียว เปลี่ยนรูปตัวละคร", cost: &[(WOOD, 3), (MEAT, 3), (HERB, 2)], need: None, eff: Eff::Outfit(2) },
    Recipe { name: "หุ่นไม้ยาม", desc: "ยามไม้เฝ้าจุดที่คุณยืน ตีมอนสเตอร์ที่เข้าใกล้ (ไม่มีความรู้สึก ไม่ต้องดูแล) · พวกสูงสุด 4", cost: &[(WOOD, 14), (STONE, 3)], need: None, eff: Eff::Guard(0) },
    Recipe { name: "โกเลมหินเฝ้าบ้าน", desc: "โกเลมหินแข็งแรง เฝ้าบ้านทั้งคืน ตีแรงกว่าหุ่นไม้ · ใช้แกนโกเลมจากบอส · พวกสูงสุด 4", cost: &[(STONE, 16), (CORE, 2), (GEM, 2)], need: None, eff: Eff::Guard(1) },
];

/// สูตรของโต๊ะคราฟต์ขั้นสูง (ขั้น 2) — ต้องมีของขั้นก่อนหน้า
pub const ADV_RECIPES: [Recipe; 5] = [
    Recipe { name: "ดาบคริสตัลดาว", desc: "พลังโจมตี 12 · ต้องมีดาบแกนโกเลม", cost: &[(CORE, 4), (SHARD, 6), (GEM, 6)], need: None, eff: Eff::Sword(5) },
    Recipe { name: "คทาดวงดาว", desc: "ยิงเวท 5 ทิศ ดาเมจสูง · ต้องมีคทาราชาสไลม์", cost: &[(SHARD, 8), (GEL, 5), (GEM, 6)], need: None, eff: Eff::Staff(3) },
    Recipe { name: "หมวกดาวตก", desc: "ลดดาเมจ 25% · เดินเร็ว +6% · ต้องมีหมวกราชา", cost: &[(SHARD, 5), (CORE, 2), (GEM, 4)], need: None, eff: Eff::Armor(0, 4) },
    Recipe { name: "เกราะอกดาวตก", desc: "ลดดาเมจ 25% · เดินเร็ว +6% · ต้องมีเกราะอกระดับ 3", cost: &[(SHARD, 8), (CORE, 3), (GEM, 6)], need: None, eff: Eff::Armor(1, 4) },
    Recipe { name: "รองเท้าดาวตก", desc: "ลดดาเมจ 25% · เดินเร็ว +16% · ต้องมีรองเท้าระดับ 3", cost: &[(SHARD, 5), (CORE, 2), (GEL, 3)], need: None, eff: Eff::Armor(2, 4) },
];

pub fn can_pay(p: &Player, c: &[(usize, i32)]) -> bool {
    c.iter().all(|&(i, n)| p.inv[i] >= n)
}
/// 0 = คราฟต์ได้, 1 = มีแล้ว, 2 = ล็อกอยู่ (ต้องมีของก่อนหน้า/พิมพ์เขียว)
pub fn recipe_state(p: &Player, r: &Recipe) -> u8 {
    let locked = r.need.map_or(false, |n| p.inv[n] < 1);
    match r.eff {
        Eff::Sword(t) => {
            if p.sword >= t { 1 } else if p.sword + 1 < t || locked { 2 } else { 0 }
        }
        Eff::Pick => if p.pick { 1 } else { 0 },
        Eff::Can => if p.can { 1 } else { 0 },
        Eff::Outfit(o) => if p.outfits & (1 << (o - 1)) != 0 { 1 } else { 0 },
        Eff::Coat => if p.coat { 1 } else if p.hides < HIDE_COAT { 2 } else { 0 },
        Eff::Boat => if p.boat { 1 } else { 0 },
        Eff::Rod => if p.rod { 1 } else { 0 },
        Eff::Bow => if p.bow { 1 } else { 0 },
        Eff::Staff(t) => {
            if p.staff >= t { 1 } else if p.staff + 1 < t || locked { 2 } else { 0 }
        }
        Eff::Armor(s, t) => {
            if p.armor[s] >= t { 1 } else if locked || (t >= 4 && p.armor[s] + 1 < t) { 2 } else { 0 }
        }
        Eff::Elem(e) => if p.staff == 0 { 2 } else if p.elems & (1 << e) != 0 { 1 } else { 0 },
        _ => if locked { 2 } else { 0 },
    }
}

impl Game {
    pub fn craft(&mut self, ri: usize) {
        self.craft_from(&RECIPES, ri);
    }
    pub fn craft_from(&mut self, list: &'static [Recipe], ri: usize) {
        let r = &list[ri];
        if recipe_state(&self.p, r) != 0 || !can_pay(&self.p, r.cost) {
            return;
        }
        if matches!(r.eff, Eff::Guard(_)) && self.rs.crew.m.len() >= crew::CREW_MAX {
            self.toast("พวกเต็มแล้ว (4) — ปล่อย/รื้อตัวเก่าที่แท็บชุมชน → พวกของเรา ก่อน");
            return;
        }
        for &(it, n) in r.cost {
            self.p.inv[it] -= n;
        }
        self.ev[0] += 1;
        match r.eff {
            Eff::Sword(t) => {
                self.p.sword = t;
                self.p.weapon = 0; self.p.bare = false;
            }
            Eff::Pick => self.p.pick = true,
            Eff::Rod => self.p.rod = true,
            Eff::Can => {
                self.p.can = true;
                self.p.water = WATER_MAX;
            }
            Eff::Fire => self.p.inv[FIRE] += 1,
            Eff::Heart => {
                self.p.maxhp += 4;
                self.p.hp = self.p.maxhp;
            }
            Eff::Bow => {
                self.p.bow = true;
                self.p.weapon = 1; self.p.bare = false;
            }
            Eff::Staff(t) => {
                self.p.staff = t;
                self.p.weapon = 2; self.p.bare = false;
            }
            Eff::Arrows(n) => self.p.inv[ARROW] += n,
            Eff::Elem(e) => {
                self.p.elems |= 1 << e;
                self.p.elem = e;
                self.p.weapon = 2; self.p.bare = false;
            }
            Eff::Armor(s, t) => self.p.armor[s] = t,
            Eff::Boat => {
                self.p.boat = true;
                self.toast("ได้เรือแล้ว! เดินลงน้ำเพื่อล่องเรือ");
            }
            Eff::Coat => {
                self.p.hides -= HIDE_COAT;
                self.p.coat = true;
                self.toast("ใส่เสื้อขนสัตว์แล้ว — อุ่นขึ้นในหน้าหนาว/หิมะ");
            }
            Eff::Outfit(o) => {
                self.p.outfits |= 1 << (o - 1);
                self.p.outfit = o;
            }
            Eff::Guard(k) => self.crew_craft(k),
        }
        play(if matches!(r.eff, Eff::Sword(_) | Eff::Bow | Eff::Staff(_) | Eff::Armor(..) | Eff::Elem(_) | Eff::Outfit(_)) { &self.sfx.upgrade } else { &self.sfx.craft });
        self.toast(format!("คราฟต์ {} สำเร็จ!", r.name));
        self.save();
    }
}
