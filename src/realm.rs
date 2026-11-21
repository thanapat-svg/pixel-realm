// realm.rs — v17: อาณาจักรในโลก · การเมือง · การค้า · สงคราม · ผู้อพยพ/ขุนนาง · ม้า/รถม้า
//
// โลกมี 4 อาณาจักร (สุ่มจาก seed แบบกำหนดแน่นอน — ไม่ต้องเก็บแผนผังลงเซฟ) แต่ละอาณาจักรมีเมืองหลวง 1 + เมืองย่อย 2
// เมืองถูก "วาด" ลงโลกตอนสร้างไทล์ (กำแพงเมือง ประตู ถนน ลาน บ้าน ปราสาท ตลาด คอกม้า ท่ารถม้า ธง)
// สิ่งที่เปลี่ยนได้ (ความสัมพันธ์ กำลังทหาร เจ้าของเมือง สงคราม พันธมิตร ชื่อเสียงของเรา ฯลฯ) อยู่ใน RealmState และถูกเซฟ
//
// ทุกอย่างมีเงื่อนไข:
//  - เมืองแต่ละอาณาจักรมีกฎหมาย (ห้ามใช้อาวุธ ห้ามตัดไม้ ค่าผ่านประตู เคอร์ฟิว ภาษีการค้า ห้ามเวทมนตร์) ฝ่าฝืน = ปรับ + ชื่อเสียงตก · ชื่อเสียงต่ำมาก = ทหารเมืองไล่จับ
//  - การทูตต้องไปที่ปราสาทของเมืองนั้น หรือมี "ทูต" ในเมืองเราถึงทำจากระยะไกลได้
//  - พันธมิตรต้องพึ่งพากัน: เมื่อพันธมิตรถูกรบ เขาจะขอความช่วยเหลือ ไม่ช่วย = ความสัมพันธ์ตก แตกหักได้
//  - อาณาจักรที่แข็งกว่าและไม่ชอบเรา จะยื่นคำขาดเรียกเงิน/ให้เป็นเมืองขึ้น ไม่ยอม = ยกทัพมา (เห็นบนแผนที่) → ศึกป้องกันเมือง
//  - ตั้งอาณาจักรเองได้เมื่อครบเงื่อนไข ออกกฎหมายเมืองตัวเองได้
use super::*;

pub const KN: usize = 4;
// วัตถุในเมือง
pub const O_STALL: u8 = 80;
pub const O_STABLE: u8 = 81;
pub const O_COACH: u8 = 82;
pub const O_BANNER: u8 = 83;
pub const PLAYER_OWN: u8 = 255;

pub const L_WEAPON: u8 = 0;
pub const L_LOG: u8 = 1;
pub const L_TOLL: u8 = 2;
pub const L_CURFEW: u8 = 3;
pub const L_TAX: u8 = 4;
pub const L_MAGIC: u8 = 5;
pub const LAW_N: [&str; 6] = ["ห้ามใช้อาวุธในเมือง", "ห้ามตัดไม้/ทุบของในเมือง", "ค่าผ่านประตูเมือง 5 เหรียญ/วัน", "เคอร์ฟิว 22:00–05:00 ห้ามอยู่นอกอาคาร", "ภาษีการค้า +15% ที่ตลาด", "ห้ามใช้เวทมนตร์ (คทา)"];
pub const TRAIT_N: [&str; 4] = ["นักรบ — ชอบขยายดินแดน", "พ่อค้า — เน้นการค้า", "รักเกียรติ — รักษาสัญญา", "เคร่งครัด — กฎเข้มงวด"];
const K_NAMES: [&str; 8] = ["สุวรรณภูมิ", "เหล็กกล้า", "ทะเลทรายทอง", "หิมะเงิน", "มรกต", "อัคนี", "จันทราเงา", "ศิลาแดง"];
const RULERS: [(&str, &str); 10] = [("พระราชา", "อรุณเทพ"), ("พระราชินี", "มณีรัตน์"), ("ลอร์ด", "กาฬสิงห์"), ("จักรพรรดิ", "วายุภักษ์"), ("พระราชา", "ธรณินทร์"), ("พระราชินี", "จันทร์เพ็ญ"), ("ลอร์ด", "เหล็กไหล"), ("ดยุก", "ศิลาธร"), ("พระราชา", "สุริยัน"), ("พระราชินี", "นภาลัย")];
const CITY_NAMES: [&str; 24] = ["ทองหล่อ", "ผาแดง", "บึงใส", "ป่าสน", "หินขาว", "ท่าเรือเก่า", "เนินลม", "ห้วยเย็น", "ปากน้ำ", "โคกงาม", "ดงยาง", "ทุ่งทอง", "ภูเงิน", "หนองบัว", "ศิลาลาย", "ลำธาร", "ผาหมอก", "ทรายแก้ว", "บ้านไร่", "วังน้ำ", "เขาหลวง", "นาคราช", "สองแคว", "ดอยคำ"];
pub const K_COLORS: [u32; 8] = [0xe0b040, 0x8a9aae, 0xe07a3a, 0x8ad0f0, 0x4ac07a, 0xd04a4a, 0x9a6ad8, 0xc0603a];
const MY_NAMES: [&str; 8] = ["อาณาจักรรุ่งอรุณ", "อาณาจักรใจเพชร", "อาณาจักรฟ้าใหม่", "อาณาจักรพนาไพร", "อาณาจักรสายรุ้ง", "อาณาจักรดาวเหนือ", "อาณาจักรศรีสวัสดิ์", "อาณาจักรเรืองฤทธิ์"];
pub const MY_COLOR: u32 = 0x60e0ff;

/// สินค้าที่ซื้อขายได้ที่ตลาดเมือง (ราคาพื้นฐาน)
pub const GOODS: [(usize, i32); 14] = [(WOOD, 2), (STONE, 2), (BERRY, 2), (MEAT, 4), (FISH, 4), (CORN, 4), (CARROT, 4), (SHROOM, 3), (HERB, 5), (GEM, 25), (POTION, 12), (ARROW, 1), (DISH_STEW, 12), (DISH_FISH, 12)];

