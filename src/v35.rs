//! v35 "พึ่งตัวเอง" — ชาวบ้านหาเอง ทำเอง ก่อนจะมาขอผู้นำ
//!
//! แนวคิด (สังคมศาสตร์): คนจริงๆ แก้ปัญหาปากท้องด้วยแรงตัวเองก่อน แล้วค่อยพึ่งชุมชน และขอผู้นำเฉพาะสิ่งที่ลำพังทำไม่ได้
//!   1) ผลิตเพื่อครัวเรือน — หิวก็ไปหาอาหาร: เก็บผลไม้/เห็ด ตกปลา (ไม่มีเบ็ดก็ทำเบ็ดเอง: ไม้ + หิน) แล้วย่างปลาเอง
//!      (ไม่มีกองไฟก็ก่อเอง) · ป่วยก็หาสมุนไพรมารักษากันเอง · สร้างบ้านก็ตัดไม้ขุดหินมาเอง
//!   2) คลังกลางของหมู่บ้าน — ของที่หามาได้เข้ากล่องกลาง ใครหิวก็หยิบกิน (ช่วยเหลือกันในชุมชน)
//!   3) ขอผู้นำ = ทางสุดท้าย — หาเองไม่ได้จริง (อดข้าวติดกัน 2 วัน / รอวัสดุเกิน 3 วัน / ของที่หาเองไม่ได้ เช่นอัญมณี)
//!      และเรื่องที่ต้องพึ่งอำนาจเท่านั้น: ความปลอดภัย (ภัยสัตว์ร้าย) · การตัดสินข้อพิพาท · อนุญาตที่ดิน
//!
//! งานใหม่ 3 อย่าง (ใช้ระบบงานเดิมของ work.rs): J_FORAGE หาของป่า · J_CRAFT ทำเอง (คันเบ็ด/กองไฟ) · J_CURE เอายาไปให้คนป่วย
use super::*;

pub const J_FORAGE: u8 = 13;
pub const J_CRAFT: u8 = 14;
pub const J_CURE: u8 = 15;
/// งานเพื่อครัวเรือน: ของที่ได้เข้ากล่องกลาง
pub const SELF_HOME: u32 = 1 << 29;
/// งานหาวัสดุสร้างบ้านตัวเอง: ของที่ได้ใส่เข้าโครงการตรงๆ
pub const SELF_PROJ: u32 = 1 << 30;
pub const FLAGS: u32 = SELF_HOME | SELF_PROJ;
pub const CR_ROD: u32 = 1;
pub const CR_FIRE: u32 = 2;
/// อาหารขั้นต่ำต่อคนในกล่องกลาง (ต่ำกว่านี้ = ออกหาอาหาร)
pub const FOOD_PER_HEAD: i32 = 2;
/// อดข้าวติดกันกี่วันถึงจะมาขอเราช่วย
pub const STUCK_FOOD_DAYS: i32 = 2;
/// รอวัสดุสร้างบ้านกี่วันถึงจะมาขอเราช่วย
pub const STUCK_BUILD_DAYS: i32 = 3;
pub const ROD_WOOD: i32 = 8;
pub const ROD_STONE: i32 = 2;
pub const FIRE_WOOD: i32 = 3;
pub const FIRE_STONE: i32 = 2;
/// หาไม่เจอแล้วพักกี่วินาทีก่อนลองใหม่
const RETRY: f32 = 20.;

#[derive(Clone, Debug, Default)]
pub struct SelfHelp {
    /// ใครมีคันเบ็ดแล้ว (เซฟ)
    pub rods: HashSet<u32>,
    /// อดข้าวติดกันกี่วัน (เซฟ)
    pub hungry: HashMap<u32, i32>,
    /// รอวัสดุสร้างบ้านมากี่วัน (เซฟ)
    pub wait: HashMap<u32, i32>,
    /// เก็บของกินเองระหว่างวันแต่ไม่มีกล่อง → ถือว่ากินอิ่มตอนเช้า
    pub snack: HashSet<u32>,
    /// ช่องที่เก็บไปแล้ววันนี้ (ผลไม้/เห็ด/ที่หาสมุนไพร) → วัน
    pub picked: HashMap<(i32, i32), i32>,
    /// หาไม่เจอ → เวลาเกมที่จะลองใหม่
    pub idle: HashMap<u32, f32>,
}

fn mk(kind: u8, tile: (i32, i32), stand: (f32, f32), dur: f32, other: u32) -> Job {
    Job { kind, tile, stand, t: 0., dur, stage: 0, carry: None, other, working: false }
}
fn tree_ok(o: u8) -> bool { is_tree(o) && o != T_SAPLING && o != 48 && o != 49 }
fn veg_ok(o: u8) -> bool { tree_ok(o) || o == 2 || o == 7 }

