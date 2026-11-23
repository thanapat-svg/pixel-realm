//! v33 "บริวาร" — โกเลมเฝ้าเมือง 3 แบบ · บอสที่ยอมมาเป็นพวก · จัดการสัตว์เลี้ยง/บริวาร
//!
//! โกเลม (หิน/เหล็ก/คริสตัล) = สิ่งประดิษฐ์ ซื่อสัตย์ 100% ไม่กินอาหาร ไม่ทรยศ — มีสกิลของตัวเอง แต่ไม่ลงไปสู้บอสตัวจริง
//! บอสที่ยอมเป็นพวก = มีใจ (บุคลิก/ความภักดี/อารมณ์) — ภักดีขึ้นหรือลดลงตามสภาพเมืองและการดูแล
//! สัตว์เลี้ยง = ต้องให้อาหาร/ลูบ ไม่งั้นหิวจนอ่อนแอ ไม่ช่วยสู้ และหนีกลับป่าในที่สุด
use super::*;

pub const CREW_MAX: usize = 4;
pub const K_WOODG: u8 = 0;
pub const K_STONEG: u8 = 1;
pub const K_IRONG: u8 = 2;
pub const K_CRYSTG: u8 = 3;
pub const K_SLIME: u8 = 4;
pub const K_BGOLEM: u8 = 5;
pub const K_CHAMP: u8 = 6;
pub const CK_N: [&str; 7] = ["หุ่นไม้ยาม", "โกเลมหิน", "โกเลมเหล็ก", "โกเลมคริสตัล", "ราชาสไลม์", "โกเลมหินยักษ์", "แม่ทัพ"];
const MNAMES: [[&str; 3]; 2] = [["วุ้นเส้น", "เจลลี่", "ปุ๊กปิ๊ก"], ["ศิลา", "ก้อนหิน", "บูรพา"]];
pub const M_FOLLOW: u8 = 0;
pub const M_GUARD: u8 = 1;
pub const M_ROAM: u8 = 2;
pub const GUARD_R: f32 = 128.;
pub const TAUNT_R: f32 = 130.;
/// เกณฑ์ตัดสินใจ: ต่ำกว่า NO = ปฏิเสธ · NO..YES = ขอคิดดู · ตั้งแต่ YES = ยินดีเป็นพวก
pub const V_NO: i32 = 42;
pub const V_YES: i32 = 62;

#[derive(Clone, Debug)]
pub struct Member {
    pub id: u32,
    pub kind: u8,
    pub nm: u8,
    pub x: f32,
    pub y: f32,
    pub post: (i32, i32),
    pub mode: u8,
    pub loyalty: i32,
    pub mood: i32,
    pub t: [i32; 6],
    /// นิสัยบอส (เฟส 2): 0 มีเกียรติ · 1 ทะเยอทะยาน · 2 หยิ่งทระนง
    pub arch: u8,
    pub fed_day: i32,
    pub sad: i32,
    pub hp: f32,
    pub down: bool,
    // ไม่เซฟ
    pub cd: f32,
    pub skill_cd: f32,
    pub hurt_cd: f32,
    pub taunt_t: f32,
    pub slam_t: f32,
    pub flash: f32,
    pub last_hit: f32,
    pub beam: (f32, f32, f32),
    pub wander: (f32, f32, f32),
    pub anim: f32,
    /// ติดสิ่งกีดขวางมานานแค่ไหน (วินาที) — เกิน 1.5 วิ วาร์ปกลับจุดเฝ้า/ข้างผู้เล่น
    pub stuck: f32,
    pub left: bool,
}
impl Member {
    pub fn new(id: u32, kind: u8, x: f32, y: f32) -> Member {
        let mut m = Member { id, kind, nm: (id % 3) as u8, x, y, post: (tfloor(x), tfloor(y)), mode: M_GUARD, loyalty: 100, mood: 100, t: [50; 6], arch: 0, fed_day: 0, sad: 0, hp: 1., down: false, cd: 0., skill_cd: 0., hurt_cd: 0., taunt_t: 0., slam_t: 0., flash: 0., last_hit: 99., beam: (0., 0., 0.), wander: (x, y, 0.), anim: 0., stuck: 0., left: false };
        m.hp = m.maxhp();
        m
    }
    /// มีใจ (บอสที่ยอมมา) ไม่ใช่หุ่น/โกเลมที่คราฟต์
    pub fn mind(&self) -> bool { self.kind >= K_SLIME }
    pub fn golem(&self) -> bool { (K_STONEG..=K_CRYSTG).contains(&self.kind) }
    pub fn maxhp(&self) -> f32 { match self.kind { K_WOODG => 30., K_STONEG => 120., K_IRONG => 90., K_CRYSTG => 50., K_SLIME => 80., K_BGOLEM => 160., _ => 200. } }
    /// ลดดาเมจที่รับ (ถึก = รับน้อย)
    fn armor(&self) -> f32 { match self.kind { K_STONEG => 0.5, K_IRONG => 0.6, K_BGOLEM => 0.7, _ => 1. } }
    pub fn name(&self) -> String {
        if self.mind() && self.kind < K_CHAMP { format!("{} ({})", MNAMES[(self.kind - K_SLIME) as usize][self.nm as usize % 3], CK_N[self.kind as usize]) } else { CK_N[self.kind as usize].to_string() }
    }
    /// บทบาทที่ตัวนี้อยากทำ (ตามบุคลิก)
    pub fn wished(&self) -> u8 {
        if !self.mind() { return M_GUARD; }
        let stay = self.t[PC] * 5 / 10 + (100 - self.t[PO]) * 3 / 10 + (100 - self.t[PE]) * 2 / 10;
        if stay >= 55 { M_GUARD } else { M_FOLLOW }
    }
    /// กำลังโจมตี: ตัวที่มีใจภักดีมากทำได้เต็ม ใจร้าวแล้วอ่อนลง
    pub fn power(&self) -> f32 { if self.mind() { 0.6 + 0.4 * self.loyalty as f32 / 100. } else { 1. } }
    fn dmg(&self, lvl: i32) -> i32 {
        let b = match self.kind { K_WOODG => 2, K_STONEG => 4 + lvl / 6, K_IRONG => 7 + lvl / 4, K_CRYSTG => 3 + lvl / 5, K_SLIME => 3 + lvl / 4, K_BGOLEM => 5 + lvl / 4, _ => 6 + lvl / 3 };
        ((b as f32 * self.power()).round() as i32).max(1)
    }
    fn cool(&self) -> f32 { match self.kind { K_WOODG => 1.2, K_STONEG => 1.4, K_IRONG => 1.6, K_CRYSTG => 1.5, K_SLIME => 1.0, _ => 1.3 } }
    fn speed(&self) -> f32 { match self.kind { K_WOODG => 56., K_STONEG => 44., K_IRONG => 48., K_CRYSTG => 52., K_SLIME => 64., _ => 50. } }
    pub fn skill_text(&self) -> &'static str {
        match self.kind {
            K_STONEG => "สกิลยั่วยุ: ดึงศัตรูรอบตัวให้มาตีตัวเอง — ยืนขวางประตูเมือง",
            K_IRONG => "สกิลทุบพื้น: ศัตรูกระเด็นและสตันสั้นๆ — เฝ้าโกดัง/เดินคุ้มกัน",
            K_CRYSTG => "สกิลลำแสงคริสตัล: ยิงไกล ศัตรูเดินช้าลง — วางแนวหลัง/บนกำแพง",
            K_WOODG => "ยามไม้ธรรมดา ตีมอนสเตอร์ที่เข้าใกล้",
            _ => "",
        }
    }
    pub fn fav(&self) -> usize { fav_of(self.kind.saturating_sub(K_SLIME)) }
}
/// ของโปรด (ให้ของขวัญได้ผลเต็ม) · ของรอง (ได้ครึ่งเดียว) — kind = ชนิดบอส 0 สไลม์ · 1 โกเลม
pub fn fav_of(kind: u8) -> usize { if kind == 1 { GEM } else { BERRY } }
fn fav2_of(kind: u8) -> usize { if kind == 1 { STONE } else { SHROOM } }

#[derive(Clone, Debug)]
pub struct Yield {
    /// ชนิดบอส (0 ราชาสไลม์ · 1 โกเลม)
    pub kind: u8,
    pub t: [i32; 6],
    pub trust: i32,
    pub gift: f32,
    pub gifts: i32,
    pub asks: i32,
    pub reply: String,
}
#[derive(Clone, Debug, Default)]
pub struct Crew {
    pub m: Vec<Member>,
    pub next: u32,
    /// ชื่อเสียงกับบอสแต่ละชนิด (ไว้ชีวิต +, ปราบทิ้ง −)
    pub rel: [i32; 2],
    pub yld: Option<Yield>,
    /// ปุ่มปล่อยที่กดครั้งแรกแล้ว รอกดยืนยัน (0 พวก · 1 สัตว์เลี้ยง · 2 ผู้ติดตามจ้าง) + เวลาจริงที่กด
    pub confirm: Option<(u8, u32, f64)>,
}

// ======================= บุคลิก =======================
fn trait_word(i: usize, hi: bool) -> &'static str {
    match (i, hi) {
        (PO, true) => "อยากรู้อยากเห็น", (PO, false) => "ชอบอะไรเดิมๆ",
        (PC, true) => "รักหน้าที่ มีวินัย", (PC, false) => "ไม่ชอบถูกผูกมัด",
        (PE, true) => "ช่างพูด ชอบมีเพื่อน", (PE, false) => "ชอบอยู่ลำพัง",
        (PA, true) => "ใจดี", (PA, false) => "ใจแข็ง ไม่ไว้ใจใครง่ายๆ",
        (PN, true) => "ขี้กลัว อ่อนไหว", (PN, false) => "นิ่งสงบ",
        (_, true) => "ซื่อตรง", (_, false) => "เจ้าเล่ห์ มองแต่ประโยชน์",
    }
}
/// บุคลิกเด่น 2 อย่าง (ที่ห่างจากกลาง 50 มากที่สุด)
pub fn trait_words(t: &[i32; 6]) -> String {
    let mut ix: Vec<usize> = (0..6).collect();
    ix.sort_by_key(|&i| -((t[i] - 50).abs()));
    ix.iter().take(2).map(|&i| trait_word(i, t[i] >= 50)).collect::<Vec<_>>().join(" · ")
}
/// บุคลิกตั้งต้นของบอสชนิดนี้ ± สุ่มเล็กน้อยตามตำแหน่งที่เกิด (ไม่ซ้ำกันทุกตัว)
pub fn boss_traits(seed: i32, kind: u8, x: f32, y: f32) -> [i32; 6] {
    let base: [i32; 6] = if kind == 0 { [72, 35, 70, 62, 45, 55] } else { [22, 85, 20, 38, 12, 72] };
    let (tx, ty) = (tfloor(x), tfloor(y));
    let mut t = base;
    for i in 0..6 { t[i] = (base[i] + ((hash(tx + i as i32 * 7, ty, seed.wrapping_add(900 + kind as i32)) - 0.5) * 30.) as i32).clamp(0, 100); }
    t
}
/// ความไว้ใจตั้งต้น: เป็นมิตร (หลัก) + ซื่อสัตย์ + ไม่อ่อนไหวง่าย + เปิดรับ + ชื่อเสียงที่เคยทำกับพวกเดียวกัน
pub fn base_trust(t: &[i32; 6], rel: i32) -> i32 {
    ((0.35 * t[PA] as f32 + 0.25 * t[PH] as f32 + 0.15 * (100 - t[PN]) as f32 + 0.10 * t[PO] as f32 + 5.).round() as i32 + rel).clamp(0, 100)
}
/// ต้นทุนด้านอิสระ (self-determination): คนรักการอยู่ลำพัง/รักหน้าที่ยิ่งเสียอิสระมาก ยิ่งไม่อยากเข้าพวก
pub fn autonomy_cost(t: &[i32; 6]) -> f32 { (100 - t[PE]) as f32 * 0.15 + t[PC] as f32 * 0.10 }
/// ของขวัญได้ผลมากน้อยตามความเป็นมิตร (คนใจดีตอบแทนง่ายกว่า) — ไม่ใช่ซื้อใจได้ทุกตัว
pub fn gift_effect(t: &[i32; 6]) -> f32 { 0.6 + 0.008 * t[PA] as f32 }
/// คะแนนความเต็มใจ = ไว้ใจ + ของขวัญ − เสียอิสระ − ตื๊อ (ถามซ้ำ = ต่อต้าน)
pub fn yield_score(y: &Yield) -> i32 {
    (y.trust as f32 + y.gift * gift_effect(&y.t) - autonomy_cost(&y.t)).round() as i32 - 8 * (y.asks - 1).max(0)
}
fn greet(t: &[i32; 6]) -> &'static str {
    if t[PN] >= 62 { "อย่าทำร้ายข้าอีกเลย... ข้ายอมแพ้" }
    else if t[PH] >= 62 { "สู้ได้สมศักดิ์ศรี ข้ายอมรับว่าแพ้" }
    else if t[PC] >= 70 { "ข้าเฝ้าถิ่นนี้มานาน... แต่เจ้าชนะแล้ว" }
    else if t[PA] >= 60 { "เจ้าเก่งจริงๆ ข้ายอมแพ้" }
    else { "ข้าแพ้แล้ว... เจ้าจะทำอะไรข้า?" }
}