#[derive(Clone, Debug)]
pub struct City { pub name: &'static str, pub pos: (i32, i32), pub r: i32, pub cap: bool, pub k0: usize, pub cheap: [usize; 2], pub want: [usize; 2] }
#[derive(Clone, Debug)]
pub struct KDef { pub name: String, pub title: &'static str, pub ruler: &'static str, pub color: u32, pub trait_: u8, pub laws: Vec<u8>, pub cap: usize, pub claim: i32 }
#[derive(Clone, Debug, Default)]
pub struct Realm { pub ks: Vec<KDef>, pub cities: Vec<City> }

impl World {
    /// อยู่ใกล้/ในเมือง (ใช้กันแคมป์ จุดสำคัญ ดันเจี้ยน เสาวาร์ป ไม่ให้ทับเมือง)
    pub fn near_city(&self, x: i32, y: i32, margin: i32) -> bool {
        self.realm.cities.iter().any(|c| (x - c.pos.0).abs() <= c.r + margin && (y - c.pos.1).abs() <= c.r + margin)
    }
}

fn landish(t: u8) -> bool { matches!(t, 2 | 3 | 4 | 5 | 7 | 8 | 15 | 16) }

impl Realm {
    pub fn build(w: &World) -> Realm {
        let s = w.seed;
        let h = |a: i32, b: i32| hash(a, b, s.wrapping_add(9100)) as f32;
        let score = |c: (i32, i32), r: i32| -> f32 {
            let mut ok = 0;
            for j in 0..11 { for i in 0..11 {
                let (x, y) = (c.0 - r + i * r * 2 / 10, c.1 - r + j * r * 2 / 10);
                if landish(w.gen_tile(x, y)) { ok += 1; }
            } }
            ok as f32 / 121.
        };
        let mut cities: Vec<City> = vec![];
        let mut ks = vec![];
        // ชื่ออาณาจักร: สับไพ่ตาม seed
        let mut names: Vec<usize> = (0..8).collect();
        for i in (1..8).rev() { let j = (h(i as i32, 1) * (i + 1) as f32) as usize % (i + 1); names.swap(i, j); }
        let base = h(0, 2) * std::f32::consts::TAU;
        let mut used_city = 0usize;
        let mut cn: Vec<usize> = (0..CITY_NAMES.len()).collect();
        for i in (1..cn.len()).rev() { let j = (h(i as i32, 77) * (i + 1) as f32) as usize % (i + 1); cn.swap(i, j); }
        let cname = |k: usize| CITY_NAMES[cn[k % cn.len()]];
        for k in 0..KN {
            let a0 = base + k as f32 * std::f32::consts::FRAC_PI_2 + (h(k as i32, 3) - 0.5) * 0.5;
            let mut best = ((0, 0), -1.0f32);
            for d in [320., 360., 280., 400., 440.] {
                for off in [0., 0.15, -0.15, 0.3, -0.3] {
                    let c = ((a0 + off).cos() * d, (a0 + off).sin() * d);
                    let c = (c.0 as i32, c.1 as i32);
                    let sc = score(c, 22);
                    if sc > best.1 { best = (c, sc); }
                }
                if best.1 > 0.8 { break; }
            }
            let cap_idx = cities.len();
            let biome = biome_spec(w, best.0);
            cities.push(City { name: cname(used_city), pos: best.0, r: 20, cap: true, k0: k, cheap: biome.0, want: biome.1 });
            used_city += 1;
            for side in [-1.0f32, 1.0] {
                let mut bt = ((0, 0), -1.0f32);
                for d in [110., 130., 95., 150.] {
                    for off in [0.0f32, 0.25, -0.25] {
                        let a = a0 + side * 1.1 + off;
                        let c = (best.0 .0 + (a.cos() * d) as i32, best.0 .1 + (a.sin() * d) as i32);
                        if cities.iter().any(|o| (o.pos.0 - c.0).abs().max((o.pos.1 - c.1).abs()) < 60) { continue; }
                        if (c.0 * c.0 + c.1 * c.1) < 150 * 150 { continue; }
                        let sc = score(c, 13);
                        if sc > bt.1 { bt = (c, sc); }
                    }
                }
                if bt.1 >= 0. {
                    let biome = biome_spec(w, bt.0);
                    cities.push(City { name: cname(used_city), pos: bt.0, r: 13, cap: false, k0: k, cheap: biome.0, want: biome.1 });
                    used_city += 1;
                }
            }
            let (title, ruler) = RULERS[(h(k as i32, 5) * 10.) as usize % 10];
            let trait_ = (h(k as i32, 6) * 4.) as u8 % 4;
            // กฎหมาย: ตามนิสัยอาณาจักร + สุ่ม
            let mut laws: Vec<u8> = match trait_ { 0 => vec![L_WEAPON, L_TOLL], 1 => vec![L_TAX, L_LOG], 2 => vec![L_WEAPON, L_LOG], _ => vec![L_CURFEW, L_MAGIC, L_WEAPON] };
            let extra = (h(k as i32, 8) * 6.) as u8 % 6;
            if !laws.contains(&extra) { laws.push(extra); }
            ks.push(KDef { name: format!("อาณาจักร{}", K_NAMES[names[k]]), title, ruler, color: K_COLORS[names[k]], trait_, laws, cap: cap_idx, claim: 190 });
        }
        Realm { ks, cities }
    }
    /// เมืองที่ช่องนี้อยู่ (รวมกำแพงรอบเมือง)
    pub fn city_at(&self, x: i32, y: i32) -> Option<usize> {
        self.cities.iter().position(|c| (x - c.pos.0).abs() <= c.r && (y - c.pos.1).abs() <= c.r)
    }
    /// ไทล์/วัตถุของผังเมือง (dx,dy จากกลางเมือง) → (ไทล์, วัตถุ)
    pub fn layout(&self, ci: usize, x: i32, y: i32) -> (u8, u8) {
        let c = &self.cities[ci];
        let (dx, dy, r) = (x - c.pos.0, y - c.pos.1, c.r);
        let (ax, ay) = (dx.abs(), dy.abs());
        if ax.max(ay) == r {
            let gate = (ax <= 1 && ay == r) || (ay <= 1 && ax == r);
            return if gate { (T_FLOORS, 0) } else { (3, 38) };
        }
        if ax <= 5 && ay <= 5 {
            // ลานกลางเมือง
            let o = match (dx, dy) {
                (0, -4) => 37,
                (-2, -4) | (2, -4) => O_BANNER,
                (-3, -2) | (3, -2) | (-3, 2) | (3, 2) => O_STALL,
                (-4, 4) => 39,
                (4, -4) => O_STABLE,
                (4, 4) => O_COACH,
                _ => 0,
            };
            return (T_FLOORS, o);
        }
        if ax <= 1 || ay <= 1 { return (T_FLOORS, 0); }
        if (dx, dy) == (2, r - 1) || (dx, dy) == (-2, r - 1) || (dx, dy) == (2, -(r - 1)) || (dx, dy) == (-2, -(r - 1)) { return (3, O_BANNER); }
        // แปลงบ้าน 7x6 นับจากกำแพงเข้ามา (ไม่ทับลานกลาง/ถนน)
        let (ox, oy) = (r - 1 - ax, r - 1 - ay);
        let (lx, ly) = (ox % 7, oy % 6);
        let (i, j) = (ox / 7, oy / 6);
        let (lo_x, lo_y) = (r - 1 - 7 * i - 6, r - 1 - 6 * j - 5);
        if lo_x < 2 || lo_y < 2 || (lo_x <= 5 && lo_y <= 5) { return (3, 0); }
        let hh = hash(c.pos.0 * 31 + dx.signum() * 7 + i, c.pos.1 * 17 + dy.signum() * 5 + j, 9200);
        if hh < 0.18 {
            // สวน
            return (3, if lx == 3 && ly == 2 { 1 } else if (lx + ly) % 3 == 0 && lx > 0 && ly > 0 && lx < 6 && ly < 5 { 3 } else { 0 });
        }
        if lx == 0 || lx == 6 || ly == 0 || ly == 5 { return (3, 0); }
        let wall = if c.cap { 11 } else { 10 };
        if lx == 1 || lx == 5 || ly == 1 || ly == 4 {
            if ly == 4 && lx == 3 { return (T_FLOORW, 12); }
            if ly == 1 && lx == 3 && hh > 0.5 { return (T_FLOORW, O_WINDOW); }
            return (T_FLOORW, wall);
        }
        let furn = match (lx, ly, (hh * 10.) as i32 % 3) {
            (2, 3, _) => 33,
            (4, 3, 0) => O_SHELF,
            (4, 3, 1) => O_WARDROBE,
            (4, 3, _) => 14,
            (4, 2, _) => O_TABLE,
            _ => 0,
        };
        (T_FLOORW, furn)
    }
}

fn biome_spec(w: &World, c: (i32, i32)) -> ([usize; 2], [usize; 2]) {
    let mut cnt = [0; 21];
    let mut water = 0;
    for j in -6..=6 { for i in -6..=6 {
        let t = w.gen_tile(c.0 + i * 5, c.1 + j * 5) as usize;
        if t < 2 { water += 1; }
        if t < 21 { cnt[t] += 1; }
    } }
    let top = (2..21).max_by_key(|&t| cnt[t]).unwrap_or(3);
    if water > 30 { return ([FISH, WOOD], [STONE, CORN]); }
    match top {
        8 | 16 => ([GEM, STONE], [FISH, BERRY]),
        7 => ([MEAT, STONE], [CORN, CARROT]),
        15 => ([HERB, SHROOM], [WOOD, MEAT]),
        5 | 6 | 17 => ([STONE, GEM], [CORN, MEAT]),
        4 => ([WOOD, SHROOM], [STONE, FISH]),
        _ => ([WOOD, BERRY], [STONE, GEM]),
    }
}

// ---------------------------------------------------------------- สถานะที่เปลี่ยนได้
#[derive(Clone, Debug)]
pub struct Army { pub x: f32, pub y: f32, pub n: u32, pub k: usize, pub vs: Option<usize> }
#[derive(Clone, Debug)]
pub struct KState {
    pub strength: f32, pub base: f32, pub wealth: f32, pub rep: i32, pub rel: [i32; KN],
    pub war: [bool; KN], pub ally: [bool; KN], pub alive: bool,
    /// ความสัมพันธ์กับเรา: 0 ไม่มี 1 สัญญาการค้า 2 พันธมิตร 3 สงคราม 4 เราเป็นเมืองขึ้น
    pub stance: u8,
    pub ult: Option<(i32, i32)>, pub vas_ask: bool, pub help: Option<(i32, usize)>,
    pub gift_day: i32, pub trib_day: i32, pub war_day: i32, pub army: Option<Army>, pub army_cd: i32,
}
#[derive(Clone, Debug)]
pub struct Petition { pub rank: u8, pub perk: u8, pub salary: i32, pub name: u8, pub look: u8, pub city: usize, pub day: i32 }
#[derive(Clone, Debug)]
pub struct Origin { pub city: usize, pub rank: u8, pub perk: u8, pub salary: i32, pub loyalty: i32, pub spy: bool }
#[derive(Clone, Debug)]
pub struct Citizen { pub path: Path, pub block: f32, pub x: f32, pub y: f32, pub tx: f32, pub ty: f32, pub look: u32, pub guard: bool, pub city: usize, pub wait: f32, pub anim: f32, pub left: bool }
#[derive(Clone, Debug)]
pub struct Siege { pub k: usize, pub total: u32, pub spawned: u32, pub t: f32, pub hold: f32, pub center: (f32, f32), pub from: (f32, f32) }

pub const RANK_N: [&str; 4] = ["ชาวเมือง", "ช่างฝีมือ", "ขุนนาง", "แม่ทัพเก่า"];
pub const PERK_N: [&str; 6] = ["คลังเชี่ยวชาญ: ภาษี +30%", "แม่ทัพ: ป้องกันเมือง +30%", "พ่อค้าใหญ่: ราคาตลาดดีขึ้น 10% · รายได้สัญญาการค้า +50%", "นักการทูต: ชื่อเสียงกับทุกอาณาจักร +1/วัน", "สถาปนิก: ก่อสร้างเร็วขึ้น 50%", "หมอหลวง: ชาวบ้านความสุข +2/วัน"];
pub const P_TREASURY: u8 = 0;
pub const P_GENERAL: u8 = 1;
pub const P_TRADE: u8 = 2;
pub const P_DIPLO: u8 = 3;
pub const P_ARCH: u8 = 4;
pub const P_HEAL: u8 = 5;
/// กฎเมืองเรา (bit)
pub const ML_LOWTAX: u8 = 1;
pub const ML_HIGHTAX: u8 = 2;
pub const ML_MILITIA: u8 = 4;
pub const ML_OPEN: u8 = 8;
pub const ML_CURFEW: u8 = 16;
pub const ML_NOWEAPON: u8 = 32;

#[derive(Clone, Debug, Default)]
pub struct RealmState {
    pub ks: Vec<KState>,
    pub owner: Vec<u8>,
    pub disc: HashSet<usize>,
    pub visited: HashSet<usize>,
    pub founded: bool,
    pub name_i: usize,
    pub laws: u8,
    pub petitions: Vec<Petition>,
    pub origins: HashMap<u32, Origin>,
    pub manual: HashMap<u32, u8>,
    pub siege_wins: i32,
    pub horse: bool,
    pub riding: bool,
    // ไม่เซฟ
    pub px: HashMap<(usize, usize), f32>,
    pub toll_day: HashMap<usize, i32>,
    pub cur_city: Option<usize>,
    pub crime_t: f32,
    pub curfew_day: i32,
    pub siege: Option<Siege>,
    pub cit: Vec<Citizen>,
    pub npaths: HashMap<u32, Path>,
    pub tick_t: f32,
    pub inited: bool,
}

impl RealmState {
    pub fn init(&mut self, r: &Realm) {
        if self.inited && self.ks.len() == r.ks.len() { return; }
        self.inited = true;
        if self.ks.len() != r.ks.len() {
            self.ks = r.ks.iter().enumerate().map(|(i, k)| {
                let base = 60. + (i as f32 * 13.) % 40. + if k.trait_ == 0 { 25. } else { 0. };
                KState { strength: base, base, wealth: 300. + (i as f32 * 71.) % 200., rep: 0, rel: [0; KN], war: [false; KN], ally: [false; KN], alive: true, stance: 0, ult: None, vas_ask: false, help: None, gift_day: -9, trib_day: 0, war_day: -99, army: None, army_cd: 0 }
            }).collect();
            // ความสัมพันธ์เริ่มต้นระหว่างอาณาจักร
            for a in 0..r.ks.len() { for b in 0..r.ks.len() {
                if a != b { let v = ((a * 37 + b * 11 + r.ks[a].trait_ as usize * 5) % 50) as i32 - 20; self.ks[a].rel[b] = v; self.ks[b].rel[a] = v; }
            } }
        }
        if self.owner.len() != r.cities.len() { self.owner = r.cities.iter().map(|c| c.k0 as u8).collect(); }
        for (i, c) in r.cities.iter().enumerate() { if c.cap { self.disc.insert(i); } }
    }
}

impl Game {
    pub fn realm(&self) -> &Realm { &self.w.realm }
    /// เมืองที่ผู้เล่นยืนอยู่
    pub fn player_city(&self) -> Option<usize> {
        if self.w.dun.is_some() { return None; }
        self.w.realm.city_at(tfloor(self.p.x), tfloor(self.p.y - 3.))
    }
    pub fn city_owner(&self, ci: usize) -> u8 { self.rs.owner.get(ci).copied().unwrap_or(self.w.realm.cities[ci].k0 as u8) }
    pub fn owner_name(&self, o: u8) -> String { if o == PLAYER_OWN { self.my_name() } else { self.w.realm.ks[o as usize].name.clone() } }
    pub fn owner_color(&self, o: u8) -> u32 { if o == PLAYER_OWN { MY_COLOR } else { self.w.realm.ks[o as usize].color } }
    pub fn my_name(&self) -> String { if self.rs.founded { MY_NAMES[self.rs.name_i % 8].to_string() } else { "ชุมชนของคุณ".to_string() } }
    /// กลางเมืองเรา = ค่าเฉลี่ยตำแหน่งเตียงชาวบ้าน (หรือเตียงเรา)
    pub fn town_center(&self) -> Option<(i32, i32)> {
        let beds: Vec<(i32, i32)> = self.residents().map(|n| n.bed).collect();
        if beds.is_empty() { return self.home_tile().map(|h| h.0); }
        let n = beds.len() as i32;
        Some((beds.iter().map(|b| b.0).sum::<i32>() / n, beds.iter().map(|b| b.1).sum::<i32>() / n))
    }
    /// อยู่ในเขตแดนของอาณาจักรไหน (วัดจากเมืองหลวง/เมืองที่ครอบครอง)
    pub fn claim_of(&self, p: (i32, i32)) -> Option<usize> {
        let r = &self.w.realm;
        let mut best: Option<(usize, i32)> = None;
        for (ci, c) in r.cities.iter().enumerate() {
            let o = self.city_owner(ci);
            if o == PLAYER_OWN || !self.rs.ks.get(o as usize).map_or(false, |k| k.alive) { continue; }
            let rad = if c.cap { r.ks[o as usize].claim } else { 110 };
            let d = (p.0 - c.pos.0).abs().max((p.1 - c.pos.1).abs());
            if d <= rad && best.map_or(true, |b| d < b.1) { best = Some((o as usize, d)); }
        }
        best.map(|b| b.0)
    }
    pub fn role_count(&self, role: u8) -> usize { self.residents().filter(|n| n.role == role && n.mood >= 20 && Game::willing(n)).count() }
    pub fn perk_on(&self, perk: u8) -> bool { self.rs.origins.iter().any(|(id, o)| o.perk == perk && o.rank >= 2 && self.idx(*id).map_or(false, |i| self.npcs[i].state == ST_HOME)) }
    /// กำลังป้องกันเมืองเรา
    pub fn defense(&self) -> f32 {
        let guards = self.role_count(2) as f32;
        let soldiers = self.role_count(R_SOLDIER) as f32;
        let mut d = 4. + guards * 2. + soldiers * 4. + self.walls() as f32 * 0.4 + if self.has_castle() { 10. } else { 0. } + self.p.lvl as f32;
        if self.role_count(R_GENERAL) > 0 { d *= 1.4; }
        if self.perk_on(P_GENERAL) { d *= 1.3; }
        if self.rs.laws & ML_MILITIA != 0 { d += self.pop() as f32 * 1.2; }
        for (k, s) in self.rs.ks.iter().enumerate() { if s.stance == 2 && s.alive { d += s.strength * 0.12; let _ = k; } }
        d
    }
    pub fn fame(&self) -> i32 {
        let pop = self.pop() as i32;
        let comfort: i32 = self.residents().filter_map(|n| self.w.room_from(n.bed)).map(|r| r.comfort as i32).sum();
        let owned = self.rs.owner.iter().filter(|&&o| o == PLAYER_OWN).count() as i32;
        pop * 4 + if self.has_castle() { 15 } else { 0 } + self.walls() as i32 / 2 + comfort * 2 + self.p.kills / 20 + self.rs.siege_wins * 15
            + if self.rs.founded { 25 } else { 0 } + owned * 20 + self.rs.ks.iter().filter(|k| k.stance == 1 || k.stance == 2).count() as i32 * 5
    }
    pub fn city_fame(&self, ci: usize) -> i32 {
        let c = &self.w.realm.cities[ci];
        let o = self.city_owner(ci);
        let k = if o == PLAYER_OWN { 30. } else { self.rs.ks[o as usize].strength * 0.4 + self.rs.ks[o as usize].wealth / 40. };
        (if c.cap { 60. } else { 25. } + k) as i32
    }
    pub fn dist_to(&self, p: (i32, i32)) -> i32 { ((p.0 as f32 * 16. - self.p.x).hypot(p.1 as f32 * 16. - self.p.y) / 16.) as i32 }
    pub fn dir_word(&self, p: (i32, i32)) -> &'static str {
        let (dx, dy) = (p.0 as f32 * 16. - self.p.x, p.1 as f32 * 16. - self.p.y);
        let a = dy.atan2(dx).to_degrees();
        match a { a if (-22.5..22.5).contains(&a) => "ตะวันออก", a if (22.5..67.5).contains(&a) => "ตะวันออกเฉียงใต้", a if (67.5..112.5).contains(&a) => "ใต้", a if (112.5..157.5).contains(&a) => "ตะวันตกเฉียงใต้", a if (-67.5..-22.5).contains(&a) => "ตะวันออกเฉียงเหนือ", a if (-112.5..-67.5).contains(&a) => "เหนือ", a if (-157.5..-112.5).contains(&a) => "ตะวันตกเฉียงเหนือ", _ => "ตะวันตก" }
    }
    pub fn law_on(&self, ci: usize, law: u8) -> bool {
        let o = self.city_owner(ci);
        if o == PLAYER_OWN { return law == L_WEAPON && self.rs.laws & ML_NOWEAPON != 0; }
        self.w.realm.ks[o as usize].laws.contains(&law)
    }
    pub fn kname(&self, k: usize) -> String { self.w.realm.ks[k].name.clone() }