impl Game {
    // ---------------------------------------------------------------- ข้อมูลครัวเรือน
    pub fn sh_stand(&self, t: (i32, i32)) -> (f32, f32) {
        for (dx, dy) in [(0, 1), (-1, 0), (1, 0), (0, -1), (-1, 1), (1, 1)] {
            let s = (t.0 + dx, t.1 + dy);
            if !self.w.solid_at(s.0, s.1) { return (s.0 as f32 * 16. + 8., s.1 as f32 * 16. + 12.); }
        }
        (t.0 as f32 * 16. + 8., t.1 as f32 * 16. + 28.)
    }
    pub fn chest_stock(&self, bed: (i32, i32), item: usize, r: i32) -> i32 {
        self.chests.iter().filter(|(k, _)| (k.0 - bed.0).abs() <= r && (k.1 - bed.1).abs() <= r).map(|(_, c)| c[item]).sum()
    }
    /// ครัวเรือน = ชาวบ้านที่เตียงอยู่ใกล้กัน (ใช้กล่องกลางร่วมกัน)
    pub fn household(&self, bed: (i32, i32)) -> i32 {
        self.npcs.iter().filter(|n| n.state == ST_HOME && (n.bed.0 - bed.0).abs() <= 16 && (n.bed.1 - bed.1).abs() <= 16).count().max(1) as i32
    }
    pub fn need_food(&self, bed: (i32, i32)) -> bool { self.food_near(bed) < self.household(bed) * FOOD_PER_HEAD }
    pub fn starving(&self, bed: (i32, i32)) -> bool { self.food_near(bed) < self.household(bed) }
    pub fn chest_near(&self, bed: (i32, i32)) -> Option<(i32, i32)> {
        self.chests.keys().copied().filter(|k| (k.0 - bed.0).abs() <= 18 && (k.1 - bed.1).abs() <= 18).min_by_key(|k| (k.0 - bed.0).abs() + (k.1 - bed.1).abs())
    }
    pub fn fire_near(&self, bed: (i32, i32)) -> Option<(i32, i32)> {
        self.w.placed.iter().filter(|(k, &o)| (o == 26 || o == 9) && (k.0 - bed.0).abs() <= 16 && (k.1 - bed.1).abs() <= 16).map(|(k, _)| *k).min_by_key(|k| (k.0 - bed.0).abs() + (k.1 - bed.1).abs())
    }
    fn take_stock(&mut self, bed: (i32, i32), item: usize, mut n: i32, r: i32) {
        let keys: Vec<(i32, i32)> = self.chests.keys().copied().filter(|k| (k.0 - bed.0).abs() <= r && (k.1 - bed.1).abs() <= r).collect();
        for k in keys { if n <= 0 { break; } let c = self.chests.get_mut(&k).unwrap(); let t = c[item].min(n); c[item] -= t; n -= t; }
    }
    fn water_spot(&self, home: (i32, i32)) -> Option<((i32, i32), (i32, i32))> {
        let w = self.find_tile(home, 18, |g, k| g.w.tile_at(k.0, k.1) < 2 && [(0, -1), (0, 1), (-1, 0), (1, 0)].iter().any(|d| !g.w.solid_at(k.0 + d.0, k.1 + d.1)))?;
        let s = [(0, -1), (0, 1), (-1, 0), (1, 0)].iter().map(|d| (w.0 + d.0, w.1 + d.1)).find(|s| !self.w.solid_at(s.0, s.1))?;
        Some((w, s))
    }
    fn fresh(&self, k: (i32, i32)) -> bool { self.rs.sh.picked.get(&k) != Some(&day_of(self)) }

    // ---------------------------------------------------------------- แหล่งของ
    fn chop_job(&self, home: (i32, i32), flag: u32) -> Option<Job> {
        let t = self.find_obj(home, 18, |g, k, o| tree_ok(o) && !g.w.placed.contains_key(&k) && g.w.realm.city_at(k.0, k.1).is_none())?;
        Some(mk(J_CHOP, t, self.sh_stand(t), 6., flag))
    }
    fn mine_job(&self, home: (i32, i32), flag: u32) -> Option<Job> {
        let t = self.find_obj(home, 20, |g, k, o| matches!(o, 4 | 5) && !g.w.placed.contains_key(&k) && g.w.realm.city_at(k.0, k.1).is_none())?;
        Some(mk(J_MINE, t, self.sh_stand(t), 6.5, flag))
    }
    fn berry_job(&self, home: (i32, i32), flag: u32) -> Option<Job> {
        let winter = season(self) == 3;
        let t = self.find_obj(home, 18, |g, k, o| g.fresh(k) && !g.w.placed.contains_key(&k) && g.w.realm.city_at(k.0, k.1).is_none()
            && (((o == 2 || o == 7) && !winter) || (is_fruit_tree(o) && g.fruit_ready(k))))?;
        Some(mk(J_FORAGE, t, self.sh_stand(t), 4., BERRY as u32 | flag))
    }
    fn shroom_job(&self, home: (i32, i32), flag: u32) -> Option<Job> {
        let t = self.find_obj(home, 18, |g, k, o| o == O_SHROOM && g.fresh(k) && !g.w.placed.contains_key(&k))?;
        Some(mk(J_FORAGE, t, self.sh_stand(t), 4., SHROOM as u32 | flag))
    }
    /// สมุนไพรไม่มีต้นเฉพาะในโลก → ค้นหาใต้ไม้พุ่ม/ต้นไม้ (เจอบ้างไม่เจอบ้าง)
    fn herb_job(&self, home: (i32, i32), flag: u32) -> Option<Job> {
        let t = self.find_obj(home, 18, |g, k, o| veg_ok(o) && g.fresh(k) && !g.w.placed.contains_key(&k) && g.w.realm.city_at(k.0, k.1).is_none())?;
        Some(mk(J_FORAGE, t, self.sh_stand(t), 6., HERB as u32 | flag))
    }

