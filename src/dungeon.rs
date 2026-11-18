//! ดันเจี้ยนใต้ดิน: ทางเข้าสุ่มตามไบโอม · ห้อง/ทางเดิน · กับดักหนาม/ป้อมลูกไฟ · มินิบอส · หีบสมบัติหายาก
//! ตัวดันเจี้ยนอยู่ใน World.dun (ระหว่างอยู่ข้างใน การถามไทล์/วัตถุทั้งหมดจะตอบจากแผนที่ดันเจี้ยน)

use super::*;

/// ขนาดเซลล์ของโลกที่ใช้สุ่มทางเข้า (ไทล์)
pub const DCELL: i32 = 52;
pub const DW: i32 = 57;
pub const DH: i32 = 39;
pub const T_FLOOR: u8 = 9;
pub const T_WALL: u8 = 10;

pub struct DRng(u64);
impl DRng {
    fn new(seed: u64) -> Self {
        DRng(seed.wrapping_mul(0x9E3779B97F4A7C15) ^ 0xD1B54A32D192ED03)
    }
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 20) as u32
    }
    /// ช่วง a..=b
    fn range(&mut self, a: i32, b: i32) -> i32 {
        if b <= a { a } else { a + (self.next() % (b - a + 1) as u32) as i32 }
    }
    fn f(&mut self) -> f32 {
        (self.next() % 10000) as f32 / 10000.
    }
}

pub struct Dungeon {
    pub cell: (i32, i32),
    pub theme: u8,
    pub tiles: Vec<u8>,
    pub objs: Vec<u8>,
    pub entry: (i32, i32),
    pub boss_at: (i32, i32),
    pub boss_kind: u8,
    pub spikes: Vec<(i32, i32)>,
    pub turrets: Vec<(i32, i32)>,
    /// หีบสมบัติ: ตำแหน่งตามรหัสหีบ (รหัสสุดท้าย = หีบรางวัลมินิบอส)
    pub chests: Vec<(i32, i32)>,
    pub spawns: Vec<(EK, f32, f32)>,
}
impl Dungeon {
    pub fn tile(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 || x >= DW || y >= DH { T_WALL } else { self.tiles[(y * DW + x) as usize] }
    }
    pub fn obj(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 || x >= DW || y >= DH { 0 } else { self.objs[(y * DW + x) as usize] }
    }
    fn set_obj(&mut self, x: i32, y: i32, o: u8) {
        if x >= 0 && y >= 0 && x < DW && y < DH {
            self.objs[(y * DW + x) as usize] = o;
        }
    }
    fn carve(&mut self, x: i32, y: i32) {
        if x > 0 && y > 0 && x < DW - 1 && y < DH - 1 {
            self.tiles[(y * DW + x) as usize] = T_FLOOR;
        }
    }
    fn free(&self, x: i32, y: i32) -> bool {
        self.tile(x, y) == T_FLOOR && self.obj(x, y) == 0
    }