impl Game {
    fn yield_boss_idx(&self) -> Option<usize> { self.bosses.iter().position(|b| b.yld && !b.dead) }
    /// บอสหมดแรง: ราชาสไลม์/โกเลมยอมแพ้ให้เลือก ชวน·ให้ของ·ปล่อย·ปราบ (true = ยอมแพ้แทนที่จะตาย)
    pub fn boss_yield_try(&mut self, i: usize) -> bool {
        let b = &self.bosses[i];
        if b.kind > 1 || self.rs.crew.yld.is_some() { return false; }
        let (kind, x, y) = (b.kind, b.x, b.y);
        let t = boss_traits(self.w.seed, kind, x, y);
        let trust = base_trust(&t, self.rs.crew.rel[kind as usize]);
        let b = &mut self.bosses[i];
        b.yld = true; b.hp = 1.; b.atk = 0; b.dash = 0.; b.burn = 0.;
        self.proj.clear();
        let reply = greet(&t).to_string();
        self.toast(format!("{} ยอมแพ้แล้ว! เลือกว่าจะทำอย่างไรกับมัน", BNAME[kind as usize]));
        play(&self.sfx.quest);
        self.rs.crew.yld = Some(Yield { kind, t, trust, gift: 0., gifts: 0, asks: 0, reply });
        true
    }
    fn boss_gone(&mut self, i: usize) {
        self.bosses[i].dead = true;
        self.boss_cd = 240.;
        self.rs.crew.yld = None;
        if self.w.dun.is_some() { self.dungeon_boss_down(); }
    }
    /// ชวนเป็นพวก
    pub fn yield_ask(&mut self) {
        let Some(i) = self.yield_boss_idx() else { self.rs.crew.yld = None; return };
        if self.rs.crew.m.len() >= CREW_MAX { self.toast("พวกเต็มแล้ว (4) — ปล่อยตัวเก่าที่แท็บบริวาร (กด N) ก่อน"); return; }
        let (kind, x, y) = (self.bosses[i].kind, self.bosses[i].x, self.bosses[i].y);
        let mut yl = self.rs.crew.yld.clone().unwrap();
        yl.asks += 1;
        let s = yield_score(&yl);
        if s >= V_YES {
            let loy = s.clamp(40, 90);
            let wish = { let mut tmp = Member::new(0, K_SLIME + kind, x, y); tmp.t = yl.t; tmp.wished() };
            let line = if wish == M_GUARD { "ได้... ข้าจะเฝ้าบ้านให้เจ้าเอง" } else { "ได้เลย! ข้าจะไปผจญภัยกับเจ้า!" };
            self.bubble_at(x, y, line);
            self.crew_join(K_SLIME + kind, yl.t, loy, x, y);
            self.p.xp += 40;
            self.check_lv();
            self.burst(x, y - 8., hx(0xff6aa0), 24, 70.);
            play(&self.sfx.lvl);
            self.boss_gone(i);
            self.toast(format!("{} ยอมมาเป็นพวกแล้ว! ดูแลมันที่แท็บบริวาร (กด N)", BNAME[kind as usize]));
            self.save();
            return;
        }
        yl.reply = if s >= V_NO {
            if yl.gifts < 2 { format!("ขอคิดดูก่อน... ถ้ามี{}ให้ข้าสักหน่อย ข้าจะไว้ใจขึ้น", INAME[fav_of(kind)]) } else { "ใจข้ายังไม่แน่ใจ... ให้เวลาข้าสักพักนะ".into() }
        } else if yl.asks >= 3 { "อย่าตื๊อข้าเลย! ยิ่งเจ้าคะยั้นคะยอ ข้ายิ่งไม่อยากไปด้วย".into() }
        else if yl.t[PA] < 42 { "ข้าไม่ไว้ใจเจ้า... ยังไม่พร้อมเป็นพวก".into() }
        else if yl.t[PC] >= 70 { "ข้ามีหน้าที่เฝ้าถิ่นนี้ ไปไหนไม่ได้".into() }
        else if yl.t[PE] < 40 { "ข้าชอบอยู่ลำพังมากกว่า".into() }
        else { "ยังไม่พร้อมจริงๆ... ขอโทษนะ".into() };
        play(&self.sfx.deny);
        self.rs.crew.yld = Some(yl);
    }
    /// ให้ของขวัญ (หลักตอบแทน): ของโปรดได้เต็ม ของรองได้ครึ่ง · ให้ซ้ำค่อยๆ ได้น้อยลง
    pub fn yield_gift(&mut self) {
        let Some(i) = self.yield_boss_idx() else { self.rs.crew.yld = None; return };
        let kind = self.bosses[i].kind;
        let mut yl = self.rs.crew.yld.clone().unwrap();
        let (f1, f2) = (fav_of(kind), fav2_of(kind));
        let (it, base) = if self.p.inv[f1] > 0 { (f1, 14.) } else if self.p.inv[f2] > 0 { (f2, 7.) } else {
            self.toast(format!("ไม่มี{}ให้มัน — {}", INAME[f1], source::item_howto(self, f1).first().map(|s| s.how.clone()).unwrap_or_default()));
            return;
        };
        if yl.gifts >= 3 { yl.reply = "พอแล้ว... ข้าไม่ได้อยากได้ของ ข้าอยากรู้ว่าเจ้าจริงใจไหม".into(); self.rs.crew.yld = Some(yl); return; }
        self.p.inv[it] -= 1;
        yl.gift += base * 0.6f32.powi(yl.gifts);
        yl.gifts += 1;
        yl.reply = format!("ขอบใจ! {} ที่ข้าชอบ", INAME[it]);
        play(&self.sfx.pickup);
        let (x, y) = (self.bosses[i].x, self.bosses[i].y);
        self.burst(x, y - 10., hx(0xffd84a), 8, 40.);
        self.rs.crew.yld = Some(yl);
    }
    /// ปล่อยไป (ไว้ชีวิต): ได้ชื่อเสียงดีกับพวกเดียวกัน + ของขอบคุณเล็กน้อย
    pub fn yield_spare(&mut self) {
        let Some(i) = self.yield_boss_idx() else { self.rs.crew.yld = None; return };
        let (kind, x, y) = (self.bosses[i].kind, self.bosses[i].x, self.bosses[i].y);
        let r = &mut self.rs.crew.rel[kind as usize];
        *r = (*r + 12).min(30);
        if kind == 0 { self.gain(GEL, 1, x, y - 8.); } else { self.gain(STONE, 3, x, y - 8.); }
        self.bubble_at(x, y, "ขอบใจที่ไว้ชีวิต... ข้าจะไม่ลืม");
        self.p.xp += 10;
        self.check_lv();
        self.burst(x, y - 8., hx(0x9ad8ff), 16, 60.);
        self.toast(format!("ไว้ชีวิต{} — ข่าวความใจดีแพร่ไปในหมู่พวกเดียวกัน", BNAME[kind as usize]));
        self.boss_gone(i);
        self.save();
    }
    /// ปราบทิ้ง (ได้ของหายากตามเดิม แต่ชื่อเสียงกับพวกเดียวกันเสีย)
    pub fn yield_slay(&mut self) {
        let Some(i) = self.yield_boss_idx() else { self.rs.crew.yld = None; return };
        let kind = self.bosses[i].kind;
        let r = &mut self.rs.crew.rel[kind as usize];
        *r = (*r - 6).max(-20);
        self.rs.crew.yld = None;
        self.bosses[i].yld = false;
        self.bosses[i].dead = true;
        let b = self.bosses[i].clone();
        self.boss_die(&b);
    }
    /// ข้อความ bubble ลอยเหนือหัว (ใช้ float)
    fn bubble_at(&mut self, x: f32, y: f32, s: &str) { self.add_float_sz(x, y - 34., s.to_string(), hx(0xffe9a0), 1.0); }

    // ======================= พวก / บริวาร =======================
    pub fn crew_join(&mut self, kind: u8, t: [i32; 6], loyalty: i32, x: f32, y: f32) {
        let id = self.rs.crew.next; self.rs.crew.next += 1;
        let mut m = Member::new(id, kind, x, y);
        m.t = t; m.loyalty = loyalty; m.mood = 70; m.fed_day = day_of(self);
        // ให้เลือกบทบาทตามที่ตัวเองอยาก (ความต้องการอิสระ)
        m.mode = m.wished();
        if m.mode == M_GUARD { m.post = (tfloor(self.p.x), tfloor(self.p.y)); }
        self.rs.crew.m.push(m);
    }
    /// คราฟต์หุ่น/โกเลมเฝ้า (เรียกหลังจ่ายของแล้ว)
    pub fn crew_craft(&mut self, kind: u8) {
        let (x, y) = (self.p.x, self.p.y + 8.);
        let id = self.rs.crew.next; self.rs.crew.next += 1;
        let mut m = Member::new(id, kind, x, y);
        m.fed_day = day_of(self);
        self.rs.crew.m.push(m);
        self.toast("วางแล้ว — ยืนตรงจุดไหนก็เฝ้าตรงนั้น · เปลี่ยนหน้าที่/ย้ายจุดได้ที่แท็บบริวาร");
    }
    pub fn crew_release(&mut self, id: u32) {
        let Some(i) = self.rs.crew.m.iter().position(|m| m.id == id) else { return };
        let m = self.rs.crew.m.remove(i);
        self.rs.crew.confirm = None;
        let (x, y) = (m.x, m.y);
        if m.mind() {
            if m.loyalty >= 70 { self.gain(GEM, 2, x, y - 8.); self.toast(format!("{} ขอบคุณที่ดูแลกันมา ทิ้งอัญมณีไว้เป็นที่ระลึก แล้วลาไปอยู่ป่า", m.name())); }
            else { self.toast(format!("{} ลาไปอยู่ป่าแล้ว", m.name())); }
            self.burst(x, y - 8., hx(0x9ad8ff), 14, 50.);
        } else {
            // รื้อถอนหุ่น/โกเลม: คืนวัสดุครึ่งหนึ่ง
            let cost: &[(usize, i32)] = RECIPES.iter().chain(ADV_RECIPES.iter()).find(|r| matches!(r.eff, Eff::Guard(k) if k == m.kind)).map(|r| r.cost).unwrap_or(&[]);
            let mut got = vec![];
            for &(it, n) in cost { let b = n / 2; if b > 0 { self.p.inv[it] += b; got.push(format!("{} {}", INAME[it], b)); } }
            self.toast(format!("รื้อ{}แล้ว คืนวัสดุ {}", m.name(), got.join(" · ")));
            self.burst(x, y - 8., hx(0xc8c8d0), 12, 50.);
        }
        self.save();
    }
    pub fn crew_set_mode(&mut self, id: u32, mode: u8) {
        let (px, py) = (tfloor(self.p.x), tfloor(self.p.y));
        let Some(m) = self.rs.crew.m.iter_mut().find(|m| m.id == id) else { return };
        m.mode = mode;
        if mode != M_FOLLOW { m.post = (px, py); m.wander = (m.x, m.y, 0.); }
        let (name, against) = (m.name(), m.mind() && m.wished() != mode);
        if against { m.mood = (m.mood - 4).max(0); }
        self.toast(match mode { M_GUARD => format!("{} จะเฝ้าตรงจุดนี้", name), M_ROAM => format!("{} จะพัก/เดินเล่นแถวนี้ (ฟื้นเลือด)", name), _ => format!("{} จะเดินตามคุณ", name) });
        self.save();
    }
    /// ให้ของขวัญพวกที่มีใจ: ของโปรดเพิ่มทั้งใจและความภักดี
    pub fn crew_gift_member(&mut self, id: u32) {
        let day = day_of(self);
        let Some(i) = self.rs.crew.m.iter().position(|m| m.id == id) else { return };
        if !self.rs.crew.m[i].mind() { self.toast("โกเลม/หุ่นไม่มีความรู้สึก ไม่ต้องให้ของขวัญ — ซื่อสัตย์ 100% และไม่กินอาหาร"); return; }
        let kind = self.rs.crew.m[i].kind.saturating_sub(K_SLIME).min(1);
        let (f1, f2) = (fav_of(kind), fav2_of(kind));
        let (it, b) = if self.p.inv[f1] > 0 { (f1, 1.) } else if self.p.inv[f2] > 0 { (f2, 0.5) } else {
            self.toast(format!("ไม่มี{} — {}", INAME[f1], source::item_howto(self, f1).first().map(|s| s.how.clone()).unwrap_or_default()));
            return;
        };
        if self.rs.crew.m[i].fed_day == day && self.rs.crew.m[i].mood >= 95 { self.toast("วันนี้อิ่มใจแล้ว พรุ่งนี้ค่อยให้นะ"); return; }
        self.p.inv[it] -= 1;
        let m = &mut self.rs.crew.m[i];
        m.mood = (m.mood + (20. * b) as i32).min(100);
        m.loyalty = (m.loyalty + (6. * b) as i32).min(100);
        m.fed_day = day;
        let (x, y, n) = (m.x, m.y, m.name());
        self.burst(x, y - 10., hx(0xff6aa0), 8, 40.);
        self.toast(format!("{} ดีใจที่ได้{}", n, INAME[it]));
        play(&self.sfx.pickup);
    }
    /// ป้องกันเมือง: ยามที่ประจำจุดช่วยเพิ่มค่าป้องกัน (ตัวที่พังอยู่ไม่นับ)
    pub fn crew_defense(&self) -> f32 {
        self.rs.crew.m.iter().filter(|m| m.mode == M_GUARD && !m.down).map(|m| m.power() * match m.kind { K_WOODG => 2., K_STONEG => 8., K_IRONG => 7., K_CRYSTG => 6., K_SLIME => 5., K_BGOLEM => 9., _ => 12. }).sum()
    }
    /// โซนยั่วยุที่กำลังทำงาน (ใช้ใน AI ศัตรู): (ลำดับ, x, y)
    pub fn crew_taunts(&self) -> Vec<(usize, f32, f32)> {
        self.rs.crew.m.iter().enumerate().filter(|(_, m)| m.kind == K_STONEG && m.taunt_t > 0. && !m.down).map(|(i, m)| (i, m.x, m.y)).collect()
    }