    // ---------------------------------------------------------------- ตัดสินใจว่าจะทำอะไรเอง
    /// ครัวเรือนอดแล้ว → คนที่ไม่ใช่ยาม/ชาวไร่/นักล่า/พ่อครัว ต้องออกหาอาหารก่อนงานประจำ
    pub fn sh_urgent(&self, id: u32) -> bool {
        let Some(i) = self.idx(id) else { return false };
        let n = &self.npcs[i];
        if matches!(n.role, 1 | 2 | 9 | R_SOLDIER | R_GENERAL | R_HUNTER | R_COOK) { return false; }
        self.starving(n.bed)
    }
    /// งานที่ทำเอง (ถ้ามี): ดูแลคนป่วย → หาอาหาร → วัสดุบ้านตัวเอง → ย่างของที่มี
    pub fn self_job(&self, id: u32) -> Option<Job> {
        if self.rs.sh.idle.get(&id).map_or(false, |&t| t > self.time) { return None; }
        let i = self.idx(id)?;
        let n = &self.npcs[i];
        if n.child || self.is_sick(id) { return None; }
        let bed = n.bed;
        let home = self.town_center().unwrap_or(bed);
        if let Some(j) = self.care_job(id, bed, home) { return Some(j); }
        if self.need_food(bed) { if let Some(j) = self.food_job(id, bed, home) { return Some(j); } }
        if let Some(j) = self.material_job(id, home) { return Some(j); }
        // มีปลาอยู่แต่ยังไม่มีที่ย่าง → ก่อกองไฟ (อาหารปรุงสุกทำให้สุขขึ้น)
        if self.chest_stock(bed, FISH, 16) >= 2 && self.fire_near(bed).is_none() { if let Some(j) = self.fire_chain(bed, home, self.chest_near(bed)) { return Some(j); } }
        self.grill_job(bed)
    }
    fn care_job(&self, id: u32, bed: (i32, i32), home: (i32, i32)) -> Option<Job> {
        let cared = |s: u32| self.rs.work.values().any(|j| j.kind == J_CURE && (j.other & !FLAGS) == s) || self.rs.work.values().any(|j| j.kind == J_FORAGE && j.other & 0xFF == HERB as u32 && self.rs.lv.sick.len() <= 1);
        let sick = self.rs.lv.sick.keys().copied().filter(|&s| s != id && !cared(s))
            .filter_map(|s| self.idx(s).map(|k| (s, k))).min_by_key(|&(_, k)| (self.npcs[k].bed.0 - bed.0).abs() + (self.npcs[k].bed.1 - bed.1).abs())?;
        let o = &self.npcs[sick.1];
        if (o.bed.0 - bed.0).abs() > 24 || (o.bed.1 - bed.1).abs() > 24 { return None; }
        let have = self.chest_stock(bed, HERB, 16) + self.chest_stock(o.bed, HERB, 16);
        if have >= 1 {
            let mut j = mk(J_CURE, o.bed, (o.x + 10., o.y), 4., SELF_HOME | sick.0);
            j.other = SELF_HOME | sick.0;
            return Some(j);
        }
        self.herb_job(home, SELF_HOME)
    }
    fn food_job(&self, id: u32, bed: (i32, i32), home: (i32, i32)) -> Option<Job> {
        let chest = self.chest_near(bed);
        let fish = self.chest_stock(bed, FISH, 16);
        let fire = self.fire_near(bed);
        // ย่างปลา: มีปลา + มีกองไฟ/หม้อ
        if let Some(j) = self.grill_job(bed) { return Some(j); }
        // มีปลาแล้วแต่ไม่มีที่ย่าง → ก่อกองไฟเอง
        if fish >= 2 && fire.is_none() { if let Some(j) = self.fire_chain(bed, home, chest) { return Some(j); } }
        let berry = self.berry_job(home, SELF_HOME);
        let fishing = self.fish_job(id, bed, home, chest);
        let alt = self.shroom_job(home, SELF_HOME);
        if id % 2 == 0 { fishing.or(berry).or(alt) } else { berry.or(fishing).or(alt) }
    }
    fn fish_job(&self, id: u32, bed: (i32, i32), home: (i32, i32), chest: Option<(i32, i32)>) -> Option<Job> {
        let (w, s) = self.water_spot(home)?;
        if self.rs.sh.rods.contains(&id) { return Some(mk(J_FISH, w, (s.0 as f32 * 16. + 8., s.1 as f32 * 16. + 12.), 9., SELF_HOME)); }
        // ไม่มีเบ็ด → ทำเอง (ต้องมีกล่องเก็บของ เพราะวัสดุหามารวมกันที่นั่น)
        let chest = chest?;
        let (wood, stone) = (self.chest_stock(bed, WOOD, 16), self.chest_stock(bed, STONE, 16));
        if wood >= ROD_WOOD && stone >= ROD_STONE { return Some(mk(J_CRAFT, chest, self.sh_stand(chest), 5., SELF_HOME | CR_ROD)); }
        if wood < ROD_WOOD { return self.chop_job(home, SELF_HOME); }
        self.mine_job(home, SELF_HOME)
    }
    fn fire_chain(&self, bed: (i32, i32), home: (i32, i32), chest: Option<(i32, i32)>) -> Option<Job> {
        let chest = chest?;
        let (wood, stone) = (self.chest_stock(bed, WOOD, 16), self.chest_stock(bed, STONE, 16));
        if wood >= FIRE_WOOD && stone >= FIRE_STONE { return Some(mk(J_CRAFT, chest, self.sh_stand(chest), 4., SELF_HOME | CR_FIRE)); }
        if wood < FIRE_WOOD { return self.chop_job(home, SELF_HOME); }
        self.mine_job(home, SELF_HOME)
    }
    /// ย่าง/ปรุง: ปลา 2 ตัว หรือ เนื้อ+เห็ด ที่กล่องใกล้เตียง และมีกองไฟ/หม้อ
    fn grill_job(&self, bed: (i32, i32)) -> Option<Job> {
        let has = self.chests.iter().any(|(k, c)| (k.0 - bed.0).abs() <= 16 && (k.1 - bed.1).abs() <= 16 && (c[FISH] >= 2 || (c[MEAT] >= 1 && c[SHROOM] >= 1)));
        if !has { return None; }
        let f = self.fire_near(bed)?;
        if self.taken(f) { return None; }
        Some(mk(J_COOK, f, self.sh_stand(f), 8., SELF_HOME))
    }
    /// ขาดวัสดุสร้างบ้านตัวเอง → ตัดไม้/ขุดหิน/เก็บผลไม้เอง ใส่เข้าโครงการ
    fn material_job(&self, id: u32, home: (i32, i32)) -> Option<Job> {
        let p = self.rs.projects.iter().find(|p| p.by == id && p.ok && !p.started)?;
        for (it, _) in self.project_lack(p) {
            let j = match it { WOOD => self.chop_job(home, SELF_PROJ), STONE => self.mine_job(home, SELF_PROJ), BERRY => self.berry_job(home, SELF_PROJ), _ => None };
            if j.is_some() { return j; }
        }
        None
    }
    /// ของที่โครงการยังขาด (หักของที่มีในกล่องใกล้เตียงเจ้าของ + ที่ได้มาตรงๆ แล้ว)
    pub fn project_lack(&self, p: &Project) -> Vec<(usize, i32)> {
        let Some(i) = self.idx(p.by) else { return vec![] };
        let bed = self.npcs[i].bed;
        p.need.iter().map(|&(it, n)| (it, n - p.got.iter().filter(|x| x.0 == it).map(|x| x.1).sum::<i32>() - self.chest_stock(bed, it, 20))).filter(|x| x.1 > 0).collect()
    }
    /// ขาดของที่หาเองไม่ได้ หรือรอมานานเกินไป → ถึงเวลาขอผู้นำ
    pub fn sh_stuck_build(&self, id: u32, lack: &[(usize, i32)]) -> bool {
        lack.iter().any(|(it, _)| !matches!(*it, WOOD | STONE | BERRY)) || self.rs.sh.wait.get(&id).copied().unwrap_or(0) >= STUCK_BUILD_DAYS
    }
    pub fn sh_stuck_food(&self, id: u32) -> bool { self.rs.sh.hungry.get(&id).copied().unwrap_or(0) >= STUCK_FOOD_DAYS }

