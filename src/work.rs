// v20 — อาชีพที่เห็นผลงานจริง · ชาวบ้านคิดสร้างบ้านเอง (ขอความช่วยเหลือถ้าเกินมือ) · บ่อน้ำ 3 ขนาด
//
// กลางวัน (7:00–17:00) ชาวบ้านที่มีอาชีพจะ "ไปทำงานจริง" ในเขตบ้าน: เดินไปที่ทำงาน → ทำงาน (เห็นท่าทาง) →
// ได้ของ → เดินเอาไปเก็บในกล่อง · ผลงานอยู่ในโลกจริง: ไร่ที่ไถ/ปลูก ต้นไม้ที่โค่น (งอกใหม่ได้) หินที่ขุดออก ฯลฯ
// ถ้าเราอยู่ไกลจากบ้าน (> 70 ช่อง) ระบบเดิมคิดผลงานให้ตอนรุ่งเช้าแทน (คนที่ทำงานจริงวันนั้นแล้วไม่นับซ้ำ)
use super::*;

pub const J_HARVEST: u8 = 0;
pub const J_WATER: u8 = 1;
pub const J_TILL: u8 = 2;
pub const J_CHOP: u8 = 3;
pub const J_MINE: u8 = 4;
pub const J_FISH: u8 = 5;
pub const J_COOK: u8 = 6;
pub const J_HEAL: u8 = 7;
pub const J_PATROL: u8 = 8;
pub const J_BUILD: u8 = 9;
pub const J_ERRAND: u8 = 10;
pub const J_HUNT: u8 = 11;
/// v30 เดินมาขออนุญาตใช้ที่ดินกับเรา
pub const J_ASK: u8 = 12;
pub const J_NAMES: [&str; 13] = ["เก็บเกี่ยว", "รดน้ำ", "ไถ-ปลูก", "ตัดไม้", "ขุดหิน", "ตกปลา", "ทำอาหาร", "ดูแลคนป่วย", "ลาดตระเวน", "ก่อสร้าง", "ทำงานเอกสาร", "ล่าสัตว์", "มาขออนุญาตใช้ที่"];
/// ระยะยิงธนูของนักล่า (พิกเซล)
pub const HUNT_RANGE: f32 = 60.;
/// ไร่ต่อชาวไร่ 1 คน
pub const PLOTS_PER_FARMER: usize = 6;

#[derive(Clone, Debug)]
pub struct Job {
    pub kind: u8,
    /// ช่องเป้าหมาย (ต้นไม้ หิน แปลงผัก ไซต์ ...)
    pub tile: (i32, i32),
    /// จุดยืนทำงาน (พิกเซล)
    pub stand: (f32, f32),
    pub t: f32,
    pub dur: f32,
    /// 0 = ไปทำงาน · 1 = ถือของไปเก็บกล่อง
    pub stage: u8,
    pub carry: Option<(usize, i32)>,
    pub other: u32,
    pub working: bool,
}

/// โครงการของชาวบ้าน (คิดเอง): สร้างบ้านของตัวเอง ฯลฯ
#[derive(Clone, Debug)]
pub struct Project { pub by: u32, pub kind: u8, pub at: (i32, i32), pub need: Vec<(usize, i32)>, pub started: bool, pub dir: u8, /** v30 เราอนุญาตที่ดินแล้ว */ pub ok: bool, /** v31 วัสดุที่เรามอบให้ตรงๆ (ไม่ต้องใส่กล่อง) */ pub got: Vec<(usize, i32)> }
/// ทิศประตูที่หันเข้าหาจุด c (ใจกลางเมือง) จากบ้านที่ประตูอยู่ p
pub fn door_toward(p: (i32, i32), c: (i32, i32)) -> u8 {
    let (dx, dy) = (c.0 - p.0, c.1 - p.1);
    if dx.abs() > dy.abs() { if dx > 0 { 3 } else { 1 } } else if dy < 0 { 2 } else { 0 }
}

pub fn project_what(kind: u8) -> &'static str {
    match kind {
        OBJ_HOUSE_S => "สร้างบ้านของตัวเอง",
        OBJ_HUT => "สร้างกระท่อมสงบๆ ห่างผู้คน",
        OBJ_HOUSE_L => "สร้างบ้านหลังใหญ่ของตัวเอง",
        OBJ_STONEHOUSE => "สร้างบ้านหินแข็งแรงของตัวเอง",
        OBJ_GARDENHOUSE => "สร้างบ้านสวนมีดอกไม้",
        39 => "ขุดบ่อน้ำให้ทุกคน", 17 => "ตั้งโรงตีเหล็ก (ทั่ง)", _ => "สร้างของ"
    }
}
/// วัสดุที่ใช้ (บ้านแบบที่ไม่มีในเมนูสร้างใช้ราคานี้)
pub fn project_cost(kind: u8) -> Vec<(usize, i32)> {
    match kind {
        OBJ_HUT => vec![(WOOD, 20)],
        OBJ_STONEHOUSE => vec![(STONE, 30), (WOOD, 10)],
        OBJ_GARDENHOUSE => vec![(WOOD, 36), (STONE, 4), (BERRY, 2)],
        _ => PIECES.iter().find(|p| p.obj == kind).map(|p| p.cost.to_vec()).unwrap_or_default(),
    }
}
/// แบบบ้านตามนิสัย + ระยะห่างจากใจกลางเมืองที่ชอบ
pub fn house_style(n: &npc::Npc) -> (u8, i32, i32) {
    let t = &n.soc.tr;
    if t[T_SOCIAL] < 30 { (OBJ_HUT, 10, 18) }
    else if t[T_GREED] > 70 { (OBJ_HOUSE_L, 5, 13) }
    else if t[T_BRAVE] > 65 { (OBJ_STONEHOUSE, 5, 14) }
    else if t[T_KIND] > 65 { (OBJ_GARDENHOUSE, 6, 14) }
    else if t[T_SOCIAL] > 65 { (OBJ_HOUSE_S, 4, 9) }
    else { (OBJ_HOUSE_S, 5, 14) }
}
pub fn work_hr(h: f32) -> bool { (7.0..17.0).contains(&h) }

impl Game {
    /// ทหาร/ทหารยามที่ไล่มอนอยู่ไม่รับงานอื่น
    fn work_active(&self) -> bool {
        self.w.dun.is_none() && work_hr(self.hour()) && self.town_center().map_or(false, |c| (c.0 as f32 * 16. - self.p.x).hypot(c.1 as f32 * 16. - self.p.y) < 70. * 16.)
    }