    // ======================= สัตว์เลี้ยง =======================
    /// ลำดับ (ในรายการสัตว์) ของสัตว์เลี้ยงตัวที่ idx
    fn pet_ix(&self, idx: usize) -> Option<usize> { self.animals.iter().enumerate().filter(|(_, a)| a.tame).nth(idx).map(|(i, _)| i) }
    /// ปล่อยสัตว์เลี้ยงกลับสู่ธรรมชาติ (idx = ลำดับในรายการสัตว์เลี้ยง)
    pub fn pet_release(&mut self, idx: usize) {
        let Some(i) = self.pet_ix(idx) else { return };
        let k = self.animals[i].k;
        let a = &mut self.animals[i];
        a.tame = false; a.st = AS::Flee; a.fed = 150.; a.t = 4.;
        let (x, y) = (a.x, a.y);
        self.burst(x, y - 8., hx(0xaaddff), 10, 40.);
        self.rs.crew.confirm = None;
        self.toast(format!("ปล่อย{}กลับสู่ธรรมชาติแล้ว — เลี้ยงตัวใหม่ได้เลย", AN_NAME[k as usize]));
        self.save();
    }
    pub fn pet_food(k: u8) -> [usize; 2] { if k == A_WOLF { [MEAT, DISH_STEW] } else { [CARROT, BERRY] } }
    /// ให้อาหารสัตว์เลี้ยง: อิ่มขึ้น ภักดีขึ้นเล็กน้อย
    pub fn pet_feed(&mut self, idx: usize) {
        let Some(i) = self.pet_ix(idx) else { return };
        let k = self.animals[i].k;
        if self.animals[i].sat >= 90. { self.toast(format!("{}อิ่มแล้ว", AN_NAME[k as usize])); return; }
        let Some(&f) = Game::pet_food(k).iter().find(|&&f| self.p.inv[f] > 0) else {
            let f = Game::pet_food(k)[0];
            self.toast(format!("ไม่มี{} — {}", INAME[f], source::item_howto(self, f).first().map(|s| s.how.clone()).unwrap_or_default()));
            return;
        };
        self.p.inv[f] -= 1;
        let a = &mut self.animals[i];
        a.sat = (a.sat + 45.).min(100.);
        a.loy = (a.loy + 4.).min(100.);
        let (x, y) = (a.x, a.y);
        self.burst(x, y - 8., hx(0xff6aa0), 6, 30.);
        self.toast(format!("{}กิน{}แล้ว อิ่มและไว้ใจขึ้น", AN_NAME[k as usize], INAME[f]));
        play(&self.sfx.eat);
    }
    /// ลูบสัตว์เลี้ยง (ฟรี แต่ทำได้ทุก ~25 วินาที)
    pub fn pet_pat(&mut self, idx: usize) {
        let Some(i) = self.pet_ix(idx) else { return };
        let k = self.animals[i].k;
        if self.animals[i].pat > 0. { self.toast(format!("เพิ่งลูบ{}ไป — รอสักครู่", AN_NAME[k as usize])); return; }
        let a = &mut self.animals[i];
        a.pat = 25.;
        a.loy = (a.loy + 3.).min(100.);
        let (x, y) = (a.x, a.y);
        self.burst(x, y - 10., hx(0xff6aa0), 8, 30.);
        self.toast(format!("ลูบ{} — มันดีใจ ไว้ใจขึ้น", AN_NAME[k as usize]));
    }
    pub fn pet_set_role(&mut self, idx: usize, role: u8) {
        let (px, py) = (self.p.x, self.p.y);
        let Some(i) = self.pet_ix(idx) else { return };
        let k = self.animals[i].k;
        let a = &mut self.animals[i];
        a.role = role.min(2);
        if role != PET_FOLLOW { a.px = px; a.py = py; a.wx = px; a.wy = py; }
        self.toast(match role { PET_GUARD => format!("{}จะเฝ้าตรงจุดนี้ (หมาป่ากัดมอนสเตอร์ที่เข้าใกล้)", AN_NAME[k as usize]), PET_ROAM => format!("{}จะพัก/เดินเล่นแถวนี้", AN_NAME[k as usize]), _ => format!("{}จะเดินตามคุณ", AN_NAME[k as usize]) });
        self.save();
    }

    // ======================= เดินเกม =======================
    pub fn crew_tick(&mut self, dt: f32) {
        // การ์ดค้างแต่บอสไม่อยู่แล้ว (ผู้เล่นตาย/เดินหนี) = เท่ากับปล่อยไป
        if self.rs.crew.yld.is_some() && self.yield_boss_idx().is_none() { self.rs.crew.yld = None; }
        if self.rs.crew.m.is_empty() { return; }
        let indun = self.w.dun.is_some();
        let (px, py, lvl) = (self.p.x, self.p.y, self.p.lvl);
        let mut ms = std::mem::take(&mut self.rs.crew.m);
        let mut notes: Vec<String> = vec![];
        for (mi, m) in ms.iter_mut().enumerate() {
            m.anim += dt;
            for t in [&mut m.cd, &mut m.skill_cd, &mut m.hurt_cd, &mut m.taunt_t, &mut m.slam_t, &mut m.flash] { if *t > 0. { *t -= dt; } }
            m.beam.2 = (m.beam.2 - dt).max(0.);
            m.last_hit += dt;
            if m.down { continue; }
            // ถูกมอนสเตอร์ที่ติดตัวตี (โกเลมถึกรับน้อย)
            if m.hurt_cd <= 0. {
                if let Some(e) = self.en.iter().find(|e| !e.dead && e.stun <= 0. && e.kb <= 0. && (e.x - m.x).hypot(e.y - m.y) < 12.) {
                    m.hp -= (e.dmg as f32 * m.armor()).max(0.5);
                    m.hurt_cd = 0.7; m.flash = 0.15; m.last_hit = 0.; m.taunt_t = m.taunt_t.max(0.);
                }
            }
            if m.hp <= 0. {
                m.hp = 0.; m.down = true; m.taunt_t = 0.;
                notes.push(format!("{} ล้มลง! จะซ่อมตัวเอง/ฟื้นตอนเช้า", m.name()));
                continue;
            }
            // ฟื้นเลือดเองเมื่อไม่ถูกตีสักพัก (พัก/เดินเล่นเร็วกว่า 3 เท่า)
            if m.last_hit > 6. && m.hp < m.maxhp() { m.hp = (m.hp + m.maxhp() * 0.02 * dt * if m.mode == M_ROAM { 3. } else { 1. }).min(m.maxhp()); }
            if indun && m.mode != M_FOLLOW { continue; }
            let (cx, cy, reach) = match m.mode {
                M_GUARD => (m.post.0 as f32 * 16. + 8., m.post.1 as f32 * 16. + 8., GUARD_R),
                M_ROAM => (m.post.0 as f32 * 16. + 8., m.post.1 as f32 * 16. + 8., 46.),
                _ => (px, py, 110.),
            };
            if m.mode == M_FOLLOW && (px - m.x).hypot(py - m.y) > 260. { m.x = px - 12. - mi as f32 * 6.; m.y = py + 4.; }
            // เป้า: มอนสเตอร์ในระยะที่ดูแล (ไม่ลงไปสู้บอส)
            let mut tgt: Option<(usize, f32, f32)> = None;
            let mut best = reach;
            for (j, e) in self.en.iter().enumerate() {
                if e.dead { continue; }
                let d = (e.x - cx).hypot(e.y - cy);
                if d < best { best = d; tgt = Some((j, e.x, e.y)); }
            }
            let spd = m.speed();
            let (mut vx, mut vy) = (0f32, 0f32);
            if let Some((j, ex, ey)) = tgt {
                let (dx, dy) = (ex - m.x, ey - m.y);
                let d = dx.hypot(dy).max(0.01);
                let dm = m.dmg(lvl);
                match m.kind {
                    K_STONEG => {
                        // ยั่วยุ: ดึงศัตรูรอบตัวมาตีตัวเอง
                        if m.skill_cd <= 0. && self.en.iter().any(|e| !e.dead && (e.x - m.x).hypot(e.y - m.y) < TAUNT_R) {
                            m.skill_cd = 9.; m.taunt_t = 4.;
                            self.add_float_sz(m.x, m.y - 34., "ยั่วยุ!".to_string(), hx(0xff7a5a), 1.2);
                        }
                    }
                    K_IRONG => {
                        // ทุบพื้น: กระแทกศัตรูรอบตัวให้กระเด็นและสตันสั้นๆ
                        if m.skill_cd <= 0. && d < 32. {
                            m.skill_cd = 5.5; m.slam_t = 0.35;
                            let near: Vec<usize> = self.en.iter().enumerate().filter(|(_, e)| !e.dead && (e.x - m.x).hypot(e.y - m.y) < 42.).map(|(i, _)| i).collect();
                            for k in near {
                                let (ux, uy) = { let e = &self.en[k]; let (a, b) = (e.x - m.x, e.y - m.y); let l = a.hypot(b).max(0.01); (a / l, b / l) };
                                self.damage_enemy(k, (dm as f32 * 1.5).round() as i32, false, ux, uy);
                                if k < self.en.len() && !self.en[k].dead { let e = &mut self.en[k]; e.kb = 0.35; e.vx = ux * 190.; e.vy = uy * 190.; e.stun = e.stun.max(1.0); }
                            }
                            self.burst(m.x, m.y - 4., hx(0xb0b8c8), 14, 90.);
                            self.shake(1.6, 0.16);
                            play_vol(&self.sfx.hit, 0.5);
                        }
                    }
                    _ => {}
                }
                if m.kind == K_CRYSTG {
                    // ยิงลำแสงจากระยะไกล: ถอยถ้าศัตรูประชิด เดินเข้าใกล้ถ้าไกลเกิน
                    if d > 105. { vx = dx / d * spd; vy = dy / d * spd; } else if d < 40. { vx = -dx / d * spd * 1.2; vy = -dy / d * spd * 1.2; }
                    if d <= 115. && m.cd <= 0. {
                        m.cd = m.cool();
                        self.damage_enemy(j, dm, false, dx / d, dy / d);
                        if j < self.en.len() && !self.en[j].dead { self.en[j].slow = self.en[j].slow.max(2.5); }
                        m.beam = (ex, ey, 0.18);
                        self.burst(ex, ey - 6., hx(0x7af0ff), 6, 40.);
                    }
                } else if d > 13. { vx = dx / d * spd * 1.3; vy = dy / d * spd * 1.3; }
                else if m.cd <= 0. {
                    m.cd = m.cool();
                    self.damage_enemy(j, dm, false, dx / d, dy / d);
                }
                if dx.abs() > 1. { m.left = dx < 0.; }
            } else {
                // ว่าง: กลับจุดเฝ้า · เดินเล่นแถวจุดพัก · หรือเดินตามผู้เล่น
                let (tx, ty, slow) = match m.mode {
                    M_GUARD => (cx, cy, 1.),
                    M_ROAM => {
                        m.wander.2 -= dt;
                        if m.wander.2 <= 0. {
                            let a = gen_range(0., std::f32::consts::TAU); let r = gen_range(0., 44.);
                            m.wander = (cx + a.cos() * r, cy + a.sin() * r, gen_range(2., 4.));
                        }
                        (m.wander.0, m.wander.1, 0.5)
                    }
                    _ => (px - 14. - (mi % 3) as f32 * 8., py + 3. + (mi / 3) as f32 * 8., 1.),
                };
                let (dx, dy) = (tx - m.x, ty - m.y);
                let d = dx.hypot(dy);
                if d > if m.mode == M_FOLLOW { 18. } else { 4. } { let s = if d > 80. { spd * 1.4 } else { spd } * slow; vx = dx / d * s; vy = dy / d * s; }
                if dx.abs() > 2. { m.left = dx < 0.; }
            }
            let (ox, oy) = (m.x, m.y);
            let (nx, ny) = (m.x + vx * dt, m.y + vy * dt);
            if !collide(&self.w, nx, m.y) { m.x = nx; }
            if !collide(&self.w, m.x, ny) { m.y = ny; }
            if (vx != 0. || vy != 0.) && (m.x - ox).hypot(m.y - oy) < 0.2 * dt * spd { m.stuck += dt; } else { m.stuck = 0.; }
            if m.stuck > 1.5 {
                m.stuck = 0.;
                if m.mode != M_FOLLOW { m.x = cx; m.y = cy; } else { m.x = px - 12. - mi as f32 * 6.; m.y = py + 4.; }
            }
        }
        let added = std::mem::take(&mut self.rs.crew.m);
        ms.extend(added);
        self.rs.crew.m = ms;
        for n in notes { self.toast(n); }
    }
    /// เช้าวันใหม่: โกเลม/หุ่นซ่อมตัวเอง · ตัวที่มีใจ — ดูแลดีภักดีขึ้น · ถูกปล่อยปละ/ถูกขัดใจ เหงาและเสียใจ แล้วลาจากไปเอง
    pub fn crew_dawn(&mut self) {
        let day = day_of(self);
        let mut leave: Vec<u32> = vec![];
        for m in self.rs.crew.m.iter_mut() {
            if m.down { m.down = false; m.hp = (m.maxhp() * 0.6).max(m.hp); }
            if !m.mind() { m.hp = m.maxhp(); continue; }
            if day - m.fed_day > 3 { m.mood = (m.mood - 10).max(0); } else { m.mood = (m.mood + 3).min(100); }
            if m.mode == M_ROAM { m.mood = (m.mood + 5).min(100); m.loyalty = (m.loyalty + 1).min(100); }
            else if m.mode != m.wished() { m.loyalty = (m.loyalty - 2).max(0); m.mood = (m.mood - 3).max(0); }
            if m.mood >= 60 { m.loyalty = (m.loyalty + 1).min(100); } else if m.mood < 30 { m.loyalty = (m.loyalty - 2).max(0); }
            if m.loyalty < 20 || m.mood < 12 { m.sad += 1; } else { m.sad = 0; }
            if m.sad >= 2 { leave.push(m.id); }
        }
        for id in leave {
            if let Some(i) = self.rs.crew.m.iter().position(|m| m.id == id) {
                let m = self.rs.crew.m.remove(i);
                self.toast(format!("{} เหงาและเสียใจ... ขอลาไปอยู่ป่า (ลองดูแลด้วยของโปรดและให้ทำสิ่งที่มันอยากทำ)", m.name()));
            }
        }
    }