    // ---------------------------------------------------------------- ผลของงาน (เรียกจาก work_finish)
    pub fn forage_result(&mut self, j: &Job) -> Option<(usize, i32)> {
        let item = (j.other & 0xFF) as usize;
        let t = j.tile;
        let day = day_of(self);
        let o = self.w.obj_at(t.0, t.1);
        self.rs.sh.picked.insert(t, day);
        match item {
            BERRY => {
                if is_fruit_tree(o) { if !self.fruit_ready(t) { return None; } self.fruit.insert(t, day); Some((BERRY, rnd(2, 3))) }
                else if o == 2 { Some((BERRY, rnd(1, 2))) }
                else if o == 7 { Some((BERRY, 1)) }
                else { None }
            }
            SHROOM => if o == O_SHROOM { Some((SHROOM, rnd(1, 2))) } else { None },
            HERB => if hash(t.0, t.1, day.wrapping_add(3500)) < 0.6 { Some((HERB, if hash(t.1, t.0, day.wrapping_add(3501)) < 0.3 { 2 } else { 1 })) } else { None },
            _ => None,
        }
    }
    pub fn craft_result(&mut self, id: u32, j: &Job) {
        let Some(i) = self.idx(id) else { return };
        let (bed, x, y) = (self.npcs[i].bed, self.npcs[i].x, self.npcs[i].y);
        match j.other & 0xFF {
            CR_ROD => {
                if self.chest_stock(bed, WOOD, 16) < ROD_WOOD || self.chest_stock(bed, STONE, 16) < ROD_STONE { return; }
                self.take_stock(bed, WOOD, ROD_WOOD, 16);
                self.take_stock(bed, STONE, ROD_STONE, 16);
                self.rs.sh.rods.insert(id);
                self.add_float(x, y - 22., "ทำคันเบ็ดได้แล้ว", hx(0xffe08a), None);
                self.npc_bubble(id, "ทำคันเบ็ดเองได้แล้ว ไปตกปลาดีกว่า");
            }
            CR_FIRE => {
                if self.chest_stock(bed, WOOD, 16) < FIRE_WOOD || self.chest_stock(bed, STONE, 16) < FIRE_STONE { return; }
                let Some(at) = self.free_spot_near(j.tile, 1, 5) else { return };
                self.take_stock(bed, WOOD, FIRE_WOOD, 16);
                self.take_stock(bed, STONE, FIRE_STONE, 16);
                self.w.placed.insert(at, 9);
                self.burst(at.0 as f32 * 16. + 8., at.1 as f32 * 16. + 10., hx(0xffa030), 10, 40.);
                self.add_float(x, y - 22., "ก่อกองไฟไว้ย่างปลา", hx(0xffe08a), None);
            }
            _ => {}
        }
    }
    pub fn cure_result(&mut self, id: u32, j: &Job) {
        let sid = j.other & !FLAGS;
        if !self.is_sick(sid) { return; }
        let (Some(a), Some(b)) = (self.idx(id), self.idx(sid)) else { return };
        let (bed, sbed) = (self.npcs[a].bed, self.npcs[b].bed);
        let at = if self.chest_stock(bed, HERB, 16) >= 1 { bed } else if self.chest_stock(sbed, HERB, 16) >= 1 { sbed } else { return };
        self.take_stock(at, HERB, 1, 16);
        self.rs.lv.sick.remove(&sid);
        let (nx, ny) = (self.npcs[b].x, self.npcs[b].y);
        self.npcs[b].mood = (self.npcs[b].mood + 12).min(100);
        self.npcs[a].mood = (self.npcs[a].mood + 2).min(100);
        self.burst(nx, ny - 12., hx(0x7aff9a), 12, 40.);
        self.npc_bubble(sid, "หายแล้ว ขอบคุณนะ");
        let (an, bn) = (NPC_NAMES[self.npcs[a].name as usize], NPC_NAMES[self.npcs[b].name as usize]);
        self.news_add(format!("{}เอาสมุนไพรไปรักษา{} จนหายป่วย", an, bn));
    }
    /// ของที่หามาได้ใส่เข้าโครงการสร้างบ้านของคนนั้นตรงๆ (true = ใส่แล้ว)
    pub fn sh_to_project(&mut self, id: u32, it: usize, k: i32) -> bool {
        let Some(p) = self.rs.projects.iter_mut().find(|p| p.by == id && !p.started) else { return false };
        p.got.push((it, k));
        true
    }
    /// ข้อความบอกว่าเขากำลังทำอะไรเองอยู่ (โชว์ในหน้าของเขา)
    pub fn self_text(&self, j: &Job) -> Option<String> {
        if j.other & FLAGS == 0 && !matches!(j.kind, J_FORAGE | J_CRAFT | J_CURE) { return None; }
        let proj = j.other & SELF_PROJ != 0;
        Some(match j.kind {
            J_FORAGE => match (j.other & 0xFF) as usize { BERRY => "ไปเก็บผลไม้มากินเอง", SHROOM => "ไปเก็บเห็ดมากินเอง", _ => "ไปหาสมุนไพรมารักษาคนป่วย" }.to_string(),
            J_CRAFT => if j.other & 0xFF == CR_ROD { "ทำคันเบ็ดจากไม้กับหินเอง จะได้ไปตกปลา" } else { "ก่อกองไฟไว้ย่างปลา" }.to_string(),
            J_CURE => "เอายาไปให้คนป่วย".to_string(),
            J_FISH => "ไปตกปลามาทำกับข้าว".to_string(),
            J_COOK => "ย่างปลา/ปรุงอาหารเอง".to_string(),
            J_CHOP => if proj { "ตัดไม้มาสร้างบ้านตัวเอง" } else { "ตัดไม้ไว้ใช้ทำของ" }.to_string(),
            J_MINE => if proj { "ขุดหินมาสร้างบ้านตัวเอง" } else { "ขุดหินไว้ใช้ทำของ" }.to_string(),
            _ => return None,
        })
    }
    /// สรุปสิ่งที่เขาพึ่งตัวเองได้/ต้องพึ่งเรา (หน้าของเขา)
    pub fn self_status(&self, id: u32) -> Vec<String> {
        let mut v = vec![];
        let Some(i) = self.idx(id) else { return v };
        let n = &self.npcs[i];
        if n.child { return v; }
        let bed = n.bed;
        if let Some(t) = self.rs.work.get(&id).and_then(|j| self.self_text(j)) { v.push(format!("ตอนนี้: {}", t)); }
        v.push(format!("อาหารกองกลาง {}/{} · {}{}", self.food_near(bed), self.household(bed) * FOOD_PER_HEAD,
            if self.rs.sh.rods.contains(&id) { "มีคันเบ็ดแล้ว" } else { "ยังไม่มีคันเบ็ด" },
            if self.fire_near(bed).is_some() { " · มีที่ย่าง" } else { "" }));
        if self.sh_stuck_food(id) { v.push("หาอาหารเองไม่ได้มาหลายวันแล้ว — ต้องพึ่งคุณ".to_string()); }
        if let Some(p) = self.rs.projects.iter().find(|p| p.by == id && p.ok && !p.started) {
            let lack = self.project_lack(p);
            if !lack.is_empty() {
                let s: Vec<String> = lack.iter().map(|(it, q)| format!("{} {}", INAME[*it], q)).collect();
                v.push(format!("{}: ขาด {}", if self.sh_stuck_build(id, &lack) { "หาเองไม่ไหว ขอแรงช่วย" } else { "กำลังหาเอง" }, s.join(" · ")));
            }
        }
        v
    }

