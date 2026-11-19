//! ระบบ NPC & สังคม: ช่วยผู้หลงทาง → ชวนมาอยู่ฐาน → ความสัมพันธ์/แต่งงาน/ลูก
//! → เตียง+อาหารกำหนดความสุข → ระดับ ค่าย (1-3 คน) / หมู่บ้าน (4-10) / อาณาจักร (>10 + ปราสาท + กำแพง)
use super::*;

pub const NPC_NAMES: [&str; 12] = ["มะลิ", "ตะวัน", "บุญเรือง", "น้ำฝน", "สายลม", "ปลายฟ้า", "ทองดี", "จันทร์เพ็ญ", "กล้า", "ใบบัว", "ไผ่", "รวงข้าว"];
pub const ROLE_N: [&str; 4] = ["ชาวบ้าน", "ชาวไร่", "ทหารยาม", "พ่อค้า"];
pub const TIER_N: [&str; 4] = ["ยังไม่มีชุมชน", "ค่าย", "หมู่บ้าน", "อาณาจักร"];
pub const LOOKS: usize = 6;
/// (ผม, ผิว, เสื้อ, เสื้อสว่าง)
pub const LOOK_PAL: [(u32, u32, u32, u32); LOOKS] = [
    (0x2a2a30, 0xf2c79a, 0xd0605a, 0xff9a90), (0xd8b040, 0xf6d2aa, 0x4a9a6a, 0x8ad0a0), (0x7a3a20, 0xc89870, 0x8a6ad0, 0xc0aaff),
    (0xe8e8f0, 0xe8b88a, 0x5a8ad0, 0xa0c4ff), (0x1a1a1a, 0x9a6a48, 0xd0a040, 0xffe080), (0xa04020, 0xf2c79a, 0x50a0a0, 0x90e0e0),
];
pub const ST_LOST: u8 = 0;
pub const ST_FOLLOW: u8 = 1;
pub const ST_HOME: u8 = 2;
pub const MARRY_COIN: i32 = 100;
pub const MARRY_GEM: i32 = 5;
const FOODS: [usize; 10] = [2, 24, 23, 14, 13, 20, 16, 17, 18, 19];

#[derive(Clone, Debug)]
pub struct Npc {
    pub id: u32,
    pub name: u8,
    pub look: u8,
    pub x: f32,
    pub y: f32,
    pub state: u8,
    pub hearts: u8,
    pub mood: i32,
    pub married: bool,
    pub child: bool,
    pub role: u8,
    pub bed: (i32, i32),
    pub anim: f32,
    pub wt: f32,
    pub wd: (f32, f32),
    pub atk_cd: f32,
    pub talked: bool,
    pub gifted: bool,
    pub moving: bool,
    pub left: bool,
}

pub const VILLAGE_OFFERS: [Offer; 5] = [
    Offer { name: "เมล็ดข้าวโพด x5", desc: "ราคาชาวบ้าน", item: SEED_CORN, qty: 5, price: 8, once: false },
    Offer { name: "เมล็ดแครอท x5", desc: "ราคาชาวบ้าน", item: SEED_CARROT, qty: 5, price: 8, once: false },
    Offer { name: "ยาฟื้นฟู x2", desc: "ฟื้นเลือด 8 หน่วย", item: POTION, qty: 2, price: 16, once: false },
    Offer { name: "ลูกธนู x20", desc: "กระสุนธนู", item: ARROW, qty: 20, price: 12, once: false },
    Offer { name: "อัญมณี x1", desc: "พ่อค้าเฉพาะทางหามาให้", item: GEM, qty: 1, price: 45, once: false },
];

fn day_of(g: &Game) -> i32 { (g.time / DAYLEN) as i32 }

impl World {
    /// NPC หลงทางในเซลล์ (กำหนดแน่นอนจาก seed) → (x, y, ชื่อ, หน้าตา)
    pub fn lost_in_cell(&self, cx: i32, cy: i32) -> Option<(f32, f32, u8, u8)> {
        if self.dun.is_some() { return None; }
        let s = self.seed.wrapping_add(777);
        if hash(cx, cy, s) > 0.22 { return None; }
        let tx = cx * CELL + 6 + (hash(cx, cy, s.wrapping_add(1)) * (CELL - 12) as f64) as i32;
        let ty = cy * CELL + 6 + (hash(cx, cy, s.wrapping_add(2)) * (CELL - 12) as f64) as i32;
        for (dx, dy) in [(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1), (1, 1)] {
            if !self.solid_at(tx + dx, ty + dy) && self.tile_at(tx + dx, ty + dy) >= 2 && self.obj_at(tx + dx, ty + dy) == 0 {
                let name = (hash(cx, cy, s.wrapping_add(3)) * NPC_NAMES.len() as f64) as u8;
                let look = (hash(cx, cy, s.wrapping_add(4)) * LOOKS as f64) as u8;
                return Some(((tx + dx) as f32 * 16. + 8., (ty + dy) as f32 * 16. + 12., name, look));
            }
        }
        None
    }
}