    /// ทุก ~1.5 วิ: แจกงานให้คนที่ว่าง
    pub fn work_tick(&mut self, dt: f32) {
        self.rs.work_t -= dt;
        if self.rs.work_t > 0. { return; }
        self.rs.work_t = 1.5;
        // ใส่ของให้ครบระหว่างวัน → เริ่มสร้างได้เลย ไม่ต้องรอพรุ่งนี้
        if self.rs.projects.iter().any(|p| !p.started) && (self.time as i32) % 8 == 0 { self.projects_try_start(); }
        if !self.work_active() {
            // หมดเวลางาน: คนที่ถือของอยู่เอาไปเก็บก่อน ที่เหลือเลิกงาน
            let carry: Vec<(u32, Job)> = self.rs.work.iter().filter(|(_, j)| j.carry.is_some()).map(|(k, j)| (*k, j.clone())).collect();
            self.rs.work.clear();
            for (id, j) in carry { if let (Some(i), Some((it, n))) = (self.idx(id), j.carry) { let bed = self.npcs[i].bed; self.give_to_chest(bed, it, n); } }
            self.rs.site_workers = 0;
            return;
        }
        let ids: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME && !n.child && n.mood >= 20 && Game::willing(n) && !self.rs.sleeping.contains(&n.id)).map(|n| n.id).collect();
        self.rs.work.retain(|id, _| ids.contains(id));
        // v30 คำขอที่ดิน: ทีละคนเดินมาหาเรา
        self.permit_tick();
        let free: Vec<u32> = ids.iter().copied().filter(|id| self.rs.work.get(id).map_or(true, |j| j.carry.is_none())).collect();
        if let Some(a) = self.permit_next_asker(&free) {
            let (x, y) = (self.p.x, self.p.y);
            self.rs.work.insert(a, Job { kind: J_ASK, tile: (tfloor(x), tfloor(y)), stand: (x, y), t: 0., dur: ASK_WAIT, stage: 0, carry: None, other: 0, working: false });
        }
        for id in ids {
            if self.rs.work.contains_key(&id) { continue; }
            if let Some(j) = self.pick_job(id) { self.rs.work.insert(id, j); }
            else if self.idx(id).map_or(false, |i| self.npcs[i].role == 1) { self.farm_ask_maybe(id); }
        }
        self.rs.site_workers = self.rs.work.values().filter(|j| j.kind == J_BUILD && j.working).count() as u32;
    }

    fn taken(&self, t: (i32, i32)) -> bool { self.rs.work.values().any(|j| j.tile == t) }

    /// เลือกงานตามอาชีพ (และนิสัย: คนขยันทำงานไกลกว่า)
    fn pick_job(&self, id: u32) -> Option<Job> {
        let i = self.idx(id)?;
        let n = &self.npcs[i];
        let bed = n.bed;
        let home = self.town_center().unwrap_or(bed);
        let r = 14;
        let stand_near = |t: (i32, i32)| -> (f32, f32) {
            // ยืนช่องข้างๆ ที่เดินได้ (ล่างก่อน)
            for (dx, dy) in [(0, 1), (-1, 0), (1, 0), (0, -1), (-1, 1), (1, 1)] {
                let s = (t.0 + dx, t.1 + dy);
                if !self.w.solid_at(s.0, s.1) { return (s.0 as f32 * 16. + 8., s.1 as f32 * 16. + 12.); }
            }
            (t.0 as f32 * 16. + 8., t.1 as f32 * 16. + 28.)
        };
        let job = |kind: u8, tile: (i32, i32), stand: (f32, f32), dur: f32| Some(Job { kind, tile, stand, t: 0., dur, stage: 0, carry: None, other: 0, working: false });
        let near = |a: (i32, i32), b: (i32, i32), d: i32| (a.0 - b.0).abs() <= d && (a.1 - b.1).abs() <= d;
        let dist = |a: (i32, i32)| (a.0 - bed.0).abs() + (a.1 - bed.1).abs();
        match n.role {
            1 => {
                // ชาวไร่: เก็บผลที่สุก → รดน้ำแปลงที่แห้ง → ไถ+ปลูกแปลงใหม่ (ถ้ายังมีไม่ครบ)
                let mine: Vec<(i32, i32)> = self.crops.keys().copied().filter(|k| near(*k, bed, r)).collect();
                if let Some(&k) = mine.iter().filter(|k| self.crops[k].stage() == 2 && !self.taken(**k)).min_by_key(|k| dist(**k)) { return job(J_HARVEST, k, stand_near(k), 2.5); }
                if let Some(&k) = mine.iter().filter(|k| self.crops[k].wet <= 0. && !self.taken(**k)).min_by_key(|k| dist(**k)) { return job(J_WATER, k, stand_near(k), 2.); }
                let farmers = self.npcs.iter().filter(|m| m.state == ST_HOME && m.role == 1 && near(m.bed, bed, r)).count().max(1);
                // v30 ไถเฉพาะในผืนที่เราอนุญาตแล้ว (ไม่มี = ไปขอที่ก่อน)
                if mine.len() < PLOTS_PER_FARMER * farmers { if let Some(k) = self.field_free_cell(bed, &|k| self.taken(k)) { return job(J_TILL, k, stand_near(k), 4.); } }
                None
            }
            R_LUMBER => {
                let t = self.find_obj(home, 18, |g, k, o| is_tree(o) && o != T_SAPLING && o != 48 && o != 49 && !g.w.placed.contains_key(&k) && g.w.realm.city_at(k.0, k.1).is_none())?;
                job(J_CHOP, t, stand_near(t), 6.)
            }
            R_MINER => {
                let t = self.find_obj(home, 20, |g, k, o| matches!(o, 4 | 5) && !g.w.placed.contains_key(&k))?;
                job(J_MINE, t, stand_near(t), 6.5)
            }
            R_HUNTER => {
                // ล่าสัตว์ป่ารอบเมืองก่อน (กวาง กระต่าย แอนทีโลป หมูป่า) — ไม่ยิงสัตว์เลี้ยง/นก
                let (hx0, hy0) = (home.0 as f32 * 16., home.1 as f32 * 16.);
                let taken: Vec<(i32, i32)> = self.rs.work.values().filter(|j| j.kind == J_HUNT).map(|j| j.tile).collect();
                if let Some(a) = self.animals.iter().filter(|a| !a.tame && !a.dead && (prey(a.k) || a.k == A_BOAR) && (a.x - hx0).hypot(a.y - hy0) < 16. * 16.)
                    .filter(|a| !taken.iter().any(|t| (t.0 as f32 - a.x).abs() < 20. && (t.1 as f32 - a.y).abs() < 20.))
                    .min_by(|a, b| (a.x - n.x).hypot(a.y - n.y).partial_cmp(&(b.x - n.x).hypot(b.y - n.y)).unwrap()) {
                    return job(J_HUNT, (a.x as i32, a.y as i32), (a.x, a.y), 2.2);
                }
                // ไม่มีสัตว์: ตกปลาที่ริมน้ำจืด/ทะเลใกล้บ้าน
                let w = self.find_tile(home, 18, |g, k| g.w.tile_at(k.0, k.1) < 2 && [(0, -1), (0, 1), (-1, 0), (1, 0)].iter().any(|d| !g.w.solid_at(k.0 + d.0, k.1 + d.1)))?;
                let s = [(0, -1), (0, 1), (-1, 0), (1, 0)].iter().map(|d| (w.0 + d.0, w.1 + d.1)).find(|s| !self.w.solid_at(s.0, s.1))?;
                job(J_FISH, w, (s.0 as f32 * 16. + 8., s.1 as f32 * 16. + 12.), 9.)
            }
            R_COOK => {
                let has = self.chests.iter().any(|(k, c)| near(*k, bed, 16) && (c[FISH] >= 2 || (c[MEAT] >= 1 && c[SHROOM] >= 1)));
                if !has { return None; }
                let t = self.w.placed.iter().filter(|(k, &o)| (o == 26 || o == 9) && near(**k, bed, 16)).map(|(k, _)| *k).min_by_key(|k| dist(*k))?;
                job(J_COOK, t, stand_near(t), 8.)
            }
            R_BUILDER => {
                let st = self.sites.iter().min_by_key(|s| dist(s.k))?;
                let c = self.site_center(st);
                job(J_BUILD, st.k, (c.0 + ((id % 3) as f32 - 1.) * 14., c.1 + 26.), 12.)
            }
            9 => {
                let o = self.npcs.iter().filter(|m| m.id != id && m.state == ST_HOME && m.mood < 55 && near(m.bed, bed, 20)).min_by_key(|m| m.mood)?;
                let mut j = job(J_HEAL, o.bed, (o.x + 10., o.y), 4.)?;
                j.other = o.id;
                Some(j)
            }
            2 | R_SOLDIER | R_GENERAL => {
                let a = (id as f32 * 1.7 + self.time * 0.05).sin() * std::f32::consts::PI;
                let rr = 9. + (id % 4) as f32;
                let p = (home.0 as f32 * 16. + 8. + a.cos() * rr * 16., home.1 as f32 * 16. + 12. + a.sin() * rr * 16.);
                if self.w.solid_at(tfloor(p.0), tfloor(p.1 - 3.)) { return None; }
                job(J_PATROL, (tfloor(p.0), tfloor(p.1)), p, 3.)
            }
            3 | R_CLERK | R_DIPLOMAT => {
                let t = self.chests.keys().copied().filter(|k| near(*k, bed, 16)).min_by_key(|k| dist(*k)).or_else(|| self.w.placed.iter().find(|(_, &o)| o == 37).map(|(k, _)| *k))?;
                job(J_ERRAND, t, stand_near(t), 6.)
            }
            _ => None,
        }
    }
    fn find_obj(&self, c: (i32, i32), r: i32, f: impl Fn(&Game, (i32, i32), u8) -> bool) -> Option<(i32, i32)> {
        let mut best: Option<((i32, i32), i32)> = None;
        for dy in -r..=r { for dx in -r..=r {
            let k = (c.0 + dx, c.1 + dy);
            let o = self.w.obj_at(k.0, k.1);
            if o != 0 && !self.taken(k) && f(self, k, o) { let d = dx.abs() + dy.abs(); if best.map_or(true, |b| d < b.1) { best = Some((k, d)); } }
        } }
        best.map(|b| b.0)
    }
    fn find_tile(&self, c: (i32, i32), r: i32, f: impl Fn(&Game, (i32, i32)) -> bool) -> Option<(i32, i32)> {
        let mut best: Option<((i32, i32), i32)> = None;
        for dy in -r..=r { for dx in -r..=r {
            let k = (c.0 + dx, c.1 + dy);
            if !self.taken(k) && f(self, k) { let d = dx.abs() + dy.abs(); if best.map_or(true, |b| d < b.1) { best = Some((k, d)); } }
        } }
        best.map(|b| b.0)
    }

    /// เรียกจาก npc_tick (ต่อคน): คืนจุดที่ควรเดินไป + ความเร็ว ถ้ามีงาน · ทำงานจนเสร็จแล้วส่งผลไปที่ done
    pub fn work_step(&mut self, n: &mut npc::Npc, dt: f32, done: &mut Vec<(u32, Job)>) -> Option<((f32, f32), f32)> {
        // v23 ป่วย = พักอยู่บ้าน ไม่ทำงาน
        if self.is_sick(n.id) || self.rs.war.wounded.contains_key(&n.id) || (self.rs.war.march && self.rs.war.army.contains(&n.id)) { self.rs.work.remove(&n.id); return None; }
        if self.rs.work.get(&n.id)?.kind == J_ASK { return self.ask_step(n, dt); }
        let j = self.rs.work.get_mut(&n.id)?;
        if j.kind == J_HUNT && j.stage == 0 {
            // ตามสัตว์ (มันวิ่งหนี) จนเข้าระยะยิง แล้วเล็งธนู
            let (tx, ty) = (j.tile.0 as f32, j.tile.1 as f32);
            let Some(a) = self.animals.iter().filter(|a| !a.tame && !a.dead && (a.x - tx).hypot(a.y - ty) < 90.).min_by(|a, b| (a.x - tx).hypot(a.y - ty).partial_cmp(&(b.x - tx).hypot(b.y - ty)).unwrap()) else {
                self.rs.work.remove(&n.id);
                return None;
            };
            let (ax, ay) = (a.x, a.y);
            j.tile = (ax as i32, ay as i32);
            let d = (n.x - ax).hypot(n.y - ay);
            n.left = ax < n.x;
            if d > HUNT_RANGE { j.working = false; j.t = (j.t - dt).max(0.); return Some(((ax, ay), 46.)); }
            j.working = true;
            j.t += dt;
            n.moving = false;
            if j.t >= j.dur { done.push((n.id, j.clone())); self.rs.work.remove(&n.id); }
            return Some(((n.x, n.y), 0.));
        }
        let d = (n.x - j.stand.0).hypot(n.y - j.stand.1);
        if d > 5. { j.working = false; return Some((j.stand, if j.stage == 1 { 34. } else { 38. })); }
        if j.stage == 1 {
            // ถึงกล่องแล้ว: วางของ
            done.push((n.id, j.clone()));
            self.rs.work.remove(&n.id);
            return None;
        }
        j.working = true;
        j.t += dt;
        n.left = (j.tile.0 as f32 * 16. + 8.) < n.x;
        n.moving = false;
        if j.t >= j.dur { done.push((n.id, j.clone())); self.rs.work.remove(&n.id); }
        Some(((n.x, n.y), 0.))
    }

    /// ผลของงานที่ทำเสร็จ (เรียกหลังวนชาวบ้านเสร็จ)
    pub fn work_finish(&mut self, list: Vec<(u32, Job)>) {
        for (id, j) in list {
            let Some(i) = self.idx(id) else { continue };
            let (nx, ny, bed) = (self.npcs[i].x, self.npcs[i].y, self.npcs[i].bed);
            if j.stage == 1 {
                if let Some((it, k)) = j.carry {
                    self.give_to_chest((tfloor(j.stand.0), tfloor(j.stand.1)), it, k);
                    self.add_float(nx, ny - 22., format!("+{}", k), hx(0xa8f070), Some(it));
                }
                continue;
            }
            self.rs.worked.insert(id);
            let t = j.tile;
            let fx = (t.0 as f32 * 16. + 8., t.1 as f32 * 16. + 10.);
            let mut got: Option<(usize, i32)> = None;
            match j.kind {
                J_HARVEST => { let bonus = self.saga_perk(1) as i32 + self.bumper_today() as i32; if let Some(c) = self.crops.get_mut(&t) {
                    if c.stage() == 2 { got = Some((HARVEST_OF[c.kind as usize], 2 + bonus)); c.grown = 0.; c.wet = WET_T; let o = c.obj(); self.w.placed.insert(t, o); }
                } },
                J_WATER => if let Some(c) = self.crops.get_mut(&t) { c.wet = WET_T; self.burst(fx.0, fx.1, hx(0x66aaff), 6, 30.); },
                J_TILL => if self.can_till(t.0, t.1) && !self.crops.contains_key(&t) {
                    self.w.removed.insert(t);
                    self.w.tmod.insert(t, T_WET);
                    let kind = ((t.0 + t.1).rem_euclid(2)) as u8;
                    let c = Crop { kind, grown: 0., wet: WET_T };
                    self.w.placed.insert(t, c.obj());
                    self.crops.insert(t, c);
                    self.burst(fx.0, fx.1, hx(0x7a5230), 6, 30.);
                },
                J_CHOP => {
                    let o = self.w.obj_at(t.0, t.1);
                    if is_tree(o) && !self.w.placed.contains_key(&t) {
                        self.w.removed.insert(t);
                        self.felled.insert(t, (0, 0));
                        let (a, b) = wood_range(o);
                        got = Some((WOOD, (a + b).max(4)));
                        self.burst(fx.0, fx.1 - 8., hx(0x3d8a3a), 12, 60.);
                    }
                }
                J_MINE => {
                    let o = self.w.obj_at(t.0, t.1);
                    if matches!(o, 4 | 5) && !self.w.placed.contains_key(&t) {
                        self.w.removed.insert(t);
                        got = Some((STONE, 4));
                        if o == 5 || hash(t.0, t.1, 9601) < 0.12 { self.give_to_chest(bed, GEM, 1); }
                        self.burst(fx.0, fx.1, hx(0x9a9a9a), 10, 50.);
                    }
                }
                J_FISH => got = Some((FISH, 2)),
                J_HUNT => {
                    let (tx, ty) = (t.0 as f32, t.1 as f32);
                    if let Some(ai) = (0..self.animals.len()).filter(|&k| { let a = &self.animals[k]; !a.tame && !a.dead && (a.x - tx).hypot(a.y - ty) < 40. }).min_by(|&a, &b| { let (p, q) = (&self.animals[a], &self.animals[b]); (p.x - tx).hypot(p.y - ty).partial_cmp(&(q.x - tx).hypot(q.y - ty)).unwrap() }) {
                        let (ax, ay, k) = (self.animals[ai].x, self.animals[ai].y, self.animals[ai].k);
                        // ลูกธนูพุ่งไปหาเป้า
                        self.burst(ax, ay - 6., hx(0xb02020), 10, 50.);
                        self.animals.remove(ai);
                        got = Some((MEAT, AN_STAT[k as usize].4.max(1)));
                        self.add_hide(1);
                    }
                }
                J_COOK => { self.cook_day_pub(bed); self.burst(fx.0, fx.1 - 6., hx(0xdddddd), 6, 20.); }
                J_HEAL => if let Some(k) = self.idx(j.other) { self.npcs[k].mood = (self.npcs[k].mood + 8).min(100); let (x, y) = (self.npcs[k].x, self.npcs[k].y - 22.); self.add_float(x, y, "สบายขึ้น", hx(0xff99cc), None); },
                J_ERRAND => { if self.npcs[i].role == R_CLERK { self.tax_pool += 2; } else if self.npcs[i].role == 3 { self.tax_pool += 1; } }
                _ => {}
            }
            // มีของ: ถือไปเก็บในกล่องที่ใกล้ที่สุด (ไม่มีกล่อง = เก็บเข้าคลังเลย)
            if let Some((it, k)) = got {
                let chest = self.chests.keys().copied().filter(|c| (c.0 - bed.0).abs() <= 18 && (c.1 - bed.1).abs() <= 18).min_by_key(|c| (c.0 - tfloor(nx)).abs() + (c.1 - tfloor(ny)).abs());
                match chest {
                    Some(c) => { self.rs.work.insert(id, Job { kind: j.kind, tile: c, stand: (c.0 as f32 * 16. + 8., c.1 as f32 * 16. + 28.), t: 0., dur: 0., stage: 1, carry: Some((it, k)), other: 0, working: false }); }
                    None => { self.give_to_chest(bed, it, k); self.add_float(nx, ny - 22., format!("+{}", k), hx(0xa8f070), Some(it)); }
                }
            }
        }
    }

    // ---------------------------------------------------------------- โครงการของชาวบ้าน
    /// ทุกเช้า: คนที่ยังไม่มีบ้าน (นอนกลางแจ้ง) คิดจะสร้างบ้านของตัวเอง · คนโลภ/รักสันโดษอยากมีบ้านของตัวเองแม้จะแชร์อยู่
    pub fn projects_dawn(&mut self) {
        if self.w.dun.is_some() { return; }
        self.rs.projects.retain(|p| !p.started);
        let me = self.bed;
        let ids: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME && !n.child && n.mood >= 25 && Game::willing(n)).map(|n| n.id).collect();
        for id in ids {
            if self.rs.projects.iter().any(|p| p.by == id) { continue; }
            let Some(i) = self.idx(id) else { continue };
            let n = &self.npcs[i];
            let outdoor = self.w.placed.get(&n.bed) != Some(&33) || self.w.room_from(n.bed).is_none();
            let loner = n.soc.tr[T_SOCIAL] < 30 || n.soc.tr[T_GREED] > 70;
            let free_indoor = self.free_beds().iter().any(|b| Some(*b) != me && self.w.room_from(*b).is_some());
            if !(outdoor && !free_indoor) && !(loner && hash(day_of(self), id as i32, 7311) < 0.15) { continue; }
            // หาที่ว่างรอบเมือง
            // แบบบ้าน+ทำเลตามนิสัย (สันโดษ = กระท่อมไกลๆ · โลภ = บ้านใหญ่ · กล้า = บ้านหิน · ใจดี = บ้านสวน · ชอบสังคม = ใกล้ลาน)
            let (kind, r0, r1) = house_style(n);
            let c = self.town_center().unwrap_or(n.bed);
            let mut at = None;
            'f: for rr in r0..=r1 { for k in 0..(rr * 8) { let a = k as f32 / (rr * 8) as f32 * std::f32::consts::TAU + id as f32; let p = (c.0 + (a.cos() * rr as f32) as i32, c.1 + (a.sin() * rr as f32) as i32 + 3); if self.prefab_blocked(kind, p, door_toward(p, c)).is_none() && !self.rs.projects.iter().any(|q| (q.at.0 - p.0).abs() < 8 && (q.at.1 - p.1).abs() < 8) && self.land_taken(&self.area_cells(kind, p, door_toward(p, c)), None).is_none() { at = Some(p); break 'f; } } }
            let Some(at) = at else { continue };
            let cost = project_cost(kind);
            let nm = NPC_NAMES[n.name as usize];
            self.news_add(format!("{}อยาก{} ({})", nm, project_what(kind), prefab_name(kind)));
            self.rs.projects.push(Project { by: id, kind, at, need: cost, started: false, dir: door_toward(at, c), ok: false, got: vec![] });
            // v30 ขอที่จากเราก่อน
            self.house_ask(id, kind, at, door_toward(at, c));
        }
        // โครงการส่วนรวม: มีคนไม่มีน้ำดื่ม → ขุดบ่อน้ำ · ชุมชนโตแล้วแต่ไม่มีทั่ง → ตั้งโรงตีเหล็ก
        let served = self.water_served();
        let thirsty: Vec<(u32, (i32, i32))> = self.npcs.iter().filter(|n| n.state == ST_HOME && !n.child && !served.contains(&n.bed)).map(|n| (n.id, n.bed)).collect();
        if let Some(&(by, bed)) = thirsty.first() {
            if !self.rs.projects.iter().any(|p| p.kind == 39) { if let Some(at) = self.free_spot_near(bed, 2, 6) { self.add_project(by, 39, at); } }
        }
        if self.tier() >= 2 && self.count_obj(17) == 0 && !self.rs.projects.iter().any(|p| p.kind == 17) {
            let by = self.npcs.iter().find(|n| n.state == ST_HOME && !n.child && matches!(n.role, R_BUILDER | R_MINER | 0)).map(|n| (n.id, n.bed));
            if let Some((by, bed)) = by { if let Some(at) = self.free_spot_near(self.town_center().unwrap_or(bed), 3, 8) { self.add_project(by, 17, at); } }
        }
        self.projects_try_start();
        // แจ้งข่าวขอความช่วยเหลือวันละครั้ง (ตอนเช้า)
        let msgs: Vec<String> = self.rs.help.values().cloned().collect();
        for m in msgs { self.news_add(m); }
    }
    fn add_project(&mut self, by: u32, kind: u8, at: (i32, i32)) {
        let cost = project_cost(kind);
        let nm = self.idx(by).map(|i| NPC_NAMES[self.npcs[i].name as usize]).unwrap_or("");
        self.news_add(format!("{}เสนอ{}", nm, project_what(kind)));
        self.rs.projects.push(Project { by, kind, at, need: cost, started: false, dir: 0, ok: true, got: vec![] });
    }
    fn free_spot_near(&self, c: (i32, i32), r0: i32, r1: i32) -> Option<(i32, i32)> {
        for rr in r0..=r1 { for dy in -rr..=rr { for dx in -rr..=rr {
            if dx.abs().max(dy.abs()) != rr { continue; }
            let k = (c.0 + dx, c.1 + dy);
            if self.can_place_at(k.0, k.1) && !self.crops.contains_key(&k) && self.room_of.get(&k).is_none() && !self.w.placed.contains_key(&(k.0, k.1 + 1)) { return Some(k); }
        } } }
        None
    }
    /// มีวัสดุในกล่องครบ → เริ่มสร้างเอง · ไม่ครบ → ขอความช่วยเหลือ (บอกว่าขาดอะไรเท่าไหร่)
    pub fn projects_try_start(&mut self) {
        let mut ps = std::mem::take(&mut self.rs.projects);
        self.rs.help.clear();
        for p in ps.iter_mut().filter(|p| !p.started && p.ok) {
            let Some(i) = self.idx(p.by) else { continue };
            let bed = self.npcs[i].bed;
            let nm = NPC_NAMES[self.npcs[i].name as usize];
            let got = p.got.clone();
            let have = |g: &Game, it: usize| got.iter().filter(|x| x.0 == it).map(|x| x.1).sum::<i32>() + g.chests.iter().filter(|(k, _)| (k.0 - bed.0).abs() <= 20 && (k.1 - bed.1).abs() <= 20).map(|(_, c)| c[it]).sum::<i32>();
            let missing: Vec<(usize, i32)> = p.need.iter().map(|&(it, n)| (it, n - have(self, it))).filter(|x| x.1 > 0).collect();
            if !missing.is_empty() {
                let s: Vec<String> = missing.iter().map(|(it, n)| format!("{} {}", INAME[*it], n)).collect();
                let msg = format!("{}: อยาก{} แต่ยังขาด {} — ช่วยมอบให้หน่อยได้ไหม", nm, project_what(p.kind), s.join(" · "));
                self.rs.help.entry(p.by).or_insert(msg);
                continue;
            }
            let prefab = is_prefab(p.kind);
            if prefab && self.prefab_blocked(p.kind, p.at, p.dir).is_some() { continue; }
            if !prefab && !self.can_place_at(p.at.0, p.at.1) { continue; }
            // หักวัสดุจากกล่องใกล้บ้าน
            for &(it, mut n) in &p.need {
                // ของที่เรามอบให้ใช้ก่อน แล้วค่อยหักจากกล่อง
                for g in p.got.iter_mut().filter(|x| x.0 == it) { let t = g.1.min(n); g.1 -= t; n -= t; }
                let keys: Vec<(i32, i32)> = self.chests.keys().copied().filter(|k| (k.0 - bed.0).abs() <= 20 && (k.1 - bed.1).abs() <= 20).collect();
                for k in keys { if n <= 0 { break; } let c = self.chests.get_mut(&k).unwrap(); let t = c[it].min(n); c[it] -= t; n -= t; }
            }
            p.started = true;
            self.rs.help.remove(&p.by);
            if prefab {
                for ((x, y), _) in self.prefab_cells(p.kind, p.at, p.dir) { if !self.w.placed.contains_key(&(x, y)) && self.w.nat_obj(x, y) != 0 { self.w.removed.insert((x, y)); } }
                self.sites.push(Site { kind: p.kind, k: p.at, prog: 0., done: 0, dir: p.dir });
                self.toast(format!("{}เริ่ม{}แล้ว! (ช่างไม้ช่วยได้ · เราอยู่ใกล้ก็เร็วขึ้น)", nm, project_what(p.kind)));
            } else {
                // ของชิ้นเดียว: สร้างเสร็จเลย
                if self.w.nat_obj(p.at.0, p.at.1) != 0 { self.w.removed.insert(p.at); }
                self.w.placed.insert(p.at, p.kind);
                self.room_t = 0.;
                self.burst(p.at.0 as f32 * 16. + 8., p.at.1 as f32 * 16. + 10., hx(0xc8a070), 10, 40.);
                self.toast(format!("{}{}เสร็จแล้ว!", nm, project_what(p.kind)));
            }
        }
        self.rs.projects = ps;
    }

    // ---------------------------------------------------------------- บ่อน้ำ
    /// เตียงไหนมีน้ำใช้: แหล่งน้ำจืดธรรมชาติ (≤20 ช่อง ไม่จำกัดคน) · บ่อกลางเมือง (≤30 ไม่จำกัด) · บ่อครัวเรือน (≤12 ได้ 6 คน) · โอ่ง (≤4 ได้ 1 คน)
    pub fn water_served(&self) -> HashSet<(i32, i32)> {
        let beds: Vec<(i32, i32)> = self.npcs.iter().filter(|n| n.state == ST_HOME).map(|n| n.bed).collect();
        let mut ok: HashSet<(i32, i32)> = HashSet::new();
        let wells: Vec<((i32, i32), u8)> = self.w.placed.iter().filter(|(_, &o)| matches!(o, 39 | O_WJAR | O_BIGWELL)).map(|(k, &o)| (*k, o)).collect();
        for &b in &beds { if wells.iter().any(|(k, o)| *o == O_BIGWELL && (k.0 - b.0).abs() <= 30 && (k.1 - b.1).abs() <= 30) { ok.insert(b); } }
        // บ่อเล็ก: แจกให้เตียงใกล้สุดก่อนจนเต็มความจุ
        for &(k, o) in wells.iter().filter(|w| w.1 != O_BIGWELL) {
            let (r, cap) = if o == O_WJAR { (4, 1) } else { (12, 6) };
            let mut near: Vec<(i32, i32)> = beds.iter().copied().filter(|b| !ok.contains(b) && (k.0 - b.0).abs() <= r && (k.1 - b.1).abs() <= r).collect();
            near.sort_by_key(|b| (k.0 - b.0).abs() + (k.1 - b.1).abs());
            for b in near.into_iter().take(cap) { ok.insert(b); }
        }
        for &b in &beds { if !ok.contains(&b) && self.fresh_water_near(b, 20) { ok.insert(b); } }
        ok
    }
    pub fn fresh_water_near(&self, b: (i32, i32), r: i32) -> bool {
        for y in b.1 - r..=b.1 + r { for x in b.0 - r..=b.0 + r { if self.w.tile_at(x, y) == 1 && !self.is_salt(x, y) { return true; } } }
        false
    }
    pub fn big_well_near(&self, b: (i32, i32)) -> bool { self.w.placed.iter().any(|(k, &o)| o == O_BIGWELL && (k.0 - b.0).abs() <= 30 && (k.1 - b.1).abs() <= 30) }
}