    pub fn generate(seed: i32, cell: (i32, i32), theme: u8, cleared: bool, opened: &HashSet<(i32, i32, i32)>) -> Dungeon {
        let mut r = DRng::new((seed as u64) ^ ((cell.0 as i64 as u64) << 32) ^ (cell.1 as i64 as u64).wrapping_mul(7919));
        let mut d = Dungeon {
            cell, theme, tiles: vec![T_WALL; (DW * DH) as usize], objs: vec![0; (DW * DH) as usize], entry: (0, 0), boss_at: (0, 0),
            boss_kind: if matches!(theme, 3 | 4 | 8) { 0 } else { 1 }, spikes: vec![], turrets: vec![], chests: vec![], spawns: vec![],
        };
        let (cw, ch) = (DW / 3, DH / 3);
        // ห้อง 3x3 เรียงงู (0,0)->(2,0)->(2,1)->(0,1)->(0,2)->(2,2)
        let order = [(0, 0), (1, 0), (2, 0), (2, 1), (1, 1), (0, 1), (0, 2), (1, 2), (2, 2)];
        let mut rooms: Vec<(i32, i32, i32, i32)> = vec![];
        for &(cx, cy) in &order {
            let w = r.range(8, cw - 3);
            let h = r.range(6, ch - 3);
            let x = cx * cw + 1 + r.range(0, cw - 2 - w);
            let y = cy * ch + 1 + r.range(0, ch - 2 - h);
            for j in 0..h {
                for i in 0..w {
                    d.carve(x + i, y + j);
                }
            }
            rooms.push((x, y, w, h));
        }
        let center = |rm: &(i32, i32, i32, i32)| (rm.0 + rm.2 / 2, rm.1 + rm.3 / 2);
        let mut links: Vec<(usize, usize)> = (0..8).map(|i| (i, i + 1)).collect();
        links.push((1, 4));
        for (a, b) in links {
            let (ax, ay) = center(&rooms[a]);
            let (bx, by) = center(&rooms[b]);
            for x in ax.min(bx)..=ax.max(bx) {
                d.carve(x, ay);
                d.carve(x, ay + 1);
            }
            for y in ay.min(by)..=ay.max(by) {
                d.carve(bx, y);
                d.carve(bx + 1, y);
            }
            // หนามกลางทางเดิน
            if a >= 1 && r.f() < 0.7 {
                let (mx, my) = ((ax + bx) / 2, ay);
                if d.tile(mx, my) == T_FLOOR && (mx - ax).abs() > 4 {
                    d.spikes.push((mx, my));
                    d.spikes.push((mx, my + 1));
                }
            }
        }
        // ห้องแรก = ทางออก
        let er = rooms[0];
        let ec = center(&er);
        d.set_obj(er.0 + 1, ec.1, 20);
        d.entry = (er.0 + 2, ec.1);
        // ห้องบอส
        let br = rooms[8];
        let bc = center(&br);
        d.boss_at = bc;
        let mut chest_rooms: Vec<usize> = vec![2, 4, 6];
        if r.f() < 0.5 { chest_rooms.push(5); }
        for (i, rm) in rooms.iter().enumerate() {
            let (x, y, w, h) = *rm;
            // คบเพลิงมุมบน
            d.set_obj(x, y, 18);
            d.set_obj(x + w - 1, y, 18);
            if i == 0 {
                continue;
            }
            if i >= 2 && i != 8 {
                let n = r.range(1, 2);
                for k in 0..n {
                    let (tx, ty) = if k == 0 { (x, y + h - 1) } else { (x + w - 1, y + h - 1) };
                    d.set_obj(tx, ty, 23);
                    d.turrets.push((tx, ty));
                }
            }
            if i != 8 {
                for _ in 0..r.range(2, 6) {
                    let (sx, sy) = (x + 1 + r.range(0, w - 3), y + 1 + r.range(0, h - 3));
                    if d.free(sx, sy) && (sx, sy) != center(rm) {
                        d.spikes.push((sx, sy));
                        d.set_obj(sx, sy, 21);
                    }
                }
            }
            if chest_rooms.contains(&i) {
                let (cx, cy) = (x + 1 + r.range(0, w - 3), y + 2 + r.range(0, h - 4));
                if d.free(cx, cy) {
                    let id = d.chests.len() as i32;
                    d.chests.push((cx, cy));
                    if !opened.contains(&(cell.0, cell.1, id)) {
                        d.set_obj(cx, cy, 24);
                    }
                }
            }
            if i != 8 {
                for _ in 0..r.range(2, 4) {
                    let (ex, ey) = (x + 1 + r.range(0, w - 3), y + 1 + r.range(0, h - 3));
                    if d.free(ex, ey) {
                        let q = r.f();
                        let k = if q < 0.4 { EK::Slime } else if q < 0.75 { EK::Skel } else { EK::Bat };
                        d.spikes.retain(|&s| s != (ex, ey));
                        d.spawns.push((k, ex as f32 * 16. + 8., ey as f32 * 16. + 12.));
                    }
                }
            }
        }
        // หีบรางวัลมินิบอส (รหัสสุดท้าย) — วางตอนบอสตาย หรือถ้าเคลียร์แล้วและยังไม่เปิด
        let bid = d.chests.len() as i32;
        d.chests.push((bc.0, bc.1 + 2));
        if cleared && !opened.contains(&(cell.0, cell.1, bid)) {
            d.set_obj(bc.0, bc.1 + 2, 24);
        }
        // ห้ามมีหนามซ้อนทับวัตถุอื่น
        let objs = d.objs.clone();
        d.spikes.retain(|&(x, y)| matches!(objs[(y * DW + x) as usize], 0 | 21));
        let sp = d.spikes.clone();
        for (x, y) in sp { if d.obj(x, y) == 0 { d.set_obj(x, y, 21); } }
        d
    }
}