    // ---------------------------------------------------------------- อาชญากรรม
    pub fn crime(&mut self, ci: usize, what: &str, fine: i32, rep: i32) {
        if self.rs.crime_t > 0. { return; }
        self.rs.crime_t = 4.;
        let o = self.city_owner(ci);
        if o == PLAYER_OWN { return; }
        let k = o as usize;
        let paid = self.p.inv[COIN] >= fine;
        if paid { self.p.inv[COIN] -= fine; }
        self.rs.ks[k].rep -= rep + if paid { 0 } else { 3 };
        self.toast(format!("ผิดกฎ{}: {} — {} (ชื่อเสียงกับ{} {})", self.w.realm.cities[ci].name, what, if paid { format!("ถูกปรับ {} เหรียญ", fine) } else { "ไม่มีเงินจ่ายค่าปรับ!".to_string() }, self.kname(k), self.rs.ks[k].rep));
        play(&self.sfx.deny);
        if self.rs.ks[k].rep <= -60 { self.city_guards_attack(ci); }
    }
    /// เรียกจากการโจมตี (ดาบ/ธนู/คทา)
    pub fn law_on_attack(&mut self, magic: bool) {
        let Some(ci) = self.player_city() else { return };
        let enemy_near = self.en.iter().any(|e| !e.dead && (e.x - self.p.x).hypot(e.y - self.p.y) < 140.);
        if enemy_near { return; }
        if magic && self.law_on(ci, L_MAGIC) { self.crime(ci, "ใช้เวทมนตร์ในเมือง", 10, 4); }
        else if self.law_on(ci, L_WEAPON) { self.crime(ci, "ใช้อาวุธในเมือง", 5, 2); }
    }
    /// ทุบของในเมือง (ของในเมืองทุบไม่พัง)
    pub fn law_on_hit(&mut self, tx: i32, ty: i32) -> bool {
        if self.w.dun.is_some() { return false; }
        let Some(ci) = self.w.realm.city_at(tx, ty) else { return false };
        if self.city_owner(ci) == PLAYER_OWN { return false; }
        if self.w.placed.contains_key(&(tx, ty)) { return false; }
        if self.w.obj_at(tx, ty) == 0 { return false; }
        if self.law_on(ci, L_LOG) { self.crime(ci, "ทุบ/ตัดของในเมือง", 8, 3); } else if self.rs.crime_t <= 0. { self.toast("ทรัพย์สินของเมือง ทุบไม่ได้"); self.rs.crime_t = 2.; }
        true
    }
    pub fn city_guards_attack(&mut self, ci: usize) {
        let o = self.city_owner(ci);
        if o == PLAYER_OWN || peaceful() { return; }
        if self.en.iter().filter(|e| e.k == EK::Soldier).count() >= 4 { return; }
        for i in 0..3 {
            let a = i as f32 * 2.1;
            let (x, y) = (self.p.x + a.cos() * 90., self.p.y + a.sin() * 70.);
            if !collide(&self.w, x, y) { self.make_enemy(EK::Soldier, x, y, 1.2, o as usize, false); }
        }
        self.toast("ทหารเมืองไล่จับคุณ! (ชื่อเสียงต่ำเกินไป — ไปปรับความสัมพันธ์ที่ปราสาทหรือออกจากเมือง)");
    }

    // ---------------------------------------------------------------- ทุกเฟรม
    pub fn realm_tick(&mut self, dt: f32) {
        if !self.rs.inited { let r = self.w.realm.clone(); self.rs.init(&r); }
        if self.rs.crime_t > 0. { self.rs.crime_t -= dt; }
        self.horse_tick();
        self.citizens_tick(dt);
        self.siege_tick(dt);
        // กองทัพเดินทัพ (80 ช่อง/วันเกม)
        let speed = 80. / DAYLEN * dt;
        let tc = self.town_center();
        let mut arrive: Vec<usize> = vec![];
        for (k, s) in self.rs.ks.iter_mut().enumerate() {
            let Some(a) = &mut s.army else { continue };
            let tgt = match a.vs { None => tc.map(|t| (t.0 as f32, t.1 as f32)), Some(ci) => Some((self.w.realm.cities[ci].pos.0 as f32, self.w.realm.cities[ci].pos.1 as f32)) };
            let Some(t) = tgt else { s.army = None; continue };
            let (dx, dy) = (t.0 - a.x, t.1 - a.y);
            let d = dx.hypot(dy);
            if d < 24. { arrive.push(k); continue; }
            a.x += dx / d * speed;
            a.y += dy / d * speed;
        }
        for k in arrive { self.army_arrive(k); }
        self.rs.tick_t -= dt;
        if self.rs.tick_t > 0. { return; }
        self.rs.tick_t = 0.5;
        // เข้า/ออกเมือง
        let pc = self.player_city();
        if pc != self.rs.cur_city {
            if let Some(ci) = pc { self.enter_city(ci); } else if let Some(old) = self.rs.cur_city { self.toast(format!("ออกจากเมือง{}", self.w.realm.cities[old].name)); }
            self.rs.cur_city = pc;
        }
        // ค้นพบเมืองย่อยเมื่อเข้าใกล้
        let (ptx, pty) = (tfloor(self.p.x), tfloor(self.p.y));
        let found: Vec<usize> = self.w.realm.cities.iter().enumerate().filter(|(i, c)| !self.rs.disc.contains(i) && (c.pos.0 - ptx).abs().max((c.pos.1 - pty).abs()) < c.r + 45).map(|(i, _)| i).collect();
        for ci in found {
            self.rs.disc.insert(ci);
            let c = &self.w.realm.cities[ci];
            self.toast(format!("ค้นพบเมือง{} ({}) — ดูบนแผนที่ได้แล้ว", c.name, self.owner_name(self.city_owner(ci))));
        }
        // เคอร์ฟิว
        if let Some(ci) = pc {
            let h = self.hour();
            let day = day_of(self);
            if self.law_on(ci, L_CURFEW) && (h >= 22. || h < 5.) && !self.indoors() && self.rs.curfew_day != day {
                self.rs.curfew_day = day;
                self.crime(ci, "ออกมาเดินช่วงเคอร์ฟิว", 5, 1);
            }
        }
    }
    fn enter_city(&mut self, ci: usize) {
        self.rs.visited.insert(ci);
        self.rs.disc.insert(ci);
        let c = self.w.realm.cities[ci].clone();
        let o = self.city_owner(ci);
        if o == PLAYER_OWN { self.toast(format!("เข้าสู่เมือง{} (เมืองของเรา)", c.name)); return; }
        let k = o as usize;
        let laws: Vec<&str> = self.w.realm.ks[k].laws.iter().map(|&l| LAW_N[l as usize]).collect();
        self.toast(format!("ยินดีต้อนรับสู่{}{} · {} · กฎ: {}", if c.cap { "เมืองหลวง" } else { "เมือง" }, c.name, self.kname(k), laws.join(" / ")));
        if self.rs.ks[k].rep <= -60 || self.rs.ks[k].stance == 3 { self.city_guards_attack(ci); return; }
        let day = day_of(self);
        if self.law_on(ci, L_TOLL) && self.rs.toll_day.get(&ci) != Some(&day) {
            self.rs.toll_day.insert(ci, day);
            if self.p.inv[COIN] >= 5 { self.p.inv[COIN] -= 5; self.rs.ks[k].wealth += 5.; self.toast("จ่ายค่าผ่านประตูเมือง 5 เหรียญ"); }
            else { self.rs.ks[k].rep -= 3; self.toast("ไม่มีเงินจ่ายค่าผ่านประตู — ลอบเข้าเมือง (ชื่อเสียงตก)"); }
        }
    }