    // ======================= เซฟ/โหลด =======================
    pub fn crew_save(&self) -> String {
        let c = &self.rs.crew;
        let mut s = format!("crewrel {} {} {}\n", c.rel[0], c.rel[1], c.next);
        for m in &c.m {
            s += &format!("crew {} {} {} {:.1} {:.1} {} {} {} {} {} {} {} {} {} {} {} {} {:.0} {} {}\n", m.id, m.kind, m.nm, m.x, m.y, m.post.0, m.post.1, m.mode, m.loyalty, m.mood, m.t[0], m.t[1], m.t[2], m.t[3], m.t[4], m.t[5], m.fed_day * 10 + m.sad.min(9), m.hp, m.down as u8, m.arch);
        }
        s
    }
    pub fn crew_load_rel(&mut self, r: &[&str]) {
        let n = |i: usize| r.get(i).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
        self.rs.crew.rel = [n(0), n(1)];
        self.rs.crew.next = n(2) as u32;
    }
    pub fn crew_load(&mut self, r: &[&str]) {
        let f = |i: usize| r.get(i).and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.);
        let n = |i: usize| f(i) as i32;
        if r.len() < 17 || self.rs.crew.m.len() >= CREW_MAX { return; }
        let kind = (n(1) as u8).min(K_CHAMP);
        let fs = n(16);
        let mut m = Member::new(n(0) as u32, kind, f(3), f(4));
        m.nm = n(2) as u8; m.post = (n(5), n(6)); m.mode = (n(7) as u8).min(2); m.loyalty = n(8); m.mood = n(9);
        m.t = [n(10), n(11), n(12), n(13), n(14), n(15)];
        m.fed_day = fs.div_euclid(10); m.sad = fs.rem_euclid(10);
        if r.len() > 17 { m.hp = f(17).clamp(0., m.maxhp()); }
        if r.len() > 18 { m.down = n(18) != 0; }
        if r.len() > 19 { m.arch = (n(19) as u8).min(2); }
        self.rs.crew.m.push(m);
        let mx = self.rs.crew.m.iter().map(|m| m.id + 1).max().unwrap_or(0);
        if self.rs.crew.next < mx { self.rs.crew.next = mx; }
    }
}

// ======================= การ์ดตัดสินใจ (บอสยอมแพ้) =======================
pub fn yield_rects_for(l: &hud::Layout, u: f32, top_h: f32) -> (Rect, [Rect; 4]) {
    let w = l.col_w.min(620. * u);
    let x = l.col_x + l.col_w / 2. - w / 2.;
    let y = 8. * u + if top_h > 0. { top_h + 6. * u } else { 0. };
    let (gap, bh) = (6. * u, 32. * u);
    if l.col_w >= 460. * u {
        let h = 120. * u;
        let bw = (w - 16. * u - gap * 3.) / 4.;
        let r = |i: usize| Rect::new(x + 8. * u + i as f32 * (bw + gap), y + h - bh - 8. * u, bw, bh);
        (Rect::new(x, y, w, h), [r(0), r(1), r(2), r(3)])
    } else {
        let h = 160. * u;
        let bw = (w - 16. * u - gap) / 2.;
        let bh = 28. * u;
        let r = |i: usize| Rect::new(x + 8. * u + (i % 2) as f32 * (bw + gap), y + h - bh * 2. - gap - 6. * u + (i / 2) as f32 * (bh + gap), bw, bh);
        (Rect::new(x, y, w, h), [r(0), r(1), r(2), r(3)])
    }
}
pub fn yield_rects(g: &Game) -> Option<(Rect, [Rect; 4])> {
    g.rs.crew.yld.as_ref()?;
    let u = ui_scale();
    Some(yield_rects_for(&hud::layout(), u, hud::top_banner_h(g, u)))
}
pub fn yield_tap(g: &mut Game, tp: Vec2) -> bool {
    let Some((r, b)) = yield_rects(g) else { return false };
    if b[0].contains(tp) { g.yield_ask(); }
    else if b[1].contains(tp) { g.yield_gift(); }
    else if b[2].contains(tp) { g.yield_spare(); }
    else if b[3].contains(tp) { g.yield_slay(); }
    else if !r.contains(tp) { return false; }
    true
}
pub fn draw_yield_card(g: &Game, font: &Font) {
    let Some((r, b)) = yield_rects(g) else { return };
    let yl = g.rs.crew.yld.as_ref().unwrap();
    let u = ui_scale();
    let gold = hx(0xff9ad0);
    draw_rectangle(r.x, r.y, r.w, r.h, Color::new(0.07, 0.05, 0.1, 0.94));
    draw_rectangle(r.x, r.y, 5. * u, r.h, gold);
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., Color::new(1., 0.6, 0.85, 0.9));
    let wide = (b[1].y - b[0].y).abs() < 1.;
    let tw = r.w - 24. * u;
    let title = format!("{} ยอมแพ้แล้ว · {}", BNAME[yl.kind as usize], trait_words(&yl.t));
    text(font, &fit_text_c(font, &title, 13. * u, tw), r.x + 12. * u, r.y + 18. * u, 13. * u, gold);
    // แถบความไว้ใจ: เส้นแบ่ง "ขอคิดดู" และ "ยินดี"
    let s = yield_score(yl).clamp(0, 100) as f32;
    let (bx, by, bw, bh) = (r.x + 12. * u, r.y + 26. * u, (r.w * 0.42).min(220. * u), 7. * u);
    draw_rectangle(bx, by, bw, bh, hx(0x2a2233));
    draw_rectangle(bx, by, bw * s / 100., bh, if s >= V_YES as f32 { hx(0x6ad06a) } else if s >= V_NO as f32 { hx(0xe8c84a) } else { hx(0xd05a5a) });
    for th in [V_NO, V_YES] { draw_rectangle(bx + bw * th as f32 / 100. - 1., by - 2. * u, 2., bh + 4. * u, WHITE); }
    text(font, "ความเต็มใจ", bx + bw + 6. * u, by + bh, 10. * u, hx(0xc8b8d8));
    let ws = wrap_th(font, &format!("\"{}\"", yl.reply), 12. * u, tw);
    let maxl = if wide { 2 } else { 3 };
    for (k, l) in ws.iter().take(maxl).enumerate() { text(font, l, r.x + 12. * u, r.y + 52. * u + k as f32 * 14. * u, 12. * u, WHITE); }
    let fav = fav_of(yl.kind);
    let labels = ["ชวนเป็นพวก".to_string(), format!("ให้{}", INAME[fav]), "ปล่อยไป".to_string(), "ปราบทิ้ง".to_string()];
    let cols = [hx(0x2f7a3a), hx(0x7a5a2f), hx(0x2f4f7a), hx(0x7a2f2f)];
    for k in 0..4 {
        let q = b[k];
        let dim = k == 1 && g.p.inv[fav] <= 0 && g.p.inv[fav2_of(yl.kind)] <= 0;
        draw_rectangle(q.x, q.y, q.w, q.h, if dim { hx(0x3a3a44) } else { cols[k] });
        draw_rectangle_lines(q.x, q.y, q.w, q.h, 2., Color::new(1., 1., 1., 0.55));
        text_c(font, &fit_text_c(font, &labels[k], 12. * u, q.w - 6. * u), q.x + q.w / 2., q.y + q.h * 0.66, 12. * u, if dim { hx(0xaaaaaa) } else { WHITE });
    }
}
fn fit_text_c(font: &Font, t: &str, size: f32, maxw: f32) -> String { source::fit_text(font, t, size, maxw) }