impl Game {
    // ---------------- ข้อมูลชุมชน ----------------
    pub fn residents(&self) -> impl Iterator<Item = &Npc> { self.npcs.iter().filter(|n| n.state == ST_HOME) }
    pub fn pop(&self) -> usize { self.residents().count() }
    pub fn beds(&self) -> Vec<(i32, i32)> { self.w.placed.iter().filter(|(_, &o)| o == 33).map(|(&k, _)| k).collect() }
    pub fn free_beds(&self) -> Vec<(i32, i32)> {
        let used: HashSet<(i32, i32)> = self.residents().map(|n| n.bed).collect();
        self.beds().into_iter().filter(|b| !used.contains(b)).collect()
    }
    pub fn count_obj(&self, o: u8) -> usize { self.w.placed.values().filter(|&&p| p == o).count() }
    pub fn has_castle(&self) -> bool { self.count_obj(37) > 0 }
    pub fn walls(&self) -> usize { self.count_obj(38) }
    /// 0 ไม่มี · 1 ค่าย · 2 หมู่บ้าน · 3 อาณาจักร
    pub fn tier(&self) -> u8 {
        let n = self.pop();
        if n == 0 { 0 } else if n <= 3 { 1 } else if n <= 10 { 2 } else if self.has_castle() && self.walls() >= 12 { 3 } else { 2 }
    }
    pub fn food_stock(&self) -> i32 { self.chests.values().map(|c| FOODS.iter().map(|&i| c[i]).sum::<i32>()).sum() }
    fn take_food(&mut self, near: (i32, i32)) -> bool {
        let keys: Vec<(i32, i32)> = self.chests.keys().copied().filter(|k| (k.0 - near.0).abs() <= 16 && (k.1 - near.1).abs() <= 16).collect();
        for k in keys {
            for &f in &FOODS {
                if let Some(c) = self.chests.get_mut(&k) {
                    if c[f] > 0 { c[f] -= 1; return true; }
                }
            }
        }
        false
    }
    fn give_to_chest(&mut self, near: (i32, i32), item: usize, n: i32) {
        let best = self.chests.keys().copied().filter(|k| (k.0 - near.0).abs() <= 16 && (k.1 - near.1).abs() <= 16).min_by_key(|k| (k.0 - near.0).abs() + (k.1 - near.1).abs());
        match best {
            Some(k) => { self.chests.get_mut(&k).unwrap()[item] += n; }
            None => { self.p.inv[item] += n; }
        }
    }
    pub fn assign_roles_pub(&mut self) { self.assign_roles(); }
    fn assign_roles(&mut self) {
        let t = self.tier();
        let mut i = 0;
        for n in self.npcs.iter_mut().filter(|n| n.state == ST_HOME) {
            if n.child { n.role = 0; continue; }
            n.role = if t <= 1 { 1 } else if i == 0 { 3 } else if i % 3 == 1 { 2 } else { 1 };
            i += 1;
        }
    }