    // ---------------------------------------------------------------- ชาวเมือง (เดินไปมา คุยได้)
    fn citizens_tick(&mut self, dt: f32) {
        let (px, py) = (self.p.x, self.p.y);
        let r = &self.w.realm;
        // สร้างชาวเมืองเมื่อเข้าใกล้เมือง · ลบเมื่อไกล
        let near: Vec<usize> = r.cities.iter().enumerate().filter(|(_, c)| ((c.pos.0 * 16) as f32 - px).abs().max(((c.pos.1 * 16) as f32 - py).abs()) < ((c.r + 25) * 16) as f32).map(|(i, _)| i).collect();
        self.rs.cit.retain(|c| near.contains(&c.city));
        if self.w.dun.is_some() { self.rs.cit.clear(); return; }
        for ci in near {
            if self.rs.cit.iter().any(|c| c.city == ci) { continue; }
            let c = &r.cities[ci];
            let n = if c.cap { 12 } else { 7 };
            for i in 0..n {
                let guard = i < if c.cap { 3 } else { 2 };
                let (x, y) = road_point(c, ci * 100 + i);
                self.rs.cit.push(Citizen { path: Path::default(), block: 0., x, y, tx: x, ty: y, look: (ci * 131 + i * 17) as u32, guard, city: ci, wait: i as f32 * 0.3, anim: 0., left: false });
            }
        }
        let mut cit = std::mem::take(&mut self.rs.cit);
        let t = self.time as usize;
        let mut budget = PATH_BUDGET;
        let w = &self.w;
        let blk = |x: f32, y: f32| { let (a, b) = (tfloor(x), tfloor(y - 3.)); w.obj_at(a, b) != 12 && w.solid_at(a, b) };
        for (i, c) in cit.iter_mut().enumerate() {
            c.anim += dt;
            let d = (c.tx - c.x).hypot(c.ty - c.y);
            let new_target = |c: &mut Citizen, extra: usize| {
                let city = &w.realm.cities[c.city];
                let p = road_point(city, t * 7 + i * 13 + extra);
                c.tx = p.0; c.ty = p.1; c.wait = 1.5 + (i % 4) as f32; c.block = 0.; c.path.clear();
            };
            if d < 3. {
                c.wait -= dt;
                if c.wait <= 0. { new_target(c, 0); }
            } else {
                // หาทางอ้อมแผงร้าน/บ่อน้ำ/บ้าน แทนเดินชนตรงๆ
                let (gx, gy) = c.path.steer(w, c.x, c.y, (c.tx, c.ty), Pass::Door, dt, &mut budget);
                let (dx, dy) = (gx - c.x, gy - c.y);
                let l = dx.hypot(dy).max(0.001);
                let sp = if c.guard { 22. } else { 28. };
                let step = (sp * dt).min(l);
                let (nx, ny) = (c.x + dx / l * step, c.y + dy / l * step);
                let (ox, oy) = (c.x, c.y);
                if !blk(nx, c.y) { c.x = nx; }
                if !blk(c.x, ny) { c.y = ny; }
                if (c.x - ox).abs() + (c.y - oy).abs() < sp * dt * 0.2 { c.block += dt; } else { c.block = (c.block - dt).max(0.); }
                // ไปไม่ถึงจริงๆ (เป้าอยู่ในที่ปิด) → เปลี่ยนที่หมาย
                if c.block > 2.5 { new_target(c, 5); }
                if dx.abs() > 0.5 { c.left = dx < 0.; }
            }
        }
        self.rs.cit = cit;
    }
    pub fn citizen_near(&self) -> Option<usize> {
        self.rs.cit.iter().enumerate().map(|(i, c)| (i, (c.x - self.p.x).hypot(c.y - (self.p.y - 3.)))).filter(|x| x.1 < 20.).min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).map(|x| x.0)
    }
    /// คุยกับชาวเมือง: ข่าวลือ/กฎ/การค้า/การเมือง
    pub fn citizen_talk(&mut self, i: usize) {
        let c = self.rs.cit[i].clone();
        let ci = c.city;
        let o = self.city_owner(ci);
        let city = self.w.realm.cities[ci].clone();
        let day = day_of(self);
        let pick = (day as usize * 7 + i * 3 + (self.time as usize / 20)) % 6;
        let line = if c.guard {
            if o == PLAYER_OWN { "ทหาร: เมืองของท่านปลอดภัยดีครับ".to_string() }
            else { let laws: Vec<&str> = self.w.realm.ks[o as usize].laws.iter().map(|&l| LAW_N[l as usize]).collect(); format!("ทหาร: ที่นี่{} มีกฎ — {}", self.kname(o as usize), laws.join(" / ")) }
        } else {
            match pick {
                0 => format!("ชาวเมือง: ที่{}ของถูกคือ{}กับ{} แต่ใครเอา{}มาขายจะได้ราคาดี", city.name, INAME[city.cheap[0]], INAME[city.cheap[1]], INAME[city.want[0]]),
                1 => self.rumor_war(),
                2 => format!("ชาวเมือง: ได้ยินว่า{}มีชื่อเสียง {} แล้วนะ คนเริ่มอยากย้ายไปอยู่", self.my_name(), self.fame()),
                3 => if o != PLAYER_OWN { format!("ชาวเมือง: {}{}ของเรา{}", self.w.realm.ks[o as usize].title, self.w.realm.ks[o as usize].ruler, ["ชอบสงคราม", "ชอบเงิน", "รักษาคำพูด", "เข้มงวดมาก"][self.w.realm.ks[o as usize].trait_ as usize]) } else { "ชาวเมือง: ดีใจที่ได้อยู่ใต้การปกครองของท่าน".to_string() },
                4 => "ชาวเมือง: ถ้าจะเดินทางไกล รถม้ากับคอกม้าอยู่ที่ลานกลางเมือง ข้างๆ ปราสาท".to_string(),
                _ => "ชาวเมือง: ไปที่ปราสาทกลางเมืองถ้าอยากเจรจากับผู้ปกครอง".to_string(),
            }
        };
        self.toast(line);
    }
    fn rumor_war(&self) -> String {
        for a in 0..self.rs.ks.len() { for b in a + 1..self.rs.ks.len() {
            if self.rs.ks[a].war[b] { return format!("ชาวเมือง: {}กับ{}กำลังรบกันอยู่ ระวังตัวนะ", self.kname(a), self.kname(b)); }
            if self.rs.ks[a].ally[b] { return format!("ชาวเมือง: {}เป็นพันธมิตรกับ{}", self.kname(a), self.kname(b)); }
        } }
        "ชาวเมือง: ช่วงนี้บ้านเมืองสงบดี".to_string()
    }

    // ---------------------------------------------------------------- ตลาด
    pub fn price(&self, ci: usize, gi: usize, buy: bool) -> i32 {
        let (item, base) = GOODS[gi];
        let c = &self.w.realm.cities[ci];
        let mut m = if c.cheap.contains(&item) { 0.6 } else if c.want.contains(&item) { 1.6 } else { 1.0 };
        m *= self.rs.px.get(&(ci, gi)).copied().unwrap_or(1.0);
        let o = self.city_owner(ci);
        let rep = if o == PLAYER_OWN { 50 } else { self.rs.ks[o as usize].rep };
        let trade = if self.perk_on(P_TRADE) { 0.1 } else { 0. };
        let v = base as f32 * m;
        let p = if buy { v * (if self.law_on(ci, L_TAX) { 1.15 } else { 1. }) * (1. - rep.clamp(-50, 100) as f32 / 400. - trade) } else { v * 0.7 * (1. + rep.clamp(-50, 100) as f32 / 400. + trade) };
        p.round().max(1.) as i32
    }
    pub fn trade(&mut self, ci: usize, gi: usize, buy: bool, n: i32) {
        let item = GOODS[gi].0;
        let o = self.city_owner(ci);
        for _ in 0..n {
            let p = self.price(ci, gi, buy);
            if buy {
                if self.p.inv[COIN] < p { self.toast("เหรียญไม่พอ"); break; }
                self.p.inv[COIN] -= p; self.p.inv[item] += 1;
                *self.rs.px.entry((ci, gi)).or_insert(1.) *= 1.03;
            } else {
                if self.p.inv[item] < 1 { break; }
                self.p.inv[item] -= 1; self.p.inv[COIN] += p;
                let e = self.rs.px.entry((ci, gi)).or_insert(1.);
                *e = (*e * 0.97).max(0.5);
            }
            if o != PLAYER_OWN { let k = &mut self.rs.ks[o as usize]; k.wealth += p as f32 * 0.2; if gen_range(0, 25) < p { k.rep = (k.rep + 1).min(100); } }
        }
        play(&self.sfx.buy);
    }

    // ---------------------------------------------------------------- การทูต
    /// ทำการทูตกับ k ได้ไหม (อยู่ในปราสาทเมืองของเขา หรือมีทูต)
    pub fn can_diplo(&self, k: usize) -> bool {
        if self.role_count(R_DIPLOMAT) > 0 { return true; }
        self.player_city().map_or(false, |ci| self.city_owner(ci) == k as u8)
    }
    pub fn diplo_gift(&mut self, k: usize) {
        let day = day_of(self);
        if self.rs.ks[k].gift_day == day { self.toast("วันนี้ให้ของขวัญไปแล้ว"); return; }
        if self.p.inv[COIN] < 50 { self.toast("ต้องมี 50 เหรียญ"); return; }
        self.p.inv[COIN] -= 50;
        let s = &mut self.rs.ks[k];
        s.gift_day = day;
        s.wealth += 50.;
        let g = if self.w.realm.ks[k].trait_ == 1 { 12 } else { 8 };
        s.rep = (s.rep + g).min(100);
        self.toast(format!("ส่งของขวัญ 50 เหรียญ — ชื่อเสียงกับ{} +{}", self.kname(k), g));
        play(&self.sfx.buy);
    }
    pub fn diplo_trade(&mut self, k: usize) {
        let s = &self.rs.ks[k];
        if s.stance == 1 { self.rs.ks[k].stance = 0; self.toast("ยกเลิกสัญญาการค้าแล้ว"); return; }
        if s.stance == 3 { self.toast("กำลังรบกันอยู่ ค้าขายไม่ได้"); return; }
        if s.rep < 15 { self.toast(format!("ต้องมีชื่อเสียงกับ{} อย่างน้อย 15 (ตอนนี้ {})", self.kname(k), s.rep)); return; }
        if self.pop() < 3 { self.toast("ต้องมีชาวบ้านอย่างน้อย 3 คน (ต้องมีของผลิตไปขาย)"); return; }
        if s.stance == 0 { self.rs.ks[k].stance = 1; }
        self.toast(format!("ทำสัญญาการค้ากับ{}แล้ว — ทุกเช้าจะขายผลผลิตส่วนเกินในกล่องให้อัตโนมัติ", self.kname(k)));
        play(&self.sfx.quest);
    }
    pub fn diplo_ally(&mut self, k: usize) {
        let s = self.rs.ks[k].clone();
        if s.stance == 2 { self.rs.ks[k].stance = 1; self.rs.ks[k].rep -= 15; self.toast(format!("ถอนตัวจากพันธมิตรกับ{} (ชื่อเสียง -15)", self.kname(k))); return; }
        let need = if self.w.realm.ks[k].trait_ == 0 { 65 } else { 50 };
        if s.stance == 3 || s.stance == 4 { self.toast("ต้องไม่อยู่ในสงคราม/ไม่เป็นเมืองขึ้นก่อน"); return; }
        if s.rep < need { self.toast(format!("ต้องมีชื่อเสียงกับ{} อย่างน้อย {} (ตอนนี้ {})", self.kname(k), need, s.rep)); return; }
        if self.pop() < 5 { self.toast("ต้องมีชาวบ้านอย่างน้อย 5 คน เขาถึงจะมองว่าเราเป็นพันธมิตรที่พึ่งพาได้"); return; }
        if let Some(e) = (0..KN).find(|&j| self.rs.ks[j].stance == 3 && s.ally[j]) { self.toast(format!("{}เป็นพันธมิตรกับ{}ซึ่งเรากำลังรบอยู่", self.kname(k), self.kname(e))); return; }
        self.rs.ks[k].stance = 2;
        self.toast(format!("เป็นพันธมิตรกับ{}แล้ว! เขาจะช่วยป้องกันเมือง — แต่เมื่อเขารบ ต้องส่งความช่วยเหลือ ไม่งั้นแตกหัก", self.kname(k)));
        play(&self.sfx.quest);
    }
    pub fn diplo_war(&mut self, k: usize) {
        if self.rs.ks[k].stance == 3 { self.toast("กำลังรบอยู่แล้ว"); return; }
        if self.pop() < 6 { self.toast("ต้องมีชาวบ้านอย่างน้อย 6 คนถึงจะประกาศสงครามได้"); return; }
        self.declare_war_on_player(k, true);
    }
    pub fn diplo_peace(&mut self, k: usize) {
        let day = day_of(self);
        let s = self.rs.ks[k].clone();
        if s.stance != 3 { return; }
        if day - s.war_day < 2 { self.toast("เพิ่งเริ่มรบ ฝ่ายนั้นยังไม่ยอมเจรจา (รออย่างน้อย 2 วัน)"); return; }
        let cost = ((s.strength - self.defense()).max(0.) * 2.) as i32 + 30;
        if self.p.inv[COIN] < cost { self.toast(format!("เขาเรียกค่าปฏิกรรมสงคราม {} เหรียญ", cost)); return; }
        self.p.inv[COIN] -= cost;
        let st = &mut self.rs.ks[k];
        st.stance = 0; st.army = None; st.rep = st.rep.max(-20); st.army_cd = day + 5; st.wealth += cost as f32;
        self.toast(format!("สงบศึกกับ{}แล้ว (จ่าย {} เหรียญ)", self.kname(k), cost));
        self.news_add(format!("{}สงบศึกกับ{}", self.my_name(), self.kname(k)));
    }
    pub fn pay_ultimatum(&mut self, k: usize, pay: bool) {
        let Some((_, amt)) = self.rs.ks[k].ult else { return };
        self.rs.ks[k].ult = None;
        if pay {
            if self.p.inv[COIN] + self.tax_pool < amt { self.toast("เงินไม่พอจ่าย"); self.rs.ks[k].ult = Some((day_of(self) + 1, amt)); return; }
            let from_pool = self.tax_pool.min(amt);
            self.tax_pool -= from_pool;
            self.p.inv[COIN] -= amt - from_pool;
            self.rs.ks[k].wealth += amt as f32;
            self.rs.ks[k].rep += 5;
            self.rs.ks[k].army_cd = day_of(self) + 6;
            self.toast(format!("จ่ายคำขาด {} เหรียญให้{} — สงบไปอีกพักใหญ่", amt, self.kname(k)));
        } else {
            self.declare_war_on_player(k, false);
        }
    }
    pub fn answer_vassal(&mut self, k: usize, accept: bool) {
        self.rs.ks[k].vas_ask = false;
        if accept {
            self.rs.ks[k].stance = 4;
            self.rs.ks[k].trib_day = day_of(self);
            self.toast(format!("ยอมเป็นเมืองขึ้นของ{} — จ่ายบรรณาการทุก 5 วัน แต่เขาจะคุ้มครองเรา", self.kname(k)));
        } else {
            self.rs.ks[k].rep -= 30;
            self.toast(format!("ปฏิเสธ{} — ระวัง เขาอาจยกทัพมา", self.kname(k)));
            if self.w.realm.ks[k].trait_ == 0 || self.rs.ks[k].rep < -40 { self.declare_war_on_player(k, false); }
        }
    }
    pub fn independence(&mut self, k: usize) {
        if self.rs.ks[k].stance != 4 { return; }
        if self.rs.ks[k].rep >= 60 && self.w.realm.ks[k].trait_ == 2 { self.rs.ks[k].stance = 0; self.toast(format!("{}ยอมรับเอกราชของเราอย่างสันติ", self.kname(k))); return; }
        self.declare_war_on_player(k, true);
        self.toast("ประกาศเอกราช! — สงครามเริ่มแล้ว");
    }
    pub fn send_help(&mut self, k: usize) {
        let Some((_, enemy)) = self.rs.ks[k].help else { return };
        if self.p.inv[COIN] + self.tax_pool < 60 { self.toast("ต้องมี 60 เหรียญ (หรือในคลังภาษี)"); return; }
        let from_pool = self.tax_pool.min(60);
        self.tax_pool -= from_pool;
        self.p.inv[COIN] -= 60 - from_pool;
        self.rs.ks[k].help = None;
        self.rs.ks[k].rep = (self.rs.ks[k].rep + 15).min(100);
        self.rs.ks[k].strength += 8.;
        self.rs.ks[enemy].rep -= 10;
        self.toast(format!("ส่งเสบียง/ทุนสงครามให้{} — พันธมิตรแน่นแฟ้นขึ้น", self.kname(k)));
    }
    pub fn declare_war_on_player(&mut self, k: usize, we_started: bool) {
        let day = day_of(self);
        let s = &mut self.rs.ks[k];
        s.stance = 3; s.war_day = day; s.ult = None; s.vas_ask = false; s.rep = s.rep.min(-30);
        s.army_cd = day + if we_started { 1 } else { 0 };
        let n = self.kname(k);
        self.news_add(format!("{}ประกาศสงครามกับ{}", if we_started { self.my_name() } else { n.clone() }, if we_started { n.clone() } else { self.my_name() }));
        self.toast(format!("สงครามกับ{}! เตรียมป้องกันเมือง — ทัพจะเดินมาภายในไม่กี่วัน (ดูบนแผนที่)", n));
        play(&self.sfx.roar);
        // พันธมิตรของเขาไม่พอใจเรา · พันธมิตรของเรา (ถ้าไม่ใช่พันธมิตรเขา) ไม่พอใจเขา
        for j in 0..KN { if j != k && self.rs.ks[k].ally[j] { self.rs.ks[j].rep -= 15; } }
    }

    // ---------------------------------------------------------------- ตั้งอาณาจักร
    pub fn found_reqs(&self) -> Vec<(String, bool)> {
        let inside = self.town_center().and_then(|t| self.claim_of(t));
        vec![
            (format!("ประชากร {}/12", self.pop()), self.pop() >= 12),
            ("มีปราสาท".to_string(), self.has_castle()),
            (format!("กำแพงเมือง {}/20", self.walls()), self.walls() >= 20),
            (format!("เหรียญ {}/300 (จ่ายตอนสถาปนา)", self.p.inv[COIN]), self.p.inv[COIN] >= 300),
            (format!("ชื่อเสียงเมือง {}/60", self.fame()), self.fame() >= 60),
            (match inside { Some(k) if self.rs.ks[k].stance == 4 => format!("ยังเป็นเมืองขึ้นของ{} — ต้องประกาศเอกราชก่อน", self.kname(k)), Some(k) => format!("อยู่ในเขตแดน{} — สถาปนาแล้วเขาจะถือว่าแยกตัว (ถ้าชื่อเสียงกับเขา < 60 = สงคราม)", self.kname(k)), None => "อยู่นอกเขตแดนอาณาจักรอื่น".to_string() }, !self.rs.ks.iter().any(|k| k.stance == 4)),
        ]
    }
    pub fn found_kingdom(&mut self) {
        if self.rs.founded { return; }
        if !self.found_reqs().iter().all(|r| r.1) { self.toast("ยังไม่ครบเงื่อนไข"); return; }
        self.p.inv[COIN] -= 300;
        self.rs.founded = true;
        self.rs.name_i = (self.w.seed.unsigned_abs() as usize + day_of(self) as usize) % 8;
        let name = self.my_name();
        self.news_add(format!("สถาปนา{}!", name));
        self.toast(format!("สถาปนา{}สำเร็จ! ออกกฎหมายได้ในแท็บอาณาจักร", name));
        play(&self.sfx.lvl);
        if let Some(k) = self.town_center().and_then(|t| self.claim_of(t)) {
            if self.rs.ks[k].rep < 60 { self.declare_war_on_player(k, false); } else { self.rs.ks[k].rep -= 20; }
        }
        for s in self.rs.ks.iter_mut() { s.rep -= 5; }
    }

    // ---------------------------------------------------------------- ผู้อพยพ
    pub fn accept_petition(&mut self, i: usize, yes: bool) {
        if i >= self.rs.petitions.len() { return; }
        let p = self.rs.petitions.remove(i);
        if !yes { self.toast(format!("ปฏิเสธคำขอของ{}", NPC_NAMES[p.name as usize])); return; }
        // ต้องมีเตียงว่างในบ้านที่น่าอยู่ ≥3 ดาว
        let bed = self.free_beds().into_iter().find(|b| self.w.room_from(*b).map_or(false, |r| r.comfort >= 3));
        let Some(bed) = bed else { self.toast("ขุนนางต้องการเตียงว่างในบ้าน 3 ดาวขึ้นไป"); self.rs.petitions.insert(i, p); return; };
        let id = self.npc_next;
        self.npc_next += 1;
        let mut n = Npc { id, name: p.name, look: p.look, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: ST_HOME, hearts: 3, mood: 70, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, p.name) };
        if p.rank == 3 { n.soc.tr[T_BRAVE] = 85; }
        self.npcs.push(n);
        let o = self.city_owner(p.city);
        let spy = o != PLAYER_OWN && self.rs.ks[o as usize].rep < 0 && hash(id as i32, p.day, 9300) < 0.3;
        self.rs.origins.insert(id, Origin { city: p.city, rank: p.rank, perk: p.perk, salary: p.salary, loyalty: 70, spy });
        if p.rank == 3 { self.rs.manual.insert(id, R_GENERAL); }
        self.assign_roles_pub();
        self.toast(format!("{} {} ย้ายมาแล้ว — {}", RANK_N[p.rank as usize], NPC_NAMES[p.name as usize], PERK_N[p.perk as usize]));
        play(&self.sfx.quest);
    }

    // ---------------------------------------------------------------- รุ่งเช้า: การเมือง การค้า สงคราม ผู้อพยพ
    pub fn realm_dawn(&mut self) {
        if !self.rs.inited { let r = self.w.realm.clone(); self.rs.init(&r); }
        let day = day_of(self);
        let n = self.rs.ks.len();
        let h = |a: i32, b: i32| hash(day.wrapping_mul(97).wrapping_add(a), b, 9400) as f32;
        // ราคาตลาดค่อยๆ กลับสู่ปกติ
        for v in self.rs.px.values_mut() { *v += (1. - *v) * 0.25; }
        // ---- อาณาจักร AI
        for a in 0..n {
            if !self.rs.ks[a].alive { continue; }
            let s = &mut self.rs.ks[a];
            s.strength = (s.strength + s.base * 0.03).min(s.base * 1.3);
            s.wealth += 20. + if self.w.realm.ks[a].trait_ == 1 { 25. } else { 0. };
            for b in 0..n {
                if a == b || !self.rs.ks[b].alive { continue; }
                let mut d = ((h(a as i32, b as i32) - 0.5) * 6.) as i32;
                if self.w.realm.ks[a].trait_ == 0 { d -= 1; }
                if self.w.realm.ks[a].trait_ == 1 && self.w.realm.ks[b].trait_ == 1 { d += 1; }
                let v = (self.rs.ks[a].rel[b] + d).clamp(-100, 100);
                self.rs.ks[a].rel[b] = v;
                self.rs.ks[b].rel[a] = v;
            }
        }
        for a in 0..n { for b in a + 1..n {
            if !self.rs.ks[a].alive || !self.rs.ks[b].alive { continue; }
            let rel = self.rs.ks[a].rel[b];
            if !self.rs.ks[a].war[b] && rel < -55 && !self.rs.ks[a].ally[b] {
                self.rs.ks[a].war[b] = true; self.rs.ks[b].war[a] = true;
                self.news_add(format!("{}ประกาศสงครามกับ{}", self.kname(a), self.kname(b)));
                // พันธมิตรถูกขอให้ช่วย
                for (x, y) in [(a, b), (b, a)] {
                    if self.rs.ks[x].stance == 2 { self.rs.ks[x].help = Some((day + 4, y)); self.toast(format!("พันธมิตร{}ขอความช่วยเหลือในสงครามกับ{} (ดูแท็บอาณาจักร)", self.kname(x), self.kname(y))); }
                    for z in 0..n { if z != x && z != y && self.rs.ks[z].ally[x] && !self.rs.ks[z].war[y] {
                        if h(z as i32, 55) < 0.6 { self.rs.ks[z].war[y] = true; self.rs.ks[y].war[z] = true; self.news_add(format!("{}เข้าร่วมสงครามช่วยพันธมิตร{}", self.kname(z), self.kname(x))); }
                        else { self.rs.ks[z].ally[x] = false; self.rs.ks[x].ally[z] = false; self.rs.ks[z].rel[x] -= 30; self.rs.ks[x].rel[z] -= 30; self.news_add(format!("{}ไม่ยอมช่วย{} — พันธมิตรแตกหัก!", self.kname(z), self.kname(x))); }
                    } }
                }
            } else if self.rs.ks[a].war[b] {
                // รบกันรายวัน
                let (sa, sb) = (self.rs.ks[a].strength, self.rs.ks[b].strength);
                let ra = sa * (0.7 + h(a as i32, 3) * 0.6);
                let rb = sb * (0.7 + h(b as i32, 4) * 0.6);
                let (win, lose) = if ra > rb { (a, b) } else { (b, a) };
                self.rs.ks[lose].strength *= 0.92;
                self.rs.ks[win].strength *= 0.97;
                if self.rs.ks[win].strength > self.rs.ks[lose].strength * 1.4 {
                    // ยึดเมืองย่อยของฝ่ายแพ้
                    if let Some(ci) = (0..self.w.realm.cities.len()).find(|&ci| self.rs.owner[ci] == lose as u8 && !self.w.realm.cities[ci].cap) {
                        self.rs.owner[ci] = win as u8;
                        self.news_add(format!("{}ยึดเมือง{}จาก{}", self.kname(win), self.w.realm.cities[ci].name, self.kname(lose)));
                    } else if self.rs.ks[win].strength > self.rs.ks[lose].strength * 2.2 {
                        let cap = self.w.realm.ks[lose].cap;
                        if self.rs.owner[cap] == lose as u8 { self.rs.owner[cap] = win as u8; }
                        self.rs.ks[lose].alive = false;
                        for x in 0..n { self.rs.ks[x].war[lose] = false; self.rs.ks[lose].war[x] = false; self.rs.ks[x].ally[lose] = false; }
                        self.news_add(format!("{}ล่มสลาย! {}ยึดเมืองหลวงได้", self.kname(lose), self.kname(win)));
                        continue;
                    }
                }
                if day % 3 == 0 && (h(a as i32 + b as i32, 9) < 0.25 || (sa < self.rs.ks[a].base * 0.5 && sb < self.rs.ks[b].base * 0.5)) {
                    self.rs.ks[a].war[b] = false; self.rs.ks[b].war[a] = false;
                    self.rs.ks[a].rel[b] = -20; self.rs.ks[b].rel[a] = -20;
                    self.news_add(format!("{}กับ{}สงบศึกกัน", self.kname(a), self.kname(b)));
                }
            } else if !self.rs.ks[a].ally[b] && rel > 60 {
                self.rs.ks[a].ally[b] = true; self.rs.ks[b].ally[a] = true;
                self.news_add(format!("{}กับ{}ทำสนธิสัญญาเป็นพันธมิตร", self.kname(a), self.kname(b)));
            } else if self.rs.ks[a].ally[b] && rel < 20 {
                self.rs.ks[a].ally[b] = false; self.rs.ks[b].ally[a] = false;
                self.news_add(format!("พันธมิตร{}กับ{}แตกหัก", self.kname(a), self.kname(b)));
            }
        } }
        // ---- ความสัมพันธ์กับเรา
        let tc = self.town_center();
        let pop = self.pop();
        let def = self.defense();
        let wealth = self.p.inv[COIN] + self.tax_pool;
        let diplo = self.role_count(R_DIPLOMAT) > 0 || self.perk_on(P_DIPLO);
        for k in 0..n {
            if !self.rs.ks[k].alive { if self.rs.ks[k].stance != 0 { self.rs.ks[k].stance = 0; } continue; }
            let st = self.rs.ks[k].clone();
            if diplo && st.stance != 3 { self.rs.ks[k].rep = (st.rep + 1).min(100); }
            // ขอความช่วยเหลือหมดเวลา
            if let Some((dl, enemy)) = st.help { if day > dl {
                self.rs.ks[k].help = None;
                self.rs.ks[k].rep -= 30;
                self.news_add(format!("{}ผิดหวังที่เราไม่ช่วยในสงครามกับ{}", self.kname(k), self.kname(enemy)));
                if self.rs.ks[k].rep < 30 && st.stance == 2 { self.rs.ks[k].stance = 1; self.news_add(format!("พันธมิตรระหว่าง{}กับ{}แตกหัก!", self.my_name(), self.kname(k))); self.toast(format!("พันธมิตรกับ{}แตกหัก เพราะไม่ไปช่วยรบ", self.kname(k))); }
            } }
            // พันธมิตรของเรารบกับใคร → ขอความช่วยเหลือ
            if st.stance == 2 && st.help.is_none() { if let Some(e) = (0..n).find(|&j| self.rs.ks[k].war[j]) { if h(k as i32, 21) < 0.3 { self.rs.ks[k].help = Some((day + 4, e)); self.toast(format!("พันธมิตร{}ขอความช่วยเหลือในสงครามกับ{}", self.kname(k), self.kname(e))); } } }
            // พันธมิตรแตกหักถ้าความสัมพันธ์ตก
            if st.stance == 2 && self.rs.ks[k].rep < 25 { self.rs.ks[k].stance = 1; self.news_add(format!("{}ยกเลิกพันธมิตรกับ{}", self.kname(k), self.my_name())); }
            // สัญญาการค้า: ขายของส่วนเกินในกล่อง
            if st.stance == 1 || st.stance == 2 { self.trade_pact_day(k); }
            // บรรณาการ (เมืองขึ้น)
            if st.stance == 4 && day - st.trib_day >= 5 {
                let amt = 25 + pop as i32 * 3;
                self.rs.ks[k].trib_day = day;
                if self.tax_pool + self.p.inv[COIN] >= amt { let fp = self.tax_pool.min(amt); self.tax_pool -= fp; self.p.inv[COIN] -= amt - fp; self.rs.ks[k].wealth += amt as f32; self.news_add(format!("ส่งบรรณาการ {} เหรียญให้{}", amt, self.kname(k))); }
                else { self.rs.ks[k].rep -= 25; self.toast(format!("ไม่มีเงินส่งบรรณาการให้{}! (ชื่อเสียง -25)", self.kname(k))); if self.rs.ks[k].rep < -30 { self.declare_war_on_player(k, false); } }
            }
            if peaceful() || tc.is_none() || pop < 3 { continue; }
            // คำขาดหมดเวลา
            if let Some((dl, _)) = st.ult { if day > dl { self.rs.ks[k].ult = None; self.toast(format!("ไม่จ่ายคำขาดของ{}!", self.kname(k))); self.declare_war_on_player(k, false); continue; } }
            // สงคราม: ส่งทัพ
            if st.stance == 3 {
                if st.army.is_none() && day >= st.army_cd {
                    let cap = self.w.realm.cities[self.w.realm.ks[k].cap].pos;
                    let size = (st.strength * 0.18).max(6.) as u32;
                    self.rs.ks[k].army = Some(Army { x: cap.0 as f32, y: cap.1 as f32, n: size, k, vs: None });
                    self.rs.ks[k].army_cd = day + 4;
                    self.news_add(format!("ทัพ{} {} นาย ยกออกจากเมืองหลวงมุ่งหน้ามาที่เมืองเรา!", self.kname(k), size));
                    self.toast(format!("ทัพ{}กำลังมา! ดูบนแผนที่ (ไอคอนดาบไขว้)", self.kname(k)));
                }
                continue;
            }
            // เขตแดน: ตั้งเมืองในเขตเขา → เรียกให้เป็นเมืองขึ้น
            let inside = tc.and_then(|t| self.claim_of(t)) == Some(k);
            if inside && st.stance == 0 && !st.vas_ask && pop >= 4 && !self.rs.founded && st.rep < 50 {
                self.rs.ks[k].vas_ask = true;
                self.toast(format!("{}{}ส่งสาส์นมา: เมืองเราอยู่ในเขตแดนของเขา ให้ยอมเป็นเมืองขึ้น (ดูแท็บอาณาจักร)", self.w.realm.ks[k].title, self.w.realm.ks[k].ruler));
                continue;
            }
            // อาณาจักรที่แข็งกว่า + ไม่ชอบเรา + เรารวย → ยื่นคำขาด
            let hostile = self.w.realm.ks[k].trait_ == 0 || st.rep < -30;
            if hostile && st.ult.is_none() && st.stance == 0 && day >= st.army_cd && st.strength * 0.25 > def * 1.2 && wealth > 120 && h(k as i32, 31) < 0.25 {
                let amt = (wealth as f32 * 0.3).max(40.) as i32;
                self.rs.ks[k].ult = Some((day + 3, amt));
                self.toast(format!("คำขาดจาก{}: จ่าย {} เหรียญภายใน 3 วัน ไม่งั้นยกทัพมา! (แท็บอาณาจักร)", self.kname(k), amt));
                self.news_add(format!("{}ยื่นคำขาดเรียกเงิน {} เหรียญจากเรา", self.kname(k), amt));
            }
        }
        // ---- เมืองที่เรายึดได้ส่งรายได้
        let owned = self.rs.owner.iter().filter(|&&o| o == PLAYER_OWN).count() as i32;
        if owned > 0 { self.tax_pool += owned * 15; }
        // ---- ผู้อพยพ / ขุนนาง / เงินเดือน
        self.migrants_day();
        self.jobs_day();
    }
    fn trade_pact_day(&mut self, k: usize) {
        let cap = self.w.realm.ks[k].cap;
        let mut income = 0;
        let mut sold = 0;
        let keys: Vec<(i32, i32)> = self.chests.keys().copied().collect();
        'o: for gi in 0..GOODS.len() {
            let item = GOODS[gi].0;
            if matches!(item, GEM | POTION | ARROW) { continue; }
            for ck in &keys {
                let have = self.chests[ck][item];
                let keep = 6;
                if have > keep {
                    let m = (have - keep).min(8 - sold);
                    let p = self.price(cap, gi, false);
                    self.chests.get_mut(ck).unwrap()[item] -= m;
                    income += p * m;
                    sold += m;
                    if sold >= 8 { break 'o; }
                }
            }
        }
        if self.perk_on(P_TRADE) { income = income * 3 / 2; }
        if income > 0 {
            self.tax_pool += income;
            self.rs.ks[k].wealth += income as f32 * 0.3;
            if day_of(self) % 3 == 0 { self.rs.ks[k].rep = (self.rs.ks[k].rep + 1).min(100); }
            self.news_add(format!("ขายผลผลิต {} ชิ้นให้{}ตามสัญญาการค้า ได้ {} เหรียญเข้าคลัง", sold, self.kname(k), income));
        }
    }
    fn migrants_day(&mut self) {
        let day = day_of(self);
        // เงินเดือนขุนนาง + ความภักดี
        let ids: Vec<u32> = self.rs.origins.keys().copied().collect();
        for id in ids {
            let Some(i) = self.idx(id) else { self.rs.origins.remove(&id); continue };
            let o = self.rs.origins[&id].clone();
            if o.rank < 2 { continue; }
            let paid = if self.tax_pool >= o.salary { self.tax_pool -= o.salary; true } else if self.p.inv[COIN] >= o.salary { self.p.inv[COIN] -= o.salary; true } else { false };
            let comfy = self.w.room_from(self.npcs[i].bed).map_or(0, |r| r.comfort);
            let mut l = o.loyalty + if paid { 2 } else { -15 } + if comfy >= 3 { 1 } else { -5 };
            l = l.clamp(0, 100);
            self.rs.origins.get_mut(&id).unwrap().loyalty = l;
            if !paid { self.news_add(format!("ไม่มีเงินเดือนจ่าย{} (ความภักดี {})", NPC_NAMES[self.npcs[i].name as usize], l)); }
            if l < 25 {
                let nm = NPC_NAMES[self.npcs[i].name as usize];
                let back = self.w.realm.cities[o.city].name;
                self.news_add(format!("{} {} ไม่พอใจ ย้ายกลับไปเมือง{}", RANK_N[o.rank as usize], nm, back));
                self.npc_dismiss(id);
                self.rs.origins.remove(&id);
            }
        }
        if self.perk_on(P_HEAL) { for n in self.npcs.iter_mut() { n.mood = (n.mood + 2).min(100); } }
        // ผู้อพยพใหม่
        if self.pop() == 0 || self.free_beds().is_empty() { return; }
        let war = self.rs.ks.iter().any(|k| k.stance == 3);
        let comfort: f32 = { let v: Vec<u8> = self.residents().filter_map(|n| self.w.room_from(n.bed)).map(|r| r.comfort).collect(); if v.is_empty() { 0. } else { v.iter().map(|&c| c as f32).sum::<f32>() / v.len() as f32 } };
        let mut p = 0.1 + self.fame() as f32 / 400. + comfort * 0.03 - if war { 0.12 } else { 0. };
        if self.rs.laws & ML_LOWTAX != 0 { p *= 1.4; }
        if self.rs.laws & ML_HIGHTAX != 0 { p *= 0.6; }
        if self.rs.laws & ML_OPEN != 0 { p *= 1.6; }
        if (hash(day, 1, 9500) as f32) > p.min(0.6) { return; }
        let cities = self.w.realm.cities.len();
        let ci = (hash(day, 2, 9500) * cities as f64) as usize % cities;
        if self.city_owner(ci) == PLAYER_OWN { return; }
        let name = (hash(day, 3, 9500) * NPC_NAMES.len() as f64) as u8 % NPC_NAMES.len() as u8;
        let look = (hash(day, 4, 9500) * npc::LOOKS as f64) as u8;
        let r = hash(day, 5, 9500) as f32;
        let noble_p = 0.12 + if self.fame() >= 50 { 0.1 } else { 0. };
        if r < noble_p + 0.08 && self.rs.petitions.len() < 3 {
            let rank = if r < noble_p { 2 } else { 3 };
            let perk = if rank == 3 { P_GENERAL } else { (hash(day, 6, 9500) * 6.) as u8 % 6 };
            let salary = 4 + (hash(day, 7, 9500) * 5.) as i32;
            self.rs.petitions.push(Petition { rank, perk, salary, name, look, city: ci, day });
            self.toast(format!("{} {} จากเมือง{} ขอย้ายมาอยู่เมืองเรา (ดูแท็บอาณาจักร)", RANK_N[rank as usize], NPC_NAMES[name as usize], self.w.realm.cities[ci].name));
            return;
        }
        // ชาวเมือง/ช่างฝีมือ ย้ายเข้าเลย
        let bed = self.free_beds()[0];
        let id = self.npc_next;
        self.npc_next += 1;
        self.npcs.push(Npc { id, name, look, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: ST_HOME, hearts: 2, mood: 60, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, name) });
        let rank = if r < 0.55 { 1 } else { 0 };
        self.rs.origins.insert(id, Origin { city: ci, rank, perk: 0, salary: 0, loyalty: 60, spy: false });
        self.assign_roles_pub();
        self.news_add(format!("{} ({}) ย้ายมาจากเมือง{}", NPC_NAMES[name as usize], RANK_N[rank as usize], self.w.realm.cities[ci].name));
        self.toast(format!("มีผู้อพยพจากเมือง{}ย้ายเข้ามาอยู่!", self.w.realm.cities[ci].name));
    }
    /// ผลผลิตอาชีพทุกเช้า
    fn jobs_day(&mut self) {
        let list: Vec<(u8, (i32, i32), u32)> = self.residents().filter(|n| n.mood >= 20 && Game::willing(n) && !n.child).map(|n| (n.role, n.bed, n.id)).collect();
        let skilled = |g: &Game, id: u32| g.rs.origins.get(&id).map_or(false, |o| o.rank == 1);
        for (role, bed, id) in list {
            let m = if skilled(self, id) { 2 } else { 1 };
            match role {
                R_LUMBER => self.give_to_chest(bed, WOOD, 5 * m),
                R_MINER => { self.give_to_chest(bed, STONE, 4 * m); if hash(day_of(self), id as i32, 9600) < 0.15 { self.give_to_chest(bed, GEM, 1); } }
                R_HUNTER => { let w = self.water_within(bed.0, bed.1, 20); self.give_to_chest(bed, if w { FISH } else { MEAT }, 2 * m); }
                R_COOK => self.cook_day(bed),
                R_HEALER => { for n in self.npcs.iter_mut() { n.mood = (n.mood + 2).min(100); } }
                R_CLERK => { self.tax_pool += 4 * m; }
                _ => {}
            }
        }
        // กฎหมายเมืองเรา
        let l = self.rs.laws;
        for n in self.npcs.iter_mut().filter(|n| n.state == ST_HOME) {
            if l & ML_LOWTAX != 0 { n.mood = (n.mood + 2).min(100); }
            if l & ML_HIGHTAX != 0 { n.mood -= 3; }
            if l & ML_MILITIA != 0 { n.mood -= 1; }
            if l & ML_CURFEW != 0 { n.mood -= 1; }
            if l & ML_NOWEAPON != 0 { n.mood = (n.mood + 1).min(100); }
        }
        if self.tier() >= 3 || self.rs.founded {
            let mut tax = self.pop() as i32 * 2;
            if l & ML_LOWTAX != 0 { tax = tax * 6 / 10; }
            if l & ML_HIGHTAX != 0 { tax = tax * 3 / 2; }
            if self.perk_on(P_TREASURY) { tax = tax * 13 / 10; }
            self.tax_pool += tax;
        }
    }
    fn cook_day(&mut self, bed: (i32, i32)) {
        let keys: Vec<(i32, i32)> = self.chests.keys().copied().filter(|k| (k.0 - bed.0).abs() <= 16 && (k.1 - bed.1).abs() <= 16).collect();
        for k in keys {
            let c = self.chests.get_mut(&k).unwrap();
            if c[FISH] >= 2 { c[FISH] -= 2; c[DISH_FISH] += 1; return; }
            if c[MEAT] >= 1 && c[SHROOM] >= 1 { c[MEAT] -= 1; c[SHROOM] -= 1; c[DISH_STEW] += 1; return; }
        }
    }

    // ---------------------------------------------------------------- สงคราม: ทัพมาถึง
    fn army_arrive(&mut self, k: usize) {
        let Some(a) = self.rs.ks[k].army.take() else { return };
        let Some(tc) = self.town_center() else { return };
        let center = (tc.0 as f32 * 16. + 8., tc.1 as f32 * 16. + 8.);
        let near = (self.p.x - center.0).hypot(self.p.y - center.1) < 60. * 16. && self.w.dun.is_none();
        if near && !peaceful() {
            let from = (a.x * 16., a.y * 16.);
            let ally_cut = if self.rs.ks.iter().any(|s| s.stance == 2 && s.alive) { 0.7 } else { 1.0 };
            let spy = self.rs.origins.iter().any(|(_, o)| o.spy && self.city_owner(o.city) == k as u8);
            let total = ((a.n as f32 * ally_cut * if spy { 1.25 } else { 1. }) as u32).clamp(4, 26);
            self.rs.siege = Some(Siege { k, total, spawned: 0, t: 0., hold: 0., center, from });
            self.toast(format!("ทัพ{}มาถึงแล้ว {} นาย! ป้องกันลานกลางเมือง — ถ้าศัตรูยึดได้ 25 วินาที = แพ้", self.kname(k), total));
            play(&self.sfx.roar);
        } else {
            self.resolve_battle(k, a.n as f32);
        }
    }
    /// คิดผลศึกแบบไม่ต้องเล่น (ผู้เล่นไม่อยู่ที่เมือง)
    pub fn resolve_battle(&mut self, k: usize, attackers: f32) {
        let spy = self.rs.origins.iter().any(|(_, o)| o.spy && self.city_owner(o.city) == k as u8);
        let d = self.defense() * if spy { 0.8 } else { 1. } * (0.8 + gen_range(0., 0.4));
        let a = attackers * (0.8 + gen_range(0., 0.4));
        if d >= a { self.battle_won(k); } else { self.battle_lost(k); }
    }
    fn battle_won(&mut self, k: usize) {
        self.rs.siege_wins += 1;
        self.rs.ks[k].strength *= 0.8;
        self.rs.ks[k].army_cd = day_of(self) + 5;
        self.news_add(format!("{}ต้านทัพ{}ได้สำเร็จ!", self.my_name(), self.kname(k)));
        self.toast(format!("ชนะ! ต้านทัพ{}ได้ — ชื่อเสียงเมืองเพิ่มขึ้น", self.kname(k)));
        play(&self.sfx.lvl);
        for n in self.npcs.iter_mut() { n.mood = (n.mood + 10).min(100); }
        self.reveal_spies(k);
        if self.rs.ks[k].strength < self.rs.ks[k].base * 0.5 && gen_range(0., 1.) < 0.5 {
            self.rs.ks[k].stance = 0;
            self.news_add(format!("{}ขอสงบศึก", self.kname(k)));
        }
    }
    fn battle_lost(&mut self, k: usize) {
        let take = self.tax_pool / 2;
        self.tax_pool -= take;
        let mut looted = take;
        for c in self.chests.values_mut() { let t = c[COIN] * 4 / 10; c[COIN] -= t; looted += t; }
        self.rs.ks[k].wealth += looted as f32;
        for n in self.npcs.iter_mut() { n.mood -= 20; }
        self.rs.ks[k].stance = 4;
        self.rs.ks[k].trib_day = day_of(self);
        self.rs.ks[k].army_cd = day_of(self) + 6;
        self.news_add(format!("เมืองแตก! {}ยึดเมืองเรา ปล้นไป {} เหรียญ — เราต้องเป็นเมืองขึ้น", self.kname(k), looted));
        self.toast(format!("แพ้ศึก! {}ปล้น {} เหรียญ และบังคับให้เป็นเมืองขึ้น (ส่งบรรณาการทุก 5 วัน)", self.kname(k), looted));
        play(&self.sfx.hurt);
        self.reveal_spies(k);
        if self.rs.founded && gen_range(0., 1.) < 0.5 { self.rs.founded = false; self.news_add("อาณาจักรของเราถูกยุบ กลับเป็นเมืองขึ้น".to_string()); }
    }
    fn reveal_spies(&mut self, k: usize) {
        let spies: Vec<u32> = self.rs.origins.iter().filter(|(_, o)| o.spy && self.city_owner(o.city) == k as u8).map(|(id, _)| *id).collect();
        for id in spies {
            if let Some(i) = self.idx(id) { let nm = NPC_NAMES[self.npcs[i].name as usize]; self.news_add(format!("{}เป็นสายลับของ{}! หนีไปแล้ว", nm, self.kname(k))); self.toast(format!("{}คือสายลับของ{}!", nm, self.kname(k))); }
            self.npc_dismiss(id);
            self.rs.origins.remove(&id);
        }
    }
    /// ศึกป้องกันเมืองแบบเล่นจริง
    fn siege_tick(&mut self, dt: f32) {
        let Some(mut s) = self.rs.siege.take() else { return };
        s.t += dt;
        // ผู้เล่นหนีไปไกล → คิดผลส่วนที่เหลือ
        if (self.p.x - s.center.0).hypot(self.p.y - s.center.1) > 75. * 16. || self.w.dun.is_some() {
            let left = (s.total - s.spawned) as f32 + self.en.iter().filter(|e| e.k == EK::Soldier && !e.dead).count() as f32;
            self.en.retain(|e| e.k != EK::Soldier);
            self.resolve_battle(s.k, left * 1.4);
            return;
        }
        if s.spawned < s.total && s.t >= s.spawned as f32 * 1.4 {
            // เกิดที่ขอบทางที่ทัพมา ~22 ช่องจากกลางเมือง
            let (dx, dy) = (s.from.0 - s.center.0, s.from.1 - s.center.1);
            let l = dx.hypot(dy).max(1.);
            let side = (s.spawned as f32 * 1.7).sin() * 90.;
            let (x, y) = (s.center.0 + dx / l * 22. * 16. - dy / l * side, s.center.1 + dy / l * 22. * 16. + dx / l * side);
            // จุดเกิดติดน้ำ/ต้นไม้ → หาที่ว่างใกล้ๆ (เดิมข้ามไปเลย = ทหารหายฟรี)
            let spot = (0..40).map(|i| { let a = i as f32 * 2.4; let r = (i as f32).sqrt() * 16.; (x + a.cos() * r, y + a.sin() * r) }).find(|&(x, y)| !collide(&self.w, x, y) && !self.w.solid_at(tfloor(x), tfloor(y - 3.)));
            if let Some((x, y)) = spot { self.make_enemy(EK::Soldier, x, y, 1.0 + s.total as f32 / 30., s.k, false); }
            s.spawned += 1;
        }
        let alive: Vec<(f32, f32)> = self.en.iter().filter(|e| e.k == EK::Soldier && !e.dead).map(|e| (e.x, e.y)).collect();
        let holding = alive.iter().any(|e| (e.0 - s.center.0).hypot(e.1 - s.center.1) < 6. * 16.);
        if holding { s.hold += dt; } else { s.hold = (s.hold - dt * 0.5).max(0.); }
        if s.hold >= 25. {
            self.en.retain(|e| e.k != EK::Soldier);
            self.battle_lost(s.k);
            return;
        }
        if s.spawned >= s.total && alive.is_empty() {
            self.battle_won(s.k);
            self.p.xp += 20;
            self.check_lv();
            return;
        }
        self.rs.siege = Some(s);
    }

    // ---------------------------------------------------------------- ม้า / รถม้า
    pub fn buy_horse(&mut self) {
        if self.rs.horse { self.toast("มีม้าแล้ว"); return; }
        if self.p.inv[COIN] < 120 { self.toast("ม้าราคา 120 เหรียญ"); return; }
        self.p.inv[COIN] -= 120;
        self.rs.horse = true;
        self.rs.riding = true;
        self.toast("ได้ม้าแล้ว! ขี่เร็วขึ้น 70% · กด H หรือปุ่ม \"ม้า\" เพื่อขึ้น/ลง");
        play(&self.sfx.quest);
    }
    pub fn toggle_ride(&mut self) {
        if !self.rs.horse { return; }
        if !self.rs.riding && !self.can_ride() { self.toast("ขี่ม้าตรงนี้ไม่ได้ (ในอาคาร/ดันเจี้ยน/เรือ)"); return; }
        self.rs.riding = !self.rs.riding;
        self.sit = None;
        self.toast(if self.rs.riding { "ขึ้นม้า" } else { "ลงจากม้า" });
    }
    fn can_ride(&self) -> bool { self.w.dun.is_none() && !self.indoors() && !self.on_boat() }
    fn horse_tick(&mut self) {
        if self.rs.riding && (!self.rs.horse || !self.can_ride() || self.sit.is_some()) { self.rs.riding = false; }
    }
    pub fn ride_mul(&self) -> f32 { if self.rs.riding { 1.7 } else { 1. } }
    /// รถม้าโดยสาร: เดินทางไปเมืองที่เคยไป (เสียเงิน + เวลาผ่านไป)
    pub fn coach_cost(&self, from: usize, to: usize) -> (i32, f32) {
        let (a, b) = (self.w.realm.cities[from].pos, self.w.realm.cities[to].pos);
        let d = ((a.0 - b.0) as f32).hypot((b.1 - a.1) as f32);
        ((5. + d / 15.) as i32, d / 120.)
    }
    pub fn coach_go(&mut self, from: usize, to: usize) {
        let (cost, hours) = self.coach_cost(from, to);
        if self.p.inv[COIN] < cost { self.toast(format!("ค่ารถม้า {} เหรียญ", cost)); return; }
        let o = self.city_owner(to);
        if o != PLAYER_OWN && (self.rs.ks[o as usize].stance == 3 || self.rs.ks[o as usize].rep <= -60) { self.toast("รถม้าไม่ยอมไปเมืองของฝ่ายที่เป็นศัตรูกับเรา"); return; }
        self.p.inv[COIN] -= cost;
        let c = self.w.realm.cities[to].clone();
        let (nx, ny) = ((c.pos.0 + 3) as f32 * 16. + 8., (c.pos.1 + 4) as f32 * 16. + 12.);
        self.time += hours / 24. * DAYLEN;
        self.p.x = nx;
        self.p.y = ny;
        self.en.clear();
        self.proj.clear();
        self.tp_comp();
        self.mini_t = 99.;
        self.toast(format!("นั่งรถม้า {:.0} ชั่วโมง ถึงเมือง{}แล้ว (ค่าโดยสาร {} เหรียญ)", hours.max(1.), c.name, cost));
        play(&self.sfx.door);
    }

    // ---------------------------------------------------------------- เซฟ
    pub fn realm_save(&self) -> String {
        let mut s = String::new();
        for (i, k) in self.rs.ks.iter().enumerate() {
            let a = k.army.as_ref();
            let mask = |v: &[bool; KN]| v.iter().enumerate().map(|(i, &b)| (b as u32) << i).sum::<u32>();
            s += &format!("rk {} {:.1} {:.1} {:.1} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {:.1} {:.1} {} {}\n", i, k.strength, k.base, k.wealth, k.rep, k.rel.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(","), mask(&k.war), mask(&k.ally), k.alive as u8, k.stance,
                k.ult.map_or(-1, |u| u.0), k.ult.map_or(0, |u| u.1), k.vas_ask as u8, k.help.map_or(-1, |h| h.0), k.help.map_or(0, |h| h.1), k.gift_day, k.trib_day, k.war_day, k.army_cd, a.is_some() as u8, a.map_or(0., |a| a.x), a.map_or(0., |a| a.y), a.map_or(0, |a| a.n), a.and_then(|a| a.vs).map_or(-1, |v| v as i32));
        }
        s += &format!("rc {}\n", self.rs.owner.iter().map(|o| o.to_string()).collect::<Vec<_>>().join(","));
        let list = |h: &HashSet<usize>| { let mut v: Vec<usize> = h.iter().copied().collect(); v.sort(); v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",") };
        s += &format!("rd {} {}\n", list(&self.rs.disc), list(&self.rs.visited));
        s += &format!("rp {} {} {} {} {} {}\n", self.rs.founded as u8, self.rs.name_i, self.rs.laws, self.rs.siege_wins, self.rs.horse as u8, self.rs.riding as u8);
        for (id, o) in &self.rs.origins { s += &format!("ro {} {} {} {} {} {} {}\n", id, o.city, o.rank, o.perk, o.salary, o.loyalty, o.spy as u8); }
        for (id, r) in &self.rs.manual { s += &format!("rr {} {}\n", id, r); }
        for p in &self.rs.petitions { s += &format!("rq {} {} {} {} {} {} {}\n", p.rank, p.perk, p.salary, p.name, p.look, p.city, p.day); }
        s
    }
    pub fn realm_load(&mut self, tag: &str, r: &[&str]) {
        if !self.rs.inited { let rr = self.w.realm.clone(); self.rs.init(&rr); }
        let f = |i: usize| r.get(i).and_then(|x| x.parse::<f32>().ok()).unwrap_or(0.);
        let n = |i: usize| r.get(i).and_then(|x| x.parse::<i32>().ok()).unwrap_or(0);
        match tag {
            "rk" => {
                let i = n(0) as usize;
                if i >= self.rs.ks.len() { return; }
                let rel: Vec<i32> = r.get(5).map_or(vec![], |x| x.split(',').filter_map(|v| v.parse().ok()).collect());
                let k = &mut self.rs.ks[i];
                k.strength = f(1); k.base = f(2); k.wealth = f(3); k.rep = n(4);
                for (j, v) in rel.iter().enumerate().take(KN) { k.rel[j] = *v; }
                let (wm, am) = (n(6) as u32, n(7) as u32);
                for j in 0..KN { k.war[j] = wm >> j & 1 == 1; k.ally[j] = am >> j & 1 == 1; }
                k.alive = n(8) == 1; k.stance = n(9) as u8;
                k.ult = if n(10) >= 0 { Some((n(10), n(11))) } else { None };
                k.vas_ask = n(12) == 1;
                k.help = if n(13) >= 0 { Some((n(13), n(14) as usize)) } else { None };
                k.gift_day = n(15); k.trib_day = n(16); k.war_day = n(17); k.army_cd = n(18);
                k.army = if n(19) == 1 { Some(Army { x: f(20), y: f(21), n: n(22) as u32, k: i, vs: if n(23) >= 0 { Some(n(23) as usize) } else { None } }) } else { None };
            }
            "rc" => { let v: Vec<u8> = r.first().map_or(vec![], |x| x.split(',').filter_map(|v| v.parse().ok()).collect()); if v.len() == self.rs.owner.len() { self.rs.owner = v; } }
            "rd" => {
                let p = |s: Option<&&str>| -> HashSet<usize> { s.map_or(HashSet::new(), |x| x.split(',').filter_map(|v| v.parse().ok()).collect()) };
                self.rs.disc.extend(p(r.first()));
                self.rs.visited = p(r.get(1));
            }
            "rp" => { self.rs.founded = n(0) == 1; self.rs.name_i = n(1) as usize; self.rs.laws = n(2) as u8; self.rs.siege_wins = n(3); self.rs.horse = n(4) == 1; self.rs.riding = n(5) == 1; }
            "ro" => { self.rs.origins.insert(n(0) as u32, Origin { city: n(1) as usize, rank: n(2) as u8, perk: n(3) as u8, salary: n(4), loyalty: n(5), spy: n(6) == 1 }); }
            "rr" => { self.rs.manual.insert(n(0) as u32, n(1) as u8); }
            "rq" => { self.rs.petitions.push(Petition { rank: n(0) as u8, perk: n(1) as u8, salary: n(2), name: n(3) as u8, look: n(4) as u8, city: n(5) as usize, day: n(6) }); }
            _ => {}
        }
    }
}