// ======================= วาดพวก =======================
pub fn draw_member(g: &Game, sp: &Sprites, v: &View, i: usize) {
    let m = &g.rs.crew.m[i];
    let (sx, sy) = ((m.x - v.cx).round(), (m.y - v.cy).round());
    let bob = if m.down { 0. } else { ((m.anim * 4.) as i32 % 2) as f32 };
    let flash = m.flash > 0.;
    let tint = match m.kind {
        K_WOODG => Color::new(0.85, 0.66, 0.42, 1.),
        K_STONEG => Color::new(0.78, 0.86, 1., 1.),
        K_IRONG => Color::new(0.58, 0.62, 0.74, 1.),
        K_CRYSTG => Color::new(0.55, 1., 1., 1.),
        _ => WHITE,
    };
    let sad = m.mind() && m.mood < 30;
    let tint = if m.down { Color::new(0.45, 0.45, 0.5, 0.85) } else if flash { Color::new(1., 0.6, 0.6, 1.) } else if sad { Color::new(0.75, 0.8, 0.95, 1.) } else { tint };
    // วงยั่วยุ (โกเลมหิน) / คลื่นกระแทก (โกเลมเหล็ก) — วาดบนพื้นก่อนตัว
    if m.taunt_t > 0. {
        let k = (m.taunt_t / 4.).clamp(0., 1.);
        let ph = (1. - (m.anim * 1.5).fract()) * TAUNT_R;
        draw_circle_lines(sx * v.s, (sy - 4.) * v.s, ph * v.s, 2., Color::new(1., 0.35, 0.25, 0.25 + 0.4 * k));
        draw_circle_lines(sx * v.s, (sy - 4.) * v.s, 18. * v.s, 2., Color::new(1., 0.45, 0.3, 0.7));
    }
    if m.slam_t > 0. {
        let k = 1. - m.slam_t / 0.35;
        draw_circle_lines(sx * v.s, (sy - 3.) * v.s, (8. + 36. * k) * v.s, 3., Color::new(0.85, 0.88, 1., 1. - k));
    }
    if m.kind == K_SLIME {
        let sq = (m.anim * 1.6 * std::f32::consts::TAU).sin() * 0.1;
        let (ww, hh) = ((22. * (1. + sq)).round(), (18. * (1. - sq)).round());
        prect(v, (sx - 9.).round(), sy - 2., 18., 3., Color::new(0., 0., 0., 0.28));
        blit_sz(sp, v, sp.ex.king, (sx - ww / 2.).floor(), sy - hh, ww, hh, tint);
    } else if m.kind == K_CHAMP {
        let r = sp.ex.comp[if m.left { 2 } else { 3 }];
        let (w, h) = ((r.w * 1.5).round(), (r.h * 1.5).round());
        prect(v, sx - 8., sy - 2., 16., 3., Color::new(0., 0., 0., 0.28));
        blit_sz(sp, v, r, (sx - w / 2.).floor(), sy - h - bob, w, h, tint);
    } else {
        let r = sp.ex.golem[0];
        let k = match m.kind { K_WOODG => 0.62, K_STONEG => 0.88, K_IRONG => 0.95, K_CRYSTG => 0.74, _ => 0.8 };
        let (w, h) = ((r.w * k).round(), (r.h * k).round());
        prect(v, sx - 9., sy - 2., 18., 3., Color::new(0., 0., 0., 0.28));
        vfx::blit_outline(sp, v, r, (sx - w / 2.).floor(), sy - h - bob, w, h, false, 0.8);
        blit_sz(sp, v, r, (sx - w / 2.).floor(), sy - h - bob, w, h, tint);
        if m.kind == K_CRYSTG && !m.down {
            // เสี้ยวคริสตัลบนบ่า
            for (dx, dy) in [(-9., 0.28), (8., 0.22)] { prect(v, sx + dx, sy - h * (1. + dy) - bob, 2., 5., hx(0xa8f8ff)); prect(v, sx + dx, sy - h * (1. + dy) - bob, 1., 3., WHITE); }
        }
        if m.kind == K_IRONG && !m.down { prect(v, sx - 4., sy - h * 0.55 - bob, 8., 1., hx(0xe08a40)); }
    }
    // ลำแสงคริสตัล
    if m.beam.2 > 0. {
        let (bx, by) = ((m.beam.0 - v.cx) * v.s, (m.beam.1 - 6. - v.cy) * v.s);
        draw_line(sx * v.s, (sy - 14.) * v.s, bx, by, 3. * v.s * 0.6, Color::new(0.5, 0.95, 1., (m.beam.2 / 0.18).clamp(0., 1.)));
        draw_line(sx * v.s, (sy - 14.) * v.s, bx, by, 1.2 * v.s * 0.6, Color::new(1., 1., 1., (m.beam.2 / 0.18).clamp(0., 1.)));
    }
    // เครื่องหมายหน้าที่: โล่ = เฝ้าจุด · ใบไม้ = พัก
    let top = sy - 30.;
    if m.mode == M_GUARD {
        let (hx0, hy0) = (sx * v.s, top * v.s);
        let s = 3. * v.s;
        draw_rectangle(hx0 - s, hy0 - s, s * 2., s * 2.4, hx(0x4a78c8));
        draw_rectangle_lines(hx0 - s, hy0 - s, s * 2., s * 2.4, 1., hx(0xe8f0ff));
    } else if m.mode == M_ROAM {
        draw_circle(sx * v.s, (top + 1.) * v.s, 2.6 * v.s, hx(0x6ad06a));
    }
    if m.down {
        // กากบาทแดงเหนือหัว
        let (cx0, cy0, r) = (sx * v.s, (top - 4.) * v.s, 3. * v.s);
        draw_line(cx0 - r, cy0 - r, cx0 + r, cy0 + r, 2., hx(0xe05050));
        draw_line(cx0 - r, cy0 + r, cx0 + r, cy0 - r, 2., hx(0xe05050));
    } else if m.hp < m.maxhp() {
        let f = (m.hp / m.maxhp()).clamp(0., 1.);
        prect(v, sx - 8., top - 6., 16., 2., hx(0x2a1010));
        prect(v, sx - 8., top - 6., 16. * f, 2., if f > 0.5 { hx(0x6ad06a) } else if f > 0.25 { hx(0xe8c84a) } else { hx(0xd05a5a) });
    }
    if sad { draw_circle((sx + 7.) * v.s, (sy - 26.) * v.s, 2. * v.s, hx(0x6ab0ff)); }
}

#[cfg(test)]
mod tests_v33 {
    use super::*;

    fn game(seed: i32) -> Game {
        let mut g = Game::new_world_seed(seed, Sfx::default());
        g.tut = TUT_DONE;
        g
    }
    fn boss_near(g: &mut Game, kind: u8) {
        g.bosses.clear();
        let (x, y) = (g.p.x + 40., g.p.y);
        g.spawn_boss_hp(kind, x, y, 10.);
    }
    fn yl(kind: u8, x: f32) -> Yield {
        let t = boss_traits(7, kind, x, 100.);
        Yield { kind, t, trust: base_trust(&t, 0), gift: 0., gifts: 0, asks: 1, reply: String::new() }
    }
    fn rate(kind: u8, gifts: i32, rel: i32) -> f32 {
        let n = 60;
        let mut yes = 0;
        for i in 0..n {
            let mut y = yl(kind, i as f32 * 53.);
            y.trust = base_trust(&y.t, rel);
            for k in 0..gifts { y.gift += 14. * 0.6f32.powi(k); }
            if yield_score(&y) >= V_YES { yes += 1; }
        }
        yes as f32 / n as f32
    }

    #[test]
    fn traits_stable_and_in_range() {
        let a = boss_traits(11, 1, 320., 640.);
        assert_eq!(a, boss_traits(11, 1, 320., 640.), "ตำแหน่งเดิม = นิสัยเดิม");
        assert_ne!(a, boss_traits(11, 1, 960., 640.), "คนละตัว นิสัยไม่ซ้ำกันเป๊ะ");
        for k in 0..2u8 { for i in 0..50 { for v in boss_traits(3, k, i as f32 * 37., i as f32 * 11.) { assert!((0..=100).contains(&v)); } } }
        assert!(!trait_words(&a).is_empty());
    }
    #[test]
    fn willingness_depends_on_personality_and_reputation() {
        // ราชาสไลม์ช่างสังคม เป็นมิตร: ชวนเฉยๆ ยังไม่พอ แต่ของขวัญช่วยได้
        let (s0, s2) = (rate(0, 0, 0), rate(0, 2, 0));
        assert!(s0 < 0.2, "ไม่ให้อะไรเลยไม่ค่อยยอม {}", s0);
        assert!(s2 > 0.6, "ให้ของโปรดสองชิ้นมักยอม {}", s2);
        // โกเลมรักหน้าที่ ชอบอยู่ลำพัง: ซื้อใจด้วยของขวัญไม่ได้
        let g3 = rate(1, 3, 0);
        assert!(g3 < 0.25, "โกเลมไม่ยอมเพราะของขวัญอย่างเดียว {}", g3);
        // แต่ถ้าเราเคยไว้ชีวิตพวกเดียวกัน ข่าวความใจดีทำให้ไว้ใจขึ้น
        let rep = rate(1, 3, 24);
        assert!(rep > 0.55 && rep > g3 + 0.3, "ชื่อเสียงดีทำให้ยอมมากขึ้น {} vs {}", rep, g3);
        // ปราบทิ้งมาก่อน = ไม่มีใครไว้ใจ
        assert!(rate(0, 2, -20) < s2 - 0.3);
    }
    #[test]
    fn pestering_backfires() {
        let mut y = yl(0, 100.);
        y.gift = 14.;
        let a = yield_score(&y);
        y.asks = 3;
        assert_eq!(yield_score(&y), a - 16, "ถามซ้ำสองครั้งเกินไป −16 (ต่อต้านเมื่อถูกตื๊อ)");
    }
    #[test]
    fn boss_yields_instead_of_dying() {
        let mut g = game(3301);
        boss_near(&mut g, 0);
        let (bx, by) = (g.bosses[0].x, g.bosses[0].y);
        g.damage_boss(0, 999, false);
        assert!(g.bosses[0].yld && !g.bosses[0].dead, "ยอมแพ้ ไม่ตาย");
        assert_eq!(g.bosses[0].hp, 1.);
        assert!(g.rs.crew.yld.is_some());
        // ตีซ้ำไม่ได้ผล + บอสหยุดสู้ (ไม่ขยับ ไม่ทำร้าย)
        g.damage_boss(0, 999, false);
        assert!(!g.bosses[0].dead);
        let hp = g.p.hp;
        for _ in 0..120 { g.upd_bosses(1. / 30.); }
        assert_eq!((g.bosses[0].x, g.bosses[0].y), (bx, by));
        assert_eq!(g.p.hp, hp);
        assert_eq!(g.bosses[0].atk, 0);
    }
    #[test]
    fn final_boss_and_champion_never_yield() {
        let mut g = game(3302);
        g.bosses.clear();
        g.spawn_boss_hp(FINAL_KIND, g.p.x + 40., g.p.y, 5.);
        g.damage_boss(0, 999, false);
        assert!(g.bosses[0].dead && g.rs.crew.yld.is_none(), "บอสสุดท้ายตายตามปกติ");
    }
    #[test]
    fn spare_gives_reputation_and_slay_costs_it() {
        let mut g = game(3303);
        boss_near(&mut g, 1);
        g.damage_boss(0, 999, false);
        g.yield_spare();
        assert!(g.bosses.iter().all(|b| b.dead) && g.rs.crew.yld.is_none());
        assert_eq!(g.rs.crew.rel, [0, 12], "ไว้ชีวิตโกเลม = ชื่อเสียงกับโกเลมดีขึ้น");
        assert!(g.boss_cd > 0.);
        boss_near(&mut g, 1);
        let (xp, kills, core) = (g.p.xp + g.p.lvl * 10 * 0, g.p.kills, g.p.inv[CORE]);
        let _ = xp;
        g.damage_boss(0, 999, false);
        g.yield_slay();
        assert!(g.bosses.iter().all(|b| b.dead));
        assert_eq!(g.rs.crew.rel[1], 6, "ปราบทิ้ง −6 จาก 12");
        assert_eq!(g.p.kills, kills + 1);
        assert!(g.p.inv[CORE] > core, "ปราบทิ้งยังได้ของหายากเหมือนเดิม");
    }
    #[test]
    fn gifts_diminish_and_cap() {
        let mut g = game(3304);
        boss_near(&mut g, 0);
        g.damage_boss(0, 999, false);
        g.p.inv[BERRY] = 10;
        let mut last = 0.;
        let mut gains = vec![];
        for _ in 0..3 { g.yield_gift(); let t = g.rs.crew.yld.as_ref().unwrap().gift; gains.push(t - last); last = t; }
        assert!(gains[0] > gains[1] && gains[1] > gains[2], "ให้ซ้ำได้น้อยลง {:?}", gains);
        assert_eq!(g.p.inv[BERRY], 7);
        g.yield_gift();
        assert_eq!(g.p.inv[BERRY], 7, "ให้เกิน 3 ครั้ง = ปฏิเสธ (กลายเป็นสินบน)");
        assert_eq!(g.rs.crew.yld.as_ref().unwrap().gifts, 3);
    }
    #[test]
    fn gift_without_item_tells_where_to_find() {
        let mut g = game(3305);
        boss_near(&mut g, 0);
        g.damage_boss(0, 999, false);
        g.p.inv[BERRY] = 0; g.p.inv[SHROOM] = 0;
        g.yield_gift();
        assert_eq!(g.rs.crew.yld.as_ref().unwrap().gifts, 0);
        assert!(g.toasts.iter().any(|t| t.0.contains("ไม่มี")));
    }
    #[test]
    fn willing_boss_joins_and_unwilling_refuses() {
        let mut g = game(3306);
        boss_near(&mut g, 0);
        g.damage_boss(0, 999, false);
        g.rs.crew.yld.as_mut().unwrap().trust = 100;
        g.yield_ask();
        assert_eq!(g.rs.crew.m.len(), 1);
        let m = &g.rs.crew.m[0];
        assert_eq!(m.kind, K_SLIME);
        assert!(m.mind() && m.loyalty >= 40);
        assert_eq!(m.mode, m.wished(), "เลือกบทบาทตามที่ตัวเองอยาก");
        assert!(g.bosses.iter().all(|b| b.dead) && g.rs.crew.yld.is_none());

        let mut g = game(3307);
        boss_near(&mut g, 1);
        g.damage_boss(0, 999, false);
        g.rs.crew.yld.as_mut().unwrap().trust = 0;
        g.yield_ask();
        assert!(g.rs.crew.m.is_empty(), "ไม่เต็มใจ = ชวนไม่ได้ ไม่มีบังคับ");
        assert!(g.rs.crew.yld.is_some() && !g.bosses[0].dead);
        assert!(!g.rs.crew.yld.as_ref().unwrap().reply.is_empty());
        g.yield_ask(); g.yield_ask();
        assert!(g.rs.crew.yld.as_ref().unwrap().reply.contains("ตื๊อ"), "ถูกตื๊อ → ต่อต้าน");
    }
    #[test]
    fn crew_is_capped_at_four() {
        let mut g = game(3308);
        for _ in 0..CREW_MAX { g.crew_join(K_SLIME, [50; 6], 60, g.p.x, g.p.y); }
        boss_near(&mut g, 0);
        g.damage_boss(0, 999, false);
        g.rs.crew.yld.as_mut().unwrap().trust = 100;
        g.yield_ask();
        assert_eq!(g.rs.crew.m.len(), CREW_MAX);
        assert!(g.rs.crew.yld.is_some(), "พวกเต็ม: ยังไม่ตัดสินใจ รอให้เลือกใหม่");
    }
    #[test]
    fn yield_card_vanishes_if_boss_gone() {
        let mut g = game(3309);
        boss_near(&mut g, 0);
        g.damage_boss(0, 999, false);
        g.bosses.clear();
        g.crew_tick(0.1);
        assert!(g.rs.crew.yld.is_none());
    }
    fn adv(k: u8) -> usize { ADV_RECIPES.iter().position(|r| matches!(r.eff, Eff::Guard(x) if x == k)).unwrap() }
    fn foe(x: f32, y: f32, hp: f32) -> Enemy {
        Enemy { path: Path::default(), night: false, burn: 0., slow: 0., stun: 0., btick: 0., sk: 0., fl: false, k: EK::Skel, x, y, hp, spd: 0., dmg: 2, xp: 1, coin: 0, vx: 0., vy: 0., kb: 0., flash: 0., wt: 0., wdx: 0., wdy: 0., anim: 0., pal: 0, dead: false }
    }
    /// วางพวกที่ห่างจากผู้เล่น (dx, dy)
    fn golem_at(g: &mut Game, kind: u8, dx: f32, dy: f32) -> usize {
        let (x, y) = (g.p.x + dx, g.p.y + dy);
        g.crew_craft(kind);
        let i = g.rs.crew.m.len() - 1;
        g.rs.crew.m[i].x = x; g.rs.crew.m[i].y = y; g.rs.crew.m[i].post = (tfloor(x), tfloor(y));
        i
    }

