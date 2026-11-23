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
// --- นิสัยบอส 4 แบบ (v33 เฟส 2)
pub const AR_HONOR: u8 = 0;
pub const AR_AMBI: u8 = 1;
pub const AR_PRIDE: u8 = 2;
pub const AR_FERAL: u8 = 3;
pub const AR_N: [&str; 4] = ["มีเกียรติ", "ทะเยอทะยาน", "หยิ่งทระนง", "ดุร้ายคลั่ง"];
pub const AR_D: [&str; 4] = [
    "ยอมเฉพาะคนที่ชนะอย่างมีเกียรติ (สู้ตัวต่อตัว) และเมืองที่น่าไปอยู่ · ไม่ทรยศ แต่ลาไปถ้าเมืองเสื่อม",
    "ยอมเข้าพวกถ้าเมืองใหญ่/กองทัพแกร่ง หรือได้บรรณาการมีค่า · ทรยศได้ถ้าเมืองอ่อนแอลง",
    "ไม่ยอมในครั้งแรก ต้องไว้ชีวิตก่อน พบกันครั้งหน้าจึงยอม · ภักดีช้า ทรยศได้ถ้าถูกทอดทิ้ง",
    "ไม่ยอมแพ้ ไม่เจรจา สู้จนตาย แต่ทิ้งของหายาก/แกนโกเลมไว้เป็นพิเศษ",
];
/// ชื่อเสียงเมือง/ความสุขชาวบ้านขั้นต่ำที่นักรบมีเกียรติต้องการ
pub const H_FAME: i32 = 40;
pub const H_MOOD: i32 = 60;
/// ทะเยอทะยาน: เมืองใหญ่ (ชื่อเสียง) หรือกองทัพแกร่ง (ป้องกัน) หรือบรรณาการ
pub const A_FAME: i32 = 80;
pub const A_DEF: f32 = 30.;
pub const A_TRIB: i32 = 6;
/// สัดส่วนนิสัยของบอสแต่ละชนิด [มีเกียรติ, ทะเยอทะยาน, หยิ่ง, ดุร้าย] เป็น % — ชนิด: 0 ราชาสไลม์ · 1 โกเลม · 2 แม่ทัพ
const AR_W: [[i32; 4]; 3] = [[25, 25, 15, 35], [35, 15, 30, 20], [25, 35, 25, 15]];
// สาเหตุที่ใจห่าง (บิต)
pub const C_FAMINE: u8 = 1;
pub const C_UNHAPPY: u8 = 2;
pub const C_WAR: u8 = 4;
pub const C_WEAK: u8 = 8;
pub const C_NEGLECT: u8 = 16;
pub const C_FORCED: u8 = 32;

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
    /// นิสัยบอส: 0 มีเกียรติ · 1 ทะเยอทะยาน · 2 หยิ่งทระนง
    pub arch: u8,
    /// สาเหตุที่ใจห่างจากเมื่อเช้า (บิต C_*) และเตือนไปแล้วหรือยัง
    pub cause: u8,
    pub warned: bool,
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
        let mut m = Member { id, kind, nm: (id % 3) as u8, x, y, post: (tfloor(x), tfloor(y)), mode: M_GUARD, loyalty: 100, mood: 100, t: [50; 6], arch: 0, fed_day: 0, sad: 0, hp: 1., down: false, cause: 0, warned: false, cd: 0., skill_cd: 0., hurt_cd: 0., taunt_t: 0., slam_t: 0., flash: 0., last_hit: 99., beam: (0., 0., 0.), wander: (x, y, 0.), anim: 0., stuck: 0., left: false };
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
/// ของโปรด (ให้ของขวัญได้ผลเต็ม) · ของรอง (ได้ครึ่งเดียว) — kind = ชนิดพวก 0 สไลม์ · 1 โกเลม · 2 แม่ทัพ
pub fn fav_of(kind: u8) -> usize { match kind { 1 => GEM, 2 => GEM, _ => BERRY } }
fn fav2_of(kind: u8) -> usize { match kind { 1 => STONE, 2 => DISH_STEW, _ => SHROOM } }
/// ชนิดบอส → ชนิดพวก
pub fn member_kind(boss_kind: u8) -> u8 { match boss_kind { 0 => K_SLIME, 1 => K_BGOLEM, _ => K_CHAMP } }
pub fn slot_of(kind: u8) -> usize { match kind { 0 => 0, 1 => 1, _ => 2 } }
/// บอสชนิดไหนยอมแพ้ได้ (ราชันย์เงาตัวสุดท้ายไม่ยอม)
pub fn yieldable(kind: u8) -> bool { matches!(kind, 0 | 1 | CHAMP) }
/// นิสัยของบอสตัวนี้ — คงที่ตามตำแหน่งที่เกิด (ตัวเดิม = นิสัยเดิม) · บอสสุดท้ายไม่มีนิสัยแบบนี้
pub fn boss_arch_for(seed: i32, kind: u8, x: f32, y: f32) -> u8 {
    if !yieldable(kind) { return AR_FERAL; }
    let r = hash(tfloor(x) + 31, tfloor(y) - 17, seed.wrapping_add(700 + kind as i32)) as f32 * 100.;
    let w = AR_W[slot_of(kind)];
    let mut acc = 0.;
    for a in 0..4 { acc += w[a] as f32; if r < acc { return a as u8; } }
    AR_FERAL
}

#[derive(Clone, Debug)]
pub struct Yield {
    /// ชนิดบอส (0 ราชาสไลม์ · 1 โกเลม · 3 แม่ทัพ)
    pub kind: u8,
    pub arch: u8,
    pub t: [i32; 6],
    /// ชนะอย่างมีเกียรติ (ไม่มีลูกน้อง/ทหารช่วยตี)
    pub fair: bool,
    /// บรรณาการที่ให้ไปแล้ว (มูลค่า: อัญมณี 3 · เศษคริสตัล 2) — ผลกับพวกทะเยอทะยาน
    pub tribute: i32,
    /// ของขวัญสะสม (เพิ่มความภักดีตั้งต้นเมื่อยอมเป็นพวก)
    pub gift: f32,
    pub gifts: i32,
    pub asks: i32,
    pub reply: String,
}
#[derive(Clone, Debug, Default)]
pub struct Crew {
    pub m: Vec<Member>,
    pub next: u32,
    /// เคยไว้ชีวิตพวกหยิ่งทระนงของบอสชนิดนั้นแล้ว (สไลม์ · โกเลม · แม่ทัพ) → ครั้งหน้ายอม
    pub spared: [bool; 3],
    /// วันที่แพ้ศึกล่าสุด (ใช้อธิบายสาเหตุที่ใจห่าง)
    pub lost_day: i32,
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
/// บุคลิกตั้งต้นของบอสชนิดนี้ ± สุ่มเล็กน้อยตามตำแหน่งที่เกิด (ไม่ซ้ำกันทุกตัว) — ใช้กำหนดว่าอยากเฝ้าบ้านหรือผจญภัย
pub fn boss_traits(seed: i32, kind: u8, x: f32, y: f32) -> [i32; 6] {
    let base: [i32; 6] = match kind { 0 => [72, 35, 70, 62, 45, 55], 1 => [22, 85, 20, 38, 12, 72], _ => [55, 70, 60, 35, 35, 45] };
    let (tx, ty) = (tfloor(x), tfloor(y));
    let mut t = base;
    for i in 0..6 { t[i] = (base[i] + ((hash(tx + i as i32 * 7, ty, seed.wrapping_add(900 + kind as i32)) - 0.5) * 30.) as i32).clamp(0, 100); }
    t
}
fn greet(arch: u8) -> &'static str {
    match arch {
        AR_HONOR => "สู้ได้สมศักดิ์ศรี... ข้ายอมรับว่าแพ้ ขอให้ข้าเลือกทางอย่างมีเกียรติ",
        AR_AMBI => "ข้าแพ้แล้ว... แต่ข้าอยากรู้ก่อนว่าเจ้ามีอะไรจะเสนอ",
        _ => "หึ! ข้าไม่ได้ศิโรราบต่อเจ้า แค่หมดแรงเท่านั้น!",
    }
}

