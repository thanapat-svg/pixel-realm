//! ระบบ NPC & สังคม: ช่วยผู้หลงทาง → ชวนมาอยู่ฐาน → ความสัมพันธ์/แต่งงาน/ลูก
//! → เตียง+อาหารกำหนดความสุข → ระดับ ค่าย (1-3 คน) / หมู่บ้าน (4-10) / อาณาจักร (>10 + ปราสาท + กำแพง)
use super::*;

pub const NPC_NAMES: [&str; 36] = ["มะลิ", "ตะวัน", "บุญเรือง", "น้ำฝน", "สายลม", "ปลายฟ้า", "ทองดี", "จันทร์เพ็ญ", "กล้า", "ใบบัว", "ไผ่", "รวงข้าว",
    "ดวงใจ", "ภูผา", "ขวัญ", "สมชาย", "แก้ว", "ธงชัย", "ลำดวน", "เก่ง", "ส้มโอ", "ต้นกล้า", "มุกดา", "วิชัย", "ฟ้าใส", "ชาติ", "น้ำผึ้ง", "แสงทอง", "ดาวเรือง", "คำปัน", "ชบา", "ภูมิ", "อ้อม", "นที", "พิมพ์", "อาทิตย์"];
/// เพศตามชื่อ (true = หญิง) — ใช้กับรูปร่างใน v7c
pub const NPC_FEM: [bool; 36] = [true, false, false, true, false, true, false, true, false, true, false, true,
    true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false];
pub const ROLE_N: [&str; 14] = ["ชาวบ้าน", "ชาวไร่", "ทหารยาม", "พ่อค้า", "ช่างไม้", "คนตัดไม้", "คนขุดแร่", "นักล่า/ชาวประมง", "พ่อครัว", "หมอ", "ทหาร", "แม่ทัพ", "ทูต", "เสมียนคลัง"];
pub const TIER_N: [&str; 4] = ["ยังไม่มีชุมชน", "ค่าย", "หมู่บ้าน", "เมือง"];
/// ค่า look = เมล็ดสุ่มรูปร่างหน้าตาของชาวบ้าน (0..250) — ดู body.rs
pub const LOOKS: usize = 250;
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
    pub soc: Soc,
}

pub const VILLAGE_OFFERS: [Offer; 5] = [
    Offer { name: "เมล็ดข้าวโพด x5", desc: "ราคาชาวบ้าน", item: SEED_CORN, qty: 5, price: 8, once: false },
    Offer { name: "เมล็ดแครอท x5", desc: "ราคาชาวบ้าน", item: SEED_CARROT, qty: 5, price: 8, once: false },
    Offer { name: "ยาฟื้นฟู x2", desc: "ฟื้นเลือด 8 หน่วย", item: POTION, qty: 2, price: 16, once: false },
    Offer { name: "ลูกธนู x20", desc: "กระสุนธนู", item: ARROW, qty: 20, price: 12, once: false },
    Offer { name: "อัญมณี x1", desc: "พ่อค้าเฉพาะทางหามาให้", item: GEM, qty: 1, price: 45, once: false },
];

pub fn day_of(g: &Game) -> i32 { (g.time / DAYLEN) as i32 }