    #[test]
    fn three_golems_have_recipes_with_the_right_materials() {
        let st = &ADV_RECIPES[adv(1)];
        let ir = &ADV_RECIPES[adv(2)];
        let cr = &ADV_RECIPES[adv(3)];
        assert_eq!((st.name, ir.name, cr.name), ("โกเลมหิน", "โกเลมเหล็ก", "โกเลมคริสตัล"));
        for r in [st, ir, cr] { assert!(r.cost.iter().any(|&(it, n)| it == CORE && n >= 1), "{} ต้องใช้แกนโกเลม", r.name); }
        assert!(st.cost.iter().any(|&(it, n)| it == STONE && n >= 20), "หิน = แกน + หินเยอะ");
        assert!(ir.cost.iter().any(|&(it, _)| it == IRON), "เหล็ก = แกน + แร่เหล็ก");
        assert!(cr.cost.iter().any(|&(it, _)| it == GEM) && cr.cost.iter().any(|&(it, _)| it == SHARD), "คริสตัล = แกน + อัญมณี/เศษคริสตัล");
        // ทุกวัสดุต้องมีบอกที่มา (v32 guide) — คนเล่นไม่ต้องเดาว่าหาจากไหน
        let g = game(3330);
        for r in [st, ir, cr] { for &(it, _) in r.cost { assert!(!source::item_howto(&g, it).is_empty(), "{} ไม่มีที่มา", INAME[it]); } }
    }
    #[test]
    fn craft_golems_cost_materials_and_respect_cap() {
        let mut g = game(3310);
        let ri = RECIPES.iter().position(|r| matches!(r.eff, Eff::Guard(0))).unwrap();
        g.p.inv[WOOD] = 14; g.p.inv[STONE] = 3;
        g.craft_from(&RECIPES, ri);
        assert_eq!((g.rs.crew.m.len(), g.rs.crew.m[0].kind, g.rs.crew.m[0].mode), (1, K_WOODG, M_GUARD));
        assert_eq!((g.p.inv[WOOD], g.p.inv[STONE]), (0, 0));
        // ไม่มีแกน = คราฟต์ไม่ได้ ของไม่หาย
        g.p.inv[CORE] = 0; g.p.inv[STONE] = 40;
        g.craft_from(&ADV_RECIPES, adv(1));
        assert_eq!((g.rs.crew.m.len(), g.p.inv[STONE]), (1, 40));
        // มีครบ = ได้ตัวที่ถูกชนิด และหักวัสดุตามจริง
        g.p.inv[CORE] = 6; g.p.inv[IRON] = 12; g.p.inv[GEM] = 6; g.p.inv[SHARD] = 4;
        let (c0, s0) = (g.p.inv[CORE], g.p.inv[STONE]);
        g.craft_from(&ADV_RECIPES, adv(1));
        assert_eq!(g.rs.crew.m.last().unwrap().kind, K_STONEG);
        assert_eq!((c0 - g.p.inv[CORE], s0 - g.p.inv[STONE]), (1, 24));
        g.craft_from(&ADV_RECIPES, adv(2));
        assert_eq!(g.rs.crew.m.last().unwrap().kind, K_IRONG);
        assert_eq!(g.p.inv[IRON], 0, "โกเลมเหล็กกินแร่เหล็ก 12");
        g.craft_from(&ADV_RECIPES, adv(3));
        assert_eq!(g.rs.crew.m.last().unwrap().kind, K_CRYSTG);
        assert_eq!((g.p.inv[GEM], g.p.inv[SHARD]), (0, 0));
        assert_eq!(g.rs.crew.m.len(), CREW_MAX);
        // พวกเต็ม ไม่หักของ
        g.p.inv[CORE] = 3; g.p.inv[STONE] = 30;
        g.craft_from(&ADV_RECIPES, adv(1));
        assert_eq!((g.rs.crew.m.len(), g.p.inv[CORE], g.p.inv[STONE]), (CREW_MAX, 3, 30), "พวกเต็ม ไม่หักของ");
    }
    #[test]
    fn golems_are_loyal_machines_with_distinct_stats() {
        let ms: Vec<Member> = [K_STONEG, K_IRONG, K_CRYSTG].iter().map(|&k| Member::new(1, k, 0., 0.)).collect();
        for m in &ms { assert!(!m.mind() && m.golem() && m.loyalty == 100 && m.mode == M_GUARD, "{}", m.name()); assert!(!m.skill_text().is_empty()); }
        assert!(ms[0].maxhp() > ms[1].maxhp() && ms[1].maxhp() > ms[2].maxhp(), "หินถึกสุด คริสตัลบางสุด");
        assert!(ms[1].dmg(10) > ms[0].dmg(10) && ms[1].dmg(10) > ms[2].dmg(10), "เหล็กตีหนักสุด");
        assert!(ms[0].armor() < 1. && ms[1].armor() < 1. && ms[2].armor() == 1.);
        let texts: Vec<_> = ms.iter().map(|m| m.skill_text()).collect();
        assert!(texts[0].contains("ยั่วยุ") && texts[1].contains("ทุบพื้น") && texts[2].contains("ลำแสง"));
    }
    #[test]
    fn stone_golem_taunt_pulls_enemies_off_the_player() {
        // ศัตรูอยู่ระหว่างผู้เล่น (ใต้) กับโกเลม (ซ้าย): ไม่ยั่วยุ → เดินหาผู้เล่น · ยั่วยุ → เดินหาโกเลม
        let mut moved = [(0f32, 0f32); 2];
        for (case, taunt) in [false, true].into_iter().enumerate() {
            let mut g = game(3331);
            g.en.clear(); g.bosses.clear();
            let (ex, ey) = (g.p.x, g.p.y - 50.);
            g.p.x = ex; g.p.y = ey + 60.;
            let gi = golem_at(&mut g, K_STONEG, -60., -60.);
            g.rs.crew.m[gi].taunt_t = if taunt { 4. } else { 0. };
            let mut e = foe(ex, ey, 99.); e.spd = 40.;
            g.en.push(e);
            assert_eq!(g.crew_taunts().len(), taunt as usize);
            for _ in 0..20 { g.upd_enemies(1. / 30.); }
            moved[case] = (g.en[0].x - ex, g.en[0].y - ey);
        }
        assert!(moved[0].1 > 1. && moved[0].0.abs() < moved[0].1, "ไม่ยั่วยุ: ศัตรูเดินหาผู้เล่น {:?}", moved[0]);
        assert!(moved[1].0 < -1. && moved[1].0.abs() > moved[1].1.abs(), "ยั่วยุ: ศัตรูเดินหาโกเลม {:?}", moved[1]);
    }
    #[test]
    fn stone_golem_triggers_taunt_with_cooldown() {
        let mut g = game(3332);
        g.en.clear(); g.bosses.clear();
        let gi = golem_at(&mut g, K_STONEG, 30., 0.);
        let (x, y) = (g.rs.crew.m[gi].x, g.rs.crew.m[gi].y);
        g.en.push(foe(x + 60., y, 99.));
        assert!(g.crew_taunts().is_empty());
        g.crew_tick(1. / 30.);
        assert_eq!(g.crew_taunts().len(), 1, "เห็นศัตรูในระยะ → ยั่วยุทันที");
        let t = g.rs.crew.m[gi].taunt_t;
        assert!(t > 3. && g.rs.crew.m[gi].skill_cd > 5.);
        for _ in 0..30 * 5 { g.crew_tick(1. / 30.); }
        assert!(g.crew_taunts().is_empty(), "ยั่วยุหมดเวลา");
        for _ in 0..30 * 6 { g.crew_tick(1. / 30.); }
        assert_eq!(g.crew_taunts().len(), 1, "คูลดาวน์หมดแล้วยั่วยุอีก");
    }
    #[test]
    fn iron_golem_slam_knocks_back_stuns_and_hurts() {
        let mut g = game(3333);
        g.en.clear(); g.bosses.clear();
        let gi = golem_at(&mut g, K_IRONG, 30., 0.);
        let (x, y) = (g.rs.crew.m[gi].x, g.rs.crew.m[gi].y);
        g.en.push(foe(x + 18., y, 60.));
        let d0 = 18.;
        g.crew_tick(1. / 30.);
        let e = &g.en[0];
        assert!(e.hp < 60., "ทุบแล้วเจ็บ");
        assert!(e.stun >= 0.9 && e.kb > 0.2, "สตัน {} กระเด็น {}", e.stun, e.kb);
        assert!(e.vx > 100., "กระเด็นออกจากโกเลม {}", e.vx);
        assert!(g.rs.crew.m[gi].slam_t > 0. && g.rs.crew.m[gi].skill_cd > 4.);
        let _ = d0;
    }
    #[test]
    fn crystal_golem_beam_hits_from_range_and_slows() {
        let mut g = game(3334);
        g.en.clear(); g.bosses.clear();
        let gi = golem_at(&mut g, K_CRYSTG, 30., 0.);
        let (x, y) = (g.rs.crew.m[gi].x, g.rs.crew.m[gi].y);
        g.en.push(foe(x + 90., y, 80.));
        g.crew_tick(1. / 30.);
        let e = &g.en[0];
        assert!(e.hp < 80., "ลำแสงโดนจากระยะ 90");
        assert!(e.slow >= 2., "โดนแล้วเดินช้าลง {}", e.slow);
        assert!(g.rs.crew.m[gi].beam.2 > 0.);
        assert!((g.rs.crew.m[gi].x - x).abs() < 3., "ยิงจากที่เดิม ไม่ต้องเข้าไปประชิด");
        // ศัตรูเข้าประชิด: ถอยหลังรักษาระยะ
        g.en[0].x = x + 20.;
        let x0 = g.rs.crew.m[gi].x;
        for _ in 0..20 { g.crew_tick(1. / 30.); }
        assert!(g.rs.crew.m[gi].x < x0 - 1., "ถอยจากศัตรูที่ประชิด");
    }
    #[test]
    fn golems_never_touch_bosses() {
        let mut g = game(3335);
        g.en.clear();
        g.bosses.clear();
        let gi = golem_at(&mut g, K_IRONG, 30., 0.);
        let (x, y) = (g.rs.crew.m[gi].x, g.rs.crew.m[gi].y);
        g.spawn_boss_hp(0, x + 14., y, 10.);
        let hp = g.bosses[0].hp;
        for _ in 0..150 { g.crew_tick(1. / 30.); }
        assert_eq!(g.bosses[0].hp, hp, "โกเลมไม่สู้บอสตัวจริง");
    }
    #[test]
    fn golem_goes_down_and_repairs_itself_without_food() {
        let mut g = game(3336);
        g.en.clear(); g.bosses.clear();
        let gi = golem_at(&mut g, K_CRYSTG, 30., 0.);
        let (x, y) = (g.rs.crew.m[gi].x, g.rs.crew.m[gi].y);
        g.rs.crew.m[gi].hp = 1.;
        let mut e = foe(x + 4., y, 99.); e.dmg = 20;
        g.en.push(e);
        for _ in 0..40 { g.crew_tick(1. / 30.); }
        assert!(g.rs.crew.m[gi].down && g.rs.crew.m[gi].hp == 0., "เลือดหมด = ล้ม");
        let d0 = g.defense();
        assert!(g.crew_defense() == 0., "ตัวที่ล้มไม่นับป้องกัน {}", d0);
        let hp = g.en[0].hp;
        for _ in 0..60 { g.crew_tick(1. / 30.); }
        assert_eq!(g.en[0].hp, hp, "ตัวที่ล้มไม่โจมตี");
        // 30 วันไม่ให้อะไรเลย: ไม่หิว ไม่ทรยศ ไม่หนี ซ่อมเองตอนเช้า
        for _ in 0..30 { g.time += DAYLEN; g.crew_dawn(); }
        let m = &g.rs.crew.m[gi];
        assert!(!m.down && m.hp == m.maxhp(), "ซ่อมตัวเองตอนเช้า");
        assert_eq!((g.rs.crew.m.len(), m.loyalty, m.mood), (1, 100, 100), "โกเลมซื่อสัตย์ 100% ไม่ต้องดูแล");
    }
    #[test]
    fn roles_follow_guard_roam_behave_differently() {
        let mut g = game(3337);
        g.en.clear(); g.bosses.clear();
        let (x0, y0) = (g.p.x, g.p.y);
        let gi = golem_at(&mut g, K_STONEG, 0., 0.);
        let id = g.rs.crew.m[gi].id;
        // เฝ้าจุด: ผู้เล่นเดินออกไปไกล โกเลมอยู่ที่เดิม
        g.crew_set_mode(id, M_GUARD);
        g.p.x = x0 + 150.;
        for _ in 0..30 * 6 { g.crew_tick(1. / 30.); }
        let m = &g.rs.crew.m[gi];
        assert!((m.x - x0).hypot(m.y - y0) < 8., "เฝ้าจุดไม่ตามไป");
        assert!(g.crew_defense() > 0., "ประจำจุด = นับป้องกันเมือง");
        // ตามฉัน: ตามไปติดตัว และไม่นับป้องกันเมือง
        g.crew_set_mode(id, M_FOLLOW);
        for _ in 0..30 * 12 { g.crew_tick(1. / 30.); }
        let m = &g.rs.crew.m[gi];
        assert!((m.x - g.p.x).hypot(m.y - g.p.y) < 45., "ตามผู้เล่น {}", (m.x - g.p.x).hypot(m.y - g.p.y));
        assert_eq!(g.crew_defense(), 0.);
        // พัก/เดินเล่น: วนอยู่แถวจุดพัก ไม่หนีไกล และฟื้นเลือดเร็วกว่าเฝ้า (จุดพัก = ตรงที่ผู้เล่นยืนตอนสั่ง)
        g.p.x = x0; g.p.y = y0; // กลับมายืนที่โล่งก่อนสั่ง (จุดพักต้องไม่ติดต้นไม้)
        g.crew_set_mode(id, M_ROAM);
        let post = g.rs.crew.m[gi].post;
        let (px, py) = (post.0 as f32 * 16. + 8., post.1 as f32 * 16. + 8.);
        g.rs.crew.m[gi].x = px; g.rs.crew.m[gi].y = py;
        g.rs.crew.m[gi].hp = 10.; g.rs.crew.m[gi].last_hit = 99.;
        g.p.x = x0 + 300.; // ผู้เล่นเดินจากไป — ต้องไม่ตามไป
        let mut far = 0f32; let mut moved = false;
        for _ in 0..30 * 20 { g.crew_tick(1. / 30.); let m = &g.rs.crew.m[gi]; far = far.max((m.x - px).hypot(m.y - py)); if (m.x - px).hypot(m.y - py) > 3. { moved = true; } }
        assert!(far < 50., "เดินเล่นแถวจุดพัก {}", far);
        assert!(moved, "เดินเล่นจริง ไม่ยืนนิ่ง");
        let roam_hp = g.rs.crew.m[gi].hp;
        g.p.x = px; g.p.y = py;
        g.crew_set_mode(id, M_GUARD);
        g.rs.crew.m[gi].hp = 10.; g.rs.crew.m[gi].last_hit = 99.;
        for _ in 0..30 * 20 { g.crew_tick(1. / 30.); }
        assert!(roam_hp > g.rs.crew.m[gi].hp + 5., "พักฟื้นเร็วกว่าเฝ้า {} vs {}", roam_hp, g.rs.crew.m[gi].hp);
    }
    #[test]
    fn roam_raises_happiness_of_minded_member() {
        let mut g = game(3338);
        g.crew_join(K_SLIME, [80, 20, 80, 70, 40, 60], 60, g.p.x, g.p.y);
        let id = g.rs.crew.m[0].id;
        g.rs.crew.m[0].mood = 50;
        g.crew_set_mode(id, M_ROAM);
        let (m0, l0) = (g.rs.crew.m[0].mood, g.rs.crew.m[0].loyalty);
        g.p.inv[BERRY] = 0;
        for _ in 0..3 { g.time += DAYLEN; g.rs.crew.m[0].fed_day = day_of(&g); g.crew_dawn(); }
        assert!(g.rs.crew.m[0].mood > m0 + 10 && g.rs.crew.m[0].loyalty > l0, "พักผ่อน = มีความสุขขึ้น");
    }
    #[test]
    fn crew_cards_show_pets_golems_and_hired_companion() {
        let mut g = game(3339);
        g.animals.clear();
        let mut a = g.new_animal(A_WOLF, g.p.x + 10., g.p.y); a.tame = true; a.sat = 10.; g.animals.push(a);
        g.crew_craft(K_IRONG);
        g.crew_join(K_SLIME, [60; 6], 55, g.p.x, g.p.y);
        let cards = menu::crew_cards(&g);
        assert_eq!(cards.iter().map(|c| c.src).collect::<Vec<_>>(), vec![1, 0, 0]);
        let pet = &cards[0];
        assert!(pet.sat.is_some() && pet.loy.is_some() && pet.warn.as_ref().map_or(false, |w| w.contains("หิว")), "สัตว์หิว = เตือน");
        let gol = &cards[1];
        assert!(gol.golem && gol.loy.is_none() && gol.sat.is_none() && gol.traits.contains("100%"), "โกเลมไม่มีแถบอารมณ์/ภักดี");
        let sl = &cards[2];
        assert!(sl.mind && sl.loy == Some(55.) && sl.sat.is_some());
        assert!(!pet.wher.is_empty() && !gol.wher.is_empty());
        // ไม่มีอะไรเลย = การ์ดว่าง (แท็บแสดงคำแนะนำแทน)
        let mut h = game(3340); h.animals.iter_mut().for_each(|a| a.tame = false);
        assert!(menu::crew_cards(&h).is_empty());
    }