/// หนามโผล่ตามจังหวะเวลา
pub fn spike_on(time: f32, x: i32, y: i32) -> bool {
    ((time * 0.6 + hash(x, y, 5) as f32 * 3.).fract()) < 0.45
}

impl World {
    /// ทางเข้าดันเจี้ยนในเซลล์ (จำผลไว้)
    pub fn dun_in_cell(&self, cx: i32, cy: i32) -> Option<(i32, i32)> {
        if self.dun.is_some() {
            return None;
        }
        if let Some(&r) = self.duns.borrow().get(&(cx, cy)) {
            return r;
        }
        let r = self.calc_dun(cx, cy);
        self.duns.borrow_mut().insert((cx, cy), r);
        r
    }
    fn calc_dun(&self, cx: i32, cy: i32) -> Option<(i32, i32)> {
        let s = self.seed.wrapping_add(9091);
        if hash(cx, cy, s) > 0.34 {
            return None;
        }
        for k in 0..6 {
            let tx = cx * DCELL + 4 + (hash(cx, cy, s.wrapping_add(1 + k * 2)) * (DCELL - 8) as f64) as i32;
            let ty = cy * DCELL + 4 + (hash(cx, cy, s.wrapping_add(2 + k * 2)) * (DCELL - 8) as f64) as i32;
            if matches!(self.tile_at(tx, ty), 2 | 3 | 4 | 5 | 7 | 8) && self.camp_near(tx, ty, 8).is_none() {
                return Some((tx, ty));
            }
        }
        None
    }
    pub fn dun_near(&self, tx: i32, ty: i32, r: i32) -> Option<(i32, i32)> {
        for cy in (ty - r).div_euclid(DCELL)..=(ty + r).div_euclid(DCELL) {
            for cx in (tx - r).div_euclid(DCELL)..=(tx + r).div_euclid(DCELL) {
                if let Some((mx, my)) = self.dun_in_cell(cx, cy) {
                    if (mx - tx).abs() <= r && (my - ty).abs() <= r {
                        return Some((mx, my));
                    }
                }
            }
        }
        None
    }
}

impl Game {
    pub fn discover_duns(&mut self) {
        if self.w.dun.is_some() {
            return;
        }
        let (tx, ty) = (tfloor(self.p.x), tfloor(self.p.y));
        if let Some(c) = self.w.dun_near(tx, ty, 26) {
            if self.dun_found.insert(c) {
                self.toast("พบทางเข้าดันเจี้ยนลับ! ดูตำแหน่ง (D) บนแผนที่");
                self.p.xp += 3;
                self.check_lv();
                self.save();
            }
        }
    }

    pub fn enter_dungeon(&mut self, tile: (i32, i32)) {
        let cell = (tile.0.div_euclid(DCELL), tile.1.div_euclid(DCELL));
        let theme = self.w.tile_at(tile.0, tile.1);
        let cleared = self.dun_clear.contains(&cell);
        let d = Dungeon::generate(self.w.seed, cell, theme, cleared, &self.dun_chests);
        let mul = 1.25 + (tile.0 as f32 - self.spawn.0 / 16.).hypot(tile.1 as f32 - self.spawn.1 / 16.) / 250.;
        self.dun_ret = (self.p.x, self.p.y);
        self.en.clear();
        self.bosses.clear();
        self.proj.clear();
        self.drops.clear();
        self.parts.clear();
        let (ex, ey) = d.entry;
        let spawns = d.spawns.clone();
        let (bk, bx, by) = (d.boss_kind, d.boss_at.0 as f32 * 16. + 8., d.boss_at.1 as f32 * 16. + 14.);
        self.w.dun = Some(Box::new(d));
        self.p.x = ex as f32 * 16. + 8.;
        self.p.y = ey as f32 * 16. + 12.;
        for (k, x, y) in spawns {
            self.make_enemy(k, x, y, mul, 0, false);
        }
        if !cleared {
            let hp = (BHP[bk as usize] * 0.6 * mul).ceil();
            self.spawn_boss_hp(bk, bx, by, hp);
        }
        self.tp_comp();
        self.toast(format!("เข้าสู่ดันเจี้ยน{}", TN[theme as usize]));
        play(&self.sfx.door);
    }