/// จุดสุ่มบนถนน/ลานในเมือง (ให้ชาวเมืองเดิน)
fn road_point(c: &City, seed: usize) -> (f32, f32) {
    let hh = |k: i32| hash(c.pos.0 + seed as i32 * 3, c.pos.1 + k, 9700) as f32;
    let along = ((hh(1) * 2. - 1.) * (c.r - 2) as f32) as i32;
    let (dx, dy) = match (hh(2) * 3.) as i32 { 0 => (along, (hh(3) * 3.) as i32 - 1), 1 => ((hh(3) * 3.) as i32 - 1, along), _ => ((hh(4) * 9.) as i32 - 4, (hh(5) * 4.) as i32 - 1) };
    ((c.pos.0 + dx) as f32 * 16. + 8., (c.pos.1 + dy) as f32 * 16. + 12.)
}

// ---------------------------------------------------------------- อาชีพ
pub const R_BUILDER: u8 = 4;
pub const R_LUMBER: u8 = 5;
pub const R_MINER: u8 = 6;
pub const R_HUNTER: u8 = 7;
pub const R_COOK: u8 = 8;
pub const R_HEALER: u8 = 9;
pub const R_SOLDIER: u8 = 10;
pub const R_GENERAL: u8 = 11;
pub const R_DIPLOMAT: u8 = 12;
pub const R_CLERK: u8 = 13;
pub const ROLE_DESC: [&str; 14] = [
    "ไม่มีงานประจำ",
    "รดน้ำ เก็บผัก ปลูกซ้ำ ผลผลิตเข้ากล่องใกล้บ้าน",
    "ฟันมอนสเตอร์ที่เข้ามาใกล้บ้าน · ป้องกันเมือง +2",
    "ขายของให้เรา (ร้านพ่อค้าหมู่บ้าน)",
    "ช่วยก่อสร้าง — ไซต์ก่อสร้างเร็วขึ้น และสร้างต่อได้แม้เราไม่อยู่",
    "ตัดไม้ ไม้ 5 ชิ้น/วัน เข้ากล่องใกล้บ้าน",
    "ขุดหิน หิน 4/วัน (บางวันได้อัญมณี)",
    "ล่าสัตว์/ตกปลา เนื้อหรือปลา 2/วัน",
    "ทำอาหารจากวัตถุดิบในกล่อง (ปลา→ปลาย่าง · เนื้อ+เห็ด→ซุป)",
    "ดูแลสุขภาพ ชาวบ้านทุกคนความสุข +2/วัน",
    "ทหารประจำการ · ป้องกันเมือง +4 · ช่วยรบตอนศึก",
    "บัญชาการทัพ · ป้องกันเมือง ×1.4 (ต้องกล้าหาญ)",
    "ทูต · เจรจาการทูตได้จากเมืองเรา + ชื่อเสียงทุกอาณาจักร +1/วัน (ต้องช่างพูด)",
    "เสมียนคลัง · เข้าคลังภาษี +4 เหรียญ/วัน",
];
/// เลือกอาชีพเองได้ไหม (เงื่อนไขตามนิสัย/ประสบการณ์)
pub fn role_ok(n: &Npc, role: u8, rs: &RealmState) -> Result<(), &'static str> {
    if n.child { return Err("เด็กยังทำงานไม่ได้"); }
    let t = &n.soc.tr;
    let vet = rs.origins.get(&n.id).map_or(false, |o| o.rank == 3);
    match role {
        R_GENERAL if t[T_BRAVE] < 60 && !vet => Err("ต้องกล้าหาญ ≥ 60 หรือเป็นแม่ทัพเก่า"),
        R_DIPLOMAT if t[T_SOCIAL] < 55 => Err("ต้องช่างพูด ≥ 55"),
        R_HEALER if t[T_KIND] < 50 => Err("ต้องใจดี ≥ 50"),
        R_SOLDIER | 2 if t[T_BRAVE] < 30 => Err("ขี้ขลาดเกินไปสำหรับงานรบ"),
        3 => Err("พ่อค้าเกมเลือกให้เอง (คนโลภ+ช่างพูดที่สุด)"),
        _ => Ok(()),
    }
}
pub fn role_parts(role: u8) -> [usize; 3] {
    match role { 2 | R_SOLDIER | R_GENERAL => skin::ROLE_PARTS[2], 3 | R_DIPLOMAT | R_CLERK => skin::ROLE_PARTS[3], 1 | R_LUMBER | R_MINER | R_HUNTER | R_BUILDER => skin::ROLE_PARTS[1], _ => skin::ROLE_PARTS[0] }
}