    // ---------------- ค้นหา/โต้ตอบ ----------------
    pub fn npc_near(&self) -> Option<u32> {
        self.npcs.iter().filter(|n| !n.child || n.state != ST_LOST).filter(|n| n.state != ST_LOST)
            .map(|n| ((n.x - self.p.x).hypot(n.y - (self.p.y - 3.)), n.id))
            .filter(|d| d.0 < 20.).min_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).map(|d| d.1)
    }
    pub fn lost_near(&self) -> Option<(i32, i32)> {
        let (pcx, pcy) = (tfloor(self.p.x).div_euclid(CELL), tfloor(self.p.y).div_euclid(CELL));
        for cy in pcy - 1..=pcy + 1 {
            for cx in pcx - 1..=pcx + 1 {
                if self.npc_seen.contains(&(cx, cy)) { continue; }
                if let Some((x, y, _, _)) = self.w.lost_in_cell(cx, cy) {
                    if (x - self.p.x).hypot(y - (self.p.y - 3.)) < 22. { return Some((cx, cy)); }
                }
            }
        }
        None
    }
    pub fn npc_hint(&self, touch: bool) -> Option<String> {
        let key = if touch { "ปุ่ม \"ใช้\"" } else { "Q" };
        if self.lost_near().is_some() { return Some(format!("{}: ช่วยผู้หลงทาง", key)); }
        let id = self.npc_near()?;
        let n = self.npcs.iter().find(|n| n.id == id)?;
        Some(format!("{}: คุยกับ{}", key, NPC_NAMES[n.name as usize]))
    }
    /// กดใช้: ช่วยคนหลงทาง หรือคุย (คืน true ถ้าจัดการแล้ว)
    pub fn npc_interact(&mut self) -> bool {
        if let Some(c) = self.lost_near() {
            if let Some((x, y, name, look)) = self.w.lost_in_cell(c.0, c.1) {
                self.npc_seen.insert(c);
                let id = self.npc_next;
                self.npc_next += 1;
                self.npcs.push(Npc { id, name, look, x, y, state: ST_FOLLOW, hearts: 2, mood: 60, married: false, child: false, role: 0, bed: (0, 0), anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false });
                self.toast(format!("ช่วย{}ไว้แล้ว! คุยแล้วเลือก \"ชวนมาอยู่ที่ฐาน\" (ต้องมีเตียงว่าง)", NPC_NAMES[name as usize]));
                play(&self.sfx.quest);
                self.burst(x, y - 8., hx(0xffee66), 12, 60.);
                self.save();
                return true;
            }
        }
        if let Some(id) = self.npc_near() {
            self.req = Some(Req::Npc(id));
            play(&self.sfx.click);
            return true;
        }
        false
    }

    fn idx(&self, id: u32) -> Option<usize> { self.npcs.iter().position(|n| n.id == id) }
    pub fn npc_talk(&mut self, id: u32) {
        let Some(i) = self.idx(id) else { return };
        if self.npcs[i].talked { self.toast("วันนี้คุยกันไปแล้ว พรุ่งนี้ค่อยมาใหม่"); return; }
        let n = &mut self.npcs[i];
        n.talked = true;
        n.hearts = (n.hearts + 1).min(10);
        n.mood = (n.mood + 8).min(100);
        let nm = NPC_NAMES[n.name as usize];
        self.toast(format!("คุยกับ{} — สนิทกันขึ้น", nm));
        play(&self.sfx.pickup);
        self.save();
    }
    pub fn npc_gift(&mut self, id: u32) {
        let Some(i) = self.idx(id) else { return };
        if self.npcs[i].gifted { self.toast("วันนี้ให้ของขวัญไปแล้ว"); return; }
        let (item, hearts) = if self.p.inv[GEM] > 0 { (GEM, 2u8) } else if self.p.inv[CORN] > 0 { (CORN, 1) } else if self.p.inv[CARROT] > 0 { (CARROT, 1) } else if self.p.inv[BERRY] > 0 { (BERRY, 1) } else { self.toast("ไม่มีของขวัญ (อัญมณี/ข้าวโพด/แครอท/เบอร์รี่)"); return; };
        self.p.inv[item] -= 1;
        let n = &mut self.npcs[i];
        n.gifted = true;
        n.hearts = (n.hearts + hearts).min(10);
        n.mood = (n.mood + 14).min(100);
        let nm = NPC_NAMES[n.name as usize];
        self.toast(format!("มอบ{}ให้{} — ดีใจมาก!", INAME[item], nm));
        play(&self.sfx.gem);
        self.save();
    }
    pub fn npc_invite(&mut self, id: u32) {
        let Some(i) = self.idx(id) else { return };
        let beds = self.free_beds();
        let (px, py) = (self.p.x, self.p.y);
        let best = beds.into_iter().min_by(|a, b| {
            let d = |k: &(i32, i32)| (k.0 as f32 * 16. - px).hypot(k.1 as f32 * 16. - py);
            d(a).partial_cmp(&d(b)).unwrap()
        });
        match best {
            Some(b) => {
                let n = &mut self.npcs[i];
                n.state = ST_HOME;
                n.bed = b;
                n.mood = n.mood.max(55);
                let nm = NPC_NAMES[n.name as usize];
                self.assign_roles();
                self.toast(format!("{}ย้ายมาอยู่ที่ฐานแล้ว!", nm));
                play(&self.sfx.upgrade);
                self.save();
            }
            None => self.toast("ไม่มีเตียงว่าง — สร้างเตียงเพิ่ม (เมนูสร้าง) แล้วชวนใหม่"),
        }
    }
    pub fn npc_marry(&mut self, id: u32) {
        let Some(i) = self.idx(id) else { return };
        let n = &self.npcs[i];
        if n.child || n.state != ST_HOME { return; }
        if self.npcs.iter().any(|x| x.married) { self.toast("แต่งงานได้คนเดียว"); return; }
        if n.hearts < 8 { self.toast("ต้องสนิทกันถึง 8/10 ก่อน (คุย/ให้ของขวัญทุกวัน)"); return; }
        if n.mood < 50 { self.toast("ตอนนี้เขายังไม่มีความสุขพอ — ดูแลอาหาร/เตียงก่อน"); return; }
        if self.p.inv[COIN] < MARRY_COIN || self.p.inv[GEM] < MARRY_GEM { self.toast(format!("ต้องใช้ {} เหรียญ + {} อัญมณี (แหวน+งานเลี้ยง)", MARRY_COIN, MARRY_GEM)); return; }
        self.p.inv[COIN] -= MARRY_COIN;
        self.p.inv[GEM] -= MARRY_GEM;
        let day = day_of(self);
        let (x, y) = (self.npcs[i].x, self.npcs[i].y);
        let n = &mut self.npcs[i];
        n.married = true;
        n.hearts = 10;
        n.mood = 100;
        self.wed_day = day;
        let nm = NPC_NAMES[self.npcs[i].name as usize];
        self.toast(format!("ยินดีด้วย! คุณแต่งงานกับ{}แล้ว", nm));
        play(&self.sfx.lvl);
        self.burst(x, y - 10., hx(0xff88aa), 24, 90.);
        self.save();
    }
    pub fn npc_buy(&mut self, i: usize) {
        let o = &VILLAGE_OFFERS[i];
        if self.p.inv[COIN] < o.price { self.toast("เหรียญไม่พอ"); play(&self.sfx.deny); return; }
        self.p.inv[COIN] -= o.price;
        self.p.inv[o.item] += o.qty;
        play(&self.sfx.buy);
        self.toast(format!("ซื้อ {} แล้ว", o.name));
        self.save();
    }
    pub fn collect_tax(&mut self) {
        if self.tax_pool <= 0 { self.toast("ยังไม่มีภาษีให้เก็บ"); return; }
        self.p.inv[COIN] += self.tax_pool;
        self.toast(format!("เก็บภาษีได้ {} เหรียญ", self.tax_pool));
        self.tax_pool = 0;
        play(&self.sfx.buy);
        self.save();
    }
    pub fn npc_dismiss(&mut self, id: u32) {
        if let Some(i) = self.idx(id) {
            let nm = NPC_NAMES[self.npcs[i].name as usize];
            self.npcs.remove(i);
            self.assign_roles();
            self.toast(format!("{}ย้ายออกไป", nm));
            self.save();
        }
    }

    // ---------------- รุ่งเช้า: อาหาร เตียง ความสุข งานชาวไร่ ภาษี ลูก ----------------
    pub fn npc_dawn(&mut self) {
        let mut fed_n = 0;
        let mut hungry_n = 0;
        let ids: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME).map(|n| n.id).collect();
        for id in &ids {
            let Some(i) = self.idx(*id) else { continue };
            let bed = self.npcs[i].bed;
            let bed_ok = self.w.placed.get(&bed) == Some(&33);
            let fed = self.take_food(bed);
            let n = &mut self.npcs[i];
            n.mood += if bed_ok { 3 } else { -25 };
            n.mood += if fed { 5 } else { -20 };
            if n.married { n.mood += 4; }
            if n.hearts >= 5 { n.mood += 2; }
            n.mood = n.mood.clamp(0, 100);
            n.talked = false;
            n.gifted = false;
            if fed { fed_n += 1 } else { hungry_n += 1 }
        }
        for n in self.npcs.iter_mut().filter(|n| n.state != ST_HOME) { n.talked = false; n.gifted = false; }
        // ย้ายออกเมื่อความสุข 0
        let leaving: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME && n.mood <= 0).map(|n| n.id).collect();
        for id in &leaving {
            if let Some(i) = self.idx(*id) {
                let nm = NPC_NAMES[self.npcs[i].name as usize];
                let why = if self.w.placed.get(&self.npcs[i].bed) != Some(&33) { "ไม่มีเตียง" } else { "หิวและไม่มีความสุข" };
                self.npcs.remove(i);
                self.toast(format!("{}ย้ายออกไปแล้ว ({})", nm, why));
            }
        }
        self.assign_roles();
        // ชาวไร่: รดน้ำ เก็บเกี่ยว ปลูกซ้ำ
        let farmers: Vec<(i32, i32)> = self.npcs.iter().filter(|n| n.state == ST_HOME && n.role == 1 && n.mood >= 20).map(|n| n.bed).collect();
        let mut harvested = 0;
        for bed in farmers {
            let keys: Vec<(i32, i32)> = self.crops.keys().copied().filter(|k| (k.0 - bed.0).abs() <= 14 && (k.1 - bed.1).abs() <= 14).take(10).collect();
            for k in keys {
                let ripe = self.crops.get(&k).map_or(false, |c| c.stage() == 2);
                if ripe {
                    let kind = self.crops[&k].kind as usize;
                    self.give_to_chest(bed, HARVEST_OF[kind], 2);
                    harvested += 2;
                    if let Some(c) = self.crops.get_mut(&k) { c.grown = 0.; c.wet = WET_T; }
                } else if let Some(c) = self.crops.get_mut(&k) {
                    c.wet = c.wet.max(WET_T);
                }
            }
        }
        // ภาษี (อาณาจักร): ชาวบ้านสุขพอจึงจ่าย
        if self.tier() == 3 {
            let happy = self.residents().filter(|n| n.mood >= 40).count() as i32;
            self.tax_pool += happy * 4;
        }
        // ลูก: หลังแต่งงาน 3 วัน มีเตียงว่าง
        let day = day_of(self);
        let spouse = self.npcs.iter().find(|n| n.married && n.state == ST_HOME).map(|n| (n.x, n.y));
        let kids = self.npcs.iter().filter(|n| n.child).count();
        if let Some((sx, sy)) = spouse {
            if day - self.wed_day >= 3 && kids < 2 && day - self.last_child >= 4 {
                if let Some(b) = self.free_beds().first().copied() {
                    let id = self.npc_next;
                    self.npc_next += 1;
                    self.npcs.push(Npc { id, name: (id as usize * 5 % NPC_NAMES.len()) as u8, look: (id % LOOKS as u32) as u8, x: sx + 4., y: sy, state: ST_HOME, hearts: 10, mood: 90, married: false, child: true, role: 0, bed: b, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false });
                    self.last_child = day;
                    self.toast(format!("ครอบครัวของคุณมีสมาชิกใหม่: {}!", NPC_NAMES[self.npcs.last().unwrap().name as usize]));
                    play(&self.sfx.lvl);
                }
            }
        }
        if !ids.is_empty() {
            let mut s = format!("รุ่งเช้า: ชาวบ้าน {} คน อิ่ม {} หิว {}", ids.len(), fed_n, hungry_n);
            if harvested > 0 { s += &format!(" · ชาวไร่เก็บผลผลิต {}", harvested); }
            self.toast(s);
        }
        self.save();
    }

    // ---------------- ทุกเฟรม ----------------
    pub fn npc_tick(&mut self, dt: f32) {
        let day = day_of(self);
        if self.last_day < 0 { self.last_day = day; }
        let mut guard = 0;
        while self.last_day < day && guard < 3 {
            self.last_day += 1;
            guard += 1;
            self.npc_dawn();
        }
        self.last_day = day;
        // ได้ยินเสียงขอความช่วยเหลือ (ครั้งเดียวต่อคน)
        self.lost_hint_t -= dt;
        if self.lost_hint_t <= 0. && self.w.dun.is_none() {
            self.lost_hint_t = 2.;
            let (pcx, pcy) = (tfloor(self.p.x).div_euclid(CELL), tfloor(self.p.y).div_euclid(CELL));
            'o: for cy in pcy - 1..=pcy + 1 {
                for cx in pcx - 1..=pcx + 1 {
                    if self.npc_seen.contains(&(cx, cy)) || self.lost_told.contains(&(cx, cy)) { continue; }
                    if let Some((x, y, _, _)) = self.w.lost_in_cell(cx, cy) {
                        if (x - self.p.x).hypot(y - self.p.y) < 150. {
                            self.lost_told.insert((cx, cy));
                            self.toast("ได้ยินเสียงเรียกขอความช่วยเหลือแถวนี้!");
                            break 'o;
                        }
                    }
                }
            }
        }
        if self.w.dun.is_some() { return; }
        let night = self.is_night_hr();
        let (px, py) = (self.p.x, self.p.y);
        let enemies: Vec<(usize, f32, f32)> = self.en.iter().enumerate().filter(|(_, e)| !e.dead).map(|(i, e)| (i, e.x, e.y)).collect();
        let mut hits: Vec<(usize, f32, f32)> = vec![];
        let mut npcs = std::mem::take(&mut self.npcs);
        for n in npcs.iter_mut() {
            n.anim += dt;
            n.atk_cd = (n.atk_cd - dt).max(0.);
            n.wt -= dt;
            let (mut tx, mut ty, mut spd) = (n.x, n.y, 0.);
            let home = (n.bed.0 as f32 * 16. + 8., n.bed.1 as f32 * 16. + 18.);
            match n.state {
                ST_FOLLOW => {
                    let d = (px - n.x).hypot(py - n.y);
                    if d > 240. { n.x = px - 12.; n.y = py; }
                    if d > 26. { tx = px; ty = py; spd = 70.; }
                }
                _ => {
                    if n.role == 2 && n.mood >= 20 {
                        // ทหารยาม: เข้าหามอนสเตอร์ใกล้บ้าน ฟันให้
                        let tgt = enemies.iter().filter(|e| (e.1 - home.0).hypot(e.2 - home.1) < 170.).min_by(|a, b| ((a.1 - n.x).hypot(a.2 - n.y)).partial_cmp(&((b.1 - n.x).hypot(b.2 - n.y))).unwrap());
                        if let Some(&(ei, ex, ey)) = tgt {
                            let d = (ex - n.x).hypot(ey - n.y);
                            if d > 14. { tx = ex; ty = ey; spd = 78.; } else if n.atk_cd <= 0. { n.atk_cd = 0.9; hits.push((ei, (ex - n.x) / d.max(1.), (ey - n.y) / d.max(1.))); }
                        }
                    }
                    if spd == 0. {
                        if night { tx = home.0; ty = home.1; if (n.x - tx).hypot(n.y - ty) > 6. { spd = 40.; } }
                        else {
                            if n.wt <= 0. { n.wt = 1.5 + (n.id % 3) as f32; let a = (n.id as f32 * 1.7 + self.time * 0.31).sin() * 3.14; n.wd = (a.cos() * 40., a.sin() * 40.); }
                            tx = home.0 + n.wd.0; ty = home.1 + n.wd.1;
                            if (n.x - tx).hypot(n.y - ty) > 4. { spd = 30.; }
                        }
                    }
                }
            }
            n.moving = false;
            if spd > 0. {
                let d = (tx - n.x).hypot(ty - n.y).max(0.01);
                let (vx, vy) = ((tx - n.x) / d * spd * dt, (ty - n.y) / d * spd * dt);
                let (nx, ny) = (n.x + vx, n.y + vy);
                if !self.w.solid_at(tfloor(nx), tfloor(n.y - 3.)) { n.x = nx; n.moving = true; }
                if !self.w.solid_at(tfloor(n.x), tfloor(ny - 3.)) { n.y = ny; n.moving = true; }
                if vx.abs() > 0.01 { n.left = vx < 0.; }
            }
        }
        self.npcs = npcs;
        let bonus = self.tier() as i32;
        for (ei, dx, dy) in hits {
            if ei < self.en.len() && !self.en[ei].dead {
                let (ex, ey) = (self.en[ei].x, self.en[ei].y - 8.);
                self.damage_enemy(ei, 2 + bonus, false, dx, dy);
                self.add_float(ex, ey - 6., "ยาม!", hx(0x88ccff), None);
            }
        }
    }
}