impl World {
    /// NPC หลงทางในเซลล์ (กำหนดแน่นอนจาก seed) → (x, y, ชื่อ, หน้าตา)
    pub fn lost_in_cell(&self, cx: i32, cy: i32) -> Option<(f32, f32, u8, u8)> {
        if self.dun.is_some() { return None; }
        let s = self.seed.wrapping_add(777);
        if hash(cx, cy, s) > 0.22 { return None; }
        let tx = cx * CELL + 6 + (hash(cx, cy, s.wrapping_add(1)) * (CELL - 12) as f64) as i32;
        let ty = cy * CELL + 6 + (hash(cx, cy, s.wrapping_add(2)) * (CELL - 12) as f64) as i32;
        if self.near_city(tx, ty, 3) { return None; }
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
    /// กินอาหาร 1 ชิ้นจากกล่องใกล้เตียง — เลือกของที่ชอบก่อน เลี่ยงของที่เกลียด (คืนไอเทมที่กิน)
    fn take_food_for(&mut self, near: (i32, i32), idx: usize) -> Option<usize> {
        let (love, hate) = (self.npcs[idx].soc.love as usize, self.npcs[idx].soc.hate as usize);
        let keys: Vec<(i32, i32)> = self.chests.keys().copied().filter(|k| (k.0 - near.0).abs() <= 16 && (k.1 - near.1).abs() <= 16).collect();
        let mut best: Option<(i32, (i32, i32), usize)> = None;
        for k in keys {
            let c = self.chests[&k];
            for &f in &FOODS {
                if c[f] <= 0 { continue; }
                let score = if f == love { 3 } else if f == hate { -3 } else if (DISH_STEW..=DISH_FISH).contains(&f) { 1 } else { 0 };
                if best.map_or(true, |b| score > b.0) { best = Some((score, k, f)); }
            }
        }
        let (_, k, f) = best?;
        self.chests.get_mut(&k)?[f] -= 1;
        Some(f)
    }
    pub fn give_to_chest(&mut self, near: (i32, i32), item: usize, n: i32) {
        let best = self.chests.keys().copied().filter(|k| (k.0 - near.0).abs() <= 16 && (k.1 - near.1).abs() <= 16).min_by_key(|k| (k.0 - near.0).abs() + (k.1 - near.1).abs());
        match best {
            Some(k) => { self.chests.get_mut(&k).unwrap()[item] += n; }
            None => { self.p.inv[item] += n; }
        }
    }
    pub fn assign_roles_pub(&mut self) { self.assign_roles(); }
    /// แบ่งงานตามความถนัดจริง: คนกล้า = ยาม · คนโลภ+ช่างพูด = พ่อค้า · ที่เหลือทำไร่
    fn assign_roles(&mut self) {
        let t = self.tier();
        let adults: Vec<usize> = (0..self.npcs.len()).filter(|&i| self.npcs[i].state == ST_HOME && !self.npcs[i].child).collect();
        // อาชีพที่ผู้เล่นเลือกเองคงไว้ · ที่เหลือเกมจัดให้
        let manual = self.rs.manual.clone();
        for i in 0..self.npcs.len() { if self.npcs[i].state == ST_HOME { self.npcs[i].role = if self.npcs[i].child { 0 } else { manual.get(&self.npcs[i].id).copied().unwrap_or(1) }; } }
        let adults: Vec<usize> = adults.into_iter().filter(|&i| !manual.contains_key(&self.npcs[i].id)).collect();
        if t <= 1 || adults.is_empty() { return; }
        let m = *adults.iter().max_by_key(|&&i| { let s = &self.npcs[i].soc.tr; (s[T_GREED] as i32 + s[T_SOCIAL] as i32, -(self.npcs[i].id as i32)) }).unwrap();
        self.npcs[m].role = 3;
        let mut rest: Vec<usize> = adults.into_iter().filter(|&i| i != m).collect();
        rest.sort_by_key(|&i| (-(self.npcs[i].soc.tr[T_BRAVE] as i32), self.npcs[i].id));
        let guards = (rest.len() / 3).max(1).min(rest.len());
        for &i in rest.iter().take(guards) { self.npcs[i].role = 2; }
    }

    // ---------------- ค้นหา/โต้ตอบ ----------------
    pub fn npc_near(&self) -> Option<u32> { self.npc_near_r(24.) }
    /// ชาวบ้านที่ใกล้ที่สุดในรัศมี r พิกเซล (ไม่นับคนหลงทาง/คนที่หลับ)
    pub fn npc_near_r(&self, r: f32) -> Option<u32> {
        self.npcs.iter().filter(|n| n.state != ST_LOST && !self.rs.sleeping.contains(&n.id))
            .map(|n| ((n.x - self.p.x).hypot(n.y - (self.p.y - 3.)), n.id))
            .filter(|d| d.0 < r).min_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).map(|d| d.1)
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
        Some(format!("{}: {}{}", key, if n.talked { "เมนู " } else { "คุยกับ" }, NPC_NAMES[n.name as usize]))
    }
    /// กดใช้: ช่วยคนหลงทาง หรือคุย (คืน true ถ้าจัดการแล้ว)
    pub fn npc_interact(&mut self) -> bool {
        if let Some(c) = self.lost_near() {
            if let Some((x, y, name, look)) = self.w.lost_in_cell(c.0, c.1) {
                self.npc_seen.insert(c);
                let id = self.npc_next;
                self.npc_next += 1;
                // ไม่ให้ชื่อซ้ำกับคนที่มีอยู่แล้ว
                let mut name = name;
                // (ชื่อใหม่เพศเดียวกัน เพื่อให้หน้าตาที่เห็นตอนหลงทางตรงกับตอนช่วยแล้ว)
                if self.npcs.iter().any(|x| x.name == name) {
                    let fem = NPC_FEM[name as usize];
                    if let Some(c) = (1..NPC_NAMES.len()).map(|k| (name as usize + k) % NPC_NAMES.len()).find(|&c| NPC_FEM[c] == fem && !self.npcs.iter().any(|x| x.name as usize == c)) { name = c as u8; }
                }
                self.npcs.push(Npc { id, name, look, x, y, state: ST_FOLLOW, hearts: 2, mood: 60, married: false, child: false, role: 0, bed: (0, 0), anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc { last_talk: day_of(self), ..Soc::generate(id, name) } });
                self.toast(format!("ช่วย{}ไว้แล้ว! คุยแล้วเลือก \"ชวนมาอยู่ที่ฐาน\" (ต้องมีเตียงว่าง)", NPC_NAMES[name as usize]));
                play(&self.sfx.quest);
                self.burst(x, y - 8., hx(0xffee66), 12, 60.);
                self.save();
                return true;
            }
        }
        if let Some(id) = self.npc_near() {
            // v19: กดใช้ = คุยทันที (ครั้งแรกของวัน) · คุยแล้วกดอีก = เปิดเมนูชาวบ้าน
            let talked = self.idx(id).map_or(true, |i| self.npcs[i].talked);
            if !talked { self.quick_talk(id); } else { self.req = Some(Req::Npc(id)); play(&self.sfx.click); }
            return true;
        }
        false
    }