// ---------------------------------------------------------------- วาด
pub fn draw_citizens_list(g: &Game, list: &mut Vec<(f32, usize)>) {
    for (i, c) in g.rs.cit.iter().enumerate() { list.push((c.y, i)); }
}
pub fn draw_citizen(g: &Game, sp: &Sprites, v: &View, i: usize) {
    let c = &g.rs.cit[i];
    let (sx, sy) = ((c.x - v.cx).round(), (c.y - v.cy).round());
    let body = Body::generate(c.look, c.look % 2 == 0);
    let moving = (c.tx - c.x).hypot(c.ty - c.y) > 2.;
    let f = if moving { ((c.anim * 7.) as usize) % 2 } else { 0 };
    prect(v, sx - 4., sy - 1., 8., 2., Color::new(0., 0., 0., 0.25));
    let parts = if c.guard { skin::ROLE_PARTS[2] } else { skin::ROLE_PARTS[(c.look % 4) as usize] };
    skin::draw_figure(sp, &body, parts, (sx - 6.) * v.s, (sy - 15.) * v.s, v.s, 0, f, c.left, WHITE);
    if c.guard {
        // ผ้าคาดสีอาณาจักร
        let col = hx(g.owner_color(g.city_owner(c.city)));
        prect(v, sx - 3., sy - 9., 6., 1., col);
    }
}
/// ม้าใต้ตัวผู้เล่น (วาดด้วยสี่เหลี่ยม)
pub fn draw_horse(v: &View, sx: f32, sy: f32, t: f32, moving: bool, left: bool, front: bool) {
    let leg = if moving { (t * 14.).sin() * 2. } else { 0. };
    let c = hx(0x8a5a30);
    let d = hx(0x5a3418);
    if front {
        prect(v, sx - 4., sy - 9., 8., 7., c);
        prect(v, sx - 3., sy - 3., 2., 3. + leg.max(0.), d);
        prect(v, sx + 1., sy - 3., 2., 3. + (-leg).max(0.), d);
        prect(v, sx - 2., sy - 13., 4., 5., c);
        prect(v, sx - 1., sy - 14., 2., 2., d);
        return;
    }
    let f = if left { -1. } else { 1. };
    prect(v, sx - 8., sy - 10., 16., 6., c);
    for (k, lx) in [-7.0f32, -4., 3., 6.].iter().enumerate() {
        let dl = if k % 2 == 0 { leg } else { -leg };
        prect(v, sx + lx * f - 0.5, sy - 4., 2., 4. + dl.max(0.) * 0.5, d);
    }
    prect(v, sx + 7. * f - if f < 0. { 3. } else { 0. }, sy - 15., 3., 7., c);
    prect(v, sx + 9. * f - if f < 0. { 4. } else { 0. }, sy - 16., 4., 3., c);
    prect(v, sx - 9. * f - if f < 0. { -1. } else { 1. }, sy - 10., 2., 6., d);
    prect(v, sx + 6. * f - if f < 0. { 2. } else { 0. }, sy - 16., 2., 5., d);
}