    // ---------- สัตว์เลี้ยง ----------
    fn pet(g: &mut Game, k: u8, dx: f32) -> usize {
        g.animals.clear();
        let mut a = g.new_animal(k, g.p.x + dx, g.p.y); a.tame = true; g.animals.push(a);
        g.animals.len() - 1
    }
    #[test]
    fn feeding_and_petting_keep_pets_happy() {
        let mut g = game(3341);
        let i = pet(&mut g, A_WOLF, 14.);
        g.animals[i].sat = 30.; g.animals[i].loy = 40.;
        g.p.inv[MEAT] = 2;
        g.pet_feed(0);
        assert_eq!(g.p.inv[MEAT], 1);
        assert!(g.animals[i].sat >= 70. && g.animals[i].loy > 40., "อิ่มขึ้น ไว้ใจขึ้น");
        g.animals[i].sat = 95.;
        g.pet_feed(0);
        assert_eq!(g.p.inv[MEAT], 1, "อิ่มแล้วไม่กินเพิ่ม (ไม่เสียของ)");
        // ไม่มีอาหาร: บอกว่าหาจากไหน ไม่หักอะไร
        g.animals[i].sat = 10.; g.p.inv[MEAT] = 0; g.p.inv[DISH_STEW] = 0;
        g.toasts.clear();
        g.pet_feed(0);
        assert_eq!(g.animals[i].sat, 10.);
        assert!(g.toasts.iter().any(|t| t.0.contains("ไม่มี")));
        // ลูบ: ฟรี ภักดีขึ้น แต่มีคูลดาวน์
        let l = g.animals[i].loy;
        g.pet_pat(0);
        assert!(g.animals[i].loy > l);
        let l = g.animals[i].loy;
        g.pet_pat(0);
        assert_eq!(g.animals[i].loy, l, "ลูบถี่เกินไปไม่ได้ผลเพิ่ม");
        // กระต่ายกินแครอท
        let j = pet(&mut g, A_RABBIT, 14.);
        g.animals[j].sat = 20.; g.p.inv[CARROT] = 1;
        g.pet_feed(0);
        assert!(g.animals[j].sat > 60. && g.p.inv[CARROT] == 0);
    }
    #[test]
    fn hungry_pet_is_weak_and_refuses_to_fight() {
        let mut g = game(3342);
        g.en.clear(); g.bosses.clear();
        let i = pet(&mut g, A_WOLF, 14.);
        let (ax, ay) = (g.animals[i].x, g.animals[i].y);
        g.animals[i].sat = 10.; g.animals[i].loy = 80.;
        g.en.push(foe(ax + 16., ay, 30.));
        for _ in 0..30 * 4 { g.pets_tick(1. / 30.); g.animals[i].sat = 10.; }
        assert_eq!(g.en[0].hp, 30., "หิวจนอ่อนแอ ไม่ยอมสู้");
        // กินอิ่มแล้วช่วยสู้
        g.animals[i].sat = 100.;
        for _ in 0..30 * 4 { g.pets_tick(1. / 30.); g.animals[i].sat = 100.; }
        assert!(g.en[0].hp < 30. || g.en[0].dead, "อิ่มแล้วกัดมอนสเตอร์ได้");
    }
    #[test]
    fn starved_neglected_pet_runs_back_to_the_wild() {
        let mut g = game(3343);
        let i = pet(&mut g, A_RABBIT, 14.);
        g.animals[i].sat = 40.; g.animals[i].loy = 50.;
        g.en.clear();
        let mut weak_seen = false;
        for _ in 0..(DAYLEN as i32 * 6) {
            g.pets_tick(1.);
            if g.animals.get(i).map_or(false, |a| a.tame && a.sat < PET_WEAK) { weak_seen = true; }
            if !g.animals[i].tame { break; }
        }
        assert!(weak_seen, "ก่อนหนีต้องผ่านช่วงอ่อนแอ (ให้เวลาแก้)");
        assert!(!g.animals[i].tame, "อดอยากนาน ๆ หนีกลับป่า");
        assert!(g.toasts.iter().any(|t| t.0.contains("หนีกลับป่า")));
        // ดูแลดีอยู่ได้ไม่รู้จบ
        let j = pet(&mut g, A_RABBIT, 14.);
        for d in 0..20 {
            g.p.inv[CARROT] = 5;
            g.pet_feed(0);
            for _ in 0..(DAYLEN as i32) { g.pets_tick(1.); }
            g.animals[j].pat = 0.; g.pet_pat(0);
        }
        assert!(g.animals[j].tame && g.animals[j].loy > 50., "ดูแลดี = อยู่ต่อและไว้ใจมากขึ้น");
    }
    #[test]
    fn pet_roles_follow_guard_and_roam() {
        let mut g = game(3344);
        g.en.clear();
        let i = pet(&mut g, A_WOLF, 14.);
        let (x0, y0) = (g.p.x, g.p.y);
        g.animals[i].sat = 100.; g.animals[i].loy = 100.;
        g.pet_set_role(0, PET_GUARD);
        g.p.x = x0 + 200.;
        for _ in 0..30 * 6 { g.pets_tick(1. / 30.); g.animals[i].sat = 100.; }
        assert!((g.animals[i].x - x0).hypot(g.animals[i].y - y0) < 10., "เฝ้าจุด ไม่ตามไป");
        g.p.x = x0 + 300.; // ไกลเกิน 260 = วาร์ปมาข้างตัว
        g.pet_set_role(0, PET_FOLLOW);
        for _ in 0..30 * 10 { g.pets_tick(1. / 30.); g.animals[i].sat = 100.; }
        assert!((g.animals[i].x - g.p.x).hypot(g.animals[i].y - g.p.y) < 45., "ตามฉัน");
        g.pet_set_role(0, PET_ROAM);
        let (rx, ry) = (g.animals[i].px, g.animals[i].py);
        let mut far = 0f32;
        for _ in 0..30 * 20 { g.pets_tick(1. / 30.); g.animals[i].sat = 100.; far = far.max((g.animals[i].x - rx).hypot(g.animals[i].y - ry)); }
        assert!(far < 60., "พัก/เดินเล่นแถวจุดนี้ {}", far);
        assert_eq!(g.animals[i].role, PET_ROAM);
        // เฝ้าจุด: หมาป่ากัดเฉพาะที่เข้าใกล้จุดประจำ ไม่วิ่งตามศัตรูไกลจุด
        g.pet_set_role(0, PET_GUARD);
        let (gx, gy) = (g.animals[i].px, g.animals[i].py);
        g.en.push(foe(gx + 200., gy, 30.));
        for _ in 0..30 * 4 { g.pets_tick(1. / 30.); g.animals[i].sat = 100.; }
        assert_eq!(g.en[0].hp, 30., "ศัตรูไกลจากจุดเฝ้า ไม่ไล่");
    }
    #[test]
    fn pet_release_frees_slot_and_pet_does_not_return_by_itself() {
        let mut g = game(3345);
        g.animals.clear();
        for k in [A_WOLF, A_RABBIT] { let mut a = g.new_animal(k, g.p.x + 20., g.p.y); a.tame = true; g.animals.push(a); }
        g.pet_release(1);
        assert_eq!(g.animals.iter().filter(|a| a.tame).count(), 1);
        assert!(!g.animals[1].tame && g.animals[1].st == AS::Flee, "กระต่ายวิ่งกลับป่า");
        for _ in 0..30 * 10 { g.pets_tick(1. / 30.); }
        assert!(!g.animals[1].tame, "ปล่อยแล้วไม่กลับมาเอง");
        assert!(g.animals[0].tame, "ตัวที่ยังไม่ปล่อยอยู่เหมือนเดิม");
        // ชวนตัวใหม่เข้ามาแทนได้
        let mut b = g.new_animal(A_RABBIT, g.p.x, g.p.y); b.tame = true; g.animals.push(b);
        assert_eq!(g.animals.iter().filter(|a| a.tame).count(), PET_MAX);
    }
    #[test]
    fn pet_state_survives_save_and_load() {
        let mut g = game(3346);
        g.animals.clear();
        let mut a = g.new_animal(A_WOLF, g.p.x + 10., g.p.y); a.tame = true; a.sat = 33.; a.loy = 71.; a.role = PET_GUARD; a.px = g.p.x + 10.; a.py = g.p.y; g.animals.push(a);
        let s = g.pets_save_str();
        assert!(s.starts_with("pet "));
        let mut h = game(3346);
        h.animals.clear();
        for l in s.lines() { let mut it = l.split_whitespace(); assert_eq!(it.next(), Some("pet")); let r: Vec<&str> = it.collect(); h.pets_load_line(&r); }
        let b = h.animals.iter().find(|a| a.tame).expect("โหลดสัตว์เลี้ยงกลับมา");
        assert_eq!((b.k, b.sat as i32, b.loy as i32, b.role), (A_WOLF, 33, 71, PET_GUARD));
        // เซฟเก่าที่ไม่มีช่องใหม่ ไม่ crash
        h.animals.clear();
        h.pets_load_line(&["0"]);
        assert_eq!(h.animals.iter().filter(|a| a.tame).count(), 1);
    }
    #[test]
    fn guards_raise_town_defense() {
        let mut g = game(3311);
        let d0 = g.defense();
        g.crew_craft(K_STONEG);
        assert!(g.defense() > d0 + 5.);
        let d1 = g.defense();
        let id = g.rs.crew.m[0].id;
        g.crew_set_mode(id, M_FOLLOW);
        assert!(g.defense() < d1, "ตามผู้เล่นไม่ได้ประจำจุด = ไม่นับป้องกันเมือง");
    }
    #[test]
    fn guard_fights_nearby_monsters() {
        let mut g = game(3312);
        g.crew_craft(K_STONEG);
        let (gx, gy) = (g.rs.crew.m[0].x, g.rs.crew.m[0].y);
        g.en.clear();
        g.en.push(Enemy { path: Path::default(), night: false, burn: 0., slow: 0., stun: 0., btick: 0., sk: 0., fl: false, k: EK::Skel, x: gx + 9., y: gy, hp: 6., spd: 0., dmg: 0, xp: 1, coin: 0, vx: 0., vy: 0., kb: 0., flash: 0., wt: 0., wdx: 0., wdy: 0., anim: 0., pal: 0, dead: false });
        for _ in 0..150 { g.crew_tick(1. / 30.); }
        assert!(g.en.iter().all(|e| e.dead || e.hp < 6.), "ยามตีมอนสเตอร์ที่เข้ามาใกล้");
    }
    #[test]
    fn follower_returns_to_player_and_guard_to_post() {
        let mut g = game(3313);
        g.crew_join(K_SLIME, [80, 20, 80, 70, 40, 60], 70, g.p.x, g.p.y);
        let id = g.rs.crew.m[0].id;
        g.crew_set_mode(id, M_FOLLOW);
        g.en.clear();
        g.rs.crew.m[0].x = g.p.x + 120.;
        for _ in 0..240 { g.crew_tick(1. / 30.); }
        let m = &g.rs.crew.m[0];
        assert!((m.x - g.p.x).hypot(m.y - g.p.y) < 40., "ตามผู้เล่น {}", (m.x - g.p.x).hypot(m.y - g.p.y));
    }
    #[test]
    fn neglected_boss_member_grows_lonely_and_leaves_but_cared_one_stays() {
        let mut g = game(3314);
        g.crew_join(K_SLIME, [70, 30, 70, 60, 45, 55], 50, g.p.x, g.p.y);
        g.crew_join(K_SLIME, [70, 30, 70, 60, 45, 55], 50, g.p.x, g.p.y);
        let (a, b) = (g.rs.crew.m[0].id, g.rs.crew.m[1].id);
        g.p.inv[BERRY] = 99;
        for _ in 0..14 {
            g.time += DAYLEN;
            g.crew_gift_member(b);
            g.crew_dawn();
        }
        assert!(g.rs.crew.m.iter().all(|m| m.id != a), "ไม่เคยดูแลเลย → เหงา เสียใจ ลาไปเอง");
        let m = g.rs.crew.m.iter().find(|m| m.id == b).expect("ดูแลดี → อยู่ต่อ");
        assert!(m.loyalty >= 60 && m.mood >= 60, "ภักดี {} อารมณ์ {}", m.loyalty, m.mood);
    }
    #[test]
    fn forcing_against_nature_erodes_loyalty() {
        let mut g = game(3315);
        let t = boss_traits(5, 1, 100., 100.);
        g.crew_join(K_BGOLEM, t, 70, g.p.x, g.p.y);
        let id = g.rs.crew.m[0].id;
        assert_eq!(g.rs.crew.m[0].mode, M_GUARD, "โกเลมรักหน้าที่ อยากเฝ้าบ้าน");
        g.p.inv[GEM] = 99;
        g.crew_set_mode(id, M_FOLLOW);
        let l0 = g.rs.crew.m[0].loyalty;
        for _ in 0..8 { g.time += DAYLEN; g.rs.crew.m[0].fed_day = day_of(&g); g.crew_dawn(); }
        assert!(g.rs.crew.m[0].loyalty < l0, "ถูกใช้งานผิดนิสัย ความภักดีลดลง");
        g.crew_set_mode(id, M_GUARD);
        assert_eq!(g.rs.crew.m[0].mode, g.rs.crew.m[0].wished());
    }
    #[test]
    fn machines_have_no_mind() {
        let mut g = game(3316);
        g.crew_craft(K_WOODG);
        g.p.inv[BERRY] = 5;
        let id = g.rs.crew.m[0].id;
        g.crew_gift_member(id);
        assert_eq!(g.p.inv[BERRY], 5, "หุ่นไม่กินของขวัญ");
        for _ in 0..30 { g.time += DAYLEN; g.crew_dawn(); }
        assert_eq!(g.rs.crew.m.len(), 1, "หุ่นไม่เหงา ไม่ลาไป");
    }
    #[test]
    fn release_returns_materials_or_keepsake() {
        let mut g = game(3317);
        g.crew_craft(K_WOODG);
        let id = g.rs.crew.m[0].id;
        let (w, s) = (g.p.inv[WOOD], g.p.inv[STONE]);
        g.crew_release(id);
        assert!(g.rs.crew.m.is_empty());
        assert_eq!((g.p.inv[WOOD] - w, g.p.inv[STONE] - s), (7, 1), "รื้อหุ่นไม้ คืนครึ่งหนึ่ง");
        g.crew_join(K_SLIME, [60; 6], 85, g.p.x, g.p.y);
        let gem = g.p.inv[GEM];
        let id = g.rs.crew.m[0].id;
        g.crew_release(id);
        assert!(g.rs.crew.m.is_empty() && g.p.inv[GEM] == gem + 2, "ภักดีมาก ทิ้งอัญมณีไว้เป็นที่ระลึก");
    }
    #[test]
    fn pets_can_be_released_and_replaced() {
        let mut g = game(3318);
        g.animals.clear();
        for k in [A_WOLF, A_RABBIT] { let mut a = g.new_animal(k, g.p.x + 20., g.p.y); a.tame = true; g.animals.push(a); }
        assert_eq!(g.animals.iter().filter(|a| a.tame).count(), PET_MAX);
        g.pet_release(0);
        assert_eq!(g.animals.iter().filter(|a| a.tame).count(), 1);
        assert!(!g.animals[0].tame && g.animals[1].tame, "ปล่อยตัวแรก ตัวที่สองยังอยู่");
        g.pet_release(5);
        assert_eq!(g.animals.iter().filter(|a| a.tame).count(), 1, "index เกิน = ไม่ทำอะไร");
    }
    #[test]
    fn crew_save_roundtrip() {
        let mut g = game(3319);
        g.rs.crew.rel = [12, -6];
        g.crew_craft(K_IRONG);
        g.crew_craft(K_CRYSTG);
        g.crew_join(K_BGOLEM, [22, 85, 20, 38, 12, 72], 66, 321.5, 87.5);
        g.rs.crew.m[0].hp = 33.; g.rs.crew.m[1].hp = 0.; g.rs.crew.m[1].down = true; g.rs.crew.m[1].mode = M_ROAM;
        g.rs.crew.m[2].mood = 44; g.rs.crew.m[2].sad = 1; g.rs.crew.m[2].fed_day = 9; g.rs.crew.m[2].post = (12, -3);
        let s = g.crew_save();
        let mut h = game(3319);
        for l in s.lines() {
            let mut it = l.split_whitespace();
            let tag = it.next().unwrap();
            let rest: Vec<&str> = it.collect();
            match tag { "crewrel" => h.crew_load_rel(&rest), "crew" => h.crew_load(&rest), _ => panic!("แท็กแปลก {}", tag) }
        }
        assert_eq!(h.rs.crew.rel, [12, -6]);
        assert_eq!(h.rs.crew.m.len(), 3);
        for (a, b) in g.rs.crew.m.iter().zip(h.rs.crew.m.iter()) {
            assert_eq!((a.id, a.kind, a.mode, a.loyalty, a.mood, a.t, a.fed_day, a.sad, a.post, a.down), (b.id, b.kind, b.mode, b.loyalty, b.mood, b.t, b.fed_day, b.sad, b.post, b.down));
            assert!((a.hp - b.hp).abs() < 0.6, "เลือดต้องจำได้ {} {}", a.hp, b.hp);
            assert!((a.x - b.x).abs() < 0.1 && (a.y - b.y).abs() < 0.1);
        }
        assert!(h.rs.crew.next >= 3, "ไม่ออก id ซ้ำ");
        // ข้อมูลเสีย/ไม่ครบ ไม่ crash
        h.crew_load(&["1", "2"]);
        assert_eq!(h.rs.crew.m.len(), 3);
    }
    #[test]
    fn yield_card_fits_on_all_screens() {
        for &(sw, sh) in &[(1280., 720.), (2400., 1080.), (1600., 720.), (2340., 1080.), (1920., 1200.), (1024., 600.), (2800., 1260.), (800., 450.)] {
            for touch in [true, false] {
                for top in [0., 42.] {
                    let u: f32 = (sh / 450f32).clamp(1., 3.);
                    let t = (sh / 450.).clamp(1., 3.2);
                    let l = hud::layout_for(sw, u, t, touch);
                    let (r, b) = yield_rects_for(&l, u, top * u);
                    assert!(r.x >= l.col_x - 0.5 && r.x + r.w <= l.col_x + l.col_w + 0.5, "อยู่ในโซนกลาง {}x{}", sw, sh);
                    assert!(!r.overlaps(&l.mm), "ไม่ทับแผนที่เล็ก {}x{}", sw, sh);
                    assert!(r.y + r.h < sh * 0.5, "ไม่บังจอเกินครึ่ง {}x{}", sw, sh);
                    for k in 0..4 {
                        assert!(b[k].x >= r.x && b[k].x + b[k].w <= r.x + r.w + 0.5 && b[k].y >= r.y && b[k].y + b[k].h <= r.y + r.h + 0.5, "ปุ่มอยู่ในการ์ด {}x{}", sw, sh);
                        assert!(b[k].w >= 44. * u.min(1.5) && b[k].h >= 28., "ปุ่มกดง่าย {}x{} {:?}", sw, sh, b[k]);
                        for q in k + 1..4 { assert!(!b[k].overlaps(&b[q]), "ปุ่มไม่ทับกัน"); }
                    }
                }
            }
        }
    }
    #[test]
    fn hud_hides_other_cards_while_boss_yields() {
        let mut g = game(3320);
        boss_near(&mut g, 0);
        g.damage_boss(0, 999, false);
        let v = View { s: 3., vw: 300., vh: 150., cx: g.p.x - 150., cy: g.p.y - 75. };
        assert!(wants::want_strip(&g, &v).is_none());
        assert!(g.permit_card().is_none());
    }
}