/// สภาพเมืองตอนนี้ (ใช้ตัดสินใจทั้งตอนบอสยอม และตอนความภักดีขึ้นลงทุกเช้า)
#[derive(Clone, Debug)]
pub struct CityRead {
    pub pop: usize,
    pub mood: i32,
    pub hungry: bool,
    pub fame: i32,
    pub defense: f32,
}
/// เงื่อนไขที่บอสพิจารณา: (ผ่านไหม, ข้อความสั้น) — แสดงบนการ์ดทีละบรรทัด
pub fn yield_conds(g: &Game, y: &Yield) -> Vec<(bool, String)> {
    let c = g.city_read();
    match y.arch {
        AR_HONOR => vec![
            (y.fair, if y.fair { "สู้ตัวต่อตัว ไม่ใช้ลูกน้องช่วย".to_string() } else { "มีลูกน้อง/ทหารช่วยตี (ไม่ใช่ชัยชนะที่มีเกียรติ)".to_string() }),
            (c.fame >= H_FAME, format!("ชื่อเสียงเมืองดี {}/{}", c.fame, H_FAME)),
            (c.pop > 0 && c.mood >= H_MOOD && !c.hungry, format!("ชาวบ้านสุขใจ {}/{}{}", c.mood, H_MOOD, if c.hungry { " · มีคนหิว" } else { "" })),
        ],
        AR_AMBI => vec![
            (c.fame >= A_FAME || c.defense >= A_DEF, format!("เมืองใหญ่ {}/{} หรือกองทัพแกร่ง {}/{}", c.fame, A_FAME, c.defense.round() as i32, A_DEF as i32)),
            (y.tribute >= A_TRIB, format!("หรือบรรณาการมีค่า {}/{}", y.tribute, A_TRIB)),
        ],
        AR_PRIDE => vec![(g.rs.crew.spared[slot_of(y.kind)], "เคยไว้ชีวิตพวกหยิ่งเผ่านี้แล้ว (พบกันครั้งที่สอง)".to_string())],
        _ => vec![],
    }
}
/// ทะเยอทะยานผ่านข้อใดข้อหนึ่งก็พอ · นิสัยอื่นต้องผ่านครบ
pub fn yield_accepts(conds: &[(bool, String)], arch: u8) -> bool {
    if conds.is_empty() { return false; }
    if arch == AR_AMBI { conds.iter().any(|c| c.0) } else { conds.iter().all(|c| c.0) }
}
pub fn cause_text(c: u8) -> String {
    let mut v: Vec<&str> = vec![];
    if c & C_FAMINE != 0 { v.push("เมืองอดอยาก"); }
    if c & C_UNHAPPY != 0 { v.push("ชาวบ้านไม่สุขใจ"); }
    if c & C_WAR != 0 { v.push("เพิ่งแพ้ศึก"); }
    if c & C_WEAK != 0 { v.push("เมืองยังเล็ก/อ่อนแอ ไม่น่าเกรงขาม"); }
    if c & C_NEGLECT != 0 { v.push("ไม่ค่อยได้รับการดูแล"); }
    if c & C_FORCED != 0 { v.push("ถูกใช้งานผิดนิสัย"); }
    if v.is_empty() { "ไม่ได้รับความไว้ใจมากพอ".to_string() } else { v.join(" · ") }
}