/// แผนที่: เมือง (ชื่อ อาณาจักร ชื่อเสียง) เขตแดน กองทัพ
pub fn draw_realm_map(g: &Game, font: &Font, mr: Rect, ctr: (i32, i32), sc: i32, big: bool, u: f32) {
    if g.w.dun.is_some() || !g.rs.inited { return; }
    let r = &g.w.realm;
    if big {
        // เขตแดน (วงรอบเมืองหลวง)
        for (ci, c) in r.cities.iter().enumerate().filter(|(_, c)| c.cap) {
            let o = g.city_owner(ci);
            if o == PLAYER_OWN { continue; }
            let k = o as usize;
            if !g.rs.ks[k].alive { continue; }
            let (mx, my, _) = map_pt(c.pos.0, c.pos.1, ctr, sc, mr);
            let rad = r.ks[k].claim as f32 / sc as f32 / 120. * mr.w;
            let col = hx(r.ks[k].color);
            let cc = Color::new(col.r, col.g, col.b, 0.45);
            let n = 72;
            for i in 0..n {
                let (a0, a1) = (i as f32 / n as f32 * std::f32::consts::TAU, (i + 1) as f32 / n as f32 * std::f32::consts::TAU);
                let (x0, y0, x1, y1) = (mx + a0.cos() * rad, my + a0.sin() * rad, mx + a1.cos() * rad, my + a1.sin() * rad);
                let inside = |x: f32, y: f32| x >= mr.x && x <= mr.x + mr.w && y >= mr.y && y <= mr.y + mr.h;
                if inside(x0, y0) && inside(x1, y1) { draw_line(x0, y0, x1, y1, 2., cc); }
            }
        }
    }
    for (ci, c) in r.cities.iter().enumerate() {
        if !g.rs.disc.contains(&ci) { continue; }
        let o = g.city_owner(ci);
        let col = hx(g.owner_color(o));
        let (mx, my, ok) = map_pt(c.pos.0, c.pos.1, ctr, sc, mr);
        let rr = if big { (if c.cap { 12. } else { 9. }) * u } else { (if c.cap { 6. } else { 4.5 }) * u };
        if ok {
            draw_rectangle(mx - rr, my - rr * 0.6, rr * 2., rr * 1.3, hx(0x1d1f2b));
            draw_rectangle_lines(mx - rr, my - rr * 0.6, rr * 2., rr * 1.3, 2., col);
            // ป้อม 3 ยอด
            for k in [-0.6f32, 0., 0.6] { draw_rectangle(mx + rr * k - rr * 0.2, my - rr * (if k == 0. { 1.2 } else { 0.9 }), rr * 0.4, rr * 0.7, col); }
            if big {
                let txt = format!("{}{} · ชื่อเสียง {}", c.name, if c.cap { " (เมืองหลวง)" } else { "" }, g.city_fame(ci));
                let fs = 11. * u;
                let d = measure_text(&txt, Some(font), fs as u16, 1.);
                draw_rectangle(mx - d.width / 2. - 3., my + rr * 0.8, d.width + 6., fs * 1.2, Color::new(0., 0., 0., 0.6));
                text_c(font, &txt, mx, my + rr * 0.8 + fs, fs, col);
            }
        } else if big && c.cap {
            // เมืองหลวงที่อยู่นอกแผนที่: ลูกศรที่ขอบ + ชื่อ + ระยะ
            edge_arrow(mr, ctr, sc, c.pos, col, c.name, font, u * 0.85, true);
        }
    }
    // กองทัพ
    for (k, s) in g.rs.ks.iter().enumerate() {
        let Some(a) = &s.army else { continue };
        let (mx, my, ok) = map_pt(a.x as i32, a.y as i32, ctr, sc, mr);
        if !ok { continue; }
        let rr = if big { 9. * u } else { 5. * u };
        let col = hx(r.ks[k].color);
        draw_circle(mx, my, rr, hx(0x401010));
        draw_line(mx - rr * 0.6, my - rr * 0.6, mx + rr * 0.6, my + rr * 0.6, 2.5, col);
        draw_line(mx + rr * 0.6, my - rr * 0.6, mx - rr * 0.6, my + rr * 0.6, 2.5, col);
        if big { text_c(font, &format!("ทัพ {} นาย", a.n), mx, my - rr - 3. * u, 11. * u, hx(0xff8080)); }
    }
}
/// แถบสถานะศึกป้องกันเมือง
pub fn draw_siege_hud(g: &Game, font: &Font, u: f32) {
    let Some(s) = &g.rs.siege else { return };
    let alive = g.en.iter().filter(|e| e.k == EK::Soldier && !e.dead).count() as u32;
    let left = s.total - s.spawned + alive;
    let sw = screen_width();
    let w = (sw * 0.5).min(420. * u);
    let (x, y) = (sw / 2. - w / 2., 92. * u);
    draw_rectangle(x, y, w, 36. * u, Color::new(0.1, 0., 0., 0.75));
    text_c(font, &format!("ศึกป้องกันเมือง: ทัพ{} · เหลือ {}/{} นาย", g.kname(s.k), left, s.total), sw / 2., y + 14. * u, 13. * u, hx(0xffb0b0));
    draw_rectangle(x + 6. * u, y + 22. * u, w - 12. * u, 8. * u, hx(0x301818));
    draw_rectangle(x + 6. * u, y + 22. * u, (w - 12. * u) * (s.hold / 25.).min(1.), 8. * u, hx(0xff4040));
}