    pub fn exit_dungeon(&mut self) {
        if self.w.dun.take().is_none() {
            return;
        }
        self.p.x = self.dun_ret.0;
        self.p.y = self.dun_ret.1;
        self.en.clear();
        self.bosses.clear();
        self.proj.clear();
        self.drops.clear();
        self.parts.clear();
        self.tp_comp();
        self.toast("ออกจากดันเจี้ยนแล้ว");
        play(&self.sfx.door);
        self.save();
    }

    /// มินิบอสในดันเจี้ยนตาย: จำว่าเคลียร์แล้ว + เปิดหีบรางวัล
    pub fn dungeon_boss_down(&mut self) {
        let Some(d) = self.w.dun.as_mut() else { return };
        self.dun_clear.insert(d.cell);
        if let Some(&(x, y)) = d.chests.last() {
            d.set_obj(x, y, 24);
        }
        self.toast("ปราบมินิบอสแล้ว! หีบสมบัติปรากฏขึ้น");
    }

    /// เปิดหีบสมบัติหายากที่ไทล์ k
    pub fn open_rare_chest(&mut self, k: (i32, i32)) {
        let Some(d) = self.w.dun.as_mut() else { return };
        let Some(id) = d.chests.iter().position(|&c| c == k) else { return };
        d.set_obj(k.0, k.1, 0);
        let cell = d.cell;
        self.dun_chests.insert((cell.0, cell.1, id as i32));
        let (x, y) = (k.0 as f32 * 16. + 8., k.1 as f32 * 16.);
        let c = rnd(40, 90);
        self.gain(COIN, c, x, y);
        self.gain(GEM, rnd(2, 5), x, y - 8.);
        self.gain(SHARD, rnd(2, 4), x, y - 16.);
        self.gain(POTION, rnd(1, 2), x, y - 24.);
        // พิมพ์เขียวที่ยังไม่มี (โอกาสสูงขึ้นถ้าเป็นหีบรางวัลบอส)
        let last = id + 1 == self.w.dun.as_ref().map_or(0, |d| d.chests.len());
        if gen_range(0., 1.) < if last { 0.8 } else { 0.3 } {
            let bp = if self.p.inv[BP_ARM] == 0 { Some(BP_ARM) } else if self.p.inv[BP_WPN] == 0 { Some(BP_WPN) } else { None };
            if let Some(b) = bp {
                self.gain(b, 1, x, y - 32.);
            }
        }
        self.p.xp += 20;
        self.check_lv();
        self.burst(x, y + 6., hx(0xffd84a), 24, 90.);
        self.toast("เปิดหีบสมบัติหายาก!");
        play(&self.sfx.chest);
        self.save();
    }

    /// กับดัก: หนามโผล่ตามจังหวะ · ป้อมยิงลูกไฟใส่ผู้เล่นที่เข้าใกล้
    pub fn upd_traps(&mut self, dt: f32) {
        let Some(d) = self.w.dun.as_ref() else { return };
        let (ptx, pty) = (tfloor(self.p.x), tfloor(self.p.y - 3.));
        let mut stab = false;
        for &(x, y) in &d.spikes {
            if (x, y) == (ptx, pty) && spike_on(self.time, x, y) {
                stab = true;
            }
        }
        let mut shots: Vec<(f32, f32)> = vec![];
        for &(x, y) in &d.turrets {
            let ph = hash(x, y, 11) as f32 * 2.6;
            let (tx, ty) = (x as f32 * 16. + 8., y as f32 * 16. + 6.);
            if (self.time + ph) % 2.6 < dt && (tx - self.p.x).hypot(ty - self.p.y) < 170. {
                shots.push((tx, ty));
            }
        }
        if stab && self.p.invul <= 0. && self.p.dash_t <= 0. {
            play(&self.sfx.trap);
            self.hurt(2, 0., -1.);
        }
        for (tx, ty) in shots {
            let (dx, dy) = (self.p.x - tx, self.p.y - 6. - ty);
            let l = dx.hypot(dy).max(0.001);
            self.proj.push(Proj { x: tx, y: ty, vx: dx / l * 105., vy: dy / l * 105., kind: 6, dmg: 2, life: 2.2, mine: false, dead: false });
            play(&self.sfx.fball);
        }
    }
}