    pub fn idx(&self, id: u32) -> Option<usize> { self.npcs.iter().position(|n| n.id == id) }
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
        if n.soc.grudge >= 15 || n.soc.anger > 0 { self.toast("เขายังมีเรื่องค้างคาใจกับคุณ — ขอโทษ/ง้อให้หายก่อน"); return; }
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
        let sid = self.npcs[i].id;
        self.social_on_marry(sid);
        self.save();
    }
    pub fn npc_buy(&mut self, id: u32, i: usize) {
        let o = &VILLAGE_OFFERS[i];
        let Some(price) = self.npc_price(id, o.price) else { self.toast("พ่อค้าไม่ยอมขายให้คุณตอนนี้ (เขาไม่พอใจคุณ)"); play(&self.sfx.deny); return; };
        if self.p.inv[COIN] < price { self.toast("เหรียญไม่พอ"); play(&self.sfx.deny); return; }
        self.p.inv[COIN] -= price;
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
        if self.idx(id).is_some() { self.social_on_dismiss(id); }
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
        self.psy_dawn();
        let mut fed_n = 0;
        let mut hungry_n = 0;
        let mut thirsty_n = 0;
        let mut roofless_n = 0;
        let ids: Vec<u32> = self.npcs.iter().filter(|n| n.state == ST_HOME).map(|n| n.id).collect();
        let served = self.water_served();
        for id in &ids {
            let Some(i) = self.idx(*id) else { continue };
            let bed = self.npcs[i].bed;
            let town_well = self.big_well_near(bed);
            let bed_ok = self.w.placed.get(&bed) == Some(&33);
            let eaten = self.take_food_for(bed, i);
            let fed = eaten.is_some();
            let has_water = served.contains(&bed);
            let room = if bed_ok { self.w.room_from(bed) } else { None };
            let house = room.is_some();
            let comfy = room.map_or(0, |r| r.comfort);
            let n = &mut self.npcs[i];
            n.mood += if bed_ok { 3 } else { -25 };
            // มีบ้านมีหลังคา = อุ่นใจ +4 · นอนกลางแจ้ง −3
            if bed_ok { if house { n.mood += 4 + if comfy >= 3 { 2 } else { 0 }; } else { n.mood -= 3; roofless_n += 1; } }
            n.mood += if fed { 5 } else { -20 };
            // น้ำดื่ม: ต้องมีแหล่งน้ำจืด/บ่อน้ำใกล้บ้าน ไม่งั้นทรมานกว่าขาดอาหาร
            if !has_water { n.mood -= 15; n.soc.grudge = (n.soc.grudge + 2).min(100); thirsty_n += 1; }
            // รสนิยมการกิน: ของโปรด +4 · ของเกลียด −4 · อาหารปรุงสุก +2
            if let Some(f) = eaten {
                n.mood += if f == n.soc.love as usize { 4 } else if f == n.soc.hate as usize { -4 } else if (DISH_STEW..=DISH_FISH).contains(&f) { 2 } else { 0 };
            }
            // คนในปกครองโทษผู้นำเมื่อลำบาก — ยิ่งใจร้อนยิ่งแค้นเร็ว
            if !bed_ok { n.soc.grudge = (n.soc.grudge + 3 + n.soc.tr[T_TEMPER] as i32 / 30).min(100); }
            if !fed { n.soc.grudge = (n.soc.grudge + 2 + n.soc.tr[T_TEMPER] as i32 / 40).min(100); n.soc.anger = (n.soc.anger + 8).min(100); }
            if n.married { n.mood += 4; }
            if town_well { n.mood += 1; }
            if n.hearts >= 5 { n.mood += 2; }
            n.mood = n.mood.clamp(0, 100);
            n.talked = false;
            n.gifted = false;
            if fed { fed_n += 1 } else { hungry_n += 1 }
        }
        for n in self.npcs.iter_mut().filter(|n| n.state != ST_HOME) { n.talked = false; n.gifted = false; }
        if roofless_n > 0 { self.news_add(format!("ชาวบ้าน {} คนนอนกลางแจ้ง — ล้อมเตียงด้วยกำแพง+ประตูให้เป็นบ้าน เขาจะมีความสุขขึ้น", roofless_n)); }
        if thirsty_n > 0 { self.news_add(format!("ชาวบ้าน {} คนไม่มีน้ำดื่มใกล้บ้าน — สร้างบ่อน้ำหรือย้ายเตียงไปใกล้แหล่งน้ำ", thirsty_n)); }
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
        let news_n = self.social_dawn();
        self.quest_reset_daily();
        self.quest_dawn();
        self.assign_roles();
        // ชาวไร่: รดน้ำ เก็บเกี่ยว ปลูกซ้ำ
        let farmers: Vec<(i32, i32)> = self.npcs.iter().filter(|n| n.state == ST_HOME && n.role == 1 && n.mood >= 20 && Game::willing(n) && !self.rs.worked.contains(&n.id)).map(|n| n.bed).collect();
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
                    self.npcs.push(Npc { id, name: (id as usize * 5 % NPC_NAMES.len()) as u8, look: (id % LOOKS as u32) as u8, x: sx + 4., y: sy, state: ST_HOME, hearts: 10, mood: 90, married: false, child: true, role: 0, bed: b, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id as usize * 5 % NPC_NAMES.len()) as u8) });
                    self.last_child = day;
                    self.toast(format!("ครอบครัวของคุณมีสมาชิกใหม่: {}!", NPC_NAMES[self.npcs.last().unwrap().name as usize]));
                    play(&self.sfx.lvl);
                }
            }
        }
        if !ids.is_empty() {
            let mut s = format!("รุ่งเช้า: ชาวบ้าน {} คน อิ่ม {} หิว {}", ids.len(), fed_n, hungry_n);
            if harvested > 0 { s += &format!(" · ชาวไร่เก็บผลผลิต {}", harvested); }
            if news_n > 0 { s += &format!(" · มีข่าวในหมู่บ้าน {} เรื่อง", news_n); }
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
            self.realm_dawn();
            self.projects_dawn();
            self.season_dawn();
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
        let hr_now = self.hour();
        let (px, py) = (self.p.x, self.p.y);
        let enemies: Vec<(usize, f32, f32)> = self.en.iter().enumerate().filter(|(_, e)| !e.dead).map(|(i, e)| (i, e.x, e.y)).collect();
        let mut hits: Vec<(usize, f32, f32)> = vec![];
        let gather = std::mem::take(&mut self.gather);
        let mut npcs = std::mem::take(&mut self.npcs);
        let mut paths = std::mem::take(&mut self.rs.npaths);
        paths.retain(|id, _| npcs.iter().any(|n| n.id == *id));
        let mut budget = PATH_BUDGET;
        let mut work_done: Vec<(u32, Job)> = vec![];
        for n in npcs.iter_mut() {
            n.anim += dt;
            n.atk_cd = (n.atk_cd - dt).max(0.);
            n.wt -= dt;
            let (mut tx, mut ty, mut spd) = (n.x, n.y, 0.);
            let home = (n.bed.0 as f32 * 16. + 8., n.bed.1 as f32 * 16. + 18.);
            // v19: ดึกแล้วขึ้นนอนบนเตียงจริง (มอนเข้าใกล้ = ตื่น) · เช้าลุกมายืนข้างเตียง
            if n.state == ST_HOME {
                let bed_ok = self.w.placed.get(&n.bed) == Some(&33);
                let danger = enemies.iter().any(|e| (e.1 - n.x).hypot(e.2 - n.y) < 70.);
                if sleep_hr(hr_now) && bed_ok && !gather.contains_key(&n.id) && !danger {
                    let lie = Game::bed_lie(n.bed);
                    if self.rs.sleeping.contains(&n.id) || (n.x - home.0).hypot(n.y - home.1) < 10. || (n.x - lie.0).hypot(n.y - lie.1) < 12. {
                        self.rs.sleeping.insert(n.id);
                        n.x = lie.0; n.y = lie.1; n.moving = false;
                        continue;
                    }
                } else if self.rs.sleeping.remove(&n.id) {
                    n.x = home.0; n.y = home.1;
                }
            }
            match n.state {
                ST_FOLLOW => {
                    let d = (px - n.x).hypot(py - n.y);
                    if d > 240. { n.x = px - 12.; n.y = py; }
                    if d > 26. { tx = px; ty = py; spd = 70.; }
                }
                _ => {
                    if matches!(n.role, 2 | R_SOLDIER | R_GENERAL) && n.mood >= 20 && Game::willing(n) {
                        // ทหารยาม: เข้าหามอนสเตอร์ใกล้บ้าน ฟันให้
                        let tgt = enemies.iter().filter(|e| (e.1 - home.0).hypot(e.2 - home.1) < 170.).min_by(|a, b| ((a.1 - n.x).hypot(a.2 - n.y)).partial_cmp(&((b.1 - n.x).hypot(b.2 - n.y))).unwrap());
                        if let Some(&(ei, ex, ey)) = tgt {
                            let d = (ex - n.x).hypot(ey - n.y);
                            if d > 14. { tx = ex; ty = ey; spd = 78.; } else if n.atk_cd <= 0. { n.atk_cd = 0.9; hits.push((ei, (ex - n.x) / d.max(1.), (ey - n.y) / d.max(1.))); }
                        }
                    }
                    // v20: เวลางาน → ไปทำงานตามอาชีพ (เห็นผลจริงในโลก)
                    let mut busy = false;
                    if spd == 0. && !night && !gather.contains_key(&n.id) {
                        if let Some((p, s)) = self.work_step(n, dt, &mut work_done) { tx = p.0; ty = p.1; spd = s; busy = true; }
                    }
                    if spd == 0. && !busy {
                        if let Some(&(seat, fire)) = gather.get(&n.id) {
                            // ตอนเย็น: ไปนั่งล้อมกองไฟ หันหน้าเข้าหาไฟ
                            tx = seat.0; ty = seat.1;
                            if (n.x - tx).hypot(n.y - ty) > 3. { spd = 40.; } else { n.x = tx; n.y = ty; n.left = fire.0 < n.x; }
                        } else if night { tx = home.0; ty = home.1; if (n.x - tx).hypot(n.y - ty) > 6. { spd = 40.; } }
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
                // หาทาง: อ้อมกำแพง/ต้นไม้/บ้าน (เข้าบ้านทางประตู) แทนเดินชนตรงๆ
                let pth = paths.entry(n.id).or_default();
                let (gx, gy) = pth.steer(&self.w, n.x, n.y, (tx, ty), Pass::Door, dt, &mut budget);
                let d = (gx - n.x).hypot(gy - n.y).max(0.01);
                let st = (spd * dt).min(d);
                let (vx, vy) = ((gx - n.x) / d * st, (gy - n.y) / d * st);
                let (nx, ny) = (n.x + vx, n.y + vy);
                // ชาวบ้านเดินผ่านประตูได้เอง (ไม่ต้องเปิดค้างไว้)
                // ช่องที่ยืนอยู่ตอนนี้ไม่นับ (ยืนข้างเตียงจุดเท้าอยู่ในช่องเตียง — เดิมทำให้เดินออกไม่ได้)
                let here = (tfloor(n.x), tfloor(n.y - 3.));
                let blk = |x: f32, y: f32| { let (a, b) = (tfloor(x), tfloor(y - 3.)); (a, b) != here && self.w.obj_at(a, b) != 12 && self.w.solid_at(a, b) };
                if !blk(nx, n.y) { n.x = nx; n.moving = true; }
                if !blk(n.x, ny) { n.y = ny; n.moving = true; }
                // กลางคืนติดกำแพงกลับบ้านไม่ได้ → เดินอ้อมเข้าบ้านไปเอง (วาร์ปไปที่เตียง)
                // ไปไม่ถึงจริงๆ (ไม่มีทาง) นานเกินไป → ไปถึงเอง (เหมือนอ้อมไกล)
                if (night || gather.contains_key(&n.id)) && !n.moving && n.state == ST_HOME && (n.x - tx).hypot(n.y - ty) > 10. && n.wt < -8. { n.x = tx; n.y = ty; n.wt = 0.; }
                if vx.abs() > 0.01 { n.left = vx < 0.; }
            }
        }
        self.npcs = npcs;
        self.rs.npaths = paths;
        if !work_done.is_empty() { self.work_finish(work_done); }
        self.gather = gather;
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
    let body0 = g.npc_body(n);
    // v19: นอนบนเตียง — หัวบนหมอน ห่มผ้า หายใจขึ้นลง มี z ลอย
    if g.npc_sleeping(n) {
        let (bx, by) = (n.bed.0 as f32 * 16. - v.cx, n.bed.1 as f32 * 16. - v.cy);
        let s2 = v.s * 0.8;
        skin::draw_figure_rot(sp, &body0, role_parts(n.role), (bx + 3.7) * v.s, (by - 0.9) * v.s, s2, -std::f32::consts::FRAC_PI_2, WHITE);
        let r = sp.obj[33];
        let br = ((t_now * 1.3 + n.id as f32).sin() * 0.5).round();
        draw_texture_ex(&sp.tex, (bx + 7.) * v.s, (by + 2. - br.max(0.)) * v.s, WHITE, DrawTextureParams { source: Some(Rect::new(r.x + 7., r.y + 2., 8., 7.)), dest_size: Some(vec2(8. * v.s, 7. * v.s)), ..Default::default() });
        for k in 0..2 {
            let ph = (t_now * 0.45 + k as f32 * 0.5 + n.id as f32 * 0.37).fract();
            let (zx, zy) = (bx + 6. + ph * 6., by - 2. - ph * 10.);
            let a = (1. - ph) * 0.9;
            let zc = Color::new(0.85, 0.9, 1., a);
            // ตัว z 4x4: แถวบน · เฉียง 2 จุด · แถวล่าง
            let (zx, zy) = (zx.round(), zy.round());
            prect(v, zx, zy, 4., 1., zc); prect(v, zx + 2., zy + 1., 1., 1., zc); prect(v, zx + 1., zy + 2., 1., 1., zc); prect(v, zx, zy + 3., 4., 1., zc);
        }
        return;
    }
    // v19: นั่งเรือตามเรา
    if n.state == ST_FOLLOW && g.rides_boat(n.x, n.y) {
        // หันเรือตามทิศที่เราพาย
        let side = g.p.dir == 2;
        draw_boat(v, sx, sy, t_now + n.id as f32, side);
        let d = if side { 2 } else { g.p.dir };
        skin::draw_figure_ex(sp, &body0, role_parts(n.role), (sx - 6.) * v.s, (sy - 17.) * v.s, v.s, d, 0, if side { g.p.flip } else { n.left }, WHITE, 0.4);
        return;
    }
    let (f, wb) = if n.moving { vfx::walk_cycle(n.anim, 8.) } else { (0, 0.) };
    let (sit, laugh) = g.psy.get(&n.id).map_or((false, 0.), |p| (p.sitting, p.laugh));
    // หัวเราะ = ตัวสั่นขึ้นลงเร็วๆ
    let bob = if n.moving { wb } else if laugh > 0. { ((t_now * 22.).sin().abs() * 1.5).round() } else { 0. };
    let body = g.npc_body(n);
    let (bw, k) = body.scale();
    vfx::char_shadow(v, sx, sy, 8. * bw);
    if sit {
        // นั่ง: หันข้างเข้าหากองไฟ
        skin::draw_figure_ex(sp, &body, role_parts(n.role), (sx - 6.) * v.s, (sy - 15. - bob) * v.s, v.s, 2, 0, n.left, WHITE, 0.4);
    } else {
        skin::draw_figure(sp, &body, role_parts(n.role), (sx - 6.) * v.s, (sy - 15. - bob) * v.s, v.s, 0, f, n.left, WHITE);
    }
    work::draw_work(g, sp, v, n, sx, sy - bob, t_now);
    if n.married {
        prect(v, sx - 2., sy - 20. * k - bob, 2., 1., hx(0xff6688));
        prect(v, sx + 1., sy - 20. * k - bob, 2., 1., hx(0xff6688));
        prect(v, sx - 1., sy - 19. * k - bob, 3., 1., hx(0xff6688));
    }
    if n.soc.anger >= 40 {
        // สัญลักษณ์โกรธ: กากบาทแดงเต้น ๆ
        let p = ((t_now * 8.).sin() * 0.8).round();
        let (cx, cy) = (sx + 4., sy - 19. * k - bob + p);
        prect(v, cx - 1., cy - 3., 2., 7., hx(0xff3030));
        prect(v, cx - 3., cy - 1., 6., 2., hx(0xff3030));
    } else if n.state == ST_HOME && n.mood < 25 {
        text_c_world(v, "!", sx, sy - 19. * k - bob, hx(0xff4444), t_now);
    } else if !g.reqs_for(n.id).is_empty() || g.rs.help.contains_key(&n.id) {
        // มีคำขอ/เรื่องให้จัดการ: กรอบคำพูดสีเหลือง
        let b = ((t_now * 4.).sin() * 1.).round();
        let (cx, cy) = (sx + 5., sy - 22. * k - bob + b);
        prect(v, cx - 3., cy - 4., 7., 7., hx(0xfff4b0));
        prect(v, cx - 2., cy + 3., 2., 2., hx(0xfff4b0));
        prect(v, cx, cy - 3., 1., 3., hx(0x6a4a10));
        prect(v, cx, cy + 1., 1., 1., hx(0x6a4a10));
    }
}
pub fn draw_lost(g: &Game, sp: &Sprites, v: &View, x: f32, y: f32, name: u8, look: u8, t_now: f32) {
    let (sx, sy) = ((x - v.cx).round(), (y - v.cy).round());
    vfx::char_shadow(v, sx, sy, 8.);
    let body = Body::generate(look as u32, NPC_FEM[name as usize % NPC_FEM.len()]);
    skin::draw_figure(sp, &body, skin::ROLE_PARTS[0], (sx - 6.) * v.s, (sy - 15.) * v.s, v.s, 0, 0, false, WHITE);
    let k = body.scale().1;
    let b = ((t_now * 5.).sin() * 1.5).round();
    prect(v, sx - 1., sy - 25. * k.max(0.9) + b, 2., 5., hx(0xffee44));
    prect(v, sx - 1., sy - 18. * k.max(0.9) + b, 2., 2., hx(0xffee44));
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
        s += &self.soc_save_str();
        s += &self.psy_save();
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
            "npc" => self.npcs.push(Npc { id: f(0) as u32, name: (f(1) as u8) % NPC_NAMES.len() as u8, look: (f(2) as u8) % LOOKS as u8, x: f(3), y: f(4), state: (f(5) as u8).min(2), hearts: (f(6) as u8).min(10), mood: f(7) as i32, married: f(8) > 0., child: f(9) > 0., role: (f(10) as u8).min(3), bed: (f(11) as i32, f(12) as i32), anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(f(0) as u32, (f(1) as u8) % NPC_NAMES.len() as u8) }),
            "npcseen" => for t in rest { if let Some((a, b)) = t.split_once(',') { if let (Ok(a), Ok(b)) = (a.parse(), b.parse()) { self.npc_seen.insert((a, b)); } } },
            _ => {}
        }
    }
}