impl Game {
    pub fn city_read(&self) -> CityRead {
        let t = self.town_stats();
        CityRead { pop: t.pop, mood: t.mood_avg, hungry: !t.hungry.is_empty() || (t.pop > 0 && t.food_days < 1.), fame: self.fame(), defense: self.defense() }
    }
    fn yield_boss_idx(&self) -> Option<usize> { self.bosses.iter().position(|b| b.yld && !b.dead) }
    /// บอสหมดแรง: ถ้ามีสติปัญญา (ไม่ดุร้ายคลั่ง) ยอมแพ้ให้เลือก ชวน·ให้ของ·ปล่อย·ปราบ (true = ยอมแพ้แทนที่จะตาย)
    pub fn boss_yield_try(&mut self, i: usize) -> bool {
        let b = &self.bosses[i];
        if !yieldable(b.kind) || b.arch == AR_FERAL || self.rs.crew.yld.is_some() { return false; }
        let (kind, arch, x, y, fair) = (b.kind, b.arch, b.x, b.y, !b.ally);
        let t = boss_traits(self.w.seed, kind, x, y);
        let name = war::boss_name(self, b);
        let b = &mut self.bosses[i];
        b.yld = true; b.hp = 1.; b.atk = 0; b.dash = 0.; b.burn = 0.;
        self.proj.clear();
        self.toast(format!("{} ({}) ยอมแพ้แล้ว! ดูเงื่อนไขของเขาแล้วเลือกว่าจะทำอย่างไร", name, AR_N[arch as usize]));
        play(&self.sfx.quest);
        self.rs.crew.yld = Some(Yield { kind, arch, t, fair, tribute: 0, gift: 0., gifts: 0, asks: 0, reply: greet(arch).to_string() });
        true
    }
    fn boss_gone(&mut self, i: usize) {
        self.bosses[i].dead = true;
        self.boss_cd = 240.;
        self.rs.crew.yld = None;
        if self.w.dun.is_some() { self.dungeon_boss_down(); }
    }
    /// แม่ทัพยอมแพ้/ถูกปล่อย/เข้าพวก = ฝ่ายศัตรูเสียขวัญเหมือนแม่ทัพล้ม (ไม่ได้ของรางวัลปราบ)
    fn champion_yielded(&mut self, b: &Boss) {
        self.rs.war.champs += 1;
        if self.rs.war.assault.is_some() { self.morale_add(-50.); self.toast("แม่ทัพยอมแพ้! ขวัญศัตรูดิ่งลง"); }
        let _ = b;
    }
    /// ชวนเป็นพวก — ตัดสินตามนิสัยของบอสและสภาพเมืองจริง ไม่ใช่ซื้อใจได้ทุกตัว
    pub fn yield_ask(&mut self) {
        let Some(i) = self.yield_boss_idx() else { self.rs.crew.yld = None; return };
        if self.rs.crew.m.len() >= CREW_MAX { self.toast("พวกเต็มแล้ว (4) — ปล่อยตัวเก่าที่แท็บบริวาร (กด N) ก่อน"); return; }
        let b = self.bosses[i].clone();
        let (kind, x, y) = (b.kind, b.x, b.y);
        let mut yl = self.rs.crew.yld.clone().unwrap();
        yl.asks += 1;
        let conds = yield_conds(self, &yl);
        if yield_accepts(&conds, yl.arch) {
            let loy = (45 + if yl.arch == AR_HONOR { 8 } else { 0 } + (yl.gift as i32).min(15)).clamp(35, 80);
            let line = match yl.arch {
                AR_HONOR => "เจ้าชนะอย่างมีเกียรติ และเมืองเจ้าน่าอยู่ — ข้ายินดีรับใช้",
                AR_AMBI => "เมืองของเจ้ายิ่งใหญ่ ข้าขอร่วมทางด้วย!",
                _ => "ครั้งก่อนเจ้าไว้ชีวิตข้า... ข้าจะไม่ลืมบุญคุณ",
            };
            self.bubble_at(x, y, line);
            self.crew_join_a(member_kind(kind), yl.arch, yl.t, loy, x, y);
            if kind == CHAMP { if let Some(m) = self.rs.crew.m.last_mut() { m.nm = b.k as u8; } }
            self.p.xp += 40;
            self.check_lv();
            self.burst(x, y - 8., hx(0xff6aa0), 24, 70.);
            play(&self.sfx.lvl);
            if kind == CHAMP { self.champion_yielded(&b); }
            self.boss_gone(i);
            self.toast(format!("{} ยอมมาเป็นพวกแล้ว! ดูแลมันที่แท็บบริวาร (กด N)", war::boss_name(self, &b)));
            self.save();
            return;
        }
        let c = self.city_read();
        yl.reply = match yl.arch {
            AR_HONOR if !yl.fair => "เจ้าใช้คนอื่นช่วยสู้ ข้าไม่นับว่าแพ้อย่างมีเกียรติ... สินบนก็ซื้อข้าไม่ได้".into(),
            AR_HONOR if c.fame < H_FAME => "ข้าไม่เห็นว่าเมืองของเจ้าเป็นที่ที่ควรไปอยู่... สร้างชื่อเสียงให้เมืองก่อน".into(),
            AR_HONOR => "ชาวบ้านของเจ้ายังไม่สุขใจ ข้าไม่อยากไปอยู่เมืองที่ผู้คนหิวหรือทุกข์".into(),
            AR_AMBI => "เมืองของเจ้ายังเล็กเกินไป ข้าไม่เห็นอนาคต... ถ้าไม่มีบรรณาการมีค่า ก็ไม่ต้องคุยกัน".into(),
            _ => "ข้าไม่ยอมเป็นลูกน้องใครง่ายๆ! ปล่อยข้าไปก่อน แล้วค่อยพบกันใหม่".into(),
        };
        if yl.asks >= 3 { yl.reply = format!("อย่าตื๊อข้าเลย! {}", yl.reply); }
        play(&self.sfx.deny);
        self.rs.crew.yld = Some(yl);
    }
    /// ให้ของ: พวกทะเยอทะยาน = บรรณาการ (อัญมณี/เศษคริสตัล ต้องถึงเกณฑ์) · นิสัยอื่น = ของขวัญเพิ่มความภักดีตั้งต้น (ไม่ทำให้เปลี่ยนใจ)
    pub fn yield_gift(&mut self) {
        let Some(i) = self.yield_boss_idx() else { self.rs.crew.yld = None; return };
        let kind = self.bosses[i].kind;
        let mut yl = self.rs.crew.yld.clone().unwrap();
        let ambi = yl.arch == AR_AMBI;
        let (f1, f2) = if ambi { (GEM, SHARD) } else { (fav_of(slot_of(kind) as u8), fav2_of(slot_of(kind) as u8)) };
        let (it, val) = if self.p.inv[f1] > 0 { (f1, if ambi { 3 } else { 14 }) } else if self.p.inv[f2] > 0 { (f2, if ambi { 2 } else { 7 }) } else {
            self.toast(format!("ไม่มี{}ให้มัน — {}", INAME[f1], source::item_howto(self, f1).first().map(|s| s.how.clone()).unwrap_or_default()));
            return;
        };
        if ambi { if yl.tribute >= A_TRIB { yl.reply = "พอแล้ว บรรณาการเท่านี้ก็เกินพอ... ถามข้าได้เลย".into(); self.rs.crew.yld = Some(yl); return; } }
        else if yl.gifts >= 3 { yl.reply = "พอแล้ว... ข้าไม่ได้อยากได้ของ ข้าอยากรู้ว่าเจ้าจริงใจไหม".into(); self.rs.crew.yld = Some(yl); return; }
        self.p.inv[it] -= 1;
        if ambi { yl.tribute += val; yl.reply = format!("ข้ารับ{}เป็นบรรณาการแล้ว ({}/{})", INAME[it], yl.tribute.min(A_TRIB), A_TRIB); }
        else {
            yl.gift += val as f32 * 0.6f32.powi(yl.gifts);
            yl.reply = match yl.arch {
                AR_HONOR => format!("ขอบใจสำหรับ{} แต่เกียรติของข้าซื้อด้วยของไม่ได้", INAME[it]),
                _ => format!("หึ... {} ข้ารับไว้ แต่ข้าไม่ได้ศิโรราบเพราะของพวกนี้", INAME[it]),
            };
        }
        yl.gifts += 1;
        play(&self.sfx.pickup);
        let (x, y) = (self.bosses[i].x, self.bosses[i].y);
        self.burst(x, y - 10., hx(0xffd84a), 8, 40.);
        self.rs.crew.yld = Some(yl);
    }
    /// ปล่อยไป (ไว้ชีวิต): ของขอบคุณเล็กน้อย · พวกหยิ่งจะจำน้ำใจไว้ พบกันครั้งหน้ายอม
    pub fn yield_spare(&mut self) {
        let Some(i) = self.yield_boss_idx() else { self.rs.crew.yld = None; return };
        let b = self.bosses[i].clone();
        let (kind, x, y) = (b.kind, b.x, b.y);
        match kind { 0 => self.gain(GEL, 1, x, y - 8.), 1 => self.gain(STONE, 3, x, y - 8.), _ => self.gain(COIN, 15, x, y - 8.) }
        let pride = self.rs.crew.yld.as_ref().map_or(false, |y| y.arch == AR_PRIDE);
        if pride { self.rs.crew.spared[slot_of(kind)] = true; }
        self.bubble_at(x, y, if pride { "...ข้าจะจำน้ำใจนี้ไว้" } else { "ขอบใจที่ไว้ชีวิต" });
        self.p.xp += 10;
        self.check_lv();
        self.burst(x, y - 8., hx(0x9ad8ff), 16, 60.);
        let nm = war::boss_name(self, &b);
        self.toast(if pride { format!("ไว้ชีวิต{} — พวกหยิ่งเผ่านี้จะจำน้ำใจ พบกันครั้งหน้าอาจยอมเข้าพวก", nm) } else { format!("ไว้ชีวิต{}", nm) });
        if kind == CHAMP { self.champion_yielded(&b); }
        self.boss_gone(i);
        self.save();
    }
    /// ปราบทิ้ง (ได้ของหายากตามเดิม)
    pub fn yield_slay(&mut self) {
        let Some(i) = self.yield_boss_idx() else { self.rs.crew.yld = None; return };
        self.rs.crew.yld = None;
        self.bosses[i].yld = false;
        self.bosses[i].dead = true;
        let b = self.bosses[i].clone();
        self.boss_die(&b);
    }
    /// พวกดุร้ายคลั่งตายแล้วทิ้งของหายากเป็นพิเศษ (แกนโกเลม/เศษคริสตัล/อัญมณี)
    pub fn feral_loot(&mut self, b: &Boss) {
        if b.arch != AR_FERAL || !yieldable(b.kind) { return; }
        let (x, y) = (b.x, b.y - 22.);
        match b.kind {
            0 => { self.gain(CORE, 1, x, y); self.gain(GEM, rnd(2, 3), x, y - 10.); }
            1 => { self.gain(CORE, rnd(2, 3), x, y); self.gain(SHARD, rnd(2, 3), x, y - 10.); }
            _ => { self.gain(CORE, 1, x, y); self.gain(SHARD, 2, x, y - 10.); self.gain(GEM, 3, x, y - 20.); }
        }
        self.toast(format!("{} ดุร้ายคลั่ง ไม่ยอมแพ้ — ทิ้งของหายากไว้เป็นพิเศษ", war::boss_name(self, b)));
    }
    /// ข้อความ bubble ลอยเหนือหัว (ใช้ float)
    fn bubble_at(&mut self, x: f32, y: f32, s: &str) { self.add_float_sz(x, y - 34., s.to_string(), hx(0xffe9a0), 1.0); }