// ---------------- วาด ----------------
pub fn draw_npc(g: &Game, sp: &Sprites, v: &View, i: usize, t_now: f32) {
    let n = &g.npcs[i];
    let (sx, sy) = ((n.x - v.cx).round(), (n.y - v.cy).round());
    let f = if n.moving { ((n.anim * 7.) as usize) % 2 } else { 0 };
    let bob = if n.moving { ((n.anim * 14.).sin().abs() * 1.0).round() } else { 0. };
    prect(v, sx - 4., sy - 1., 8., 2., Color::new(0., 0., 0., 0.25));
    let r = sp.skin.npc[(n.look as usize * 4 + n.role as usize) * 2 + f];
    let k = if n.child { 0.72 } else { 1. };
    blit_sz_flip(sp, v, r, sx - 6. * k, sy - 15. * k - bob, r.w * k, r.h * k, n.left);
    if n.married {
        prect(v, sx - 2., sy - 20. * k - bob, 2., 1., hx(0xff6688));
        prect(v, sx + 1., sy - 20. * k - bob, 2., 1., hx(0xff6688));
        prect(v, sx - 1., sy - 19. * k - bob, 3., 1., hx(0xff6688));
    }
    if n.state == ST_HOME && n.mood < 25 {
        text_c_world(v, "!", sx, sy - 19. * k - bob, hx(0xff4444), t_now);
    }
}
pub fn draw_lost(g: &Game, sp: &Sprites, v: &View, x: f32, y: f32, look: u8, t_now: f32) {
    let (sx, sy) = ((x - v.cx).round(), (y - v.cy).round());
    prect(v, sx - 4., sy - 1., 8., 2., Color::new(0., 0., 0., 0.25));
    let r = sp.skin.npc[(look as usize * 4) * 2];
    blit_sz_flip(sp, v, r, sx - 6., sy - 15., r.w, r.h, false);
    let b = ((t_now * 5.).sin() * 1.5).round();
    prect(v, sx - 1., sy - 25. + b, 2., 5., hx(0xffee44));
    prect(v, sx - 1., sy - 18. + b, 2., 2., hx(0xffee44));
    let _ = g;
}
fn blit_sz_flip(sp: &Sprites, v: &View, r: Rect, x: f32, y: f32, w: f32, h: f32, flip: bool) {
    draw_texture_ex(&sp.tex, x * v.s, y * v.s, WHITE, DrawTextureParams { source: Some(r), dest_size: Some(vec2(w * v.s, h * v.s)), flip_x: flip, ..Default::default() });
}
fn text_c_world(v: &View, _s: &str, x: f32, y: f32, c: Color, t: f32) {
    let b = ((t * 6.).sin() * 1.).round();
    prect(v, x - 1., y - 5. + b, 2., 4., c);
    prect(v, x - 1., y + b, 2., 1., c);
}