    // ---------------------------------------------------------------- ทุกเช้า (ตอนเราอยู่ไกล ชาวบ้านก็ยังพึ่งตัวเองต่อ)
    pub fn selfhelp_day(&mut self) {
        let day = day_of(self);
        self.rs.sh.picked.retain(|_, d| *d >= day - 1);
        self.rs.sh.idle.clear();
        // นับวันที่รอวัสดุ
        let builders: HashSet<u32> = self.rs.projects.iter().filter(|p| p.ok && !p.started).map(|p| p.by).collect();
        self.rs.sh.wait.retain(|id, _| builders.contains(id));
        for id in builders { *self.rs.sh.wait.entry(id).or_insert(0) += 1; }
        // คนที่ไม่ได้ทำงานให้เห็นจริงเมื่อวาน → คิดผลแบบย่อ
        let list: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME && !n.child && n.mood >= 20 && Game::willing(n) && !self.rs.worked.contains(&n.id) && !self.is_sick(n.id)).map(|n| n.id).collect();
        for id in list {
            let Some(i) = self.idx(id) else { continue };
            let bed = self.npcs[i].bed;
            let home = self.town_center().unwrap_or(bed);
            let chest = self.chest_near(bed);
            // 1) อาหาร
            if chest.is_some() && self.need_food(bed) {
                let water = self.water_spot(home).is_some();
                if self.rs.sh.rods.contains(&id) && water { self.give_to_chest(bed, FISH, 2); }
                else if self.berry_job(home, 0).is_some() { self.give_to_chest(bed, BERRY, 2); }
                else if water && self.chest_stock(bed, WOOD, 16) >= ROD_WOOD && self.chest_stock(bed, STONE, 16) >= ROD_STONE {
                    self.take_stock(bed, WOOD, ROD_WOOD, 16); self.take_stock(bed, STONE, ROD_STONE, 16);
                    self.rs.sh.rods.insert(id);
                }
            }
            // 2) ย่างปลา
            if self.grill_job(bed).is_some() { self.cook_day_pub(bed); }
            // 3) วัสดุบ้านตัวเอง
            if let Some(p) = self.rs.projects.iter().find(|p| p.by == id && p.ok && !p.started) {
                let lack = self.project_lack(p);
                let pick = lack.iter().find_map(|&(it, _)| match it { WOOD if self.chop_job(home, 0).is_some() => Some(WOOD), STONE if self.mine_job(home, 0).is_some() => Some(STONE), BERRY if self.berry_job(home, 0).is_some() => Some(BERRY), _ => None });
                if let Some(it) = pick { self.sh_to_project(id, it, if it == BERRY { 2 } else { 5 }); }
            }
        }
        // 4) คนป่วย: มียาในกล่อง = ถูกรักษา · ไม่มี = ชาวบ้านออกหาให้ (เจอ 60%)
        let sick: Vec<u32> = self.rs.lv.sick.keys().copied().collect();
        for sid in sick {
            let Some(b) = self.idx(sid) else { continue };
            let sbed = self.npcs[b].bed;
            if self.chest_stock(sbed, HERB, 16) < 1 && hash(sid as i32, day, 3510) < 0.6 && self.npcs.iter().any(|n| n.id != sid && n.state == ST_HOME && !n.child && !self.is_sick(n.id) && (n.bed.0 - sbed.0).abs() <= 24) {
                self.give_to_chest(sbed, HERB, 1);
            }
            if self.chest_stock(sbed, HERB, 16) >= 1 && self.chest_near(sbed).is_some() {
                self.take_stock(sbed, HERB, 1, 16);
                self.rs.lv.sick.remove(&sid);
                self.npcs[b].mood = (self.npcs[b].mood + 12).min(100);
                self.news_add(format!("เพื่อนบ้านหาสมุนไพรมารักษา{}จนหายป่วย", NPC_NAMES[self.npcs[b].name as usize]));
            }
        }
    }

    // ---------------------------------------------------------------- ความปลอดภัย: เรื่องที่ต้องพึ่งผู้นำจริงๆ
    /// สัตว์ร้ายที่อยู่ใกล้บ้านเขา (≤ 22 ช่อง) — ภัยที่ชาวบ้านจัดการเองไม่ได้
    pub fn threat_near(&self, bed: (i32, i32)) -> Option<u8> {
        let (bx, by) = (bed.0 as f32 * 16., bed.1 as f32 * 16.);
        self.animals.iter().filter(|a| !a.tame && !a.dead && predator(a.k) && (a.x - bx).hypot(a.y - by) < 22. * 16.)
            .min_by(|a, b| (a.x - bx).hypot(a.y - by).partial_cmp(&(b.x - bx).hypot(b.y - by)).unwrap()).map(|a| a.k)
    }
    /// มีหินแร่ (อัญมณี) ให้ขุดเองใกล้เมืองไหม — ถ้ามี คนโลภไปขุดเองได้ ไม่ต้องขอเรา
    pub fn gem_rock_near(&self, bed: (i32, i32)) -> bool {
        self.find_obj(bed, 22, |g, k, o| o == 5 && !g.w.placed.contains_key(&k)).is_some()
    }

    // ---------------------------------------------------------------- เซฟ
    pub fn sh_save(&self) -> String {
        let sh = &self.rs.sh;
        let csv = |v: Vec<String>| if v.is_empty() { "-".to_string() } else { v.join(",") };
        let mut rods: Vec<u32> = sh.rods.iter().copied().collect(); rods.sort();
        let mut hu: Vec<(u32, i32)> = sh.hungry.iter().map(|(k, v)| (*k, *v)).collect(); hu.sort();
        let mut wa: Vec<(u32, i32)> = sh.wait.iter().map(|(k, v)| (*k, *v)).collect(); wa.sort();
        format!("shelp {} {} {}\n", csv(rods.iter().map(|x| x.to_string()).collect()), csv(hu.iter().map(|(a, b)| format!("{}:{}", a, b)).collect()), csv(wa.iter().map(|(a, b)| format!("{}:{}", a, b)).collect()))
    }
    pub fn sh_load(&mut self, r: &[&str]) {
        let pairs = |s: Option<&&str>| -> Vec<(u32, i32)> { s.map_or(vec![], |x| x.split(',').filter_map(|p| { let mut it = p.split(':'); Some((it.next()?.parse().ok()?, it.next()?.parse().ok()?)) }).collect()) };
        self.rs.sh.rods = r.first().map_or(HashSet::new(), |x| x.split(',').filter_map(|v| v.parse().ok()).collect());
        self.rs.sh.hungry = pairs(r.get(1)).into_iter().collect();
        self.rs.sh.wait = pairs(r.get(2)).into_iter().collect();
    }
}