// ---------------------------------------------------------------- วาด: ท่าทำงาน + ของที่ถือ
pub fn draw_work(g: &Game, sp: &Sprites, v: &View, n: &npc::Npc, sx: f32, sy: f32, t: f32) {
    let Some(j) = g.rs.work.get(&n.id) else { return };
    if let Some((it, _)) = j.carry {
        // ถือของไว้เหนือหัว
        let b = ((t * 6.).sin() * 0.6).round();
        blit_sz(sp, v, sp.icons[it], sx - 4., sy - 27. + b, 8., 8., WHITE);
        return;
    }
    if !j.working { return; }
    let f = if n.left { -1. } else { 1. };
    let ph = (t * if j.kind == J_FISH { 1. } else { 5. } + n.id as f32).sin();
    let wood = hx(0x7a4e26);
    let iron = hx(0xb8bcc6);
    match j.kind {
        J_CHOP | J_MINE | J_TILL | J_BUILD | J_HARVEST => {
            // ด้ามเครื่องมือแกว่งขึ้นลง + หัวเครื่องมือ
            let a = ph * 0.9 - 0.3;
            let (hx_, hy_) = (sx + f * (3. + a.cos() * 6.), sy - 10. - a.sin() * 6.);
            for k in 0..5 { let q = k as f32 / 4.; prect(v, (sx + f * 2. + (hx_ - sx - f * 2.) * q).round(), (sy - 9. + (hy_ - sy + 9.) * q).round(), 1., 1., wood); }
            let head = match j.kind { J_CHOP => iron, J_MINE => hx(0x8a8a92), J_TILL => hx(0x9a7a50), J_BUILD => hx(0x6a6a72), _ => hx(0xc8b040) };
            prect(v, hx_.round() - 1., hy_.round() - 1., 3., 2., head);
            if ph > 0.95 && matches!(j.kind, J_CHOP | J_MINE) { prect(v, (sx + f * 10.).round(), sy - 8., 1., 1., WHITE); }
        }
        J_WATER => {
            // บัวรดน้ำ + หยดน้ำ
            prect(v, sx + f * 4. - 2., sy - 9., 4., 3., hx(0x5a8ac8));
            for k in 0..3 { let q = ((t * 2.5 + k as f32 * 0.33).fract()) * 6.; prect(v, (sx + f * (7. + q * 0.5)).round(), (sy - 8. + q).round(), 1., 1., hx(0x9ad0ff)); }
        }
        J_FISH => {
            // คันเบ็ด + สายเบ็ด + ทุ่นลอย
            let tip = (sx + f * 9., sy - 16.);
            for k in 0..6 { let q = k as f32 / 5.; prect(v, (sx + f * 2. + (tip.0 - sx - f * 2.) * q).round(), (sy - 9. + (tip.1 - sy + 9.) * q).round(), 1., 1., wood); }
            let bob = (j.tile.0 as f32 * 16. + 8. - v.cx, j.tile.1 as f32 * 16. + 8. - v.cy + ph * 0.8);
            draw_line(tip.0 * v.s, tip.1 * v.s, bob.0 * v.s, bob.1 * v.s, 1., Color::new(1., 1., 1., 0.6));
            prect(v, bob.0.round() - 1., bob.1.round() - 1., 2., 2., hx(0xff3a3a));
        }
        J_COOK => { for k in 0..3 { let q = ((t * 0.8 + k as f32 * 0.33).fract()) * 10.; prect(v, (sx + f * 9. + (q * 0.8).sin() * 2.).round(), (sy - 12. - q).round(), 2., 2., Color::new(0.9, 0.9, 0.9, 0.6 * (1. - q / 10.))); } }
        J_HUNT => {
            // คันธนู (โค้ง) + สายที่ดึงค้าง + ลูกศร ชี้ไปหาสัตว์
            let (ax, ay) = (j.tile.0 as f32 - v.cx, j.tile.1 as f32 - v.cy - 6.);
            let (dx, dy) = (ax - sx, ay - (sy - 9.));
            let l = dx.hypot(dy).max(1.);
            let (ux, uy) = (dx / l, dy / l);
            let (bx, by) = (sx + ux * 5., sy - 9. + uy * 5.);
            for k in -3..=3 { let q = k as f32; prect(v, (bx - uy * q + ux * (1. - (q * q) / 9.) * 2.).round(), (by + ux * q + uy * (1. - (q * q) / 9.) * 2.).round(), 1., 1., wood); }
            let pull = (j.t / j.dur).min(1.) * 3.;
            prect(v, (bx - ux * pull).round(), (by - uy * pull).round(), 1., 1., WHITE);
            for k in 0..6 { prect(v, (bx - ux * pull + ux * k as f32).round(), (by - uy * pull + uy * k as f32).round(), 1., 1., if k == 5 { iron } else { hx(0xd8c8a0) }); }
        }
        J_HEAL => { let b = ((t * 3.).sin() * 1.5).round(); prect(v, sx - 1., sy - 26. + b, 3., 1., hx(0xff6aa0)); prect(v, sx, sy - 27. + b, 1., 3., hx(0xff6aa0)); }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn mk(id: u32, role: u8, bed: (i32, i32)) -> npc::Npc {
        let mut n = npc::Npc { id, name: (id % 12) as u8, look: 0, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: ST_HOME, hearts: 3, mood: 70, married: false, child: false, role, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id % 12) as u8) };
        n.soc.anger = 0; n.soc.grudge = 0;
        n
    }
    fn setup(g: &mut Game) -> (i32, i32) {
        let (sx, sy) = (tfloor(g.spawn.0) + 80, tfloor(g.spawn.1) + 80);
        for y in sy - 22..=sy + 22 { for x in sx - 22..=sx + 22 { g.w.tmod.insert((x, y), 3); g.w.removed.insert((x, y)); } }
        g.w.placed.insert((sx, sy), 33);
        g.w.placed.insert((sx - 2, sy), 14);
        g.chests.insert((sx - 2, sy), [0; NI]);
        g.bed = Some((sx + 40, sy));
        g.p.x = sx as f32 * 16.; g.p.y = (sy + 6) as f32 * 16.;
        while !work_hr(g.hour()) || g.hour() > 9. { g.time += 3.; }
        g.last_day = day_of(g);
        (sx, sy)
    }
    fn run(g: &mut Game, secs: f32) { let dt = 1. / 30.; let mut t = 0.; while t < secs { g.npc_tick(dt); g.work_tick(dt); g.upd_crops(dt); t += dt; } }
    #[test]
    fn farmer_tills_plants_waters_and_harvests_visibly() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = setup(&mut g);
        g.npcs.push(mk(1, 1, (sx, sy)));
        g.rs.manual.insert(1, 1);
        // v30 ชาวไร่มาขอที่ก่อน → อนุญาต
        run(&mut g, 20.);
        g.permit_yes(1);
        run(&mut g, 60.);
        let plots: Vec<(i32, i32)> = g.crops.keys().copied().filter(|k| (k.0 - sx).abs() <= 14 && (k.1 - sy).abs() <= 14).collect();
        assert!(plots.len() >= 3, "ชาวไร่ไถ+ปลูกแปลงใหม่ {}", plots.len());
        for k in &plots { assert!(matches!(g.w.tile_at(k.0, k.1), T_SOIL | T_WET), "เห็นดินไถ"); assert!(g.w.placed.get(k).map_or(false, |o| (27..=32).contains(o)), "เห็นต้นพืช"); }
        // ทำให้สุก → เก็บเกี่ยวแล้วถือไปเก็บในกล่อง
        for c in g.crops.values_mut() { c.grown = 999.; }
        run(&mut g, 40.);
        let ch = g.chests[&(sx - 2, sy)];
        assert!(ch[CORN] + ch[CARROT] >= 2, "ผลผลิตเข้ากล่อง {:?}", (ch[CORN], ch[CARROT]));
        assert!(g.rs.worked.contains(&1));
    }
    #[test]
    fn lumberjack_fells_real_tree_and_it_regrows_later() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = setup(&mut g);
        let tree = (sx + 6, sy - 3);
        g.w.removed.remove(&tree);
        g.w.placed.insert((sx + 30, sy + 30), 0);
        // ปลูกต้นไม้ธรรมชาติ: ใช้ placed ไม่ได้ (คนตัดไม้ไม่ตัดต้นที่ผู้เล่นปลูก) → หาต้นไม้ธรรมชาติจริงใกล้ๆ แทน
        let mut nat = None;
        'f: for r in 1..40 { for dy in -r..=r { for dx in -r..=r { let k = (sx + dx, sy + dy); g.w.removed.remove(&k); g.w.tmod.remove(&k); let o = g.w.nat_obj(k.0, k.1); if is_tree(o) && o != T_SAPLING && g.w.realm.city_at(k.0, k.1).is_none() { nat = Some(k); break 'f; } g.w.removed.insert(k); g.w.tmod.insert(k, 3); } } }
        let t = nat.expect("มีต้นไม้ธรรมชาติ");
        let _ = tree;
        g.npcs.push(mk(2, R_LUMBER, (sx, sy)));
        g.rs.manual.insert(2, R_LUMBER);
        g.npcs[0].x = t.0 as f32 * 16. + 8.; g.npcs[0].y = (t.1 + 1) as f32 * 16. + 12.;
        let w0 = g.chests[&(sx - 2, sy)][WOOD] + g.p.inv[WOOD];
        run(&mut g, 90.);
        assert!(g.w.removed.contains(&t) && g.felled.contains_key(&t), "ต้นไม้ถูกโค่นจริงและจะงอกใหม่");
        assert!(g.chests[&(sx - 2, sy)][WOOD] + g.p.inv[WOOD] > w0, "ได้ไม้");
    }
    #[test]
    fn wells_have_capacity_and_radius() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = setup(&mut g);
        for y in sy - 45..=sy + 45 { for x in sx - 45..=sx + 45 { g.w.tmod.insert((x, y), 3); } }
        for i in 0..8 { let b = (sx + i * 2, sy + 5); g.w.placed.insert(b, 33); g.npcs.push(mk(10 + i as u32, 0, b)); }
        assert!(g.water_served().is_empty(), "เริ่มต้นไม่มีน้ำ");
        g.w.placed.insert((sx + 3, sy + 7), 39);
        assert_eq!(g.water_served().len(), 6, "บ่อครัวเรือนได้ 6 คน");
        g.w.placed.insert((sx + 14, sy + 6), O_WJAR);
        assert_eq!(g.water_served().len(), 7, "โอ่ง +1 คน");
        g.w.placed.insert((sx - 20, sy - 10), O_BIGWELL);
        assert_eq!(g.water_served().len(), 8, "บ่อกลางเมืองไม่จำกัด");
    }
    #[test]
    fn homeless_villager_plans_own_house_asks_for_help_then_builds() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = setup(&mut g);
        // พื้นที่สร้างบ้านต้องเป็นดินธรรมชาติ (ไม่ใช่ช่องที่ดัดแปลง) — คืนช่องที่เป็นพื้นเดินได้อยู่แล้ว
        let keys: Vec<(i32, i32)> = g.w.tmod.keys().copied().collect();
        for k in keys { let t = g.w.gen_tile(k.0, k.1); if !SOLIDT[t as usize] && t != 6 { g.w.tmod.remove(&k); } }
        g.w.chunks.borrow_mut().clear();
        g.npcs.push(mk(3, 0, (sx, sy)));
        g.projects_dawn();
        assert_eq!(g.rs.projects.iter().filter(|p| is_prefab(p.kind)).count(), 1, "คิดจะสร้างบ้าน");
        g.permit_yes(3);
        assert!(g.rs.help.contains_key(&3), "ของไม่พอ → ขอความช่วยเหลือ");
        assert!(g.rs.help[&3].contains("ไม้"));
        // ใส่ของให้ → เริ่มสร้างเอง
        for it in [WOOD, STONE, BERRY] { g.chests.get_mut(&(sx - 2, sy)).unwrap()[it] = 60; }
        g.projects_try_start();
        assert!(g.rs.projects.iter().any(|p| is_prefab(p.kind) && p.started) && !g.sites.is_empty(), "เริ่มก่อสร้าง");
        assert!(g.chests[&(sx - 2, sy)][WOOD] < 60 || g.chests[&(sx - 2, sy)][STONE] < 60, "ใช้วัสดุจากกล่อง");
    }
    #[test]
    fn thirsty_town_proposes_well_and_builds_when_materials_arrive() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = setup(&mut g);
        for y in sy - 45..=sy + 45 { for x in sx - 45..=sx + 45 { g.w.tmod.insert((x, y), 3); } }
        g.w.chunks.borrow_mut().clear();
        // มีบ้านแล้ว (ไม่คิดสร้างบ้าน) แต่ไม่มีน้ำ
        for x in sx - 2..=sx + 2 { g.w.placed.insert((x, sy - 2), 11); g.w.placed.insert((x, sy + 2), if x == sx { 12 } else { 11 }); }
        for y in sy - 2..=sy + 2 { g.w.placed.insert((sx - 2, y), 11); g.w.placed.insert((sx + 2, y), 11); }
        g.w.placed.remove(&(sx - 2, sy)); g.chests.remove(&(sx - 2, sy));
        g.w.placed.insert((sx + 6, sy), 14); g.chests.insert((sx + 6, sy), [0; NI]);
        g.w.placed.insert((sx - 2, sy), 11);
        g.npcs.push(mk(4, 0, (sx, sy)));
        g.projects_dawn();
        assert!(g.rs.projects.iter().any(|p| p.kind == 39), "เสนอขุดบ่อน้ำ {:?}", g.rs.projects.iter().map(|p| p.kind).collect::<Vec<_>>());
        assert!(g.rs.help.get(&4).map_or(false, |m| m.contains("บ่อน้ำ")));
        g.chests.get_mut(&(sx + 6, sy)).unwrap()[STONE] = 20;
        g.chests.get_mut(&(sx + 6, sy)).unwrap()[WOOD] = 10;
        g.projects_try_start();
        assert!(g.w.placed.values().any(|&o| o == 39), "บ่อน้ำสร้างเสร็จ");
        assert!(g.water_served().contains(&(sx, sy)), "ตอนนี้มีน้ำใช้");
    }
    #[test]
    fn hunter_shoots_real_animal_and_brings_meat() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = setup(&mut g);
        g.animals.clear();
        g.npcs.push(mk(5, R_HUNTER, (sx, sy)));
        g.rs.manual.insert(5, R_HUNTER);
        let mut a = g.new_animal(A_DEER, (sx + 6) as f32 * 16., (sy + 2) as f32 * 16.);
        a.bold = 1.; // ไม่หนีไกล (ทดสอบให้แน่นอน)
        g.animals.push(a);
        let m0 = g.chests[&(sx - 2, sy)][MEAT];
        let mut t = 0.;
        while t < 60. { g.npc_tick(1. / 30.); g.work_tick(1. / 30.); t += 1. / 30.; if g.chests[&(sx - 2, sy)][MEAT] > m0 { break; } }
        assert!(g.animals.iter().all(|a| a.k != A_DEER), "กวางถูกล่า");
        assert!(g.chests[&(sx - 2, sy)][MEAT] > m0, "เนื้อเข้ากล่อง");
    }
    #[test]
    fn house_style_follows_personality() {
        let mut n = mk(9, 0, (0, 0));
        for x in n.soc.tr.iter_mut() { *x = 50; }
        n.soc.tr[T_SOCIAL] = 10;
        assert_eq!(house_style(&n).0, OBJ_HUT, "สันโดษ → กระท่อมไกลๆ");
        assert!(house_style(&n).1 >= 10);
        n.soc.tr[T_SOCIAL] = 50; n.soc.tr[T_GREED] = 90;
        assert_eq!(house_style(&n).0, OBJ_HOUSE_L, "โลภ → บ้านใหญ่");
        n.soc.tr[T_GREED] = 50; n.soc.tr[T_BRAVE] = 90;
        assert_eq!(house_style(&n).0, OBJ_STONEHOUSE);
        n.soc.tr[T_BRAVE] = 50; n.soc.tr[T_KIND] = 90;
        assert_eq!(house_style(&n).0, OBJ_GARDENHOUSE);
        // ทุกแบบสร้างได้จริง: เป็นห้องที่มีเตียง + ประตู
        for k in [OBJ_HUT, OBJ_HOUSE_S, OBJ_HOUSE_L, OBJ_STONEHOUSE, OBJ_GARDENHOUSE] {
            let lay = prefab_layout(k);
            assert!(lay.iter().any(|(_, c)| matches!(c, Cell::Wall(12))), "มีประตู {}", k);
            assert!(lay.iter().any(|(_, c)| matches!(c, Cell::Floor(_, 33) | Cell::Ground(33))), "มีเตียง {}", k);
            assert!(!project_cost(k).is_empty(), "มีราคา {}", k);
            assert!(prefab_time(k) > 0.);
        }
    }
}