// ---------------- เซฟ ----------------
impl Game {
    pub fn npc_save_str(&self) -> String {
        let mut s = format!("town {} {} {} {} {}\n", self.npc_next, self.last_day, self.tax_pool, self.wed_day, self.last_child);
        for n in &self.npcs {
            s += &format!("npc {} {} {} {} {} {} {} {} {} {} {} {} {}\n", n.id, n.name, n.look, n.x, n.y, n.state, n.hearts, n.mood, n.married as u8, n.child as u8, n.role, n.bed.0, n.bed.1);
        }
        if !self.npc_seen.is_empty() {
            s += "npcseen";
            for (x, y) in &self.npc_seen { s += &format!(" {},{}", x, y); }
            s += "\n";
        }
        s
    }
    pub fn npc_load_line(&mut self, tag: &str, rest: &[&str]) {
        let f = |i: usize| rest.get(i).and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.);
        match tag {
            "town" => { self.npc_next = f(0) as u32; self.last_day = f(1) as i32; self.tax_pool = f(2) as i32; self.wed_day = f(3) as i32; self.last_child = f(4) as i32; }
            "npc" => self.npcs.push(Npc { id: f(0) as u32, name: (f(1) as u8) % NPC_NAMES.len() as u8, look: (f(2) as u8) % LOOKS as u8, x: f(3), y: f(4), state: (f(5) as u8).min(2), hearts: (f(6) as u8).min(10), mood: f(7) as i32, married: f(8) > 0., child: f(9) > 0., role: (f(10) as u8).min(3), bed: (f(11) as i32, f(12) as i32), anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false }),
            "npcseen" => for t in rest { if let Some((a, b)) = t.split_once(',') { if let (Ok(a), Ok(b)) = (a.parse(), b.parse()) { self.npc_seen.insert((a, b)); } } },
            _ => {}
        }
    }
}