    // ======================= พวก / บริวาร =======================
    pub fn crew_join(&mut self, kind: u8, t: [i32; 6], loyalty: i32, x: f32, y: f32) { self.crew_join_a(kind, AR_HONOR, t, loyalty, x, y); }
    pub fn crew_join_a(&mut self, kind: u8, arch: u8, t: [i32; 6], loyalty: i32, x: f32, y: f32) {
        let id = self.rs.crew.next; self.rs.crew.next += 1;
        let mut m = Member::new(id, kind, x, y);
        m.t = t; m.loyalty = loyalty; m.mood = 70; m.fed_day = day_of(self); m.arch = arch.min(2);
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
    /// ชนะ/แพ้ศึก: พวกที่มีใจดีใจหรือเสียกำลังใจ (ทะเยอทะยานรักผู้ชนะ เกลียดผู้แพ้ที่สุด)
    pub fn crew_battle(&mut self, won: bool) {
        let day = day_of(self);
        if !won { self.rs.crew.lost_day = day.max(1); }
        let mut names: Vec<String> = vec![];
        for m in self.rs.crew.m.iter_mut().filter(|m| m.mind()) {
            let a = m.arch.min(2) as usize;
            if won { m.loyalty = (m.loyalty + [3, 6, 2][a]).min(100); m.mood = (m.mood + 5).min(100); }
            else { m.loyalty = (m.loyalty - [4, 10, 8][a]).max(0); m.cause |= C_WAR; names.push(m.name()); }
        }
        if !won && !names.is_empty() { self.toast(format!("แพ้ศึก! {} เสียกำลังใจ — ความภักดีลดลง", names.join(", "))); }
    }
    pub fn tip_text(c: u8) -> &'static str {
        if c & C_FAMINE != 0 { "แจกจ่ายอาหารให้ชาวบ้านก่อน" }
        else if c & C_UNHAPPY != 0 { "ทำให้ชาวบ้านสุขใจ (อาหาร เตียง ความสะดวกสบาย)" }
        else if c & C_WAR != 0 { "เสริมกำลังป้องกันเมือง แล้วชนะศึกครั้งหน้า" }
        else if c & C_WEAK != 0 { "ขยายเมือง/เสริมกองทัพให้น่าเกรงขาม" }
        else if c & C_FORCED != 0 { "ให้ทำหน้าที่ที่ตัวเองอยากทำ" }
        else { "ให้ของโปรดหรือให้พักผ่อน" }
    }
    /// เช้าวันใหม่: โกเลม/หุ่นซ่อมตัวเอง · ตัวที่มีใจ — ความภักดีขึ้นลงตามสภาพเมือง (อดอยาก/ไม่สุข/แพ้ศึก/เมืองอ่อนแอ)
    /// ใจห่าง → เตือนล่วงหน้า 2 ขั้น → มีเกียรติลาไปอย่างสงบ · ทะเยอทะยาน/หยิ่งทรยศกลับมาเป็นศัตรู
    pub fn crew_dawn(&mut self) {
        let day = day_of(self);
        let c = self.city_read();
        let famine = c.pop > 0 && c.hungry;
        let unhappy = c.pop > 0 && c.mood < 40;
        let thriving = c.pop >= 3 && c.mood >= 65 && !c.hungry;
        let big = c.fame >= A_FAME || c.defense >= A_DEF;
        let lost = self.rs.crew.lost_day > 0 && day - self.rs.crew.lost_day <= 1;
        let indun = self.w.dun.is_some();
        let mut out: Vec<(u32, bool)> = vec![];
        let mut notes: Vec<String> = vec![];
        for m in self.rs.crew.m.iter_mut() {
            if m.down { m.down = false; m.hp = (m.maxhp() * 0.6).max(m.hp); }
            if !m.mind() { m.hp = m.maxhp(); continue; }
            let a = m.arch.min(2) as usize;
            let (mut d, mut cause) = (0i32, 0u8);
            if thriving { d += 2; }
            if famine { d -= [2, 4, 4][a]; cause |= C_FAMINE; }
            if unhappy { d -= [4, 2, 3][a]; cause |= C_UNHAPPY; }
            if lost { cause |= C_WAR; }
            if m.arch == AR_AMBI { if big { d += 1; } else { d -= 3; cause |= C_WEAK; } }
            if day - m.fed_day > 3 { m.mood = (m.mood - 10).max(0); } else { m.mood = (m.mood + 3).min(100); }
            if m.mode == M_ROAM { m.mood = (m.mood + 5).min(100); d += 1; }
            else if m.mode != m.wished() { d -= 2; m.mood = (m.mood - 3).max(0); cause |= C_FORCED; }
            if m.mood >= 60 { d += 1; } else if m.mood < 30 { d -= 2; cause |= C_NEGLECT; }
            if m.arch == AR_PRIDE && d > 0 { d = (d + 1) / 2; }
            m.loyalty = (m.loyalty + d).clamp(0, 100);
            m.cause = cause;
            if m.loyalty >= 55 { m.warned = false; m.sad = 0; continue; }
            let betray = m.arch != AR_HONOR;
            if m.loyalty < 40 && !m.warned {
                m.warned = true;
                notes.push(format!("{} ใจเริ่มห่าง: {} — {}", m.name(), cause_text(cause), Self::tip_text(cause)));
            }
            if m.loyalty < 22 {
                m.sad += 1;
                if m.sad >= 2 && indun { m.sad = 1; }
                else if m.sad == 1 { notes.push(format!("{} จะ{}แล้ว! ถ้าพรุ่งนี้ความภักดียังต่ำ ({})", m.name(), if betray { "ทรยศ" } else { "ลาไป" }, Self::tip_text(cause))); }
                else { out.push((m.id, betray)); }
            } else { m.sad = 0; }
        }
        for n in notes { self.toast(n); }
        for (id, betray) in out {
            let Some(i) = self.rs.crew.m.iter().position(|m| m.id == id) else { continue };
            let m = self.rs.crew.m.remove(i);
            if betray { self.crew_betray(&m); }
            else { self.toast(format!("{} ลาไปอย่างสงบ... เมืองนี้ไม่น่าอยู่แล้ว ({})", m.name(), cause_text(m.cause))); }
        }
    }
    /// ทรยศ: กลับมาเป็นบอสศัตรูที่แค้น (ดุร้ายคลั่ง ไม่ยอมแพ้อีก) ใกล้ตัวผู้เล่น
    fn crew_betray(&mut self, m: &Member) {
        let kind = match m.kind { K_SLIME => 0, K_BGOLEM => 1, _ => CHAMP };
        let (px, py) = (self.p.x, self.p.y);
        let spot = (0..24).map(|i| { let a = i as f32 * 0.9; (px + a.cos() * 120., py + a.sin() * 120.) }).find(|&(x, y)| self.walkable(x, y)).unwrap_or((px + 40., py));
        let mul = 1. + (spot.0 - self.spawn.0).hypot(spot.1 - self.spawn.1) / 16. / 400.;
        let hp = (BHP[kind as usize] * mul * 0.8).ceil();
        self.spawn_boss_hp(kind, spot.0, spot.1, hp);
        if let Some(b) = self.bosses.last_mut() {
            b.arch = AR_FERAL;
            if kind == CHAMP { b.k = (m.nm as usize).min(self.w.realm.ks.len().saturating_sub(1)); b.cd = 1.2; }
        }
        if kind == CHAMP { let k = self.bosses.last().map_or(0, |b| b.k); let st = war::champ_style_calc(self, k); if let Some(b) = self.bosses.last_mut() { b.style = st; } }
        self.burst(spot.0, spot.1 - 10., hx(0xff4a4a), 24, 90.);
        self.shake(2.0, 0.4);
        self.toast(format!("ทรยศ! {} หมดศรัทธา กลับมาเป็นศัตรู ({})", m.name(), cause_text(m.cause)));
    }