/// ท่าทางหาของป่า: ก้มเก็บ มือขยับ + ใบไม้ปลิว
pub fn draw_forage(v: &View, sx: f32, sy: f32, t: f32, left: bool, id: u32) {
    let f = if left { -1. } else { 1. };
    let ph = (t * 4. + id as f32).sin();
    let dip = (ph * 1.5).round();
    prect(v, (sx + f * 5.).round(), (sy - 6. + dip).round(), 2., 2., hx(0xe8c8a0));
    for k in 0..2 { let q = ((t * 1.6 + k as f32 * 0.5).fract()) * 7.; prect(v, (sx + f * (6. + q * 0.4)).round(), (sy - 4. - q).round(), 1., 1., hx(0x5ac85a)); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::work::{work_hr, Project};

    fn mkn(id: u32, role: u8, bed: (i32, i32)) -> npc::Npc {
        let mut n = npc::Npc { id, name: (id % 12) as u8, look: 0, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: ST_HOME, hearts: 3, mood: 70, married: false, child: false, role, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id % 12) as u8) };
        n.soc.anger = 0; n.soc.grudge = 0;
        n
    }
    /// หมู่บ้านทดสอบ: แกนกลางโล่ง (เตียง+กล่อง) · ป่า (ต้นไม้+พุ่มผลไม้) ทางขวา · เขาหิน ทางซ้าย · บ่อน้ำ ด้านล่าง
    fn village() -> (Game, (i32, i32)) {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = (tfloor(g.spawn.0) + 80, tfloor(g.spawn.1) + 80);
        for y in sy - 26..=sy + 26 { for x in sx - 26..=sx + 26 { g.w.tmod.insert((x, y), 3); g.w.removed.insert((x, y)); } }
        for y in sy - 5..=sy + 5 { for x in sx + 8..=sx + 17 { g.w.tmod.insert((x, y), 4); g.w.removed.remove(&(x, y)); } }
        for y in sy - 5..=sy + 5 { for x in sx - 17..=sx - 8 { g.w.tmod.insert((x, y), 5); g.w.removed.remove(&(x, y)); } }
        for y in sy + 10..=sy + 13 { for x in sx - 3..=sx + 3 { g.w.tmod.insert((x, y), 1); } }
        g.w.placed.insert((sx, sy), 33);
        g.w.placed.insert((sx - 2, sy), 14);
        g.chests.insert((sx - 2, sy), [0; NI]);
        g.bed = Some((sx + 40, sy));
        g.p.x = sx as f32 * 16.; g.p.y = (sy + 6) as f32 * 16.;
        g.en.clear(); g.animals.clear();
        while !work_hr(g.hour()) || g.hour() > 9. { g.time += 3.; }
        g.last_day = day_of(&g);
        (g, (sx, sy))
    }
    fn run(g: &mut Game, secs: f32) { let dt = 1. / 30.; let mut t = 0.; while t < secs { g.npc_tick(dt); g.work_tick(dt); t += dt; } }
    fn chest(g: &mut Game, sx: i32, sy: i32) -> &mut [i32; NI] { g.chests.get_mut(&(sx - 2, sy)).unwrap() }
    fn villager(g: &mut Game, id: u32, bed: (i32, i32)) {
        g.npcs.push(mkn(id, 0, bed));
        g.rs.manual.insert(id, 0);
    }

    #[test]
    fn test_village_has_resources() {
        let (g, (sx, sy)) = village();
        let home = (sx, sy);
        assert!(g.chop_job(home, 0).is_some(), "มีต้นไม้ให้ตัด");
        assert!(g.mine_job(home, 0).is_some(), "มีหินให้ขุด");
        assert!(g.water_spot(home).is_some(), "มีน้ำให้ตกปลา");
    }

    #[test]
    fn hungry_villager_makes_rod_fishes_and_grills_without_asking() {
        let (mut g, (sx, sy)) = village();
        villager(&mut g, 2, (sx, sy));
        { let c = chest(&mut g, sx, sy); c[WOOD] = 20; c[STONE] = 8; }
        assert!(g.need_food((sx, sy)) && g.starving((sx, sy)));
        g.rs.sh.picked.clear();
        run(&mut g, 150.);
        assert!(g.rs.sh.rods.contains(&2), "ทำคันเบ็ดเองจากไม้+หิน");
        let c = g.chests[&(sx - 2, sy)];
        assert!(c[FISH] + c[DISH_FISH] > 0, "ตกปลาได้เอง: ปลา {} ปลาย่าง {}", c[FISH], c[DISH_FISH]);
        assert!(g.fire_near((sx, sy)).is_some(), "ก่อกองไฟเองเพื่อย่างปลา");
        assert!(c[DISH_FISH] >= 1, "ย่างปลาเอง");
        assert!(g.nreqs.is_empty() && g.rs.help.is_empty(), "ไม่ได้ขอเราเลย");
    }

    #[test]
    fn gathers_wood_and_stone_when_chest_is_empty() {
        let (mut g, (sx, sy)) = village();
        villager(&mut g, 2, (sx, sy));
        run(&mut g, 200.);
        let c = g.chests[&(sx - 2, sy)];
        assert!(c[WOOD] > 0 || g.rs.sh.rods.contains(&2), "ตัดไม้เอง");
        assert!(c[STONE] > 0 || g.rs.sh.rods.contains(&2), "ขุดหินเอง");
        assert!(g.rs.sh.rods.contains(&2) || c[BERRY] > 0, "สุดท้ายมีทางหาอาหาร");
    }

    #[test]
    fn berry_picker_eats_from_the_wild() {
        let (mut g, (sx, sy)) = village();
        villager(&mut g, 3, (sx, sy)); // เลขคี่ = ชอบเก็บผลไม้ก่อน
        g.w.tmod.insert((sx, sy + 12), 3);
        run(&mut g, 120.);
        let c = g.chests[&(sx - 2, sy)];
        assert!(c[BERRY] + c[FISH] + c[SHROOM] + c[DISH_FISH] > 0 || g.rs.sh.snack.contains(&3), "หาอาหารได้เอง");
    }

    #[test]
    fn builder_collects_own_materials_and_only_asks_when_stuck() {
        let (mut g, (sx, sy)) = village();
        villager(&mut g, 2, (sx, sy));
        { let c = chest(&mut g, sx, sy); c[BERRY] = 9; } // กินพอ ไม่ต้องหาอาหาร
        let at = (sx + 3, sy + 4);
        g.rs.projects.push(Project { by: 2, kind: OBJ_HUT, at, need: vec![(WOOD, 20)], started: false, dir: 0, ok: true, got: vec![] });
        g.projects_try_start();
        assert!(g.rs.help.is_empty(), "ยังไม่ถึงเวลาขอ: กำลังหาวัสดุเอง");
        assert!(g.wants(2).iter().all(|w| !matches!(w.k, WantK::Build(_))), "ไม่โชว์ปุ่มขอวัสดุ");
        run(&mut g, 150.);
        let got: i32 = g.rs.projects.iter().filter(|p| p.by == 2).flat_map(|p| p.got.iter()).filter(|x| x.0 == WOOD).map(|x| x.1).sum();
        assert!(got > 0 || g.rs.projects.iter().all(|p| p.by != 2 || p.started), "ตัดไม้มาใส่โครงการเอง ({})", got);
        // รอเกิน 3 วัน → ขอแรงช่วย
        g.rs.sh.wait.insert(2, STUCK_BUILD_DAYS);
        g.rs.projects.iter_mut().for_each(|p| p.got.clear());
        g.rs.projects.iter_mut().for_each(|p| p.started = false);
        g.chests.get_mut(&(sx - 2, sy)).unwrap()[WOOD] = 0;
        g.projects_try_start();
        assert!(g.rs.help.contains_key(&2) || g.rs.projects.iter().any(|p| p.started), "หาเองไม่ไหว → ขอ");
    }

    #[test]
    fn unsourceable_material_asks_immediately() {
        let (mut g, (sx, sy)) = village();
        villager(&mut g, 2, (sx, sy));
        g.rs.projects.push(Project { by: 2, kind: 39, at: (sx + 3, sy + 4), need: vec![(GEM, 1), (STONE, 4)], started: false, dir: 0, ok: true, got: vec![] });
        g.projects_try_start();
        assert!(g.rs.help.contains_key(&2), "อัญมณีหาเองไม่ได้ → ขอเรา");
    }

    #[test]
    fn declining_build_help_does_not_cancel_the_project() {
        let (mut g, (sx, sy)) = village();
        villager(&mut g, 2, (sx, sy));
        g.rs.projects.push(Project { by: 2, kind: 39, at: (sx + 3, sy + 4), need: vec![(GEM, 1)], started: false, dir: 0, ok: true, got: vec![] });
        g.projects_try_start();
        let w = g.wants(2).into_iter().find(|w| matches!(w.k, WantK::Build(_))).expect("ขาดอัญมณี = ขอ");
        g.want_no(2, &w.k);
        assert!(g.rs.projects.iter().any(|p| p.by == 2), "ไม่ให้ก็ไม่เลิกโครงการ");
        assert!(g.wants(2).iter().all(|w| !matches!(w.k, WantK::Build(_))), "วันนั้นไม่ถามซ้ำ");
    }

    #[test]
    fn neighbor_cures_the_sick_with_herb() {
        let (mut g, (sx, sy)) = village();
        villager(&mut g, 2, (sx, sy));
        g.npcs.push(mkn(5, 0, (sx + 2, sy + 1))); g.rs.manual.insert(5, 0);
        { let c = chest(&mut g, sx, sy); c[HERB] = 1; c[BERRY] = 20; }
        g.rs.lv.sick.insert(5, 0);
        run(&mut g, 60.);
        assert!(!g.is_sick(5), "เพื่อนบ้านเอายาไปให้");
        assert_eq!(g.chests[&(sx - 2, sy)][HERB], 0);
    }
    #[test]
    fn neighbor_searches_for_herb_when_none_in_stock() {
        let (mut g, (sx, sy)) = village();
        villager(&mut g, 2, (sx, sy));
        g.npcs.push(mkn(5, 0, (sx + 2, sy + 1))); g.rs.manual.insert(5, 0);
        { let c = chest(&mut g, sx, sy); c[BERRY] = 20; }
        g.rs.lv.sick.insert(5, 0);
        let j = g.self_job(2).expect("มีงาน");
        assert_eq!((j.kind, (j.other & 0xFF) as usize), (J_FORAGE, HERB), "ไปหาสมุนไพร");
    }

    #[test]
    fn no_food_request_until_two_days_hungry() {
        let (mut g, (sx, sy)) = village();
        villager(&mut g, 2, (sx, sy));
        g.npcs[0].mood = 40;
        g.npcs[0].soc.love = BERRY as u8;
        let mut asked = false;
        for d in 0..30 { g.time = DAYLEN * (3 + d) as f32; g.last_day = day_of(&g); g.nreqs.clear(); g.quest_dawn(); if g.nreqs.iter().any(|r| matches!(r.k, QK::Fetch { .. })) { asked = true; } }
        assert!(!asked, "ยังไม่อดจริง = ไม่ขอของกิน");
        g.rs.sh.hungry.insert(2, STUCK_FOOD_DAYS);
        for d in 0..30 { g.time = DAYLEN * (40 + d) as f32; g.last_day = day_of(&g); g.nreqs.clear(); g.quest_dawn(); if g.nreqs.iter().any(|r| matches!(r.k, QK::Fetch { .. })) { asked = true; } }
        assert!(asked, "อดมา 2 วันแล้วหาเองไม่ได้ = ขอ");
    }

    #[test]
    fn predator_near_home_makes_villagers_ask_for_protection() {
        let (mut g, (sx, sy)) = village();
        villager(&mut g, 2, (sx, sy));
        g.npcs[0].soc.tr[T_BRAVE] = 90;
        let (x, y) = (sx as f32 * 16. + 100., sy as f32 * 16.);
        let a = g.new_animal(A_WOLF, x, y); g.animals.push(a);
        assert_eq!(g.threat_near((sx, sy)), Some(A_WOLF));
        let mut asked = false;
        for d in 0..30 { g.time = DAYLEN * (3 + d) as f32; g.last_day = day_of(&g); g.nreqs.clear(); g.quest_dawn(); if g.nreqs.iter().any(|r| matches!(r.k, QK::Hunt { .. })) { asked = true; } }
        assert!(asked, "หมาป่าใกล้บ้าน = ขอความคุ้มครอง แม้เป็นคนกล้า");
    }

    #[test]
    fn offline_day_still_provisions() {
        let (mut g, (sx, sy)) = village();
        villager(&mut g, 3, (sx, sy));
        g.rs.sh.picked.clear();
        g.selfhelp_day();
        let c = g.chests[&(sx - 2, sy)];
        assert!(c[BERRY] + c[FISH] > 0, "เราไม่อยู่ ก็ยังหาอาหารให้ครัวเรือน");
    }

    #[test]
    fn save_roundtrip_keeps_rods_hunger_and_wait() {
        let (mut g, _) = village();
        g.rs.sh.rods.insert(4); g.rs.sh.rods.insert(9);
        g.rs.sh.hungry.insert(4, 2);
        g.rs.sh.wait.insert(9, 3);
        let s = g.sh_save();
        let mut h = Game::new_world_seed(4242, Sfx::default());
        let line: Vec<&str> = s.trim().split_whitespace().collect();
        assert_eq!(line[0], "shelp");
        h.sh_load(&line[1..]);
        assert_eq!(h.rs.sh.rods, g.rs.sh.rods);
        assert_eq!(h.rs.sh.hungry, g.rs.sh.hungry);
        assert_eq!(h.rs.sh.wait, g.rs.sh.wait);
        let empty = Game::new_world_seed(4242, Sfx::default()).sh_save();
        let mut e = Game::new_world_seed(4242, Sfx::default());
        e.sh_load(&empty.trim().split_whitespace().collect::<Vec<_>>()[1..]);
        assert!(e.rs.sh.rods.is_empty() && e.rs.sh.hungry.is_empty());
    }
}