    // ======================= เซฟ/โหลด =======================
    pub fn crew_save(&self) -> String {
        let c = &self.rs.crew;
        let mut s = format!("crewst {} {} {} {} {}\n", c.next, c.spared[0] as u8, c.spared[1] as u8, c.spared[2] as u8, c.lost_day);
        for m in &c.m {
            s += &format!("crew {} {} {} {:.1} {:.1} {} {} {} {} {} {} {} {} {} {} {} {} {:.0} {} {} {} {}\n", m.id, m.kind, m.nm, m.x, m.y, m.post.0, m.post.1, m.mode, m.loyalty, m.mood, m.t[0], m.t[1], m.t[2], m.t[3], m.t[4], m.t[5], m.fed_day * 10 + m.sad.min(9), m.hp, m.down as u8, m.arch, m.cause, m.warned as u8);
        }
        s
    }
    /// บรรทัดเซฟรุ่นก่อน (crewrel r0 r1 next) — เอาแค่เลขถัดไป
    pub fn crew_load_rel(&mut self, r: &[&str]) {
        let next = r.get(2).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0).max(0) as u32;
        if self.rs.crew.next < next { self.rs.crew.next = next; }
    }
    pub fn crew_load_st(&mut self, r: &[&str]) {
        let n = |i: usize| r.get(i).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
        self.rs.crew.next = n(0).max(0) as u32;
        self.rs.crew.spared = [n(1) != 0, n(2) != 0, n(3) != 0];
        self.rs.crew.lost_day = n(4);
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
        if r.len() > 20 { m.cause = n(20) as u8; }
        if r.len() > 21 { m.warned = n(21) != 0; }
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
        let h = 156. * u;
        let bw = (w - 16. * u - gap * 3.) / 4.;
        let r = |i: usize| Rect::new(x + 8. * u + i as f32 * (bw + gap), y + h - bh - 8. * u, bw, bh);
        (Rect::new(x, y, w, h), [r(0), r(1), r(2), r(3)])
    } else {
        let h = 164. * u;
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
    let acol = match yl.arch { AR_HONOR => hx(0xffd866), AR_AMBI => hx(0x7ad8ff), _ => hx(0xff9a66) };
    draw_rectangle(r.x, r.y, r.w, r.h, Color::new(0.07, 0.05, 0.1, 0.94));
    draw_rectangle(r.x, r.y, 5. * u, r.h, acol);
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., Color::new(acol.r, acol.g, acol.b, 0.9));
    let wide = (b[1].y - b[0].y).abs() < 1.;
    let tw = r.w - 24. * u;
    let nm = g.bosses.iter().find(|q| q.yld && !q.dead).map(|q| war::boss_name(g, q)).unwrap_or_else(|| BNAME[yl.kind as usize].to_string());
    text(font, &fit_text_c(font, &format!("{} ยอมแพ้แล้ว · นิสัย{}", nm, AR_N[yl.arch as usize]), 13. * u, tw), r.x + 12. * u, r.y + 18. * u, 13. * u, acol);
    // เงื่อนไขที่เขาพิจารณา: จุดเขียว = ผ่านแล้ว · จุดแดง = ยังไม่ผ่าน (อ่านสถานะเมืองสดๆ)
    let conds = yield_conds(g, yl);
    let lh = 13. * u;
    for (j, (ok, t)) in conds.iter().enumerate() {
        let cy = r.y + 32. * u + j as f32 * lh;
        draw_rectangle(r.x + 12. * u, cy + 1. * u, 7. * u, 7. * u, if *ok { hx(0x6ad06a) } else { hx(0xd05a5a) });
        text(font, &fit_text_c(font, t, 11. * u, tw - 16. * u), r.x + 24. * u, cy + 8. * u, 11. * u, if *ok { hx(0xcfeccf) } else { hx(0xf0c8c8) });
    }
    let ry = r.y + 32. * u + conds.len().max(1) as f32 * lh + 8. * u;
    let ws = wrap_th(font, &format!("\"{}\"", yl.reply), 11.5 * u, tw);
    for (k, l) in ws.iter().take(2).enumerate() { text(font, l, r.x + 12. * u, ry + k as f32 * 13. * u, 11.5 * u, WHITE); }
    let ambi = yl.arch == AR_AMBI;
    let fav = if ambi { GEM } else { fav_of(slot_of(yl.kind) as u8) };
    let fav2 = if ambi { SHARD } else { fav2_of(slot_of(yl.kind) as u8) };
    let labels = ["ชวนเป็นพวก".to_string(), if ambi { format!("บรรณาการ{}", INAME[fav]) } else { format!("ให้{}", INAME[fav]) }, "ปล่อยไป".to_string(), "ปราบทิ้ง".to_string()];
    let ok = yield_accepts(&conds, yl.arch);
    let cols = [if ok { hx(0x2f9a3a) } else { hx(0x2f5a3a) }, hx(0x7a5a2f), hx(0x2f4f7a), hx(0x7a2f2f)];
    for k in 0..4 {
        let q = b[k];
        let dim = k == 1 && g.p.inv[fav] <= 0 && g.p.inv[fav2] <= 0;
        draw_rectangle(q.x, q.y, q.w, q.h, if dim { hx(0x3a3a44) } else { cols[k] });
        draw_rectangle_lines(q.x, q.y, q.w, q.h, 2., Color::new(1., 1., 1., if k == 0 && ok { 0.95 } else { 0.55 }));
        text_c(font, &fit_text_c(font, &labels[k], 12. * u, q.w - 6. * u), q.x + q.w / 2., q.y + q.h * 0.66, 12. * u, if dim { hx(0xaaaaaa) } else { WHITE });
    }
    let _ = wide;
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
    /// เมืองทดสอบ: pop คน อารมณ์ mood · fed = มีอาหารในหีบ · fame_wins = ชนะศึกกี่ครั้ง (ครั้งละ +15 ชื่อเสียง)
    fn town(g: &mut Game, pop: usize, mood: i32, fed: bool, wins: i32) {
        g.npcs.clear(); g.chests.clear();
        let (sx, sy) = (tfloor(g.p.x), tfloor(g.p.y));
        for i in 0..pop {
            let b = (sx + 2 + i as i32 * 2, sy + 4);
            g.npcs.push(Npc { id: 100 + i as u32, name: (i % 12) as u8, look: 0, x: 0., y: 0., state: ST_HOME, hearts: 3, mood, married: false, child: false, role: 0, bed: b, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(100 + i as u32, (i % 12) as u8) });
        }
        if fed { let mut ch = [0; NI]; ch[BERRY] = 40 + pop as i32 * 4; g.chests.insert((sx + 1, sy + 4), ch); }
        g.rs.siege_wins = wins;
    }
    fn good_town(g: &mut Game) { town(g, 6, 75, true, 3); }
    /// บอสยอมแพ้ที่ข้างตัว ด้วยนิสัยที่กำหนด
    fn yielded(g: &mut Game, kind: u8, arch: u8) {
        g.bosses.clear(); g.rs.crew.yld = None;
        let (x, y) = (g.p.x + 40., g.p.y);
        g.spawn_boss_hp(kind, x, y, 10.);
        g.bosses[0].arch = arch;
        g.damage_boss(0, 999, false);
    }
    fn reply(g: &Game) -> String { g.rs.crew.yld.as_ref().map(|y| y.reply.clone()).unwrap_or_default() }

    #[test]
    fn archetypes_are_stable_and_cover_all_four() {
        for kind in [0u8, 1, CHAMP] {
            let mut cnt = [0usize; 4];
            for i in 0..600 { cnt[boss_arch_for(7, kind, i as f32 * 53., (i / 7) as f32 * 61.) as usize] += 1; }
            assert!(cnt.iter().all(|&c| c > 20), "ทุกนิสัยมีโอกาสเกิด {:?} kind {}", cnt, kind);
            assert_eq!(boss_arch_for(7, kind, 320., 640.), boss_arch_for(7, kind, 320., 640.), "ตำแหน่งเดิม = นิสัยเดิม");
        }
        assert_eq!(boss_arch_for(7, FINAL_KIND, 1., 1.), AR_FERAL, "ราชันย์เงาไม่ยอมแพ้");
        assert!(!yieldable(FINAL_KIND) && yieldable(0) && yieldable(1) && yieldable(CHAMP));
        let mut g = game(3300);
        g.bosses.clear();
        g.spawn_boss_hp(0, g.p.x + 40., g.p.y, 10.);
        assert_eq!(g.bosses[0].arch, boss_arch_for(g.w.seed, 0, g.p.x + 40., g.p.y), "บอสที่เกิดได้นิสัยตามตำแหน่ง");
        let (name, tag, _, _) = war::boss_bar_info(&g).unwrap();
        assert!(!name.is_empty() && tag.contains("นิสัย:") && tag.contains(AR_N[g.bosses[0].arch as usize]), "แถบเลือดบอกนิสัยล่วงหน้า {}", tag);
        let t = boss_traits(11, 1, 320., 640.);
        assert_eq!(t, boss_traits(11, 1, 320., 640.));
        assert!(!trait_words(&t).is_empty() && t.iter().all(|v| (0..=100).contains(v)));
    }
    #[test]
    fn non_feral_boss_yields_instead_of_dying() {
        let mut g = game(3301);
        yielded(&mut g, 0, AR_HONOR);
        let (bx, by) = (g.bosses[0].x, g.bosses[0].y);
        assert!(g.bosses[0].yld && !g.bosses[0].dead);
        assert_eq!(g.bosses[0].hp, 1.);
        assert!(g.rs.crew.yld.is_some());
        g.damage_boss(0, 999, false);
        assert!(!g.bosses[0].dead, "ตีซ้ำไม่ได้ผล");
        let hp = g.p.hp;
        for _ in 0..120 { g.upd_bosses(1. / 30.); }
        assert_eq!((g.bosses[0].x, g.bosses[0].y), (bx, by));
        assert_eq!((g.p.hp, g.bosses[0].atk), (hp, 0), "ยอมแพ้แล้วหยุดสู้");
    }
    #[test]
    fn feral_boss_never_yields_and_drops_extra_rare_loot() {
        let mut g = game(3302);
        g.bosses.clear();
        g.spawn_boss_hp(0, g.p.x + 40., g.p.y, 5.);
        g.bosses[0].arch = AR_FERAL;
        let (core, gem) = (g.p.inv[CORE], g.p.inv[GEM]);
        g.damage_boss(0, 999, false);
        assert!(g.bosses[0].dead && g.rs.crew.yld.is_none() && !g.bosses[0].yld, "ดุร้ายคลั่ง = ตายเลย ไม่มีการ์ดยอมแพ้");
        assert!(g.p.inv[CORE] > core, "ราชาสไลม์ปกติไม่ทิ้งแกน แต่ตัวดุร้ายทิ้ง");
        assert!(g.p.inv[GEM] >= gem + 3, "อัญมณีปกติ 1-3 + โบนัส 2-3");
        // โกเลมดุร้าย: แกนมากกว่าปกติ + เศษคริสตัล
        let mut h = game(3303);
        h.bosses.clear();
        h.spawn_boss_hp(1, h.p.x + 40., h.p.y, 5.);
        h.bosses[0].arch = AR_FERAL;
        let (c0, s0) = (h.p.inv[CORE], h.p.inv[SHARD]);
        h.damage_boss(0, 999, false);
        assert!(h.p.inv[CORE] >= c0 + 4 && h.p.inv[SHARD] >= s0 + 2, "แกน 2-4 + โบนัส 2-3, เศษคริสตัล 2-3");
        // บอสสุดท้ายตายตามปกติ ไม่มีโบนัสนี้
        let mut f = game(3304);
        f.bosses.clear();
        f.spawn_boss_hp(FINAL_KIND, f.p.x + 40., f.p.y, 5.);
        f.damage_boss(0, 999, false);
        assert!(f.bosses[0].dead && f.rs.crew.yld.is_none());
    }
    #[test]
    fn honorable_needs_fair_win_and_a_good_city() {
        // ชนะตัวต่อตัว + เมืองดี = ยอม
        let mut g = game(3305);
        good_town(&mut g);
        yielded(&mut g, 0, AR_HONOR);
        let conds = yield_conds(&g, g.rs.crew.yld.as_ref().unwrap());
        assert_eq!(conds.len(), 3);
        assert!(conds.iter().all(|c| c.0), "เงื่อนไขครบ {:?}", conds);
        g.yield_ask();
        assert_eq!(g.rs.crew.m.len(), 1);
        assert_eq!((g.rs.crew.m[0].kind, g.rs.crew.m[0].arch), (K_SLIME, AR_HONOR));
        assert!(g.bosses.iter().all(|b| b.dead) && g.rs.crew.yld.is_none());
        // มีลูกน้องช่วยตี = ไม่ใช่ชัยชนะที่มีเกียรติ แม้เมืองดี และให้ของก็ซื้อไม่ได้
        let mut g = game(3306);
        good_town(&mut g);
        g.bosses.clear();
        g.spawn_boss_hp(0, g.p.x + 40., g.p.y, 10.);
        g.bosses[0].arch = AR_HONOR;
        g.bosses[0].ally = true;
        g.damage_boss(0, 999, false);
        assert!(!g.rs.crew.yld.as_ref().unwrap().fair);
        g.p.inv[BERRY] = 9; g.p.inv[GEM] = 9;
        g.yield_gift(); g.yield_gift();
        g.yield_ask();
        assert!(g.rs.crew.m.is_empty() && g.rs.crew.yld.is_some(), "ไม่ใช่ชัยชนะที่มีเกียรติ ซื้อใจไม่ได้");
        assert!(reply(&g).contains("ช่วย"), "บอกเหตุผล: {}", reply(&g));
        // เมืองหิว/ไม่สุข = ไม่ยอม
        let mut g = game(3307);
        town(&mut g, 6, 75, false, 3);
        yielded(&mut g, 0, AR_HONOR);
        g.yield_ask();
        assert!(g.rs.crew.m.is_empty() && reply(&g).contains("ไม่สุขใจ"), "{}", reply(&g));
        let mut g = game(3308);
        town(&mut g, 6, 30, true, 3);
        yielded(&mut g, 0, AR_HONOR);
        g.yield_ask();
        assert!(g.rs.crew.m.is_empty(), "ชาวบ้านไม่สุข");
        let mut g = game(3309);
        town(&mut g, 2, 75, true, 0);
        yielded(&mut g, 0, AR_HONOR);
        g.yield_ask();
        assert!(g.rs.crew.m.is_empty() && reply(&g).contains("ชื่อเสียง"), "เมืองยังไม่มีชื่อเสียง {}", reply(&g));
    }
    #[test]
    fn ally_damage_marks_the_boss() {
        let mut g = game(3310);
        g.bosses.clear();
        g.spawn_boss_hp(1, g.p.x + 40., g.p.y, 50.);
        assert!(!g.bosses[0].ally);
        g.hit_boss(0, 3);
        assert!(!g.bosses[0].ally, "ผู้เล่นตีเอง ไม่นับว่ามีผู้ช่วย");
        g.bosses[0].ally = true;
        assert!(g.bosses[0].ally);
    }
    #[test]
    fn ambitious_joins_for_a_big_city_or_tribute_only() {
        // เมืองเล็ก ไม่มีบรรณาการ = ปฏิเสธ
        let mut g = game(3311);
        town(&mut g, 2, 60, true, 0);
        yielded(&mut g, 1, AR_AMBI);
        g.yield_ask();
        assert!(g.rs.crew.m.is_empty() && reply(&g).contains("เล็ก"), "{}", reply(&g));
        // บรรณาการ: อัญมณี 3 + เศษคริสตัล 2 ยังไม่ถึง 6
        g.p.inv[GEM] = 1; g.p.inv[SHARD] = 1;
        g.yield_gift();
        g.yield_gift();
        assert_eq!((g.p.inv[GEM], g.p.inv[SHARD]), (0, 0));
        assert_eq!(g.rs.crew.yld.as_ref().unwrap().tribute, 5);
        g.yield_ask();
        assert!(g.rs.crew.m.is_empty(), "5/6 ยังไม่พอ");
        g.p.inv[SHARD] = 1;
        g.yield_gift();
        g.yield_ask();
        assert_eq!(g.rs.crew.m.len(), 1, "ครบ 6 = ยอม");
        assert_eq!((g.rs.crew.m[0].kind, g.rs.crew.m[0].arch), (K_BGOLEM, AR_AMBI));
        // เมืองใหญ่ (ชื่อเสียงสูง) = ยอมโดยไม่ต้องจ่ายอะไร
        let mut g = game(3312);
        town(&mut g, 6, 60, true, 4);
        assert!(g.fame() >= A_FAME, "fame {}", g.fame());
        yielded(&mut g, 0, AR_AMBI);
        g.yield_ask();
        assert_eq!(g.rs.crew.m.len(), 1, "เมืองใหญ่ ไม่ต้องบรรณาการ");
        // ไม่มีอัญมณี = บอกว่าหาจากไหน
        let mut g = game(3313);
        yielded(&mut g, 0, AR_AMBI);
        g.p.inv[GEM] = 0; g.p.inv[SHARD] = 0;
        g.toasts.clear();
        g.yield_gift();
        assert_eq!(g.rs.crew.yld.as_ref().unwrap().tribute, 0);
        assert!(g.toasts.iter().any(|t| t.0.contains("ไม่มี")), "บอกที่มาของอัญมณี");
    }
    #[test]
    fn stubborn_refuses_first_meeting_then_accepts_after_being_spared() {
        let mut g = game(3314);
        yielded(&mut g, 0, AR_PRIDE);
        g.p.inv[BERRY] = 9;
        g.yield_gift(); g.yield_gift();
        g.yield_ask();
        assert!(g.rs.crew.m.is_empty() && reply(&g).contains("ปล่อย"), "ครั้งแรก ไม่ยอม ของก็ซื้อไม่ได้: {}", reply(&g));
        assert!(!g.rs.crew.spared[0]);
        g.yield_spare();
        assert!(g.rs.crew.spared[0] && !g.rs.crew.spared[1], "จำน้ำใจ เฉพาะเผ่าเดียวกัน");
        assert!(g.bosses.iter().all(|b| b.dead) && g.rs.crew.yld.is_none());
        // พบกันครั้งที่สอง ยอมเข้าพวก
        yielded(&mut g, 0, AR_PRIDE);
        let conds = yield_conds(&g, g.rs.crew.yld.as_ref().unwrap());
        assert!(conds.len() == 1 && conds[0].0);
        g.yield_ask();
        assert_eq!(g.rs.crew.m.len(), 1);
        assert_eq!(g.rs.crew.m[0].arch, AR_PRIDE);
        // โกเลมหยิ่งเป็นอีกเผ่า: ยังไม่เคยไว้ชีวิต = ยังไม่ยอม
        yielded(&mut g, 1, AR_PRIDE);
        g.yield_ask();
        assert_eq!(g.rs.crew.m.len(), 1);
        // ไว้ชีวิตบอสนิสัยอื่น ไม่ทำให้พวกหยิ่งเปลี่ยนใจ
        let mut h = game(3315);
        yielded(&mut h, 1, AR_HONOR);
        h.yield_spare();
        assert_eq!(h.rs.crew.spared, [false; 3]);
    }
    #[test]
    fn spare_and_slay_rewards() {
        let mut g = game(3316);
        yielded(&mut g, 1, AR_HONOR);
        let st = g.p.inv[STONE];
        g.yield_spare();
        assert!(g.p.inv[STONE] > st && g.boss_cd > 0.);
        yielded(&mut g, 1, AR_AMBI);
        let (kills, core) = (g.p.kills, g.p.inv[CORE]);
        g.yield_slay();
        assert!(g.bosses.iter().all(|b| b.dead) && g.rs.crew.yld.is_none());
        assert_eq!(g.p.kills, kills + 1);
        assert!(g.p.inv[CORE] > core, "ปราบทิ้งยังได้ของหายาก");
    }
    #[test]
    fn crew_is_capped_at_four() {
        let mut g = game(3317);
        good_town(&mut g);
        for _ in 0..CREW_MAX { g.crew_join(K_SLIME, [50; 6], 60, g.p.x, g.p.y); }
        yielded(&mut g, 0, AR_HONOR);
        g.yield_ask();
        assert_eq!(g.rs.crew.m.len(), CREW_MAX);
        assert!(g.rs.crew.yld.is_some(), "พวกเต็ม: ยังไม่ตัดสินใจ รอให้เลือกใหม่");
    }
    #[test]
    fn yield_card_vanishes_if_boss_gone() {
        let mut g = game(3318);
        yielded(&mut g, 0, AR_HONOR);
        g.bosses.clear();
        g.crew_tick(0.1);
        assert!(g.rs.crew.yld.is_none());
    }
    #[test]
    fn champion_can_yield_join_and_final_boss_cannot() {
        let mut g = game(3319);
        let r = g.w.realm.clone(); g.rs.init(&r);
        good_town(&mut g);
        g.bosses.clear();
        g.spawn_champion(0, g.p.x + 40., g.p.y);
        g.bosses[0].arch = AR_AMBI;
        g.p.inv[GEM] = 2;
        g.damage_boss(0, 99999, false);
        assert!(g.bosses[0].yld && !g.bosses[0].dead, "แม่ทัพยอมแพ้ได้");
        let champs = g.rs.war.champs;
        g.yield_gift(); g.yield_gift();
        g.yield_ask();
        assert_eq!(g.rs.crew.m.len(), 1);
        assert_eq!((g.rs.crew.m[0].kind, g.rs.crew.m[0].nm), (K_CHAMP, 0), "แม่ทัพเข้าพวก จำอาณาจักรเดิม");
        assert!(g.bosses[0].dead && g.rs.war.champs == champs + 1, "นับเป็นแม่ทัพพ่ายแล้ว ศึกเดินต่อได้");
        // ราชันย์เงาไม่มีวันยอม
        g.bosses.clear();
        g.spawn_boss_hp(FINAL_KIND, g.p.x + 40., g.p.y, 5.);
        g.damage_boss(0, 999, false);
        assert!(g.bosses[0].dead && g.rs.crew.yld.is_none());
    }

    // ---------- ความภักดี: ขึ้นลงตามสภาพเมือง ----------
    fn member(g: &mut Game, kind: u8, arch: u8, loy: i32) -> u32 {
        g.crew_join_a(kind, arch, [50, 50, 50, 50, 50, 50], loy, g.p.x, g.p.y);
        let i = g.rs.crew.m.len() - 1;
        g.rs.crew.m[i].mode = g.rs.crew.m[i].wished();
        g.rs.crew.m[i].id
    }
    fn dawns(g: &mut Game, n: usize) {
        for _ in 0..n {
            g.time += DAYLEN;
            let d = day_of(g);
            for m in g.rs.crew.m.iter_mut() { m.fed_day = d; }
            g.crew_dawn();
        }
    }
    fn loy(g: &Game, id: u32) -> i32 { g.rs.crew.m.iter().find(|m| m.id == id).map_or(-1, |m| m.loyalty) }

    #[test]
    fn loyalty_rises_in_a_thriving_city_and_falls_in_famine() {
        let mut g = game(3320);
        good_town(&mut g);
        let a = member(&mut g, K_SLIME, AR_HONOR, 50);
        dawns(&mut g, 5);
        assert!(loy(&g, a) >= 65, "เมืองเจริญ ภักดีขึ้น {}", loy(&g, a));
        // อดอยาก: ภักดีร่วง และบอกสาเหตุ
        let mut h = game(3321);
        town(&mut h, 6, 75, false, 3);
        let b = member(&mut h, K_SLIME, AR_HONOR, 70);
        dawns(&mut h, 4);
        assert!(loy(&h, b) < 70, "เมืองอดอยาก ภักดีลด {}", loy(&h, b));
        assert!(h.rs.crew.m[0].cause & C_FAMINE != 0 && cause_text(h.rs.crew.m[0].cause).contains("อดอยาก"));
        // ชาวบ้านไม่สุข
        let mut k = game(3322);
        town(&mut k, 6, 20, true, 3);
        let c = member(&mut k, K_SLIME, AR_HONOR, 70);
        dawns(&mut k, 3);
        assert!(loy(&k, c) < 70 && k.rs.crew.m[0].cause & C_UNHAPPY != 0);
    }
    #[test]
    fn archetypes_react_differently_to_the_same_city() {
        // เมืองอ่อนแอ (ไม่มีชื่อเสียง/ป้องกัน) แต่ไม่อดอยาก: ทะเยอทะยานเสียใจ มีเกียรติไม่เป็นไร
        let mut g = game(3323);
        town(&mut g, 4, 70, true, 0);
        let h = member(&mut g, K_SLIME, AR_HONOR, 60);
        let a = member(&mut g, K_SLIME, AR_AMBI, 60);
        dawns(&mut g, 3);
        assert!(loy(&g, a) < loy(&g, h), "ทะเยอทะยานต้องการเมืองที่น่าเกรงขาม {} vs {}", loy(&g, a), loy(&g, h));
        assert!(g.rs.crew.m.iter().find(|m| m.id == a).unwrap().cause & C_WEAK != 0);
        // เมืองใหญ่: ทะเยอทะยานพอใจ
        let mut g = game(3324);
        town(&mut g, 6, 70, true, 5);
        let a = member(&mut g, K_SLIME, AR_AMBI, 60);
        dawns(&mut g, 3);
        assert!(loy(&g, a) > 60);
        // หยิ่ง: ภักดีโตช้ากว่าเมื่อเมืองดี
        let mut g = game(3325);
        good_town(&mut g);
        let h = member(&mut g, K_SLIME, AR_HONOR, 50);
        let p = member(&mut g, K_SLIME, AR_PRIDE, 50);
        dawns(&mut g, 6);
        assert!(loy(&g, p) < loy(&g, h), "พวกหยิ่งไว้ใจช้า {} vs {}", loy(&g, p), loy(&g, h));
    }
    #[test]
    fn winning_and_losing_battles_move_loyalty() {
        let mut g = game(3326);
        let (h, a, p) = (member(&mut g, K_SLIME, AR_HONOR, 50), member(&mut g, K_SLIME, AR_AMBI, 50), member(&mut g, K_SLIME, AR_PRIDE, 50));
        let golem = { g.crew_craft(K_STONEG); g.rs.crew.m.last().unwrap().id };
        g.crew_battle(true);
        assert_eq!((loy(&g, h), loy(&g, a), loy(&g, p)), (53, 56, 52), "ชนะศึก: ทะเยอทะยานดีใจสุด");
        g.crew_battle(false);
        assert_eq!((loy(&g, h), loy(&g, a), loy(&g, p)), (49, 46, 44), "แพ้ศึก: ทะเยอทะยานเสียใจสุด");
        assert_eq!(loy(&g, golem), 100, "โกเลมไม่หวั่นไหว");
        assert!(g.rs.crew.m.iter().find(|m| m.id == a).unwrap().cause & C_WAR != 0);
        assert!(g.toasts.iter().any(|t| t.0.contains("แพ้ศึก")));
    }
    #[test]
    fn honorable_warns_twice_then_leaves_peacefully() {
        let mut g = game(3327);
        town(&mut g, 6, 20, false, 0);
        let id = member(&mut g, K_SLIME, AR_HONOR, 52);
        let mut warned = false; let mut final_warn = false; let mut left = false;
        for _ in 0..40 {
            g.toasts.clear();
            dawns(&mut g, 1);
            if g.toasts.iter().any(|t| t.0.contains("ใจเริ่มห่าง") && t.0.contains("อดอยาก")) { warned = true; }
            if g.toasts.iter().any(|t| t.0.contains("จะลาไปแล้ว")) { final_warn = true; }
            if g.rs.crew.m.is_empty() { left = g.toasts.iter().any(|t| t.0.contains("ลาไปอย่างสงบ")); break; }
        }
        assert!(warned && final_warn, "เตือนล่วงหน้า 2 ขั้น");
        assert!(left && loy(&g, id) == -1, "มีเกียรติ = ลาไปอย่างสงบ");
        assert!(g.bosses.iter().all(|b| b.dead), "ไม่ทรยศ ไม่มีศัตรูโผล่");
    }
    #[test]
    fn ambitious_betrays_and_comes_back_as_an_enraged_enemy() {
        let mut g = game(3328);
        g.bosses.clear();
        town(&mut g, 4, 20, false, 0);
        let id = member(&mut g, K_BGOLEM, AR_AMBI, 40);
        let mut final_warn = false;
        for _ in 0..30 {
            g.toasts.clear();
            dawns(&mut g, 1);
            if g.toasts.iter().any(|t| t.0.contains("จะทรยศแล้ว")) { final_warn = true; }
            if g.rs.crew.m.is_empty() { break; }
        }
        assert!(final_warn, "เตือนก่อนทรยศ");
        assert!(g.rs.crew.m.is_empty() && loy(&g, id) == -1);
        assert!(g.toasts.iter().any(|t| t.0.contains("ทรยศ")));
        let b = g.bosses.iter().find(|b| !b.dead).expect("กลับมาเป็นศัตรู");
        assert_eq!((b.kind, b.arch), (1, AR_FERAL), "แค้นแล้ว ไม่ยอมแพ้อีก");
        assert!(b.hp > 10. && (b.x - g.p.x).hypot(b.y - g.p.y) < 200.);
        // ทรยศแล้วตีล้มได้ตามปกติ (ไม่มีการ์ดยอมแพ้) และทิ้งของหายาก
        let core = g.p.inv[CORE];
        g.bosses[0].hp = 1.;
        g.damage_boss(0, 999, false);
        assert!(g.bosses[0].dead && g.rs.crew.yld.is_none() && g.p.inv[CORE] > core);
    }
    #[test]
    fn stubborn_also_betrays_and_champion_betrayal_returns_as_champion() {
        let mut g = game(3329);
        let r = g.w.realm.clone(); g.rs.init(&r);
        g.bosses.clear();
        town(&mut g, 4, 20, false, 0);
        member(&mut g, K_SLIME, AR_PRIDE, 30);
        member(&mut g, K_CHAMP, AR_AMBI, 30);
        g.rs.crew.m[1].nm = 0;
        for _ in 0..20 { dawns(&mut g, 1); }
        assert!(g.rs.crew.m.is_empty());
        let kinds: Vec<u8> = g.bosses.iter().filter(|b| !b.dead).map(|b| b.kind).collect();
        assert!(kinds.contains(&0) && kinds.contains(&CHAMP), "ทั้งสองกลับมาเป็นศัตรู {:?}", kinds);
    }
    #[test]
    fn nobody_leaves_while_the_player_is_underground() {
        let mut g = game(3330);
        town(&mut g, 4, 20, false, 0);
        member(&mut g, K_SLIME, AR_AMBI, 15);
        g.w.dun = Some(Box::new(Dungeon::generate_cave(777, (1, 1), false, &HashSet::new(), 0)));
        dawns(&mut g, 6);
        assert_eq!(g.rs.crew.m.len(), 1, "อยู่ในดันเจี้ยน = รอไว้ก่อน ไม่ทรยศกลางทาง");
    }
    #[test]
    fn gifts_and_rest_hold_a_wavering_member() {
        let mut g = game(3331);
        town(&mut g, 4, 70, false, 0);
        let id = member(&mut g, K_SLIME, AR_AMBI, 36);
        g.p.inv[BERRY] = 20;
        for _ in 0..5 { g.crew_gift_member(id); g.time += 1.; }
        assert!(loy(&g, id) >= 44, "ให้ของโปรดซ้ำ ภักดีขึ้น {}", loy(&g, id));
        let m = &g.rs.crew.m[0];
        assert!(m.mood > 70);
    }
    #[test]
    fn crew_cards_show_archetype_and_warning_with_causes() {
        let mut g = game(3332);
        g.animals.iter_mut().for_each(|a| a.tame = false);
        let id = member(&mut g, K_SLIME, AR_AMBI, 30);
        { let m = g.rs.crew.m.iter_mut().find(|m| m.id == id).unwrap(); m.cause = C_WEAK | C_FAMINE; m.sad = 1; }
        let cards = menu::crew_cards(&g);
        let c = cards.iter().find(|c| c.mind).unwrap();
        assert!(c.traits.contains("ทะเยอทะยาน"));
        let w = c.warn.clone().unwrap();
        assert!(w.contains("อดอยาก") && w.contains("ทรยศ") && w.contains("เหลือ 1 วัน"), "{}", w);
        assert!(!c.info.is_empty());
    }
    #[test]
    fn card_text_lines_fit_above_buttons_on_all_screens() {
        // 3 เงื่อนไข + คำตอบ 2 บรรทัด ต้องจบก่อนแถวปุ่มบนจอทุกขนาด
        for &(sw, sh) in &[(1280., 720.), (2400., 1080.), (800., 450.), (1024., 600.), (2340., 1080.)] {
            let u: f32 = (sh / 450f32).clamp(1., 3.);
            let l = hud::layout_for(sw, u, (sh / 450.).clamp(1., 3.2), true);
            let (r, b) = yield_rects_for(&l, u, 0.);
            let last_baseline = 32. * u + 3. * 13. * u + 8. * u + 13. * u;
            assert!(last_baseline < b[0].y - r.y, "ข้อความไม่ทับปุ่ม {}x{}", sw, sh);
        }
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
        for _ in 0..45 {
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
        g.rs.crew.spared = [true, false, true]; g.rs.crew.lost_day = 5;
        g.crew_craft(K_IRONG);
        g.crew_craft(K_CRYSTG);
        g.crew_join(K_BGOLEM, [22, 85, 20, 38, 12, 72], 66, 321.5, 87.5);
        g.rs.crew.m[0].hp = 33.; g.rs.crew.m[1].hp = 0.; g.rs.crew.m[1].down = true; g.rs.crew.m[1].mode = M_ROAM;
        g.rs.crew.m[2].arch = AR_PRIDE; g.rs.crew.m[2].cause = C_WEAK | C_WAR; g.rs.crew.m[2].warned = true; g.rs.crew.m[2].mood = 44; g.rs.crew.m[2].sad = 1; g.rs.crew.m[2].fed_day = 9; g.rs.crew.m[2].post = (12, -3);
        let s = g.crew_save();
        let mut h = game(3319);
        for l in s.lines() {
            let mut it = l.split_whitespace();
            let tag = it.next().unwrap();
            let rest: Vec<&str> = it.collect();
            match tag { "crewst" => h.crew_load_st(&rest), "crew" => h.crew_load(&rest), _ => panic!("แท็กแปลก {}", tag) }
        }
        assert_eq!((h.rs.crew.spared, h.rs.crew.lost_day), ([true, false, true], 5));
        assert_eq!(h.rs.crew.m.len(), 3);
        for (a, b) in g.rs.crew.m.iter().zip(h.rs.crew.m.iter()) {
            assert_eq!((a.id, a.kind, a.mode, a.loyalty, a.mood, a.t, a.fed_day), (b.id, b.kind, b.mode, b.loyalty, b.mood, b.t, b.fed_day));
            assert_eq!((a.sad, a.post, a.down, a.arch, a.cause, a.warned), (b.sad, b.post, b.down, b.arch, b.cause, b.warned));
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
