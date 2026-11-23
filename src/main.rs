//! Pixel Realm — เกม open world 2D pixel (Rust + macroquad)
//! รันได้บน Windows / Linux / macOS และ build เป็น Android APK ได้

#[allow(unused_imports)]
use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

mod art;
mod assets;
mod extra;
mod farm;
mod guide;
mod audio;
mod base;
mod body;
mod boss;
mod camp;
mod combat;
mod dungeon;
mod menu;
mod needs;
mod npc;
mod prefs;
mod rpg;
mod skin;
mod social;
mod trees;
mod land;
mod seasons;
mod animals;
mod quests;
mod balance;
mod mcui;
mod v11;
mod saves;
mod finale;
mod house;
use house::*;
mod psy;
use psy::*;
mod ambience;
use ambience::*;
mod nav;
use nav::*;
mod realm;
mod path;
use path::*;
mod vfx;
mod prof;
mod v19;
use v19::*;
mod work;
use work::*;
mod advisor;
use advisor::*;
mod saga;
use saga::*;
mod living;
use living::*;
mod war;
use war::*;
mod hud;
mod mover;
use mover::*;
use realm::*;
use extra::*;
use farm::*;
use guide::*;
use audio::*;
use base::*;
use body::*;
use boss::*;
use camp::*;
use combat::*;
use dungeon::*;
use menu::*;
use needs::*;
use npc::*;
use rpg::*;
use skin::*;
use social::*;
use trees::*;
use land::*;
use seasons::*;
use animals::*;
use quests::*;
use balance::*;
use mcui::*;
use v11::*;
use saves::*;
use finale::*;

// ======================= ค่าคงที่ =======================
const DAYLEN: f32 = 240.0;
const TN: [&str; 23] = [
    "ทะเลลึก", "แหล่งน้ำ", "ชายหาด", "ทุ่งหญ้า", "ป่าทึบ", "เขตหินผา", "ภูเขา", "ดินแดนหิมะ", "ทะเลทราย", "ดันเจี้ยน", "กำแพงดันเจี้ยน", "ดินไถ", "ดินเปียก", "พื้นถ้ำ", "ผนังถ้ำ",
    "หนองน้ำ", "สะวันนา", "เถ้าภูเขาไฟ", "ลาวา", "พื้นไม้", "พื้นหิน", "ถนน", "สะพาน",
];
const TCOL: [u32; 23] = [
    0x1d4f91, 0x2f7fd0, 0xe6d29a, 0x5fae4a, 0x3f8a3a, 0x8a8a8a, 0x5c5450, 0xeef4f8, 0xe8b86a, 0x3a3a4c, 0x14141e, 0x7a5230, 0x52341e, 0x3a343c, 0x1a1618,
    0x4f6136, 0xc4b25c, 0x4a4446, 0xe0500e, 0xb07a44, 0x8e8a84, 0xb0905e, 0x8a5a30,
];
// tiles: 0 deep,1 water,2 sand,3 grass,4 forest,5 stone,6 mountain,7 snow,8 desert
const SOLIDT: [bool; 23] = [true, true, false, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false];
// objects: 1 tree,2 bush,3 flower,4 rock,5 ore,6 chest,7 cactus,8 pine,9 campfire,
//   10 กำแพงไม้,11 กำแพงหิน,12 ประตูปิด,13 ประตูเปิด,14 กล่องเก็บของ,15 พ่อค้า,16 เต็นท์
//   37 ปราสาท,38 กำแพงเมือง,39 บ่อน้ำ
//   17 โต๊ะตีเหล็ก,18 คบเพลิง,19 ทางเข้าดันเจี้ยน,20 ทางออกดันเจี้ยน,21 หนามหุบ,22 หนามโผล่,23 ป้อมลูกไฟ,24 หีบสมบัติหายาก
const OSOLID: [bool; 86] = [false, true, true, false, true, true, true, true, true, true, true, true, true, false, true, true, true, true, true, false, false, false, false, true, true, true, true, false, false, false, false, false, false, true, true, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, false, true, false, true, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, true, false, false, true, true, true, true, true, true, false, true, true, true, false, true, true];
const OHP: [i32; 86] = [0, 3, 1, 0, 3, 4, 1, 2, 3, 2, 5, 8, 4, 4, 5, 0, 0, 6, 1, 0, 0, 0, 0, 0, 0, 0, 5, 1, 1, 1, 1, 1, 1, 4, 8, 0, 0, 20, 8, 6, 3, 3, 3, 4, 2, 3, 6, 1, 0, 0, 1, 3, 1, 3, 1, 8, 10, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 5, 1, 1, 3, 1, 1, 3, 1, 1, 5, 3, 2, 1, 99, 99, 99, 99, 99, 99];

const WOOD: usize = 0;
const STONE: usize = 1;
const BERRY: usize = 2;
const GEM: usize = 3;
const COIN: usize = 4;
const FIRE: usize = 5;
const ARROW: usize = 6;
const GEL: usize = 7;
const CORE: usize = 8;
const POTION: usize = 9;
const BP_ARM: usize = 10;
const BP_WPN: usize = 11;
const SHARD: usize = 12;
const MEAT: usize = 13;
const SHROOM: usize = 14;
const HERB: usize = 15;
const DISH_STEW: usize = 16;
const DISH_SPEED: usize = 17;
const DISH_CRIT: usize = 18;
const DISH_FISH: usize = 19;
const FISH: usize = 20;
const SEED_CORN: usize = 21;
const SEED_CARROT: usize = 22;
const CORN: usize = 23;
const CARROT: usize = 24;
const NI: usize = 25;
// ดัชนีไอคอน (0..11 ตรงกับไอเทม)
const ICON_SWORD: usize = 25;
const IC_BOW: usize = 26;
const IC_STAFF: usize = 27;
const IC_HELM: usize = 28;
const IC_CHEST: usize = 29;
const IC_BOOT: usize = 30;
const IC_WWALL: usize = 31;
const IC_SWALL: usize = 32;
const IC_DOOR: usize = 33;
const IC_BOX: usize = 34;
const IC_ANVIL: usize = 35;
const IC_TORCH: usize = 36;
const IC_POT: usize = 37;
const IC_HOE: usize = 38;
const IC_BED: usize = 39;
const IC_ATABLE: usize = 40;
const IC_CAN: usize = 41;
const INAME: [&str; NI] = ["ไม้", "หิน", "ผลไม้", "อัญมณี", "เหรียญ", "กองไฟ", "ลูกธนู", "เจลราชาสไลม์", "แกนโกเลม", "ยาฟื้นฟู", "พิมพ์เขียวเกราะ", "พิมพ์เขียวอาวุธ", "เศษดาว", "เนื้อสัตว์", "เห็ด", "สมุนไพร", "สเต็กเนื้อย่าง", "ซุปเห็ดสายลม", "ชาสมุนไพรคมเขี้ยว", "ปลาย่าง", "ปลา", "เมล็ดข้าวโพด", "เมล็ดแครอท", "ข้าวโพด", "แครอท"];

const SWN: [&str; 6] = ["มือเปล่า", "ดาบไม้", "ดาบหิน", "ดาบอัญมณี", "ดาบแกนโกเลม", "ดาบคริสตัลดาว"];
const SWD: [i32; 6] = [1, 2, 3, 5, 8, 12];
const SWC: [u32; 6] = [0xffffff, 0xd8a060, 0xd8d8e0, 0x6ff8ff, 0xff9a40, 0xc880ff];
const DV: [(f32, f32); 4] = [(0., 1.), (0., -1.), (1., 0.), (-1., 0.)];

// ======================= noise =======================
fn hash(x: i32, y: i32, s: i32) -> f64 {
    let mut h = x
        .wrapping_mul(374761393)
        .wrapping_add(y.wrapping_mul(668265263))
        .wrapping_add(s.wrapping_mul(982451653));
    h = (h ^ ((h as u32) >> 13) as i32).wrapping_mul(1274126177);
    h ^= ((h as u32) >> 16) as i32;
    (h as u32) as f64 / 4294967296.0
}
fn vnoise(x: f64, y: f64, s: i32) -> f64 {
    let (xf0, yf0) = (x.floor(), y.floor());
    let (xf, yf) = (x - xf0, y - yf0);
    let (xi, yi) = (xf0 as i32, yf0 as i32);
    let u = xf * xf * (3. - 2. * xf);
    let v = yf * yf * (3. - 2. * yf);
    let a = hash(xi, yi, s);
    let b = hash(xi + 1, yi, s);
    let c = hash(xi, yi + 1, s);
    let d = hash(xi + 1, yi + 1, s);
    a + (b - a) * u + (c - a) * v + (a - b - c + d) * u * v
}
fn fbm(x: f64, y: f64, s: i32) -> f64 {
    let (mut t, mut a, mut f, mut n) = (0., 1., 1., 0.);
    for i in 0..5 {
        t += vnoise(x * f, y * f, s.wrapping_add(i * 131)) * a;
        n += a;
        a *= 0.5;
        f *= 2.;
    }
    t / n
}
fn rnd(a: i32, b: i32) -> i32 {
    gen_range(a, b + 1)
}
fn hx(c: u32) -> Color {
    Color::from_hex(c)
}

// ======================= world =======================
struct World {
    seed: i32,
    chunks: RefCell<HashMap<(i32, i32), Box<[u8; 1024]>>>,
    tmod: HashMap<(i32, i32), u8>,
    removed: HashSet<(i32, i32)>,
    placed: HashMap<(i32, i32), u8>,
    camps: RefCell<HashMap<(i32, i32), Option<(i32, i32)>>>,
    duns: RefCell<HashMap<(i32, i32), Option<(i32, i32)>>>,
    wps: RefCell<HashMap<(i32, i32), Option<(i32, i32)>>>,
    dun: Option<Box<Dungeon>>,
    volc: RefCell<HashMap<(i32, i32), Option<(i32, i32)>>>,
    marks: RefCell<HashMap<(i32, i32), Option<(i32, i32, u8)>>>,
    camp_kinds: RefCell<HashMap<(i32, i32), u8>>,
    /// อาณาจักร/เมืองในโลก (สร้างจาก seed ตอนสร้างโลก)
    realm: std::rc::Rc<Realm>,
}
impl World {
    fn new(seed: i32) -> Self {
        let mut w = World { seed, chunks: RefCell::new(HashMap::new()), tmod: HashMap::new(), removed: HashSet::new(), placed: HashMap::new(), camps: RefCell::new(HashMap::new()), duns: RefCell::new(HashMap::new()), wps: RefCell::new(HashMap::new()), dun: None, volc: RefCell::new(HashMap::new()), marks: RefCell::new(HashMap::new()), camp_kinds: RefCell::new(HashMap::new()), realm: Default::default() };
        w.realm = std::rc::Rc::new(Realm::build(&w));
        // ไทล์ที่สร้างก่อนมีเมืองต้องสร้างใหม่ (ให้ผังเมืองทับ)
        w.chunks.borrow_mut().clear();
        w.camps.borrow_mut().clear();
        w
    }
    fn gen_tile(&self, x: i32, y: i32) -> u8 {
        if let Some(ci) = self.realm.city_at(x, y) { return self.realm.layout(ci, x, y).0; }
        // v23 ถนนระหว่างเมือง · ข้ามน้ำ/ลาวา = สะพาน
        if !self.realm.roads.is_empty() && self.realm.roads.contains(&(x, y)) {
            let b = self.gen_base(x, y);
            return if b < 2 || b == T_LAVA { T_BRIDGE } else { T_ROAD };
        }
        self.gen_base(x, y)
    }
    /// ภูมิประเทศธรรมชาติ (ไม่รวมเมือง/ถนน)
    fn gen_base(&self, x: i32, y: i32) -> u8 {
        if let Some(t) = self.volcano_tile(x, y) { return t; }
        let s = self.seed;
        let e = fbm(x as f64 / 48., y as f64 / 48., s);
        let m = fbm(x as f64 / 70. + 500., y as f64 / 70. + 500., s.wrapping_add(77));
        if e < 0.36 { return 0; }
        if e < 0.42 { return 1; }
        if e < 0.445 { return 2; }
        if e < 0.60 {
            if m < 0.38 { return 8; }
            if m < 0.43 { return T_SAVANNA; }
            if m > 0.62 && e < 0.5 { return T_SWAMP; }
            if m > 0.56 { return 4; }
            return 3;
        }
        if e < 0.655 { return 5; }
        if e < 0.685 { return if hash(x, y, s.wrapping_add(3)) < 0.18 { 5 } else { 6 }; }
        7
    }
    fn tile_at(&self, x: i32, y: i32) -> u8 {
        if let Some(d) = &self.dun { return d.tile(x, y); }
        if !self.tmod.is_empty() {
            if let Some(&m) = self.tmod.get(&(x, y)) { return m; }
        }
        let (cx, cy) = (x >> 5, y >> 5);
        let mut ch = self.chunks.borrow_mut();
        if !ch.contains_key(&(cx, cy)) {
            // เต็ม: ทิ้งเฉพาะชังก์ที่อยู่ไกล (เดิมล้างทิ้งหมด → ต้องสร้างใหม่ทั้งจอ+แผนที่ = ค้าง)
            if ch.len() > 900 { ch.retain(|k, _| (k.0 - cx).abs() <= 12 && (k.1 - cy).abs() <= 12); }
            let mut c = Box::new([0u8; 1024]);
            for j in 0..32 {
                for i in 0..32 {
                    c[(j * 32 + i) as usize] = self.gen_tile(cx * 32 + i, cy * 32 + j);
                }
            }
            ch.insert((cx, cy), c);
        }
        ch[&(cx, cy)][(((y - (cy << 5)) << 5) + (x - (cx << 5))) as usize]
    }
    fn nat_obj(&self, tx: i32, ty: i32) -> u8 {
        if let Some(ci) = self.realm.city_at(tx, ty) { return self.realm.layout(ci, tx, ty).1; }
        if let Some(o) = self.camp_obj(tx, ty) { return o; }
        if self.dun_in_cell(tx.div_euclid(DCELL), ty.div_euclid(DCELL)) == Some((tx, ty)) { return if self.is_cave_cell(tx.div_euclid(DCELL), ty.div_euclid(DCELL)) { 36 } else { 19 }; }
        if self.wp_in_cell(tx.div_euclid(WPC), ty.div_euclid(WPC)) == Some((tx, ty)) { return 25; }
        if let Some(o) = self.mark_obj(tx, ty) { return o; }
        let t = self.tile_at(tx, ty);
        let r = hash(tx, ty, self.seed.wrapping_add(5));
        match t {
            3 => if r < 0.035 { self.tree_at(tx, ty, 3) } else if r < 0.05 { 2 } else if r < 0.13 { 3 } else if r > 0.9993 { 6 } else { 0 },
            4 => if r < 0.27 { self.tree_at(tx, ty, 4) } else if r < 0.32 { 2 } else if r < 0.35 { 3 } else if r > 0.998 { 6 } else { 0 },
            2 => if r < 0.012 { T_PALM } else if r > 0.9985 { 6 } else { 0 },
            8 => if r < 0.025 { 7 } else if r < 0.031 { T_DEAD } else if r > 0.998 { 6 } else if (0.40..0.75).contains(&r) && self.water_near2(tx, ty) { T_PALM } else { 0 },
            5 => if r < 0.10 { 4 } else if r < 0.125 { 5 } else if r < 0.145 { self.tree_at(tx, ty, 5) } else if r > 0.997 { 6 } else { 0 },
            7 => if r < 0.06 { self.tree_at(tx, ty, 7) } else if r < 0.075 { 5 } else if r < 0.085 { T_DEAD } else if r > 0.996 { 6 } else { 0 },
            T_SWAMP | T_SAVANNA | T_ASH => self.land_nat(t, tx, ty, r),
            _ => 0,
        }
    }
    fn obj_at(&self, tx: i32, ty: i32) -> u8 {
        if let Some(d) = &self.dun { return d.obj(tx, ty); }
        if let Some(&p) = self.placed.get(&(tx, ty)) { return p; }
        if self.removed.contains(&(tx, ty)) { return 0; }
        self.nat_obj(tx, ty)
    }
    fn solid_at(&self, tx: i32, ty: i32) -> bool {
        SOLIDT[self.tile_at(tx, ty) as usize] || OSOLID[self.obj_at(tx, ty) as usize]
    }
    fn near_fire(&self, tx: i32, ty: i32, r: i32) -> Option<(i32, i32)> {
        if self.dun.is_some() { return None; }
        self.placed.iter().find(|(k, &v)| v == 9 && (k.0 - tx).abs() <= r && (k.1 - ty).abs() <= r).map(|(k, _)| *k)
    }
}
fn tfloor(v: f32) -> i32 {
    (v / 16.).floor() as i32
}
/// กล่องชนของวัตถุในช่อง (พิกเซลในช่อง x0,y0,x1,y1) — ต้นไม้ชนแค่ลำต้น พุ่มไม้/หินเล็กกว่าช่อง
/// จึงแทรกผ่านช่องว่างระหว่างต้นไม้ได้เหมือนที่ตาเห็น · สิ่งก่อสร้าง/กำแพงยังเต็มช่อง (ห้องต้องปิดสนิท)
pub fn obj_box(o: u8) -> (f32, f32, f32, f32) {
    if o == 47 { return (6., 10., 10., 16.); } // ต้นกล้า
    if trees::is_tree(o) { return (5., 8., 11., 16.); } // ลำต้น
    match o {
        2 => (3., 7., 13., 16.),          // พุ่มไม้
        4 | 5 => (2., 5., 14., 16.),      // ก้อนหิน/แร่
        7 => (4., 6., 12., 16.),          // กระบองเพชร
        _ => (0., 0., 16., 16.),
    }
}
fn collide(w: &World, x: f32, y: f32) -> bool {
    let (x0, x1, y0, y1) = (x - 4., x + 4., y - 4., y);
    let (l, r, t, b) = (tfloor(x0), tfloor(x1), tfloor(y0), tfloor(y1));
    for ty in t..=b {
        for tx in l..=r {
            if SOLIDT[w.tile_at(tx, ty) as usize] { return true; }
            let o = w.obj_at(tx, ty);
            if o == 0 || !OSOLID[o as usize] { continue; }
            let (a, c, bb, d) = obj_box(o);
            let (ox0, oy0) = (tx as f32 * 16., ty as f32 * 16.);
            if x1 > ox0 + a && x0 < ox0 + bb && y1 > oy0 + c && y0 < oy0 + d { return true; }
        }
    }
    false
}

// ======================= sprites / atlas =======================
fn spr(rows: &[&str], pal: &[(char, u32)]) -> Image {
    let w = rows.iter().map(|r| r.chars().count()).max().unwrap_or(1);
    let mut im = Image::gen_image_color(w as u16, rows.len() as u16, BLANK);
    for (y, r) in rows.iter().enumerate() {
        for (x, ch) in r.chars().enumerate() {
            if let Some(&(_, c)) = pal.iter().find(|p| p.0 == ch) {
                im.set_pixel(x as u32, y as u32, hx(c));
            }
        }
    }
    im
}
enum Extra { None, Waves(u32, i32), Tufts(u32), Mountain }
/// ไทล์ดินไถ: พื้นสีน้ำตาลมีร่องแถวเป็นแนวนอน
fn soil_tex(base: u32, furrow: u32, speck: u32, s: i32) -> Image {
    let mut im = mk_tex(base, &[speck, furrow], 12, s, Extra::None);
    for y in [3u32, 7, 11, 15] {
        for x in 0..16u32 {
            im.set_pixel(x, y, hx(furrow));
        }
    }
    im
}
fn mk_tex(base: u32, specks: &[u32], n: i32, s: i32, ex: Extra) -> Image {
    let mut im = Image::gen_image_color(16, 16, hx(base));
    for i in 0..n {
        let px = (hash(i, s, 11) * 16.) as u32;
        let py = (hash(s, i, 23) * 16.) as u32;
        im.set_pixel(px, py, hx(specks[i as usize % specks.len()]));
    }
    match ex {
        Extra::None => {}
        Extra::Waves(c, f) => {
            for i in 0..3 {
                let px = ((hash(i, s, 5) * 12.) as i32 + f * 2) % 14;
                let py = (hash(s, i, 7) * 14.) as i32 + 1;
                for k in 0..3 { im.set_pixel((px + k) as u32, py as u32, hx(c)); }
            }
        }
        Extra::Tufts(c) => {
            for i in 0..3 {
                let px = (hash(i, s, 3) * 13.) as u32 + 1;
                let py = (hash(s, i, 9) * 12.) as u32 + 2;
                for (dx, dy) in [(0, 0), (0, 1), (2, 0), (2, 1), (1, 1), (1, 2)] {
                    im.set_pixel(px + dx, py + dy, hx(c));
                }
            }
        }
        Extra::Mountain => {
            for x in 0..16 {
                im.set_pixel(x, 7, hx(0x4a423e));
                im.set_pixel(x, 0, hx(0x857b73));
                im.set_pixel(x, 8, hx(0x857b73));
            }
            for y in 0..7 { im.set_pixel(5, y, hx(0x4a423e)); }
            for y in 8..16 { im.set_pixel(11, y, hx(0x4a423e)); }
        }
    }
    im
}
struct Atlas { img: Image, cx: u16, cy: u16, rowh: u16 }
impl Atlas {
    fn add(&mut self, src: &Image) -> Rect {
        let (w, h) = (src.width, src.height);
        if self.cx + w + 1 > self.img.width {
            self.cx = 0;
            self.cy += self.rowh + 1;
            self.rowh = 0;
        }
        assert!(self.cy + h <= self.img.height, "atlas เต็ม — เพิ่มความสูงใน build_sprites");
        for y in 0..h {
            for x in 0..w {
                self.img.set_pixel((self.cx + x) as u32, (self.cy + y) as u32, src.get_pixel(x as u32, y as u32));
            }
        }
        let r = Rect::new(self.cx as f32, self.cy as f32, w as f32, h as f32);
        self.cx += w + 1;
        self.rowh = self.rowh.max(h);
        r
    }
}
struct Sprites {
    tex: Texture2D,
    tiles: Vec<Vec<Rect>>,
    player: Vec<Vec<Rect>>, // [dir 0 down,1 up,2 side][frame]
    slime: Vec<Rect>,
    skel: Rect,
    bat: [Rect; 2],
    obj: Vec<Rect>,
    icons: Vec<Rect>,
    ex: art::Extras,
    skin: Skin,
    animals: Vec<Rect>,
    edges: vfx::Edges,
}
fn build_sprites(ov: &assets::SpriteOv) -> Sprites {
    let mut at = Atlas { img: Image::gen_image_color(512, 1024, BLANK), cx: 0, cy: 0, rowh: 0 };
    // ---- tiles
    let mut tiles = vec![];
    let defs: Vec<Box<dyn Fn(i32) -> Image>> = vec![
        Box::new(|i| mk_tex(0x1d4f91, &[0x22579f, 0x1a4884], 10, i >> 1, Extra::Waves(0x3a70b8, i & 1))),
        Box::new(|i| mk_tex(0x2f7fd0, &[0x3588d8, 0x2a76c4], 10, 10 + (i >> 1), Extra::Waves(0x7ab8f0, i & 1))),
        Box::new(|i| mk_tex(0xe6d29a, &[0xd4bd80, 0xf2e2b2, 0xcdb478], 14, 20 + i, Extra::None)),
        Box::new(|i| mk_tex(0x5fae4a, &[0x529e40, 0x6dbc56], 16, 30 + i, if i < 2 { Extra::Tufts(0x4a9439) } else { Extra::None })),
        Box::new(|i| mk_tex(0x3f8a3a, &[0x357a30, 0x4a9644, 0x2f6e2b], 18, 40 + i, Extra::Tufts(0x2f6e2b))),
        Box::new(|i| mk_tex(0x8a8a8a, &[0x777777, 0x9a9a9a, 0x6e6e6e], 22, 50 + i, Extra::None)),
        Box::new(|i| mk_tex(0x6b625c, &[0x5a524c, 0x7a716a], 20, 60 + i, Extra::Mountain)),
        Box::new(|i| mk_tex(0xeef4f8, &[0xd8e6ef, 0xffffff, 0xcfdfea], 14, 70 + i, Extra::None)),
        Box::new(|i| mk_tex(0xe8b86a, &[0xd9a456, 0xf2c87e, 0xcf9a4c], 16, 80 + i, Extra::None)),
        Box::new(|i| mk_tex(0x3a3a4c, &[0x32323f, 0x444458, 0x2c2c38], 16, 90 + i, Extra::None)),
        Box::new(|i| mk_tex(0x16161f, &[0x1e1e2a, 0x10101a, 0x1a1a26], 14, 100 + i, Extra::None)),
        Box::new(|i| soil_tex(0x7a5230, 0x5e3c20, 0x8c6238, 110 + i)),
        Box::new(|i| soil_tex(0x52341e, 0x3a2414, 0x5e3e24, 120 + i)),
        Box::new(|i| mk_tex(0x2e2a30, &[0x3a363e, 0x241f26, 0x35303a], 18, 130 + i, Extra::None)),
        Box::new(|i| mk_tex(0x4a4440, &[0x3e3834, 0x57504a, 0x2e2926], 20, 140 + i, Extra::Mountain)),
        Box::new(swamp_tex),
        Box::new(savanna_tex),
        Box::new(ash_tex),
        Box::new(lava_tex),
        Box::new(|i| house::floor_tex(false, i)),
        Box::new(|i| house::floor_tex(true, i)),
        Box::new(living::road_tex),
        Box::new(living::bridge_tex),
    ];
    for d in &defs {
        tiles.push((0..4).map(|i| at.add(&d(i))).collect::<Vec<_>>());
    }
    // ---- player
    let pp: &[(char, u32)] = &[('h', 0x5a3a1e), ('s', 0xf2c79a), ('e', 0x222222), ('m', 0xc9705a), ('r', 0x3a6fd8), ('R', 0xbcd4ff), ('b', 0x5a3a1e), ('p', 0x34405a), ('k', 0x1d1d22)];
    let pd = ["...hhhhhh...", "..hhhhhhhh..", "..hhhhhhhh..", "..hssssssh..", "..sesssses..", "..ssssssss..", "...ssmmss...", "..rrrrrrrr..", ".srrrRRrrrs.", ".srrrrrrrrs.", ".s.rrrrrr.s.", "...bbbbbb..."];
    let pu = ["...hhhhhh...", "..hhhhhhhh..", "..hhhhhhhh..", "..hhhhhhhh..", "..hhhhhhhh..", "..shhhhhhs..", "...ssssss...", "..rrrrrrrr..", ".srrrrrrrrs.", ".srrrrrrrrs.", ".s.rrrrrr.s.", "...bbbbbb..."];
    let pr = ["...hhhhhh...", "..hhhhhhhh..", "..hhhhhhhh..", "..hhhsssss..", "..hhhssses..", "..hhssssss..", "...sssss....", "...rrrrrr...", "...rrrsrr...", "...rrrsrr...", "...rrrsrr...", "...bbbbbb..."];
    let lf = [["...pp..pp...", "...pp..pp...", "...kk..kk..."], ["...pp..pp...", "...kk..pp...", ".......kk..."], ["...pp..pp...", "...pp..kk...", "...kk......."]];
    let ls = [["....pppp....", "....pppp....", "....kkkk...."], ["...pp..pp...", "..pp....pp..", "..kk....kk.."], ["....pppp....", "....pp.pp...", "....kk.kk..."]];
    let mut player = vec![vec![], vec![], vec![]];
    for f in 0..3 {
        let mk = |body: &[&str; 12], legs: &[&str; 3]| {
            let mut v: Vec<&str> = body.to_vec();
            v.extend_from_slice(legs);
            spr(&v, pp)
        };
        player[0].push(at.add(&mk(&pd, &lf[f])));
        player[1].push(at.add(&mk(&pu, &lf[f])));
        player[2].push(at.add(&mk(&pr, &ls[f])));
    }
    // ---- enemies
    let sl = ["....gggg....", "..gggggggg..", ".gglggggggg.", ".glggggggggg", "ggggeggggegg", "ggggeggggegg", "gggggggggggg", "gggggmmggggg", ".gggggggggg.", "..dddddddd.."];
    let slp: [[u32; 5]; 4] = [
        [0x66cc66, 0xccffcc, 0x112233, 0x338833, 0x33aa33],
        [0xf0a040, 0xffe0a0, 0x332211, 0xaa5522, 0xbb6633],
        [0x7cc8f0, 0xe8f8ff, 0x112233, 0x336688, 0x4488aa],
        [0x4aa86a, 0xaaeeee, 0x001122, 0x226644, 0x227755],
    ];
    let slime_img: Vec<Image> = slp.iter().map(|p| spr(&sl, &[('g', p[0]), ('l', p[1]), ('e', p[2]), ('m', p[3]), ('d', p[4])])).collect();
    let slime: Vec<Rect> = slime_img.iter().map(|im| at.add(im)).collect();
    let fl_slime: Vec<Rect> = slime_img.iter().map(|im| at.add(&art::white_of(im))).collect();
    let skel_img = spr(&["...wwww...", "..wwwwww..", "..wkwwkw..", "..wwwwww..", "...wkkw...", "....ww....", ".wwwwwwww.", "w..wkkw..w", "w.wwwwww.w", "...wkkw...", "....ww....", "...wwww...", "...w..w...", "...w..w...", "..ww..ww.."], &[('w', 0xe8e8d8), ('k', 0x333333)]);
    let skel = at.add(&skel_img);
    let fl_skel = at.add(&art::white_of(&skel_img));
    let bp: &[(char, u32)] = &[('w', 0x5a4670), ('k', 0x2a1f33), ('r', 0xff4444)];
    let bat_img = [
        spr(&["w..........w", "ww..k..k..ww", "wwwkkkkkkwww", ".wwkrkkrkww.", "..wkkkkkkw..", "....k..k....", "............"], bp),
        spr(&["............", "....k..k....", "...kkkkkk...", "..wkrkkrkw..", ".wwwkkkkwww.", "ww........ww", "w..........w"], bp),
    ];
    let bat = [at.add(&bat_img[0]), at.add(&bat_img[1])];
    let fl_bat = [at.add(&art::white_of(&bat_img[0])), at.add(&art::white_of(&bat_img[1]))];
    // ---- objects
    let rp: &[(char, u32)] = &[('k', 0x3a3a40), ('L', 0x9a9aa4), ('l', 0xc8c8d0), ('D', 0x6a6a74), ('c', 0x4ef0ff), ('C', 0xb0ffff)];
    let mut obj = vec![Rect::new(0., 0., 1., 1.); 86];
    obj[1] = at.add(&spr(&["......gggg......", "....gGGGGGGg....", "...GGGgGGGGGG...", "..GGGGGGGgGGGG..", ".GGgGGGGGGGGGGG.", ".GGGGGGGGGGgGGG.", "GGGGGgGGGGGGGGGG", "GGGGGGGGGGGGgGGG", "GGgGGGGGGGGGGGGG", "GGGGGGGGgGGGGGGd", ".GGGGGGGGGGGGGd.", ".dGGGgGGGGGGGdd.", "..ddGGGGGGGGdd..", "...dddGGGGddd...", ".....ddTTdd.....", "......TTTT......", "......TTtT......", "......TTTT......", "......TtTT......", "......TTTT......", ".....TTTTTT.....", "....tt....tt...."],
        &[('g', 0x7cd06a), ('G', 0x3f9a3f), ('d', 0x2a6a2a), ('T', 0x7a4a24), ('t', 0x5a3418)]));
    obj[2] = at.add(&spr(&["................", "................", "................", "................", "................", "................", "......gggg......", "....gggggggg....", "...ggrgggggrg...", "..gggggggggggg..", "..ggggrggggggg..", ".gggggggggrgggg.", ".ggrgggggggggg..", "..dgggggggggd...", "...dddddddddd...", "................"], &[('g', 0x3d9440), ('r', 0xe8344a), ('d', 0x2a6a2a)]));
    obj[4] = at.add(&spr(&["................", "................", "................", "................", "................", ".....kkkkk......", "....kLLLLLkk....", "...kLLlLLLLLk...", "..kLLLLLLlLLLk..", "..kLlLLLLLLLLk..", ".kLLLLLLLLLLLDk.", ".kLLLLlLLLLLDDk.", ".kDLLLLLLLLDDDk.", "..kDDDDDDDDDDk..", "...kkkkkkkkkk...", "................"], rp));
    obj[5] = at.add(&spr(&["................", "................", "................", "................", "................", ".....kkkkk......", "....kLLLLLkk....", "...kLLcCLLLLk...", "..kLLLcLLlLLLk..", "..kLlLLLLLcCLk..", ".kLLLLLLLLcLLDk.", ".kLcCLlLLLLLDDk.", ".kDcLLLLLLLDDDk.", "..kDDDDDDDDDDk..", "...kkkkkkkkkk...", "................"], rp));
    obj[6] = at.add(&spr(&["................", "................", "................", "................", "...oooooooooo...", "..oyyyyyyyyyyo..", "..oyYYYYYYYYyo..", "..oyyyyyyyyyyo..", "..oooooGGooooo..", "..oyyyyGGyyyyo..", "..oyyyyyyyyyyo..", "..oyyyyyyyyyyo..", "..oyyyyyyyyyyo..", "..oooooooooooo..", "................", "................"], &[('o', 0x5a3010), ('y', 0xa0642a), ('Y', 0xc88a40), ('G', 0xffd84a)]));
    obj[7] = at.add(&spr(&["................", "................", "......cccc......", ".....cCcccc.....", ".....cCcccc.....", "..cc.cCcccc.....", "..cc.cCcccc.cc..", "..cc.cCcccc.cc..", "..ccccCcccc.cc..", "...cccCcccccc...", ".....cCcccc.....", ".....cCcccc.....", ".....cCcccc.....", ".....cCcccc.....", "....dddddddd....", "................"], &[('c', 0x3c9a4a), ('C', 0x6fd07a), ('d', 0xc9a35a)]));
    obj[8] = at.add(&spr(&[".......gg.......", "......gggg......", ".....gwgggg.....", "....gggggwgg....", "......gggg......", "....ggwggggg....", "...gggggggwgg...", "..ggwgggggggg...", ".....gggggg.....", "...gggggwgggg...", "..gggwgggggggg..", ".gggggggggwgggg.", "......tttt......", "......tttt......", ".....tttttt.....", "................"], &[('g', 0x2f6b4f), ('w', 0xeef6ff), ('t', 0x6a4022)]));
    let firep: &[(char, u32)] = &[('y', 0xffe680), ('o', 0xffa630), ('r', 0xff5a1a), ('R', 0xfff2a0), ('l', 0x6a4022), ('L', 0x8a5a32), ('s', 0x888888)];
    obj[9] = at.add(&spr(&["................", "................", "................", "................", "................", "................", ".......y........", "......yoy.......", ".....yoroy......", "....yorrroy.....", ".....orRro......", "......rRr.......", "...lLlLLlLl.....", "..lsslLLlssl....", "...lLLllLLl.....", "................"], firep));
    // ---- icons 8x8
    let mut icons = vec![
        at.add(&spr(&["........", "..ttttt.", ".tTTTTTo", "tTTTTToO", "tTTTTToo", ".tTTTTTo", "..ttttt.", "........"], &[('t', 0x5a3418), ('T', 0x8a5a32), ('o', 0xc89a60), ('O', 0xe0b880)])),
        at.add(&spr(&["........", "..kkkk..", ".kLLLlk.", "kLLlLLLk", "kLLLLLDk", "kDLLLDDk", ".kkkkkk.", "........"], rp)),
        at.add(&spr(&["...gg...", "....g...", ".rr.rr..", "rRrrRrr.", "rrrrrrr.", ".rrrrr..", "..rrr...", "........"], &[('g', 0x3d9440), ('r', 0xe8344a), ('R', 0xffb0b8)])),
        at.add(&spr(&["........", "..cccc..", ".cCCccc.", "ccCcccdc", ".cccccd.", "..ccdd..", "...cd...", "........"], &[('c', 0x4ef0ff), ('C', 0xe0ffff), ('d', 0x20a0c0)])),
        at.add(&spr(&["..oooo..", ".oyyyyo.", "oyYyyyyo", "oyYyyyyo", "oyyyyyyo", "oyyyyyYo", ".oyyyyo.", "..oooo.."], &[('o', 0xa06a10), ('y', 0xffd84a), ('Y', 0xfff2a0)])),
        at.add(&spr(&["....y...", "...yo...", "..yoro..", ".yorrRo.", ".orRRro.", "..oRRo..", "lLlLLlLl", ".lLllLl."], firep)),
    ];
    let mut ex = art::build_extra(&mut at, &mut obj, &mut icons);
    trees::build_trees(&mut at, &mut obj);
    land::build_land(&mut at, &mut obj);
    let an_rects = animals::build_animals(&mut at);
    ex.flash_slime = [fl_slime[0], fl_slime[1], fl_slime[2], fl_slime[3]];
    ex.flash_skel = fl_skel;
    ex.flash_bat = fl_bat;
    let skin = skin::build(&mut at, &pd, &pu, &pr, &lf, &ls, pp);
    ov.apply(&mut at, &mut tiles, &mut obj, &mut icons);
    let edges = vfx::build_edges(&mut at, &tiles);
    let tex = Texture2D::from_image(&at.img);
    tex.set_filter(FilterMode::Nearest);
    Sprites { tex, tiles, player, slime, skel, bat, obj, icons, ex, skin, animals: an_rects, edges }
}

// ======================= entities =======================
#[derive(Clone, Copy, PartialEq, Debug)]
enum EK { Slime, Skel, Bat, Soldier, Scorp, Wolf, Crab, Imp }
#[derive(Clone)]
struct Enemy { path: Path, k: EK, x: f32, y: f32, hp: f32, spd: f32, dmg: i32, xp: i32, coin: i32, vx: f32, vy: f32, kb: f32, flash: f32, wt: f32, wdx: f32, wdy: f32, anim: f32, pal: usize, dead: bool, night: bool, burn: f32, slow: f32, stun: f32, btick: f32, sk: f32, fl: bool }
struct Particle { x: f32, y: f32, vx: f32, vy: f32, l: f32, c: Color }
struct Float { x: f32, y: f32, t: String, c: Color, l: f32, icon: Option<usize>, sz: f32 }
struct Player { x: f32, y: f32, dir: usize, flip: bool, hp: i32, maxhp: i32, xp: i32, lvl: i32, sword: usize, pick: bool, rod: bool, elem: u8, elems: u8, inv: [i32; NI], invul: f32, kbx: f32, kby: f32, kbt: f32, atk_t: f32, atk_cd: f32, anim: f32, moving: bool, regen: f32, last_hit: f32, kills: i32, weapon: u8, bow: bool, staff: u8, armor: [u8; 3], outfit: u8, outfits: u8, sp: i32, st: [u8; 4], wplus: [u8; 3], aplus: [u8; 3], dash_t: f32, dash_cd: f32, dash_dir: Vec2, ghost_t: f32, buff_spd: f32, buff_crit: f32, can: bool, water: i32, hunger: f32, thirst: f32, rest: f32, sick: f32, starve: f32, body: Body, hides: i32, coat: bool, boat: bool, bare: bool }
impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player { x, y, dir: 0, flip: false, hp: 10, maxhp: 10, xp: 0, lvl: 1, sword: 0, pick: false, rod: false, elem: 0, elems: 1, inv: [0, 0, 2, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], invul: 0., kbx: 0., kby: 0., kbt: 0., atk_t: 0., atk_cd: 0., anim: 0., moving: false, regen: 0., last_hit: 0., kills: 0, weapon: 0, bow: false, staff: 0, armor: [0; 3], outfit: 0, outfits: 0, sp: 0, st: [0; 4], wplus: [0; 3], aplus: [0; 3], dash_t: 0., dash_cd: 0., dash_dir: Vec2::ZERO, ghost_t: 0., buff_spd: 0., buff_crit: 0., can: false, water: 0, hunger: 100., thirst: 100., rest: 100., sick: 0., starve: 0., body: Body::default(), hides: 0, coat: false, boat: false, bare: false }
    }
    fn facing(&self) -> (f32, f32) {
        DV[if self.dir == 2 && self.flip { 3 } else { self.dir }]
    }
}

#[derive(Default)]
struct Input { hb: Option<usize>, mv: Vec2, atk: bool, eat: bool, fire: bool, menu: Option<usize>, map: bool, act: bool, swap: bool, place: bool, cancel: bool, potion: bool, aim: Option<Vec2>, dash: bool, tile_tap: Option<Vec2>, place_hold: bool, ride: bool, build_tap: Option<Vec2>, build_start: bool, build_end: bool, rot: bool, form: bool, horn: bool }

// ======================= game =======================
struct Game {
    w: World,
    p: Player,
    en: Vec<Enemy>,
    parts: Vec<Particle>,
    floats: Vec<Float>,
    toasts: Vec<(String, f32)>,
    time: f32,
    spawn: (f32, f32),
    seen: HashSet<u8>,
    obj_dmg: HashMap<(i32, i32), i32>,
    shake: ((i32, i32), f32),
    hurt_flash: f32,
    darkv: f32,
    last_biome: i32,
    spawn_t: f32,
    save_t: f32,
    mini_t: f32,
    died: bool,
    big_map: bool,
    sfx: Sfx,
    proj: Vec<Proj>,
    bosses: Vec<Boss>,
    /// v24 ท่าตายของมอนที่เพิ่งล้ม (ไม่เซฟ)
    dying: Vec<war::Dying>,
    boss_cd: f32,
    boss_t: f32,
    chests: HashMap<(i32, i32), [i32; NI]>,
    camps_found: HashSet<(i32, i32)>,
    marks_found: HashMap<(i32, i32), u8>,
    blessed: HashMap<(i32, i32), i32>,
    lava_t: f32,
    lava_told: bool,
    felled: HashMap<(i32, i32), (i32, u8)>,
    planted: HashMap<(i32, i32), i32>,
    animals: Vec<Animal>,
    an_spawn_t: f32,
    an_told: HashSet<u8>,
    nreqs: Vec<NReq>,
    tips: Vec<(String, f32)>,
    tips_seen: u32,
    rooms: Vec<Room>,
    room_of: HashMap<(i32, i32), usize>,
    room_t: f32,
    paint_last: Option<(i32, i32)>,
    auto_doors: HashSet<(i32, i32)>,
    sites: Vec<Site>,
    rs: RealmState,
    saga: Saga,
    pins: Vec<(i32, i32)>,
    pin_on: Option<usize>,
    home_cd: f32,
    step_d: f32,
    step_last: usize,
    step_tracks: Vec<StepTrack>,
    step_budget: f32,
    amb: [f32; AMB_N],
    amb_t: f32,
    hammer_t: f32,
    aim_last: Option<(f32, f32)>,
    aim_last_t: f32,
    site_wait_t: f32,
    sit: Option<(i32, i32)>,
    sit_t: f32,
    psy: HashMap<u32, Psy>,
    psy_t: f32,
    gather: HashMap<u32, ((f32, f32), (f32, f32))>,
    an_hunted: u32,
    codex_done: u32,
    adv_done: u32,
    boated: bool,
    final_won: bool,
    ending: f32,
    camp_t: f32,
    build_sel: Option<usize>,
    /// v23 ทิศที่จะวางบ้านสำเร็จรูป (0 ประตูลง · 1 ซ้าย · 2 ขึ้น · 3 ขวา) — จำไว้ข้ามการวาง
    build_dir: u8,
    req: Option<Req>,
    // ---- v3
    drops: Vec<Loot>,
    comp: Option<Comp>,
    ghosts: Vec<(f32, f32, usize, bool, f32)>,
    quest: Option<Quest>,
    qn: HashMap<(i32, i32), i32>,
    shop_camp: Option<(i32, i32)>,
    fish: Option<Fishing>,
    torches: Vec<(f32, f32)>,
    torch_t: f32,
    dun_found: HashSet<(i32, i32)>,
    wp_found: HashSet<(i32, i32)>,
    dun_clear: HashSet<(i32, i32)>,
    dun_chests: HashSet<(i32, i32, i32)>,
    dun_ret: (f32, f32),
    dun_t: f32,
    rain: f32,
    rain_on: bool,
    rain_t: f32,
    thunder: f32,
    cshake: (f32, f32),
    bgm_on: bool,
    to_title: bool,
    npcs: Vec<Npc>,
    rels: HashMap<(u32, u32), i32>,
    news: Vec<(i32, String)>,
    npc_next: u32,
    npc_seen: HashSet<(i32, i32)>,
    lost_told: HashSet<(i32, i32)>,
    lost_hint_t: f32,
    last_day: i32,
    tax_pool: i32,
    wed_day: i32,
    last_child: i32,
    tut: u8,
    tut_ev0: [u32; EV_N],
    ev: [u32; EV_N],
    tut_flash: f32,
    hitstop: f32,
    sfx_on: bool,
    auto_aim: bool,
    pick_cd: f32,
    // ---- v5 ฟาร์ม/เตียง/Grid Snap/สภาพอากาศ
    crops: HashMap<(i32, i32), Crop>,
    fruit: HashMap<(i32, i32), i32>,
    crop_t: f32,
    bed: Option<(i32, i32)>,
    seed_sel: u8,
    sleep_fx: f32,
    build_cur: Option<(i32, i32)>,
    cur_pos: (f32, f32),
    weather_kind: u8,
}

fn find_spawn(w: &World) -> (i32, i32) {
    for r in 0..400 {
        let n = 8 * r + 1;
        for a in 0..n {
            let ang = a as f32 / n as f32 * std::f32::consts::TAU;
            let (tx, ty) = ((ang.cos() * r as f32).round() as i32, (ang.sin() * r as f32).round() as i32);
            let t = w.tile_at(tx, ty);
            if (t == 3 || t == 4) && w.obj_at(tx, ty) == 0 && !w.solid_at(tx + 1, ty) && !w.solid_at(tx, ty + 1) {
                return (tx, ty);
            }
        }
    }
    (0, 0)
}

/// เซฟของช่องที่เลือกอยู่ (ทดสอบ: PIXEL_REALM_SAVE ใช้ไฟล์นั้นตรงๆ)
fn save_path() -> String {
    if std::env::var("PIXEL_REALM_SAVE").is_ok() { return save_base(); }
    slot_path(prefs::slot())
}
/// เซฟช่อง 1 (ตำแหน่งเดิม) — ไฟล์ตั้งค่า .cfg ผูกกับไฟล์นี้เสมอ
fn save_base() -> String {
    if let Ok(p) = std::env::var("PIXEL_REALM_SAVE") {
        return p;
    }
    #[cfg(target_os = "android")]
    {
        let dir = "/data/data/rust.pixel_realm/files";
        let _ = std::fs::create_dir_all(dir);
        return format!("{}/save.txt", dir);
    }
    #[allow(unreachable_code)]
    "pixel_realm_save.txt".to_string()
}

impl Game {
    fn blank(w: World, sfx: Sfx) -> Game {
        Game {
            w, p: Player::new(0., 0.), en: vec![], parts: vec![], floats: vec![], toasts: vec![], time: 0., spawn: (0., 0.),
            seen: HashSet::new(), obj_dmg: HashMap::new(), shake: ((0, 0), 0.), hurt_flash: 0., darkv: 0., last_biome: -1,
            spawn_t: 0., save_t: 0., mini_t: 99., died: false, big_map: false, sfx,
            proj: vec![], bosses: vec![], dying: vec![], boss_cd: 0., boss_t: 2., chests: HashMap::new(), camps_found: HashSet::new(), marks_found: HashMap::new(), blessed: HashMap::new(), lava_t: 0., lava_told: false, felled: HashMap::new(), planted: HashMap::new(), animals: vec![], an_spawn_t: 2., an_told: HashSet::new(), nreqs: vec![], tips: vec![], tips_seen: 0, rooms: vec![], room_of: HashMap::new(), room_t: 0., paint_last: None, auto_doors: HashSet::new(), sites: vec![], rs: RealmState::default(), saga: Saga::default(), pins: vec![], pin_on: None, home_cd: 0., step_d: 0., step_last: 0, step_tracks: vec![], step_budget: 4., amb: [0.; AMB_N], amb_t: 0., hammer_t: 0., aim_last: None, aim_last_t: 0., site_wait_t: 0., sit: None, sit_t: 0., psy: HashMap::new(), psy_t: 0., gather: HashMap::new(), an_hunted: 0, codex_done: 0, adv_done: 0, boated: false, final_won: false, ending: 0., camp_t: 0.,
            build_sel: None, build_dir: 0, req: None,
            drops: vec![], comp: None, ghosts: vec![], quest: None, qn: HashMap::new(), shop_camp: None, fish: None, torches: vec![], torch_t: 0.,
            dun_found: HashSet::new(), wp_found: HashSet::new(), dun_clear: HashSet::new(), dun_chests: HashSet::new(), dun_ret: (0., 0.), dun_t: 0.,
            rain: 0., rain_on: false, rain_t: 90., thunder: 0., cshake: (0., 0.), bgm_on: true, to_title: false, npcs: vec![], rels: HashMap::new(), news: vec![], npc_next: 1, npc_seen: HashSet::new(), lost_told: HashSet::new(), lost_hint_t: 1., last_day: -1, tax_pool: 0, wed_day: 0, last_child: -99, tut: TUT_DONE, tut_ev0: [0; EV_N], ev: [0; EV_N], tut_flash: 0., hitstop: 0., sfx_on: true,
            auto_aim: true, pick_cd: 0.,
            crops: HashMap::new(), fruit: HashMap::new(), crop_t: 0., bed: None, seed_sel: 0, sleep_fx: 0., build_cur: None, cur_pos: (0., 0.), weather_kind: 1,
        }
    }
    fn new_world(sfx: Sfx) -> Self {
        Game::new_world_seed(gen_range(1, i32::MAX), sfx)
    }
    fn new_world_seed(seed: i32, sfx: Sfx) -> Self {
        let w = World::new(seed);
        let (sx, sy) = find_spawn(&w);
        let mut g = Game::blank(w, sfx);
        g.spawn = (sx as f32 * 16. + 8., sy as f32 * 16. + 12.);
        g.p = Player::new(g.spawn.0, g.spawn.1);
        g.tut = 0;
        g.time = DAYLEN * 0.05;
        g
    }

    // ---------- save / load (ไฟล์ข้อความง่ายๆ) ----------
    fn save_string(&self) -> String {
        let p = &self.p;
        let mut s = String::new();
        s += &format!("seed {}\nspawn {} {}\ntime {}\n", self.w.seed, self.spawn.0, self.spawn.1, self.time);
        let (px, py) = if self.w.dun.is_some() { self.dun_ret } else { (p.x, p.y) };
        s += &format!("p {} {} {} {} {} {} {} {} {}\n", px, py, p.hp, p.maxhp, p.xp, p.lvl, p.sword, p.pick as i32, p.kills);
        s += "inv";
        for v in p.inv { s += &format!(" {}", v); }
        s += &format!("\ngear {} {} {} {} {} {} {} {} {}\n", p.weapon, p.bow as i32, p.staff, p.armor[0], p.armor[1], p.armor[2], p.outfit, p.outfits, p.bare as u8);
        s += &format!("bcd {}\n", self.boss_cd);
        s += &format!("opt {} {} {}\n", self.sfx_on as i32, self.bgm_on as i32, self.auto_aim as i32);
        s += &format!("rpg {} {} {} {} {} {} {} {} {} {}\n", p.sp, p.st[0], p.st[1], p.st[2], p.st[3], p.wplus[0], p.wplus[1], p.wplus[2], p.aplus[0], p.aplus[1]);
        s += &format!("rpg2 {}\n", p.aplus[2]);
        if let Some(q) = &self.quest { s += &format!("quest {} {} {} {} {} {} {} {}\n", q.camp.0, q.camp.1, q.kind, q.goal, q.prog, q.coin, q.shard, q.bp); }
        s += &format!("buff {} {}\n", p.buff_spd, p.buff_crit);
        s += &format!("elem {} {}\nrod {}\n", p.elem, p.elems, p.rod as i32);
        s += &format!("farm {} {} {}\n", p.can as i32, p.water, self.seed_sel);
        s += &format!("tut {}\n", self.tut);
        s += &format!("need {:.1} {:.1} {:.1} {:.1}\n", p.hunger, p.thirst, p.rest, p.sick);
        s += &p.body.save_str();
        s += &self.npc_save_str();
        s += &self.sites_save();
        s += &self.nav_save();
        s += &self.realm_save();
        s += &self.saga_save();
        s += &self.living_save();
        s += &self.war_save();
        if let Some(b) = self.bed { s += &format!("bed {} {}\n", b.0, b.1); }
        s += "crops";
        for ((x, y), c) in &self.crops { s += &format!(" {},{},{},{:.1},{:.1}", x, y, c.kind, c.grown, c.wet); }
        s += "\n";
        s += &self.fruit_save_str();
        s += "wps";
        for (x, y) in &self.wp_found { s += &format!(" {},{}", x, y); }
        s += "\nqn";
        for ((x, y), n) in &self.qn { s += &format!(" {},{},{}", x, y, n); }
        s += "\n";
        if let Some(c) = &self.comp { s += &format!("comp {}\n", c.kind); }
        s += "dunfound";
        for (x, y) in &self.dun_found { s += &format!(" {},{}", x, y); }
        s += "\ndunclr";
        for (x, y) in &self.dun_clear { s += &format!(" {},{}", x, y); }
        s += "\ndunchest";
        for (x, y, i) in &self.dun_chests { s += &format!(" {},{},{}", x, y, i); }
        s += "\n";
        s += &self.marks_save_str();
        s += &self.seasons_save_str();
        s += &self.quests_save_str();
        s += &self.balance_save_str();
        s += "camps";
        for (x, y) in &self.camps_found { s += &format!(" {},{}", x, y); }
        s += "\nseen";
        for b in &self.seen { s += &format!(" {}", b); }
        s += "\nrem";
        for (x, y) in &self.w.removed { s += &format!(" {},{}", x, y); }
        s += "\nplc";
        for ((x, y), v) in &self.w.placed { s += &format!(" {},{},{}", x, y, v); }
        s += "\ntmod";
        for ((x, y), v) in &self.w.tmod { s += &format!(" {},{},{}", x, y, v); }
        s += "\n";
        for ((x, y), c) in &self.chests {
            s += &format!("chest {} {}", x, y);
            for v in c { s += &format!(" {}", v); }
            s += "\n";
        }
        s
    }
    fn save(&self) {
        prof::t("save", || { let _ = write_atomic(&save_path(), &self.save_string()); });
    }
    fn load(sfx: Sfx) -> Result<Game, Sfx> {
        let Ok(txt) = std::fs::read_to_string(save_path()) else { return Err(sfx) };
        Game::load_str(&txt, sfx)
    }
    fn load_str(txt: &str, sfx: Sfx) -> Result<Game, Sfx> {
        let mut g = Game::new_world_placeholder(sfx);
        let mut ok = false;
        for line in txt.lines() {
            let mut it = line.split_whitespace();
            let Some(tag) = it.next() else { continue };
            let rest: Vec<&str> = it.collect();
            let f = |i: usize| rest.get(i).and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.);
            match tag {
                "seed" => { g.w = World::new(rest.first().and_then(|v| v.parse().ok()).unwrap_or(1)); ok = true; }
                "spawn" => g.spawn = (f(0), f(1)),
                "time" => g.time = f(0),
                "p" => {
                    let p = &mut g.p;
                    p.x = f(0); p.y = f(1); p.hp = f(2) as i32; p.maxhp = f(3) as i32; p.xp = f(4) as i32; p.lvl = f(5) as i32;
                    p.sword = (f(6) as usize).min(5); p.pick = f(7) > 0.; p.kills = f(8) as i32;
                    for i in 0..6 { p.inv[i] = f(9 + i) as i32; }
                }
                "inv" => for i in 0..NI { g.p.inv[i] = f(i) as i32; },
                "gear" => {
                    let p = &mut g.p;
                    p.weapon = (f(0) as u8).min(2);
                    p.bow = f(1) > 0.;
                    p.staff = (f(2) as u8).min(3);
                    for i in 0..3 { p.armor[i] = (f(3 + i) as u8).min(4); }
                    p.outfits = (f(7) as u8) & 3; p.outfit = (f(6) as u8).min(2);
                    if !p.has_weapon(p.weapon) { p.weapon = 0; }
                    p.bare = f(8) > 0. && p.weapon == 0;
                }
                "bcd" => g.boss_cd = f(0),
                "opt" => {
                    g.sfx_on = f(0) > 0.; g.bgm_on = f(1) > 0.; g.auto_aim = f(2) > 0.;
                    SFX_ON.store(g.sfx_on, std::sync::atomic::Ordering::Relaxed);
                }
                "rpg" => {
                    let p = &mut g.p;
                    p.sp = f(0) as i32;
                    for i in 0..4 { p.st[i] = (f(1 + i) as u8).min(ST_MAX); }
                    for i in 0..3 { p.wplus[i] = (f(5 + i) as u8).min(PLUS_MAX); }
                    for i in 0..2 { p.aplus[i] = (f(8 + i) as u8).min(PLUS_MAX); }
                }
                "rod" => g.p.rod = f(0) > 0.,
                "tut" => g.tut = f(0) as u8,
                "body" => g.p.body = Body::load(&(0..rest.len().min(10)).map(|i| f(i)).collect::<Vec<f32>>()),
                "need" => { g.p.hunger = f(0).clamp(0., 100.); g.p.thirst = f(1).clamp(0., 100.); g.p.rest = f(2).clamp(0., 100.); g.p.sick = f(3).max(0.); }
                "town" | "npc" | "npcseen" => g.npc_load_line(tag, &rest),
                "nsoc" | "nrel" | "nnews" => g.soc_load_line(tag, &rest),
                "npsy" => g.psy_load(&rest),
                "site" => g.sites_load(&rest),
                "pins" => g.nav_load(&rest),
                "rk" | "rc" | "rd" | "rp" | "ro" | "rr" | "rq" => g.realm_load(tag, &rest),
                "nreq" => g.quests_load_line(&rest),
                "saga" => g.saga_load(&rest),
                "living" => g.living_load(&rest),
                "war" => g.war_load(&rest),
                "v9" => g.balance_load_line(&rest),
                "pet" => g.pets_load_line(&rest),
                "farm" => { g.p.can = f(0) > 0.; g.p.water = (f(1) as i32).clamp(0, WATER_MAX); g.seed_sel = (f(2) as u8).min(1); }
                "bed" => g.bed = Some((f(0) as i32, f(1) as i32)),
                "fruit" => g.fruit_load_line(&rest),
                "crops" => for v in &rest {
                    let n: Vec<f32> = v.split(',').filter_map(|x| x.parse().ok()).collect();
                    if n.len() == 5 && (n[2] as u8) < 2 { g.crops.insert((n[0] as i32, n[1] as i32), Crop { kind: n[2] as u8, grown: n[3], wet: n[4] }); }
                },
                "elem" => { g.p.elem = (f(0) as u8).min(2); g.p.elems = (f(1) as u8) & 7 | 1; }
                "buff" => { g.p.buff_spd = f(0); g.p.buff_crit = f(1); }
                "rpg2" => g.p.aplus[2] = (f(0) as u8).min(PLUS_MAX),
                "quest" => {
                    g.quest = Some(Quest { camp: (f(0) as i32, f(1) as i32), kind: f(2) as u8, goal: f(3) as i32, prog: f(4) as i32, coin: f(5) as i32, shard: f(6) as i32, bp: f(7) as u8 });
                }
                "comp" => {
                    let k = (f(0) as u8).min(1);
                    g.comp = Some(Comp { kind: k, x: 0., y: 0., atk_cd: 0., anim: 0., left: false });
                }
                "qn" | "dunfound" | "dunclr" | "dunchest" | "wps" => {
                    for v in &rest {
                        let n: Vec<i32> = v.split(',').filter_map(|x| x.parse().ok()).collect();
                        match (tag, n.len()) {
                            ("qn", 3) => { g.qn.insert((n[0], n[1]), n[2]); }
                            ("wps", 2) => { g.wp_found.insert((n[0], n[1])); }
                            ("dunfound", 2) => { g.dun_found.insert((n[0], n[1])); }
                            ("dunclr", 2) => { g.dun_clear.insert((n[0], n[1])); }
                            ("dunchest", 3) => { g.dun_chests.insert((n[0], n[1], n[2])); }
                            _ => {}
                        }
                    }
                }
                "chest" => {
                    let mut c = [0i32; NI];
                    for i in 0..NI { c[i] = f(2 + i) as i32; }
                    g.chests.insert((f(0) as i32, f(1) as i32), c);
                }
                "camps" => for v in &rest {
                    let n: Vec<i32> = v.split(',').filter_map(|x| x.parse().ok()).collect();
                    if n.len() == 2 { g.camps_found.insert((n[0], n[1])); }
                },
                "marks" | "bless" => g.marks_load_line(tag, &rest),
                "felled" | "planted" => g.seasons_load_line(tag, &rest),
                "seen" => for v in &rest { if let Ok(b) = v.parse() { g.seen.insert(b); } },
                "rem" | "plc" | "tmod" => {
                    for v in &rest {
                        let n: Vec<i32> = v.split(',').filter_map(|x| x.parse().ok()).collect();
                        match (tag, n.len()) {
                            ("rem", 2) => { g.w.removed.insert((n[0], n[1])); }
                            ("plc", 3) => { g.w.placed.insert((n[0], n[1]), n[2] as u8); }
                            ("tmod", 3) => { g.w.tmod.insert((n[0], n[1]), n[2] as u8); }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some(c) = g.comp.as_mut() { c.x = g.p.x - 10.; c.y = g.p.y; }
        if ok && g.p.maxhp > 0 { if g.p.hp <= 0 { g.p.hp = g.p.maxhp; } Ok(g) } else { Err(g.sfx) }
    }
    fn new_world_placeholder(sfx: Sfx) -> Game {
        Game::blank(World::new(1), sfx)
    }

    // ---------- helpers ----------
    fn toast(&mut self, t: impl Into<String>) {
        let t: String = t.into();
        let life = hud::toast_life(&t);
        // ข้อความซ้ำกับอันล่าสุด → ต่อเวลาแทนการซ้อน
        if let Some(last) = self.toasts.last_mut() { if last.0 == t { last.1 = life; return; } }
        // v27 เรื่องเดียวกันมาติดๆ (เช่น "ภารกิจสำเร็จ: …" หลายอัน) → รวมเป็นกล่องเดียว
        let p = hud::toast_prefix(&t);
        if let (Some(last), Some(p)) = (p.and_then(|p| self.toasts.iter_mut().rev().find(|x| hud::toast_prefix(&x.0) == Some(p) && x.1 > 1.)), p) {
            if last.0.chars().count() + t.chars().count() < 170 {
                let rest = &t[p.len() + 2..];
                last.0 = format!("{} · {}", last.0, rest);
                last.1 = hud::toast_life(&last.0);
                return;
            }
        }
        self.toasts.push((t, life));
        while self.toasts.len() > hud::TOAST_MAX { self.toasts.remove(0); }
    }
    fn add_float(&mut self, x: f32, y: f32, t: impl Into<String>, c: Color, icon: Option<usize>) {
        self.floats.push(Float { x, y, t: t.into(), c, l: 1., icon, sz: 1. });
    }
    fn burst(&mut self, x: f32, y: f32, c: Color, n: usize, sp: f32) {
        for _ in 0..n {
            let a = gen_range(0., std::f32::consts::TAU);
            let s = gen_range(0., sp);
            self.parts.push(Particle { x, y, vx: a.cos() * s, vy: a.sin() * s - 20., l: gen_range(0.4, 0.8), c });
        }
    }
    fn gain(&mut self, item: usize, n: i32, x: f32, y: f32) {
        self.p.inv[item] += n;
        self.add_float(x, y, format!("+{}", n), WHITE, Some(item));
        self.quest_item(item, n);
        if self.pick_cd <= 0. {
            self.pick_cd = 0.05;
            play(&self.sfx.pickup);
        }
    }
    fn check_lv(&mut self) {
        while self.p.xp >= self.p.lvl * 10 {
            self.p.xp -= self.p.lvl * 10;
            self.p.lvl += 1;
            self.p.maxhp += 2;
            self.p.hp = self.p.maxhp;
            self.p.sp += SP_PER_LV;
            let l = self.p.lvl;
            self.toast(format!("เลเวลอัป! Lv {} (+{} แต้มสเตตัส — {})", l, SP_PER_LV, if hud::touch_on() { "เมนู → ตัวละคร" } else { "กด P" }));
            play(&self.sfx.lvl);
        }
    }

    // ---------- actions ----------
    fn eat(&mut self) {
        if self.p.inv[BERRY] <= 0 {
            if self.p.inv[POTION] > 0 { self.drink(); } else { self.toast("ไม่มีผลไม้"); }
            return;
        }
        if self.p.hp >= self.p.maxhp && self.p.hunger >= 95. { self.toast(if self.survival_mode() == 0 { "เลือดเต็มแล้ว" } else { "อิ่มแล้ว เลือดก็เต็ม" }); return; }
        self.p.inv[BERRY] -= 1;
        self.feed(12.);
        self.p.hp = (self.p.hp + 3).min(self.p.maxhp);
        let (x, y) = (self.p.x, self.p.y - 18.);
        self.add_float(x, y, "+3 HP", hx(0xff6666), None);
        play(&self.sfx.eat);
    }
    fn drink(&mut self) {
        if self.p.inv[POTION] <= 0 { self.toast("ไม่มียาฟื้นฟู — ซื้อได้จากพ่อค้า"); return; }
        if self.p.hp >= self.p.maxhp { self.toast("เลือดเต็มแล้ว"); return; }
        self.p.inv[POTION] -= 1;
        self.p.hp = (self.p.hp + 8).min(self.p.maxhp);
        let (x, y) = (self.p.x, self.p.y - 18.);
        self.add_float(x, y, "+8 HP", hx(0xff6666), None);
        play(&self.sfx.eat);
    }
    fn place_fire(&mut self) {
        if self.w.dun.is_some() { self.toast("วางกองไฟในดันเจี้ยนไม่ได้"); return; }
        if self.p.inv[FIRE] <= 0 { self.toast("ไม่มีกองไฟ — คราฟต์ได้ (C)"); return; }
        let (dx, dy) = self.p.facing();
        let (tx, ty) = (tfloor(self.p.x + dx * 14.), tfloor(self.p.y - 3. + dy * 14.));
        let o = self.w.obj_at(tx, ty);
        if SOLIDT[self.w.tile_at(tx, ty) as usize] || OSOLID[o as usize] { self.toast("วางตรงนี้ไม่ได้"); return; }
        self.w.placed.insert((tx, ty), 9);
        self.ev[1] += 1;
        self.p.inv[FIRE] -= 1;
        play(&self.sfx.fire);
        self.burst(tx as f32 * 16. + 8., ty as f32 * 16. + 10., hx(0xffa630), 10, 50.);
        self.save();
    }
    fn hit_object(&mut self, tx: i32, ty: i32, dmg: i32) -> bool {
        if self.law_on_hit(tx, ty) { return true; }
        let k = (tx, ty);
        let in_dun = self.w.dun.is_some();
        if in_dun && !matches!(self.w.obj_at(tx, ty), 4 | 5 | O_CRYSTAL) { return false; }
        let o = self.w.obj_at(tx, ty);
        if (19..=25).contains(&o) || (27..=32).contains(&o) || o == 36 { return true; }
        if o == 0 || o == 3 {
            if self.w.tile_at(tx, ty) == 6 {
                if !self.p.pick { self.toast("ต้องมีอีเต้อถึงจะขุดภูเขาได้"); return true; }
                let h = self.obj_dmg.get(&k).copied().unwrap_or(0) + 1;
                self.shake = (k, 0.2);
                self.burst(tx as f32 * 16. + 8., ty as f32 * 16. + 8., hx(0x6b625c), 5, 50.);
                play(&self.sfx.rock);
                if h >= 4 {
                    self.obj_dmg.remove(&k);
                    self.w.tmod.insert(k, 5);
                    self.gain(STONE, 2, tx as f32 * 16. + 8., ty as f32 * 16.);
                } else {
                    self.obj_dmg.insert(k, h);
                }
                return true;
            }
            return false;
        }
        if o == 15 || o == 16 || land_fixed(o) { return true; }
        if o == 9 && !self.w.placed.contains_key(&k) { return true; }
        if o == 14 && self.chests.get(&k).map_or(false, |c| c.iter().any(|&n| n > 0)) {
            self.toast("เอาของออกจากกล่องก่อนรื้อ");
            return true;
        }
        let h = self.obj_dmg.get(&k).copied().unwrap_or(0) + dmg;
        self.shake = (k, 0.2);
        let col = hx(match o { 1 => 0x3f9a3f, 2 => 0x3d9440, 4 => 0x9a9aa4, 5 => 0x4ef0ff, 6 => 0xc88a40, 7 => 0x3c9a4a, 8 => 0x2f6b4f, 10 => 0x9a6a38, 11 => 0x9a9aa6, 12 | 13 => 0xa06a30, 14 => 0xc88a40, 17 => 0x8a909c, 26 => 0x6a6a74, 40..=47 | O_SWTREE | O_ACACIA => tree_col(o), O_REED | O_DRYGRASS => 0x9aa040, O_SHROOM => 0xd8402a, O_RUINWALL | O_PILLAR | O_ACHEST => 0x9e9a90, O_OBSIDIAN => 0x4a3a62, O_CRYSTAL => 0x7ad8ff, O_BONES => 0xe8e2d0, _ => 0xffa630 });
        let (x, y) = (tx as f32 * 16. + 8., ty as f32 * 16.);
        self.burst(x, y + 6., col, 5, 50.);
        if matches!(o, 4 | 5 | 11 | 17 | 26 | 34 | O_RUINWALL | O_PILLAR | O_OBSIDIAN | O_CRYSTAL | O_ACHEST) { play(&self.sfx.rock) } else { play(&self.sfx.chop) }
        if h < OHP[o as usize] { self.obj_dmg.insert(k, h); return true; }
        self.obj_dmg.remove(&k);
        if in_dun {
            if let Some(d) = self.w.dun.as_mut() { d.set_obj(tx, ty, 0); }
        } else if self.w.placed.remove(&k).is_none() {
            self.w.removed.insert(k);
        }
        self.burst(x, y + 8., col, 12, 70.);
        match o {
            1 => self.gain(WOOD, rnd(2, 3), x, y),
            8 => self.gain(WOOD, 2, x, y),
            40 | 43 | 44 | 45 | 46 | 47 => { let (a, b) = wood_range(o); self.gain(WOOD, rnd(a, b), x, y); }
            41 => {
                self.gain(WOOD, rnd(2, 3), x, y);
                if self.fruit_ready(k) { self.gain(BERRY, rnd(1, 2), x, y - 8.); }
                self.fruit.remove(&k);
            }
            42 => { self.gain(WOOD, 2, x, y); self.fruit.remove(&k); }
            O_SWTREE | O_ACACIA => { let (a, b) = wood_range(o); self.gain(WOOD, rnd(a, b), x, y); }
            O_SHROOM => self.gain(SHROOM, rnd(1, 2), x, y),
            O_RUINWALL => self.gain(STONE, 2, x, y),
            O_PILLAR => self.gain(STONE, 3, x, y),
            O_OBSIDIAN => { self.gain(STONE, 1, x, y); self.gain(GEM, 1, x, y - 8.); play(&self.sfx.gem); }
            O_CRYSTAL => { self.gain(GEM, 2, x, y); if gen_range(0., 1.) < 0.3 { self.gain(SHARD, 1, x, y - 8.); } play(&self.sfx.gem); }
            O_ACHEST => self.open_ancient(x, y),
            2 => { let n = if season(self) == 3 { rnd(0, 1) } else { rnd(1, 3) }; if n > 0 { self.gain(BERRY, n, x, y); } }
            7 => self.gain(BERRY, 1, x, y),
            4 => self.gain(STONE, rnd(2, 3), x, y),
            5 => {
                self.gain(STONE, 1, x, y);
                self.gain(GEM, if in_dun { rnd(1, 2) } else { 1 }, x, y - 8.);
                if in_dun && gen_range(0., 1.) < 0.2 { self.gain(SHARD, 1, x, y - 16.); }
                play(&self.sfx.gem);
            }
            9 => self.gain(FIRE, 1, x, y),
            17 => self.gain(STONE, 5, x, y),
            26 => self.gain(STONE, 3, x, y),
            33 => self.gain(WOOD, 6, x, y),
            34 => { self.gain(STONE, 8, x, y); self.gain(GEM, 1, x, y - 8.); }
            37 => { self.gain(STONE, 25, x, y); self.gain(WOOD, 10, x, y - 8.); }
            38 => self.gain(STONE, 3, x, y),
            18 => self.gain(WOOD, 1, x, y),
            10 => self.gain(WOOD, 1, x, y),
            O_TABLE => self.gain(WOOD, 3, x, y),
            O_CHAIR | O_RUG => self.gain(WOOD, 1, x, y),
            O_SHELF | O_WARDROBE => self.gain(WOOD, 4, x, y),
            O_PLANT | O_LAMP => self.gain(STONE, 1, x, y),
            O_WINDOW => self.gain(WOOD, 2, x, y),
            O_TENTWALL | O_LOG => self.gain(WOOD, 1, x, y),
            11 => self.gain(STONE, 2, x, y),
            12 | 13 => self.gain(WOOD, 2, x, y),
            14 => { self.chests.remove(&k); self.gain(WOOD, 4, x, y); self.gain(STONE, 1, x, y - 8.); }
            6 => {
                let c = rnd(5, 15);
                self.gain(COIN, c, x, y);
                if gen_range(0., 1.) < 0.7 { self.gain(GEM, rnd(1, 2), x, y - 8.); }
                self.gain(BERRY, rnd(1, 3), x, y - 16.);
                self.p.xp += 5;
                self.check_lv();
                self.toast(format!("เปิดหีบสมบัติ! ได้ {} เหรียญ", c));
                play(&self.sfx.chest);
            }
            _ => {}
        }
        if !in_dun && is_tree(o) { self.on_tree_felled(k); }
        self.save();
        true
    }
    fn kill(&mut self, e: &Enemy) {
        let c = match e.k { EK::Slime => hx(self_slime_col(e.pal)), EK::Bat => hx(0x5a4670), EK::Skel => hx(0xe8e8d8), EK::Soldier => hx(0xc8c8d0), EK::Scorp => hx(0xc8823a), EK::Wolf => hx(0xc8ccd8), EK::Crab => hx(0xd8483a), EK::Imp => hx(0xff7a30) };
        self.burst(e.x, e.y - 6., c, 8, 60.);
        self.morale_on_kill(e);
        self.dying.push(war::Dying { k: e.k, x: e.x, y: e.y, pal: e.pal, t: 0., left: e.vx < 0., seed: (e.anim * 1000.) as u32 });
        self.beast_loot(e);
        self.p.xp += e.xp;
        self.p.kills += 1;
        self.ev[2] += 1;
        self.hitstop = self.hitstop.max(0.08);
        let (lx, ly) = (e.x, e.y - 6.);
        self.spawn_loot(lx, ly, COIN, e.coin);
        if e.k == EK::Bat && gen_range(0., 1.) < 0.3 { self.spawn_loot(lx, ly, BERRY, 1); }
        if e.k == EK::Skel && gen_range(0., 1.) < 0.25 { self.spawn_loot(lx, ly, GEM, 1); }
        if e.k == EK::Skel && self.p.bow && gen_range(0., 1.) < 0.4 { self.spawn_loot(lx, ly, ARROW, rnd(2, 4)); }
        if e.k == EK::Bat && gen_range(0., 1.) < 0.45 { self.spawn_loot(lx, ly, MEAT, 1); }
        if e.k == EK::Slime && gen_range(0., 1.) < 0.35 { self.spawn_loot(lx, ly, SHROOM, 1); }
        if e.k == EK::Slime && gen_range(0., 1.) < 0.12 { self.spawn_loot(lx, ly, SEED_OF[rnd(0, 1) as usize], 1); }
        if e.k == EK::Skel && gen_range(0., 1.) < 0.35 { self.spawn_loot(lx, ly, HERB, 1); }
        if e.night && gen_range(0., 1.) < 0.3 { self.spawn_loot(lx, ly, [MEAT, SHROOM, HERB][rnd(0, 2) as usize], 1); }
        if e.night && gen_range(0., 1.) < 0.65 { self.spawn_loot(lx, ly, SHARD, rnd(1, 2)); }
        if e.night && gen_range(0., 1.) < 0.2 { self.spawn_loot(lx, ly, GEM, 1); }
        self.quest_kill(e.k, e.night);
        play(&self.sfx.kill);
        self.check_lv();
    }
    fn do_attack(&mut self) {
        self.p.atk_cd = 0.32;
        self.p.atk_t = 0.18;
        let (dx, dy) = self.p.facing();
        let dmg = SWD[if self.p.bare { 0 } else { self.p.sword }];
        let (fx, fy) = (self.p.x + dx * 12., self.p.y - 6. + dy * 12.);
        let mut hit = false;
        for i in 0..self.en.len() {
            let e = &self.en[i];
            if e.dead { continue; }
            let ey = e.y - if e.k == EK::Bat { 14. } else { 5. };
            if (e.x - fx).hypot(ey - fy) < 15. {
                hit = true;
                self.hit_enemy(i, dmg, dx, dy);
            }
        }
        for i in 0..self.bosses.len() {
            let b = &self.bosses[i];
            if !b.dead && (b.x - fx).hypot(b.y - 12. - fy) < BOSS_R + 12. {
                hit = true;
                self.hit_boss(i, dmg);
            }
        }
        if self.melee_animals(fx, fy, dmg, dx, dy) { hit = true; }
        // v19: ดาบผ่านตัวพวกเดียวกัน (ชาวบ้าน ผู้ติดตาม สัตว์เลี้ยง) — ไม่บาดเจ็บ ไม่โกรธ
        if !hit {
            let (tx, ty) = (tfloor(self.p.x + dx * 11.), tfloor(self.p.y - 3. + dy * 11.));
            hit = self.hit_object(tx, ty, if self.p.sword >= 2 && !self.p.bare { 2 } else { 1 });
        } else {
            play(&self.sfx.hit);
        }
        if !hit { play(&self.sfx.swing); }
    }
    fn hurt(&mut self, d: i32, nx: f32, ny: f32) {
        let df = d as f32 * dmg_taken_mul() * (1. - self.p.reduction());
        let d = df.floor() as i32 + if gen_range(0., 1.) < df.fract() { 1 } else { 0 };
        self.p.hp -= d;
        self.p.invul = if d > 0 { 1. } else { 0.6 };
        self.p.kbx = nx * 130.;
        self.p.kby = ny * 130.;
        self.p.kbt = 0.15;
        self.hurt_flash = 0.35;
        self.shake(if d > 0 { 2.2 } else { 0.8 }, 0.2);
        self.p.last_hit = 0.;
        let (x, y) = (self.p.x, self.p.y - 20.);
        if d > 0 { self.add_float(x, y, format!("-{}", d), hx(0xff4444), None); } else { self.add_float(x, y, "ป้องกัน", hx(0x88ccff), None); }
        play(&self.sfx.hurt);
        if self.p.hp <= 0 { self.faint("หมดสติ..."); }
    }
    fn faint(&mut self, why: &str) {
        {
            let lost = self.p.inv[COIN] / 2;
            self.p.inv[COIN] -= lost;
            self.p.hp = self.p.maxhp;
            self.w.dun = None;
            let rp = self.respawn_pt();
            self.p.x = rp.0;
            self.p.y = rp.1;
            self.died = true;
            self.p.hunger = self.p.hunger.max(35.); self.p.thirst = self.p.thirst.max(35.); self.p.starve = 0.;
            self.proj.clear();
            self.drops.clear();
            self.tp_comp();
            self.toast(format!("{} ฟื้นที่จุดเกิด (เสีย {} เหรียญ)", why, lost));
            self.save();
        }
    }
    fn try_spawn(&mut self) {
        if self.w.dun.is_some() { return; }
        let night = self.darkv > 0.45;
        if peaceful() { return; }
        let ncap = if self.in_grace() { 5 } else { [6, 10, 12][diff() as usize] };
        if self.en.len() >= if night { ncap } else { 4 } { return; }
        let a = gen_range(0., std::f32::consts::TAU);
        let r = gen_range(13., 19.) * 16.;
        let (x, y) = (self.p.x + a.cos() * r, self.p.y + a.sin() * r);
        let (tx, ty) = (tfloor(x), tfloor(y));
        let t = self.w.tile_at(tx, ty);
        if SOLIDT[t as usize] || OSOLID[self.w.obj_at(tx, ty) as usize] || self.w.near_fire(tx, ty, 6).is_some() || self.w.camp_near(tx, ty, 10).is_some() || self.w.near_city(tx, ty, 6) { return; }
        let k = if night {
            let q = gen_range(0., 1.);
            if q < 0.5 { EK::Skel } else if q < 0.8 { EK::Bat } else { EK::Slime }
        } else { EK::Slime };
        if self.in_safe(x, y) { return; }
        let mul = 1. + (x - self.spawn.0).hypot(y - self.spawn.1) / 16. / 250.;
        let elite = night && !self.in_grace() && gen_range(0., 1.) < 0.4;
        let pal = match t { 8 => 1, 7 => 2, 4 => 3, _ => 0 };
        // v24 ศัตรูประจำภูมิประเทศ: แมงป่อง (ทะเลทราย) · หมาป่า (หิมะ เป็นฝูง) · ปู (ชายหาด) · ปีศาจเถ้า (ภูเขาไฟ)
        let k = match (t, gen_range(0., 1.) < 0.65) { (8, true) => EK::Scorp, (7, true) => EK::Wolf, (2, true) => EK::Crab, (17, true) => EK::Imp, _ => k };
        self.make_enemy(k, x, y, mul, pal, elite);
        if k == EK::Wolf { for j in 0..rnd(1, 2) { let (wx, wy) = (x + 14. + j as f32 * 12., y + 10.); if !collide(&self.w, wx, wy) { self.make_enemy(EK::Wolf, wx, wy, mul, pal, elite); } } }
    }
    fn upd_enemies(&mut self, dt: f32) {
        let siege_c = self.rs.siege.as_ref().map(|s| s.center);
        let assault_on = self.rs.war.assault.is_some();
        let allies: Vec<(f32, f32)> = if assault_on { self.army_pos() } else { vec![] };
        let mut path_budget = PATH_BUDGET + 1;
        for d in self.dying.iter_mut() { d.t += dt; }
        self.dying.retain(|d| d.t < war::DIE_T);
        let mut wall_hits: Vec<(i32, i32)> = vec![];
        let mut en = std::mem::take(&mut self.en);
        for e in en.iter_mut() {
            if e.dead { continue; }
            e.anim += dt;
            if e.flash > 0. { e.flash -= dt; }
            if e.slow > 0. { e.slow -= dt; }
            if e.stun > 0. { e.stun -= dt; if gen_range(0., 1.) < dt * 14. { self.parts.push(Particle { x: e.x + gen_range(-5., 5.), y: e.y - 14., vx: 0., vy: -6., l: 0.25, c: hx(0xfff060) }); } }
            if e.burn > 0. {
                e.burn -= dt;
                e.btick += dt;
                if gen_range(0., 1.) < dt * 16. { self.parts.push(Particle { x: e.x + gen_range(-4., 4.), y: e.y - 8., vx: 0., vy: -24., l: 0.4, c: hx(if gen_range(0., 1.) < 0.5 { 0xff8a20 } else { 0xffd040 }) }); }
                if e.btick >= 0.5 {
                    e.btick = 0.;
                    let bd = (((2. + self.p.lvl as f32 * 0.3) * self.p.atk_mul(2)).round() as i32).max(1);
                    e.hp -= bd as f32;
                    self.add_float(e.x, e.y - 14., format!("-{}", bd), hx(0xff8a20), None);
                    if e.hp <= 0. {
                        e.dead = true;
                        let c = e.clone();
                        self.kill(&c);
                        continue;
                    }
                }
            }
            let (dx, dy) = (self.p.x - e.x, self.p.y - e.y);
            let d = dx.hypot(dy).max(0.001);
            if d > 440. && self.w.dun.is_none() && !(e.k == EK::Soldier && (siege_c.is_some() || assault_on)) { e.dead = true; continue; }
            // v25 ขวัญแตก: วิ่งหนีออกจากศึก (หนีพ้น = หายไป ไม่ดรอปของ)
            if e.fl {
                let l = d.max(1.);
                let (ux, uy) = (-dx / l, -dy / l);
                let sp = e.spd * 1.3 * dt;
                for (ax, ay) in [(ux, uy), (ux - uy * 0.8, uy + ux * 0.8), (ux + uy * 0.8, uy - ux * 0.8)] {
                    let (nx, ny) = (e.x + ax * sp, e.y + ay * sp);
                    if !collide(&self.w, nx, ny) { e.x = nx; e.y = ny; if ax.abs() > 0.1 { e.vx = ax; } break; }
                }
                e.anim += dt;
                e.flash = (e.flash - dt).max(0.);
                e.sk = e.sk.min(0.) - dt;
                if d > 220. || e.sk < -6. { e.dead = true; self.burst(e.x, e.y - 6., hx(0xb8a888), 6, 30.); }
                continue;
            }
            let (mut mx, mut my): (f32, f32);
            if e.kb > 0. {
                e.kb -= dt;
                mx = e.vx;
                my = e.vy;
            } else {
                let fire = if self.darkv > 0.3 { self.w.near_fire(tfloor(e.x), tfloor(e.y), 3) } else { None };
                if let Some((fx, fy)) = fire {
                    let (ax, ay) = (fx as f32 * 16. + 8. - e.x, fy as f32 * 16. + 8. - e.y);
                    let fd = ax.hypot(ay).max(0.001);
                    mx = -ax / fd * e.spd;
                    my = -ay / fd * e.spd;
                } else if e.k == EK::Soldier {
                    // ทัพบุก: มุ่งลานกลางเมือง (ถ้าเราเข้าใกล้ค่อยหันมาสู้) · ยามเมือง: ไล่จับเรา — หาทางอ้อมสิ่งกีดขวาง
                    let goal = match siege_c { Some(c) if d > 90. => c, _ => allies.iter().copied().chain(std::iter::once((self.p.x, self.p.y))).min_by(|a, b| (a.0 - e.x).hypot(a.1 - e.y).partial_cmp(&(b.0 - e.x).hypot(b.1 - e.y)).unwrap()).unwrap_or((self.p.x, self.p.y)) };
                    let mode = if siege_c.is_some() { Pass::Breach } else { Pass::Walk };
                    let (gx, gy) = e.path.steer(&self.w, e.x, e.y, goal, mode, dt, &mut path_budget);
                    let (ax, ay) = (gx - e.x, gy - e.y);
                    let l = ax.hypot(ay).max(0.001);
                    let sp = if l < 2. { 0. } else { e.spd };
                    mx = ax / l * sp;
                    my = ay / l * sp;
                } else if let Some((a, b)) = self.beast_ai(e, dx, dy, d, dt) {
                    mx = a;
                    my = b;
                } else if d < 130. {
                    mx = dx / d * e.spd;
                    my = dy / d * e.spd;
                } else {
                    e.wt -= dt;
                    if e.wt <= 0. {
                        e.wt = gen_range(1., 3.);
                        let a = gen_range(0., std::f32::consts::TAU);
                        let go = gen_range(0., 1.) < 0.6;
                        e.wdx = if go { a.cos() } else { 0. };
                        e.wdy = if go { a.sin() } else { 0. };
                    }
                    mx = e.wdx * e.spd * 0.5;
                    my = e.wdy * e.spd * 0.5;
                }
                if e.k == EK::Slime {
                    let ph = (e.anim * 1.6).fract();
                    if ph > 0.55 { mx = 0.; my = 0.; } else { mx *= 1.6; my *= 1.6; }
                }
                if e.k == EK::Bat {
                    mx += (e.anim * 5.).cos() * 25.;
                    my += (e.anim * 7.).sin() * 25.;
                }
            }
            if e.kb <= 0. && e.k != EK::Soldier {
                if let Some((tx, ty)) = torch_near(&self.torches, e.x, e.y) {
                    let (ax, ay) = (e.x - tx, e.y - ty);
                    let l = ax.hypot(ay).max(0.001);
                    mx = ax / l * e.spd.max(30.) * 1.4;
                    my = ay / l * e.spd.max(30.) * 1.4;
                }
            }
            if e.kb <= 0. {
                if e.stun > 0. { mx = 0.; my = 0.; } else if e.slow > 0. { mx *= 0.45; my *= 0.45; }
            }
            if e.k == EK::Bat || e.k == EK::Imp {
                e.x += mx * dt;
                e.y += my * dt;
            } else {
                let (ox, oy) = (e.x, e.y);
                if !collide(&self.w, e.x + mx * dt, e.y) { e.x += mx * dt; }
                if !collide(&self.w, e.x, e.y + my * dt) { e.y += my * dt; }
                // ทหารติดกำแพง/ประตูของเรา → ทุบ
                if e.k == EK::Soldier && siege_c.is_some() && (e.x - ox).abs() + (e.y - oy).abs() < 0.01 && (mx != 0. || my != 0.) {
                    e.wt -= dt;
                    if e.wt <= 0. {
                        e.wt = 0.8;
                        let l = mx.hypot(my).max(0.001);
                        let k = (tfloor(e.x + mx / l * 10.), tfloor(e.y - 3. + my / l * 10.));
                        if self.w.placed.get(&k).map_or(false, |&o| is_wall_obj(o) || o == 38) { wall_hits.push(k); }
                    }
                }
            }
            if e.k == EK::Skel && self.darkv < 0.15 {
                e.hp -= dt * 2.;
                if gen_range(0., 1.) < 0.2 {
                    self.parts.push(Particle { x: e.x + gen_range(-4., 4.), y: e.y - 8., vx: 0., vy: -20., l: 0.5, c: GRAY });
                }
                if e.hp <= 0. {
                    e.dead = true;
                    let (x, y) = (e.x, e.y);
                    self.burst(x, y - 6., LIGHTGRAY, 10, 50.);
                }
            }
            if e.k == EK::Bat && self.darkv < 0.15 && gen_range(0., 1.) < dt * 0.3 { e.dead = true; }
            let lunge = matches!(e.k, EK::Scorp | EK::Wolf) && e.kb > 0. && e.flash <= 0.;
            if d < 11. && self.p.invul <= 0. && (e.kb <= 0. || lunge) && e.stun <= 0. && !e.dead {
                let dm = ((e.dmg as f32 * self.morale_dmg_mul(e)).round() as i32).max(1);
                self.hurt(dm, dx / d, dy / d);
            }
        }
        // v25 กันศัตรูเดินซ้อนกันเป็นก้อน (ตารางช่อง เช็กเฉพาะช่องรอบตัว)
        {
            let ids: Vec<usize> = (0..en.len()).filter(|&i| !en[i].dead && en[i].k != EK::Bat && en[i].k != EK::Imp && en[i].kb <= 0.).collect();
            if ids.len() >= 2 {
                let pts: Vec<(f32, f32)> = ids.iter().map(|&i| (en[i].x, en[i].y)).collect();
                let dd = war::separation(&pts, 0, 10.);
                let k = (dt * 10.).min(1.);
                for (j, &i) in ids.iter().enumerate() {
                    let (nx, ny) = (en[i].x + dd[j].0 * k, en[i].y + dd[j].1 * k);
                    if !collide(&self.w, nx, ny) { en[i].x = nx; en[i].y = ny; }
                }
            }
        }
        en.retain(|e| !e.dead);
        if self.died { en.clear(); self.died = false; }
        self.en = en;
        for k in wall_hits {
            let h = self.obj_dmg.get(&k).copied().unwrap_or(0) + 1;
            self.shake = (k, 0.2);
            self.burst(k.0 as f32 * 16. + 8., k.1 as f32 * 16. + 8., hx(0x9a9aa6), 5, 40.);
            play_vol(&self.sfx.hit, 0.5);
            let hp = match self.w.placed.get(&k) { Some(&38) => 14, Some(&11) => 9, _ => 6 };
            if h >= hp { self.obj_dmg.remove(&k); self.w.placed.remove(&k); self.room_t = 0.; self.toast("ทหารศัตรูพังกำแพง!"); } else { self.obj_dmg.insert(k, h); }
        }
    }

    fn update(&mut self, dt: f32, inp: &Input) {
        if self.ending_tick(dt, inp) { return; }
        self.time += dt;
        let hr = ((self.time / DAYLEN * 24.) + 6.) % 24.;
        self.darkv = (((hr - 12.).abs() - 5.) / 2.).clamp(0., 1.) * 0.82;
        self.darkv = if let Some(d) = &self.w.dun { if d.cave { 0.8 } else { 0.62 } } else { (self.darkv + self.rain * 0.18).min(0.88) };
        if inp.eat { self.eat(); }
        if inp.fire { self.place_fire(); }
        if inp.potion { self.drink(); }
        if let Some(k) = inp.hb { self.hotbar_use(k); }
        if inp.swap { self.swap_weapon(); }
        if inp.dash { self.try_dash(inp.mv); }
        if inp.act { self.interact(); }
        if inp.place { self.place_piece(); self.paint_last = None; }
        if inp.ride { self.toggle_ride(); }
        prof::t("paint_tick", || self.paint_tick(inp.place_hold));
        if inp.rot { self.rotate_build(); }
        if inp.form { self.cycle_formation(); }
        if inp.horn { self.blow_horn(); }
        if inp.cancel && self.build_sel.is_some() && !self.move_cancel() { self.build_sel = None; self.toast("ยกเลิกโหมดสร้าง"); }
        if self.rs.carry.is_some() && !self.moving_on() { self.rs.carry = None; }
        if self.build_sel.is_none() { self.build_cur = None; }
        if self.build_cur.is_some() && (self.p.x - self.cur_pos.0).hypot(self.p.y - self.cur_pos.1) > 20. { self.build_cur = None; }
        if self.sleep_fx > 0. { self.sleep_fx -= dt; }
        prof::t("tut_update", || self.tut_update(dt));
        prof::t("npc_tick", || self.npc_tick(dt));
        let wmul = self.weather_mul();
        // movement
        let mut mv = inp.mv;
        if self.sit.is_some() { if mv.length() > 0.3 || inp.dash { self.sit = None; } else { mv = Vec2::ZERO; } }
        let il = mv.length();
        if il > 1. { mv /= il; }
        let boating = self.on_boat();
        if boating && !self.boated { self.boated = true; }
        let nmul = self.need_speed_mul() * if boating { 1.35 } else { self.ground_speed_mul() } * self.ride_mul();
        let p = &mut self.p;
        p.moving = il > 0.15;
        if p.moving && p.atk_t <= 0. {
            if mv.x.abs() > mv.y.abs() { p.dir = 2; p.flip = mv.x < 0.; } else { p.dir = if mv.y > 0. { 0 } else { 1 }; }
        }
        let sp = 64. * p.speed_mul() * wmul * nmul * if p.atk_t > 0. { 0.5 } else { 1. };
        let (mut mx, mut my) = if p.dash_t > 0. { (p.dash_dir.x * 250. * dt, p.dash_dir.y * 250. * dt) } else if p.moving { (mv.x * sp * dt, mv.y * sp * dt) } else { (0., 0.) };
        if p.kbt > 0. { p.kbt -= dt; mx += p.kbx * dt; my += p.kby * dt; }
        let boat = p.boat;
        let (ox, oy) = (p.x, p.y);
        // ชนมุม: ดันตัวเลี่ยงให้เองเล็กน้อย (ลื่นผ่านมุมต้นไม้/กำแพงแทนการติดค้าง)
        let blocked_x = mx != 0. && collide_p(&self.w, p.x + mx, p.y, boat);
        if !blocked_x { p.x += mx; } else if my.abs() < mx.abs() * 0.5 {
            for d in [1., 2., 3., 4., 5., 6.] {
                let s = if !collide_p(&self.w, p.x + mx, p.y - d, boat) && !collide_p(&self.w, p.x, p.y - d, boat) { -1. } else if !collide_p(&self.w, p.x + mx, p.y + d, boat) && !collide_p(&self.w, p.x, p.y + d, boat) { 1. } else { 0. };
                if s != 0. { p.y += s * mx.abs().min(d); break; }
            }
        }
        let blocked_y = my != 0. && collide_p(&self.w, p.x, p.y + my, boat);
        if !blocked_y { p.y += my; } else if mx.abs() < my.abs() * 0.5 {
            for d in [1., 2., 3., 4., 5., 6.] {
                let s = if !collide_p(&self.w, p.x - d, p.y + my, boat) && !collide_p(&self.w, p.x - d, p.y, boat) { -1. } else if !collide_p(&self.w, p.x + d, p.y + my, boat) && !collide_p(&self.w, p.x + d, p.y, boat) { 1. } else { 0. };
                if s != 0. { p.x += s * my.abs().min(d); break; }
            }
        }
        let moved = (p.x - ox).hypot(p.y - oy);
        if p.moving { p.anim += dt; }
        if p.invul > 0. { p.invul -= dt; }
        if p.atk_t > 0. { p.atk_t -= dt; }
        if p.atk_cd > 0. { p.atk_cd -= dt; }
        prof::t("step_tick", || self.step_tick(moved));
        self.amb_t -= dt;
        if self.amb_t <= 0. { self.amb_t = 0.25; self.amb = self.amb_targets(); }
        self.aim_last_t -= dt;
        if self.aim_last_t <= 0. { self.aim_last = None; }
        prof::t("auto_door", || self.auto_door(mv));
        if inp.atk && self.p.atk_cd <= 0. {
            prof::t("law_on_attack", || self.law_on_attack(self.p.weapon == 2));
            let mut aim = inp.aim.filter(|a| a.length() > 0.01);
            if aim.is_none() && self.auto_aim {
                // ดาบ: หาเป้าไกลขึ้นนิด (ศัตรูโดนตีกระเด็นออกไปยังหันตามได้)
                aim = self.auto_target(if self.p.weapon == 0 { 40. } else { 180. });
            }
            if let Some(a) = aim { self.face(a); self.aim_last = Some((self.p.x + a.x, self.p.y - 6. + a.y)); self.aim_last_t = 1.5; }
            if self.p.weapon == 0 { self.do_attack(); } else { self.shoot(aim); }
        }
        // ออร่าอาวุธตามระดับตีบวก
        let wp = self.p.wplus[self.p.weapon as usize];
        if wp > 0 && gen_range(0., 1.) < dt * (3. + 4. * wp as f32) {
            let (fx, fy) = self.p.facing();
            let (ax, ay) = (self.p.x + fx * 8. + gen_range(-3., 3.), self.p.y - 8. + fy * 4. + gen_range(-4., 4.));
            self.parts.push(Particle { x: ax, y: ay, vx: gen_range(-8., 8.), vy: -16., l: 0.5, c: hx(AURA[wp as usize]) });
        }
        // regen
        self.p.last_hit += dt;
        self.p.regen += dt;
        let fire_near = self.w.near_fire(tfloor(self.p.x), tfloor(self.p.y), 3).is_some();
        prof::t("needs_update", || self.needs_update(dt));
        if self.p.regen > if fire_near { 1.5 } else { 8. } && self.p.last_hit > 4. && self.p.hp < self.p.maxhp && self.can_regen() {
            self.p.regen = 0.;
            self.p.hp += 1;
            if fire_near { let (x, y) = (self.p.x, self.p.y - 20.); self.add_float(x, y, "+1 HP", hx(0xff8888), None); }
        }
        // biome discovery
        let bt = self.w.tile_at(tfloor(self.p.x), tfloor(self.p.y));
        if self.w.dun.is_none() && bt as i32 != self.last_biome {
            self.last_biome = bt as i32;
            if (bt <= 8 || (15..=17).contains(&bt)) && self.seen.insert(bt) && self.seen.len() > 1 {
                prof::t("toast", || self.toast(format!("ค้นพบ: {}", TN[bt as usize])));
                self.p.xp += 3;
                prof::t("check_lv", || self.check_lv());
            }
        }
        // enemies
        self.spawn_t -= dt;
        if self.spawn_t <= 0. { self.spawn_t = 1.; self.try_spawn(); }
        prof::t("animals_tick", || self.animals_tick(dt));
        if self.w.dun.is_none() { self.pets_tick(dt); }
        if peaceful() && self.w.dun.is_none() && !self.en.is_empty() { self.en.clear(); }
        self.pick_cd -= dt;
        prof::t("upd_buffs", || self.upd_buffs(dt));
        prof::t("upd_fish", || self.upd_fish(dt));
        self.torch_t -= dt;
        if self.torch_t <= 0. { self.torch_t = 0.5; self.refresh_torches(); }
        prof::t("upd_dash", || self.upd_dash(dt));
        prof::t("upd_weather", || self.upd_weather(dt));
        prof::t("upd_crops", || self.upd_crops(dt));
        prof::t("upd_traps", || self.upd_traps(dt));
        prof::t("upd_proj", || self.upd_proj(dt));
        prof::t("upd_bosses", || self.upd_bosses(dt));
        self.boss_t -= dt;
        if self.boss_t <= 0. { self.boss_t = 2.; self.try_spawn_boss(); }
        if self.boss_cd > 0. { self.boss_cd -= dt; }
        self.camp_t -= dt;
        if self.camp_t <= 0. { self.camp_t = 1.; self.discover_camps(); self.discover_marks(); self.tips_check(); }
        prof::t("house_tick", || self.house_tick(dt));
        prof::t("sites_tick", || self.sites_tick(dt));
        prof::t("world_steps_tick", || self.world_steps_tick(dt));
        prof::t("nav_tick", || self.nav_tick(dt));
        prof::t("realm_tick", || self.realm_tick(dt));
        prof::t("psy_tick", || self.psy_tick(dt));
        prof::t("land_tick", || self.land_tick(dt));
        prof::t("upd_enemies", || self.upd_enemies(dt));
        prof::t("upd_drops", || self.upd_drops(dt));
        prof::t("upd_comp", || self.upd_comp(dt));
        prof::t("v19_tick", || self.v19_tick(dt));
        prof::t("work_tick", || self.work_tick(dt));
        prof::t("advisor", || self.advisor_tick(dt));
        self.rs.saga_t -= dt;
        if self.rs.saga_t <= 0. { self.rs.saga_t = 2.; prof::t("saga", || self.saga_tick()); }
        prof::t("living", || self.living_tick(dt));
        prof::t("war", || self.war_tick(dt));
        self.dun_t -= dt;
        if self.dun_t <= 0. { self.dun_t = 1.; self.discover_duns(); self.discover_wps(); }
        // fx
        for p in self.parts.iter_mut() { p.x += p.vx * dt; p.y += p.vy * dt; p.vy += 120. * dt; p.l -= dt; }
        self.parts.retain(|p| p.l > 0.);
        for f in self.floats.iter_mut() { f.y -= 18. * dt; f.l -= dt; }
        self.floats.retain(|f| f.l > 0.);
        for t in self.toasts.iter_mut() { t.1 -= dt; }
        for t in self.tips.iter_mut() { t.1 -= dt; }
        self.tips.retain(|t| t.1 > 0.);
        self.toasts.retain(|t| t.1 > 0.);
        if self.shake.1 > 0. { self.shake.1 -= dt; }
        if self.cshake.1 > 0. { self.cshake.1 -= dt; }
        if self.hurt_flash > 0. { self.hurt_flash -= dt; }
        self.save_t += dt;
        if self.save_t > 10. { self.save_t = 0.; self.save(); }
    }
}
fn self_slime_col(pal: usize) -> u32 {
    [0x66cc66, 0xf0a040, 0x7cc8f0, 0x4aa86a][pal]
}

// ======================= rendering =======================
struct View { s: f32, vw: f32, vh: f32, cx: f32, cy: f32 }
fn blit(sp: &Sprites, v: &View, r: Rect, x: f32, y: f32, flip: bool, c: Color) {
    draw_texture_ex(&sp.tex, x * v.s, y * v.s, c, DrawTextureParams { source: Some(r), dest_size: Some(vec2(r.w * v.s, r.h * v.s)), flip_x: flip, ..Default::default() });
}
fn blit_sz(sp: &Sprites, v: &View, r: Rect, x: f32, y: f32, w: f32, h: f32, c: Color) {
    draw_texture_ex(&sp.tex, x * v.s, y * v.s, c, DrawTextureParams { source: Some(r), dest_size: Some(vec2(w * v.s, h * v.s)), ..Default::default() });
}
fn prect(v: &View, x: f32, y: f32, w: f32, h: f32, c: Color) {
    draw_rectangle(x * v.s, y * v.s, w * v.s, h * v.s, c);
}
fn make_view(cam_x: f32, cam_y: f32) -> View {
    let (sw, sh) = (screen_width(), screen_height());
    let s = (sw.min(sh) / 200.).floor().max(2.);
    let (vw, vh) = ((sw / s).ceil(), (sh / s).ceil());
    View { s, vw, vh, cx: (cam_x - vw / 2.).round(), cy: (cam_y - vh / 2. - 6.).round() }
}

enum Draw { Obj(u8, i32, i32), Enemy(usize), Animal(usize), Boss(usize), Player, Loot(usize), Comp, Npc(usize), Lost(f32, f32, u8, u8), Citizen(usize), Caravan(usize), Dying(usize), Banner }

fn render_world(g: &Game, sp: &Sprites, v: &View, t_now: f32, font: &Font) {
    let w = &g.w;
    let (tx0, ty0) = ((v.cx / 16.).floor() as i32, (v.cy / 16.).floor() as i32);
    let (tx1, ty1) = (((v.cx + v.vw) / 16.).floor() as i32, ((v.cy + v.vh) / 16.).floor() as i32);
    let wf = ((t_now / 0.6) as i32 % 2) as usize;
    let foam = Color::new(0.86, 0.94, 1., 0.75);
    let mut lava_l: Vec<(f32, f32, f32, u8)> = vec![];
    let ssn = season(g);
    let indun = w.dun.is_some();
    vfx::set_frame(g, t_now);
    for ty in ty0..=ty1 {
        for tx in tx0..=tx1 {
            let t = w.tile_at(tx, ty) as usize;
            let var = (hash(tx, ty, w.seed.wrapping_add(9)) * 4.) as usize;
            let (px, py) = (tx as f32 * 16. - v.cx, ty as f32 * 16. - v.cy);
            let idx = if t < 2 || t == T_LAVA as usize { (var & 2) | wf } else { var };
            blit(sp, v, sp.tiles[t][idx], px, py, false, if indun { WHITE } else { ground_tint(ssn, t as u8) });
            // รอยต่อพื้น (ขอบหยัก + ฟองคลื่นริมฝั่ง) แล้วรายละเอียดเล็ก (ประกายน้ำ ใบหญ้า)
            let shore = if indun { 0 } else { vfx::draw_edges(w, sp, v, tx, ty, t as u8, px, py, ssn, wf) };
            if !indun { vfx::tile_detail(v, tx, ty, t as u8, px, py, ssn, w.seed); }
            if ssn == 3 && !indun && wintry(t as u8) { prect(v, px, py, 16., 16., Color::new(1., 1., 1., 0.5 + hash(tx, ty, 919) as f32 * 0.08)); }
            if t == T_LAVA as usize && (tx + ty) % 2 == 0 { lava_l.push((px + 8., py + 8., pulse_r(30., t_now, tx as f32 * 0.7 + ty as f32, 1.2), 4)); }
            if t < 2 && fish_spot(w.seed, tx, ty) {
                let ph = ((t_now * 0.9 + hash(tx, ty, 3) as f32 * 6.).fract()) * 6.;
                draw_circle_lines((px + 8.) * v.s, (py + 8.) * v.s, ph * v.s, 1., Color::new(1., 1., 1., 0.6 * (1. - ph / 6.)));
            }
            if t < 2 {
                if shore & 1 == 0 && w.tile_at(tx, ty - 1) > 1 { prect(v, px, py, 16., 1., foam); }
                if shore & 4 == 0 && w.tile_at(tx, ty + 1) > 1 { prect(v, px, py + 15., 16., 1., foam); }
                if shore & 8 == 0 && w.tile_at(tx - 1, ty) > 1 { prect(v, px, py, 1., 16., foam); }
                if shore & 2 == 0 && w.tile_at(tx + 1, ty) > 1 { prect(v, px + 15., py, 1., 16., foam); }
            } else if t == 6 {
                if w.tile_at(tx, ty + 1) != 6 { prect(v, px, py + 13., 16., 3., hx(0x3e3733)); }
                if let Some(&h) = g.obj_dmg.get(&(tx, ty)) {
                    for i in 0..h * 2 { prect(v, px + 3. + i as f32 * 2., py + 5. + (i % 2) as f32 * 3., 1., 3., hx(0x2a2522)); }
                }
            }
        }
    }
    draw_telegraphs(g, v);
    draw_interior(g, v, t_now, 0);
    let hidden = interior_hidden(g);
    // y-sort
    let mut list: Vec<(f32, Draw)> = vec![];
    let mut lights: Vec<(f32, f32, f32, u8)> = lava_l;
    for ty in ty0 - 1..=ty1 + 3 {
        for tx in tx0 - 2..=tx1 + 2 {
            let o = w.obj_at(tx, ty);
            if o == 0 || hidden.contains(&(tx, ty)) { continue; }
            if o == O_LAMP {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 3. - v.cy, pulse_r(60., t_now, tx as f32 * 1.1, 1.), 2));
            }
            if o == O_WINDOW && g.darkv > 0.3 && w.dun.is_none() {
                // หน้าต่างบ้านเรืองแสงตอนกลางคืน
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 10. - v.cy, 30., 2));
            } else if o == O_BANNER {
                // โคมที่ธงเมือง
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. - 6. - v.cy, pulse_r(58., t_now, tx as f32 * 0.9, 1.), 2));
            } else if o == 37 && w.realm.city_at(tx, ty).is_some() {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 4. - v.cy, pulse_r(70., t_now, tx as f32, 1.5), 1));
            } else if o == O_STALL {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 6. - v.cy, 24., 2));
            }
            if o == 9 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 10. - v.cy, pulse_r(80., t_now, tx as f32 * 1.7 + ty as f32 * 0.9, 2.5), 1));
            } else if o == 15 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 8. - v.cy, 58., 1));
            } else if o == 18 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 6. - v.cy, pulse_r(66., t_now, tx as f32 * 1.7 + ty as f32 * 0.9, 2.), 1));
            } else if o == 19 || o == 20 || o == 36 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 8. - v.cy, 40., 0));
            } else if o == 25 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 6. - v.cy, 46. + (t_now * 1.6 + tx as f32).sin() * 4., 0));
            } else if o == 26 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 8. - v.cy, pulse_r(44., t_now, tx as f32 * 1.7 + ty as f32 * 0.9, 1.5), 1));
            } else if o == O_ALTAR || o == O_CRYSTAL {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 4. - v.cy, pulse_r(34., t_now, tx as f32 * 1.3, 1.5), 0));
            } else if o == O_GIANT {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. - 30. - v.cy, 40., 0));
            } else if o == 24 || o == 23 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 8. - v.cy, 26., 0));
            }
            let flat = matches!(o, 3 | 19 | 20 | 21 | 22 | 36 | O_RUG) || land_flat(o);
            list.push((ty as f32 * 16. + if flat { 0. } else { 14. }, Draw::Obj(o, tx, ty)));
        }
    }
    for (i, e) in g.en.iter().enumerate() {
        let (sx, sy) = (e.x - v.cx, e.y - v.cy);
        if sx < -20. || sx > v.vw + 20. || sy < -30. || sy > v.vh + 30. { continue; }
        list.push((e.y, Draw::Enemy(i)));
        if e.k == EK::Skel && g.darkv > 0.3 { lights.push((sx, sy - 11., 12., 0)); }
        if e.k == EK::Imp && g.darkv > 0.2 { lights.push((sx, sy - 14., 18., 1)); }
    }
    if let Some(b) = g.rs.war.banner { list.push((b.1, Draw::Banner)); }
    for (i, d) in g.dying.iter().enumerate() {
        if (d.x - g.p.x).abs() < 440. && (d.y - g.p.y).abs() < 320. { list.push((d.y - 0.5, Draw::Dying(i))); }
    }
    list.push((g.p.y, Draw::Player));
    for (i, a) in g.animals.iter().enumerate() {
        let (sx, sy) = (a.x - v.cx, a.y - v.cy);
        if sx < -30. || sx > v.vw + 30. || sy < -40. || sy > v.vh + 20. { continue; }
        list.push((a.y, Draw::Animal(i)));
    }
    for (i, l) in g.drops.iter().enumerate() {
        let (sx, sy) = (l.x - v.cx, l.y - v.cy);
        if sx < -10. || sx > v.vw + 10. || sy < -10. || sy > v.vh + 10. { continue; }
        list.push((l.y, Draw::Loot(i)));
        lights.push((sx, sy - 4., 10., 0));
    }
    if let Some(c) = &g.comp { list.push((c.y, Draw::Comp)); }
    for (i, n) in g.npcs.iter().enumerate() {
        let (sx, sy) = (n.x - v.cx, n.y - v.cy);
        if sx < -12. || sx > v.vw + 12. || sy < -20. || sy > v.vh + 12. { continue; }
        list.push((n.y, Draw::Npc(i)));
    }
    for (i, c) in g.rs.lv.cara.iter().enumerate() {
        if (c.x - g.p.x).abs() < 440. && (c.y - g.p.y).abs() < 320. { list.push((c.y, Draw::Caravan(i))); }
    }
    for (i, c) in g.rs.cit.iter().enumerate() {
        if (c.x - g.p.x).abs() < 420. && (c.y - g.p.y).abs() < 300. { list.push((c.y, Draw::Citizen(i))); }
    }
    if g.w.dun.is_none() {
        let (pcx, pcy) = (tfloor(v.cx + v.vw / 2.).div_euclid(CELL), tfloor(v.cy + v.vh / 2.).div_euclid(CELL));
        for cy in pcy - 1..=pcy + 1 {
            for cx in pcx - 1..=pcx + 1 {
                if g.npc_seen.contains(&(cx, cy)) { continue; }
                if let Some((x, y, name, look)) = g.w.lost_in_cell(cx, cy) {
                    let (sx, sy) = (x - v.cx, y - v.cy);
                    if sx < -12. || sx > v.vw + 12. || sy < -30. || sy > v.vh + 12. { continue; }
                    list.push((y, Draw::Lost(x, y, name, look)));
                }
            }
        }
    }
    for (i, b) in g.bosses.iter().enumerate() {
        let (sx, sy) = (b.x - v.cx, b.y - v.cy);
        if sx < -40. || sx > v.vw + 40. || sy < -50. || sy > v.vh + 50. { continue; }
        list.push((b.y, Draw::Boss(i)));
        if g.darkv > 0.3 { lights.push((sx, sy - 14., 26., 0)); }
    }
    list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    // เงาของสิ่งสูงทอดบนพื้นก่อน (อยู่ใต้ทุกอย่าง)
    if !indun {
        for (_, d) in &list {
            if let Draw::Obj(o, tx, ty) = *d {
                if hidden.contains(&(tx, ty)) { continue; }
                let o2 = g.display_obj(o, (tx, ty));
                vfx::obj_shadow(v, tx as f32 * 16. - v.cx, ty as f32 * 16. - v.cy, sp.obj[o2 as usize], o2);
            }
        }
    }
    for (_, d) in &list {
        match *d {
            Draw::Obj(o, tx, ty) => {
                let (px, py) = (tx as f32 * 16. - v.cx, ty as f32 * 16. - v.cy);
                if o == O_BANNER {
                    // ธงสีของเจ้าของเมือง โบกพลิ้ว
                    let col = g.w.realm.city_at(tx, ty).map_or(hx(0xcc3333), |ci| hx(g.owner_color(g.city_owner(ci))));
                    prect(v, px + 7., py - 10., 2., 26., hx(0x5a3418));
                    prect(v, px + 6., py - 11., 4., 2., hx(0xd8b040));
                    let wv = (t_now * 3. + tx as f32).sin();
                    for k in 0..6 { prect(v, px + 9. + k as f32, py - 8. + (wv * k as f32 * 0.25).round(), 1., 9. - (k / 2) as f32, col); }
                    continue;
                }
                if o == 3 {
                    if season(g) == 3 && w.dun.is_none() { continue; }
                    let h = hash(tx, ty, w.seed.wrapping_add(13));
                    let c = hx([0xff6b8a, 0xffd84a, 0xffffff, 0xb98cff][(h * 4.) as usize]);
                    let st = hx(0x3d7a2e);
                    prect(v, px + 5., py + 9., 1., 3., st);
                    prect(v, px + 11., py + 4., 1., 3., st);
                    prect(v, px + 4., py + 8., 3., 1., c);
                    prect(v, px + 5., py + 7., 1., 3., c);
                    prect(v, px + 10., py + 3., 3., 1., c);
                    prect(v, px + 11., py + 2., 1., 3., c);
                    prect(v, px + 5., py + 8., 1., 1., hx(0xffef9a));
                    prect(v, px + 11., py + 3., 1., 1., hx(0xffef9a));
                    continue;
                }
                let sx = if g.shake.0 == (tx, ty) && g.shake.1 > 0. { (g.shake.1 * 80.).sin().round() } else { 0. };
                let near_m = o == 15 && (g.p.x - (tx as f32 * 16. + 8.)).hypot(g.p.y - (ty as f32 * 16. + 8.)) < 48.;
                let o = if o == 21 || o == 22 { if spike_on(g.time, tx, ty) { 22 } else { 21 } } else { o };
                let o = g.display_obj(o, (tx, ty));
                let r = if o == 15 { sp.ex.merchant[if near_m && (t_now * 3.) as i32 % 2 == 0 { 1 } else { 0 }] } else { sp.obj[o as usize] };
                let (px, py) = if matches!(o, 19 | 20 | 21 | 22) { (px, py - (r.h - 16.).max(0.) + (r.h - 16.).max(0.)) } else { (px, py) };
                let ox = if r.w > 16. { 8. - (r.w / 2.).floor() } else { 0. };
                let tc = if w.dun.is_some() { WHITE } else { tree_tint(season(g), o) };
                if !indun && (is_tree(o) || matches!(o, 2 | O_REED | O_DRYGRASS)) && sx == 0. {
                    vfx::blit_sway(sp, v, r, px + ox, py + 16. - r.h, tc, tx as f32 * 0.73 + ty as f32 * 0.41);
                } else {
                    blit(sp, v, r, px + sx + ox, py + 16. - r.h, false, tc);
                }
                if near_m { text_c(font, "พ่อค้า", (px + 8.) * v.s, (py + 16. - r.h - 3.) * v.s, 6. * v.s, hx(0xffee99)); }
                if o == 9 {
                    let c = if gen_range(0., 1.) < 0.5 { hx(0xffe680) } else { hx(0xff8a30) };
                    prect(v, px + 6. + rnd(0, 3) as f32, py + 4. + rnd(0, 3) as f32, 1., 1., c);
                }
            }
            Draw::Enemy(i) => {
                let e = &g.en[i];
                let (sx, sy) = ((e.x - v.cx).round(), (e.y - v.cy).round());
                let white = e.flash > 0. && e.flash > 0.12;
                let c = if e.stun > 0. && (e.anim * 12.) as i32 % 2 == 0 { Color::new(1., 1., 0.5, 1.) } else if e.burn > 0. { Color::new(1., 0.7, 0.5, 1.) } else if e.slow > 0. { Color::new(0.6, 0.8, 1., 1.) } else { WHITE };
                let shadow = Color::new(0., 0., 0., 0.25);
                match e.k {
                    EK::Slime => {
                        let sq = (e.anim * 1.6 * std::f32::consts::TAU).sin() * 0.15;
                        let (ww, hh) = ((12. * (1. + sq)).round(), (10. * (1. - sq)).round());
                        vfx::char_shadow(v, sx, sy, 10.);
                        if e.night { prect(v, sx - 7., sy - hh - 1., 14., hh + 2., Color::new(0.7, 0.3, 1., 0.28)); }
                        vfx::blit_outline(sp, v, sp.slime[e.pal], (sx - ww / 2.).floor(), sy - hh, ww, hh, false, 1.);
                        blit_sz(sp, v, if white { sp.ex.flash_slime[e.pal] } else { sp.slime[e.pal] }, (sx - ww / 2.).floor(), sy - hh, ww, hh, c);
                    }
                    EK::Skel => {
                        let bob = ((e.anim * 6.) as i32 % 2) as f32;
                        vfx::char_shadow(v, sx, sy, 8.);
                        if e.night { prect(v, sx - 6., sy - 16., 12., 16., Color::new(0.7, 0.3, 1., 0.28)); }
                        // โดนตี: เซถอยหลัง 1 พิกเซล
                        let kx = if e.kb > 0. { e.vx.signum() } else { 0. };
                        vfx::blit_outline(sp, v, sp.skel, sx - 5. + kx, sy - 15. - bob, sp.skel.w, sp.skel.h, false, 1.);
                        blit(sp, v, if white { sp.ex.flash_skel } else { sp.skel }, sx - 5. + kx, sy - 15. - bob, false, c);
                    }
                    EK::Soldier => {
                        // ทหารอาณาจักร: คนใส่เกราะ + ผ้าคาดสีอาณาจักร
                        vfx::char_shadow(v, sx, sy, 8.);
                        let body = Body::generate(1000 + e.pal as u32 * 7, false);
                        let (f, wb) = vfx::walk_cycle(e.anim, 9.);
                        let sy = sy - wb;
                        let tint = if white { Color::new(2., 2., 2., 1.) } else { c };
                        skin::draw_figure(sp, &body, skin::ROLE_PARTS[2], (sx - 6.) * v.s, (sy - 15.) * v.s, v.s, 0, f, e.vx < 0., tint);
                        let kc = if e.pal < g.w.realm.ks.len() { hx(g.w.realm.ks[e.pal].color) } else { RED };
                        prect(v, sx - 3., sy - 9., 6., 1., kc);
                        prect(v, sx - 1., sy - 18., 2., 2., kc);
                        if white { war::hit_flash(v, sx, sy, (e.flash - 0.1) * 8.); }
                        if e.fl { prect(v, sx + 4., sy - 24., 2., 5., WHITE); prect(v, sx + 4., sy - 18., 2., 2., WHITE); }
                    }
                    EK::Scorp | EK::Wolf | EK::Crab | EK::Imp => {
                        let tint = if white { Color::new(1., 1., 1., 1.) } else { c };
                        war::draw_beast(v, e.k, sx, sy, e.anim, e.vx < 0., tint, white, false, 1., e.sk);
                    }
                    EK::Bat => {
                        prect(v, sx - 3., sy - 1., 6., 2., Color::new(0., 0., 0., 0.2));
                        if e.night { prect(v, sx - 7., sy - 19., 14., 12., Color::new(0.7, 0.3, 1., 0.28)); }
                        let bi = (e.anim * 8.) as usize % 2;
                        vfx::blit_outline(sp, v, sp.bat[bi], sx - 6., sy - 18. + ((e.anim * 4.).sin() * 2.).round(), sp.bat[bi].w, sp.bat[bi].h, false, 0.8);
                        blit(sp, v, if white { sp.ex.flash_bat[bi] } else { sp.bat[bi] }, sx - 6., sy - 18. + ((e.anim * 4.).sin() * 2.).round(), false, c);
                    }
                }
            }
            Draw::Boss(i) => draw_boss(g, sp, v, i),
            Draw::Animal(i) => draw_animal(g, sp, v, i),
            Draw::Loot(i) => draw_loot(g, sp, v, i, t_now),
            Draw::Comp => draw_comp(g, sp, v),
            Draw::Npc(i) => draw_npc(g, sp, v, i, t_now),
            Draw::Citizen(i) => draw_citizen(g, sp, v, i),
            Draw::Caravan(i) => living::draw_caravan(g, sp, v, i, t_now),
            Draw::Dying(i) => war::draw_dying(g, sp, v, i),
            Draw::Banner => war::draw_banner(g, sp, v, t_now),
            Draw::Lost(x, y, name, look) => draw_lost(g, sp, v, x, y, name, look, t_now),
            Draw::Player => {
                let p = &g.p;
                let (sx, sy) = ((p.x - v.cx).round(), (p.y - v.cy).round());
                // ขี่ม้า: วาดม้าก่อน แล้วยกตัวผู้เล่นขึ้นไปนั่งบนหลังม้า
                if g.rs.riding {
                    vfx::char_shadow(v, sx, sy, 16.);
                    draw_horse(v, sx, sy, t_now, p.moving, p.dir == 2 && p.flip, p.dir != 2);
                }
                let sy = if g.rs.riding { sy - 8. } else { sy };
                for &(gx, gy, gd, gf, gl) in &g.ghosts {
                    blit(sp, v, sp.player[gd][1], (gx - v.cx).round() - 6., (gy - v.cy).round() - 15., gd == 2 && gf, Color::new(0.5, 0.8, 1., gl * 1.2));
                }
                if !(p.invul > 0. && ((p.invul * 15.) as i32) % 2 == 1) {
                    // เดิน 4 จังหวะ (ตัวยกตอนขาผ่านกัน) · ขี่ม้า/เรือ ไม่ก้าว
                    let (f, wb) = if p.moving && !g.rs.riding && !g.on_boat() { vfx::walk_cycle(p.anim, 9.) } else { (0, 0.) };
                    vfx::char_shadow(v, sx, sy, 8.);
                    let sy = sy - wb;
                    if full_meteor(p) {
                        // ชุดเกราะดาวตกครบ 3 ชิ้น: ออร่าม่วงเต้น + ประกายโคจร
                        let pu_ = 0.5 + 0.5 * (t_now * 3.).sin();
                        draw_circle(sx * v.s, (sy - 8.) * v.s, (9. + pu_ * 2.) * v.s, Color::new(0.78, 0.5, 1., 0.12 + 0.08 * pu_));
                        for k in 0..3 {
                            let a = t_now * 2.2 + k as f32 * 2.09;
                            prect(v, sx + a.cos() * 9. - 0.5, sy - 8. + a.sin() * 6. - 0.5, 1., 1., Color::new(1., 0.9, 1., 0.9));
                        }
                    }
                    // ดาบในมือ (ตอนไม่ได้ฟัน) — หันขึ้นวาดหลังตัว หันทางอื่นวาดหน้าตัว
                    let held_sword = p.weapon == 0 && p.sword > 0 && !p.bare && p.atk_t <= 0.;
                    let draw_sword = |front: bool| {
                        let left = p.dir == 3 || (p.dir == 2 && p.flip);
                        let (ox, oy) = match p.dir { 0 => (2., -11.), 1 => (2., -13.), _ => if left { (-9., -12.) } else { (1., -12.) } };
                        let _ = front;
                        draw_texture_ex(&sp.tex, ((sx + ox) * v.s).round(), ((sy + oy) * v.s).round(), hx(SWC[p.sword]), DrawTextureParams { source: Some(sp.icons[ICON_SWORD]), dest_size: Some(vec2(9. * v.s, 9. * v.s)), flip_x: left, ..Default::default() });
                    };
                    if g.on_boat() { draw_boat(v, sx, sy, t_now, p.dir == 2); }
                    let tool = tool_in_hand(g);
                    let held_sword = held_sword && tool == 0;
                    if held_sword && p.dir == 1 { draw_sword(false); }
                    if g.sit.is_some() { skin::draw_skin_sit(sp, p, (sx - 6.) * v.s, (sy - 15.) * v.s, v.s, p.dir, p.dir == 2 && p.flip, WHITE); }
                    else {
                        // ท่าฟัน: โน้มตัวไปทางที่ฟัน · โดนตี: ตัวแดงแวบ
                        let (lx, ly) = if p.atk_t > 0. { let (fx, fy) = p.facing(); ((fx * 1.5).round(), (fy * 1.5).round()) } else { (0., 0.) };
                        let pc = if p.kbt > 0. { Color::new(1., 0.55, 0.55, 1.) } else { WHITE };
                        draw_skin(sp, p, (sx - 6. + lx) * v.s, (sy - 15. + ly) * v.s, v.s, p.dir, f, p.dir == 2 && p.flip, pc);
                    }
                    if held_sword && p.dir != 1 { draw_sword(true); }
                    if tool > 0 && p.dir != 1 {
                        let left = p.dir == 2 && p.flip;
                        let (ox, oy) = match p.dir { 0 => (2., -11.), _ => if left { (-9., -12.) } else { (1., -12.) } };
                        let (x0, y0, ps) = (((sx + ox) * v.s).round(), ((sy + oy) * v.s).round(), 9. * v.s / 12.);
                        if tool == 1 { pix_icon_pick(x0, y0, ps); } else { pix_icon_rod(x0, y0, ps); }
                    }
                    if p.weapon > 0 {
                        let (fx, fy) = p.facing();
                        let r = sp.icons[if p.weapon == 1 { IC_BOW } else { IC_STAFF }];
                        blit(sp, v, r, sx + fx * 8. - 4., sy - 11. + fy * 4., fx < 0., WHITE);
                        let wp = p.wplus[p.weapon as usize];
                        if wp > 0 { let mut ac = hx(AURA[wp as usize]); ac.a = 0.25 + 0.1 * (t_now * 6.).sin(); draw_circle((sx + fx * 8.) * v.s, (sy - 8. + fy * 4.) * v.s, (3. + wp as f32) * v.s, ac); }
                    }
                }
                if p.atk_t > 0. {
                    let (dx, dy) = p.facing();
                    let base = dy.atan2(dx);
                    let prog = 1. - p.atk_t / 0.18;
                    let r = 9. + if p.sword > 0 && !p.bare { 3. } else { 0. };
                    let swc = if p.wplus[0] > 0 { AURA[p.wplus[0] as usize] } else { SWC[p.sword] };
                    for i in 0..7 {
                        let a = base - 1.1 + prog * 2.2 * (i as f32 / 6.);
                        let mut c = hx(swc);
                        c.a = 0.4 + 0.6 * (i as f32 / 6.);
                        prect(v, (sx + a.cos() * r).round() - 1., (sy - 6. + a.sin() * r).round() - 1., 2., 2., c);
                    }
                }
            }
        }
    }
    draw_proj(g, sp, v);
    for pr in &g.proj { if matches!(pr.kind, 1 | 7 | 8 | 9) { lights.push((pr.x - v.cx, pr.y - v.cy, pulse_r(26., t_now, pr.x * 0.3, 1.5), 0)); } }
    draw_interior(g, v, t_now, 1);
    draw_roofs(g, v);
    draw_sites(g, sp, v, font, t_now);
    draw_bubbles(g, v, font);
    living::draw_living(g, v, t_now);
    living::draw_caravan_labels(g, v, font);
    if let Some(sel) = g.build_sel {
        let (tx, ty) = g.build_target();
        // กริดช่วยวางให้ตรงช่อง (เฉพาะช่องที่อยู่ในระยะ)
        let (ptx, pty) = (tfloor(g.p.x), tfloor(g.p.y - 3.));
        for j in -9..=9 {
            for i in -9..=9 {
                let c = (ptx + i, pty + j);
                if g.build_reach(c) {
                    draw_rectangle_lines((c.0 as f32 * 16. - v.cx) * v.s, (c.1 as f32 * 16. - v.cy) * v.s, 16. * v.s, 16. * v.s, 1., Color::new(1., 1., 1., 0.14));
                }
            }
        }
        let po = PIECES[sel].obj;
        if po == OBJ_MOVE {
            mover::draw_move_preview(g, sp, v, font, t_now);
        } else if is_prefab(po) {
            // พรีวิวทั้งหลัง: เขียว = วางได้ · แดง = มีของขวาง (ข้อความบอกเหตุผล)
            let bad = g.prefab_blocked(po, (tx, ty), g.build_dir).or(if can_pay(&g.p, PIECES[sel].cost) { None } else { Some("วัสดุไม่พอ") });
            let tint = if bad.is_none() { Color::new(0.75, 1., 0.75, 0.6) } else { Color::new(1., 0.55, 0.55, 0.55) };
            for ((x, y), c) in g.prefab_cells(po, (tx, ty), g.build_dir) {
                let (px, py) = (x as f32 * 16. - v.cx, y as f32 * 16. - v.cy);
                match c {
                    Cell::Wall(o) => blit(sp, v, sp.obj[o as usize], px, py, false, tint),
                    Cell::Floor(t, f) => {
                        blit(sp, v, sp.tiles[t as usize][0], px, py, false, tint);
                        if f != 0 { let r = sp.obj[f as usize]; blit(sp, v, r, px + 8. - r.w / 2., py + 16. - r.h, false, tint); }
                    }
                    Cell::Ground(f) => {
                        prect(v, px, py, 16., 16., Color::new(tint.r, tint.g, tint.b, 0.18));
                        if f != 0 { let r = sp.obj[f as usize]; blit(sp, v, r, px + 8. - r.w / 2., py + 16. - r.h, false, tint); }
                    }
                }
            }
            let lay = prefab_layout_d(po, g.build_dir);
            let (x0, y0) = (lay.iter().map(|c| c.0 .0).min().unwrap(), lay.iter().map(|c| c.0 .1).min().unwrap());
            let (x1, y1) = (lay.iter().map(|c| c.0 .0).max().unwrap(), lay.iter().map(|c| c.0 .1).max().unwrap());
            let lc = if bad.is_none() { Color::new(0.3, 1., 0.3, 0.9) } else { Color::new(1., 0.3, 0.3, 0.9) };
            draw_rectangle_lines(((tx + x0) as f32 * 16. - v.cx) * v.s, ((ty + y0) as f32 * 16. - v.cy) * v.s, (x1 - x0 + 1) as f32 * 16. * v.s, (y1 - y0 + 1) as f32 * 16. * v.s, 3., lc);
            if po != OBJ_FIRERING {
                // ลูกศรบอกทางเข้า: ชี้ออกจากประตู
                let (ox, oy) = rot_off((0, 1), g.build_dir);
                let (cx0, cy0) = (((tx as f32 + 0.5) * 16. - v.cx) * v.s, ((ty as f32 + 0.5) * 16. - v.cy) * v.s);
                let (ax, ay) = (cx0 + ox as f32 * 14. * v.s, cy0 + oy as f32 * 14. * v.s);
                let (px2, py2) = (-oy as f32, ox as f32);
                let tip = vec2(ax + ox as f32 * 7. * v.s, ay + oy as f32 * 7. * v.s);
                let pul = 0.7 + 0.3 * (t_now * 5.).sin();
                draw_triangle(tip, vec2(ax + px2 * 5. * v.s, ay + py2 * 5. * v.s), vec2(ax - px2 * 5. * v.s, ay - py2 * 5. * v.s), Color::new(1., 0.9, 0.3, pul));
                draw_line(cx0 + ox as f32 * 6. * v.s, cy0 + oy as f32 * 6. * v.s, ax, ay, 2. * v.s, Color::new(1., 0.9, 0.3, pul));
            }
            if let Some(why) = bad {
                let (mx, my) = (((tx as f32) * 16. + 8. - v.cx) * v.s, ((ty + 1) as f32 * 16. + 4. - v.cy) * v.s);
                let fs = (6. * v.s) as u16;
                let d = measure_text(why, Some(font), fs, 1.);
                draw_rectangle(mx - d.width / 2. - 6., my, d.width + 12., fs as f32 + 8., Color::new(0., 0., 0., 0.7));
                draw_text_ex(why, mx - d.width / 2., my + fs as f32, TextParams { font: Some(font), font_size: fs, color: hx(0xff9a9a), ..Default::default() });
            }
        } else {
        let till = po == OBJ_TILL;
        let floor = po == OBJ_FLOOR_W || po == OBJ_FLOOR_S;
        let ok = if !g.build_reach((tx, ty)) { false } else if till { g.can_till(tx, ty) } else if floor { g.build_reach((tx, ty)) && !SOLIDT[g.w.tile_at(tx, ty) as usize] } else { g.can_place_at(tx, ty) && can_pay(&g.p, PIECES[sel].cost) };
        let (px, py) = (tx as f32 * 16. - v.cx, ty as f32 * 16. - v.cy);
        let r = if till { sp.tiles[T_SOIL as usize][0] } else if floor { sp.tiles[if po == OBJ_FLOOR_W { T_FLOORW } else { T_FLOORS } as usize][0] } else { sp.obj[po as usize] };
        blit(sp, v, r, px, py + 16. - r.h, false, Color::new(1., 1., 1., 0.6));
        let lc = if ok { Color::new(0.3, 1., 0.3, 0.9) } else { Color::new(1., 0.3, 0.3, 0.9) };
        draw_rectangle_lines(px * v.s, py * v.s, 16. * v.s, 16. * v.s, 2., lc);
        }
    }
    if let Some(f) = &g.fish {
        let (bx, by) = (f.tx as f32 * 16. + 8., f.ty as f32 * 16. + 8. + if f.bite_t > 0. { 2. } else { (t_now * 3.).sin() * 0.8 });
        let (fx, fy) = g.p.facing();
        let (sx, sy) = (g.p.x + fx * 6., g.p.y - 9.);
        draw_line((sx - v.cx) * v.s, (sy - v.cy) * v.s, (bx - v.cx) * v.s, (by - v.cy) * v.s, v.s.max(1.), Color::new(1., 1., 1., 0.7));
        draw_circle((bx - v.cx) * v.s, (by - v.cy) * v.s, 2. * v.s, hx(0xff3a3a));
        draw_circle_lines((bx - v.cx) * v.s, (by - v.cy) * v.s, (3. + (t_now * 4.).sin().abs() * 2.) * v.s, 1., Color::new(1., 1., 1., 0.5));
        if f.bite_t > 0. { text_c(font, "!", (g.p.x - v.cx) * v.s, (g.p.y - 26. - v.cy) * v.s, 14. * v.s, hx(0xffee44)); }
    }
    // particles
    for p in &g.parts {
        let mut c = p.c;
        c.a = (p.l * 2.).min(1.);
        prect(v, (p.x - v.cx).round(), (p.y - v.cy).round(), 1., 1., c);
    }
    if prefs::weather_fx() && !g.indoors() { draw_weather(g, v, t_now); }
    vfx::draw_ambient(g, v, &mut lights);
    living::fest_lights(g, v, &mut lights);
    // แสง+โทนสี (shader) — ถ้าใช้ไม่ได้ใช้ระบบเดิม (เซลล์สี่เหลี่ยมบน CPU)
    let pr0 = if g.p.weapon == 2 && g.p.staff > 0 { pulse_r(62., t_now, 0., 0.) } else { 48. };
    if g.darkv > 0.02 && g.p.weapon == 2 && g.p.staff > 0 {
        let mc = hx(ELEM_C[g.p.elem as usize]);
        for k in 1..4 { draw_circle((g.p.x - v.cx).round() * v.s, ((g.p.y - v.cy).round() - 6.) * v.s, pr0 * v.s * 0.2 * k as f32, Color::new(mc.r, mc.g, mc.b, 0.05 * g.darkv)); }
    }
    let mut lit_l = lights.clone();
    lit_l.insert(0, ((g.p.x - v.cx).round(), (g.p.y - v.cy).round() - 6., pr0, if g.p.weapon == 2 && g.p.staff > 0 { 3 } else { 0 }));
    let shader_ok = vfx::lighting(g, v, &mut lit_l);
    if g.darkv > 0.02 && !shader_ok {
        // คทาเวทมนตร์: แสงรอบตัวกว้างกว่าและเต้นตามจังหวะ มีสีตามธาตุ
        let staff = g.p.weapon == 2 && g.p.staff > 0;
        let pr = if staff { pulse_r(62., t_now, 0., 0.) } else { 48. };
        lights.push(((g.p.x - v.cx).round(), (g.p.y - v.cy).round() - 6., pr, 0));
        if staff {
            let mc = hx(ELEM_C[g.p.elem as usize]);
            for k in 1..4 {
                draw_circle((g.p.x - v.cx).round() * v.s, ((g.p.y - v.cy).round() - 6.) * v.s, pr * v.s * 0.2 * k as f32, Color::new(mc.r, mc.g, mc.b, 0.05 * g.darkv));
            }
        }
        let lq = prefs::light_q();
        if lq == 0 {
            // ปิดแสงไดนามิก: มืดเรียบ (อ่อนกว่าเดิมเล็กน้อยให้ยังมองเห็น)
            prect(v, 0., 0., v.vw, v.vh, Color::new(0.03, 0.04, 0.15, g.darkv * 0.62));
        } else {
            let cell = if lq == 1 { 10. } else { 4. };
            let mut y = 0.;
            while y < v.vh {
                let mut x = 0.;
                while x < v.vw {
                    let (cx, cy) = (x + cell / 2., y + cell / 2.);
                    let mut lit = 0f32;
                    for &(lx, ly, lr, _) in &lights {
                        let d = (cx - lx).hypot(cy - ly);
                        if d < lr { lit = lit.max(((1. - d / lr) * 1.6).min(1.)); }
                    }
                    let a = g.darkv * (1. - lit);
                    if a > 0.01 { prect(v, x, y, cell, cell, Color::new(0.03, 0.04, 0.15, a)); }
                    x += cell;
                }
                y += cell;
            }
        }
        for &(lx, ly, lr, fire) in &lights {
            if fire == 1 {
                for k in 1..4 {
                    draw_circle(lx * v.s, ly * v.s, lr * v.s * 0.2 * k as f32, Color::new(1., 0.55, 0.15, 0.05 * g.darkv));
                }
            }
        }
    }
    // floats
    for f in &g.floats {
        let fs = (5. * f.sz * v.s) as u16;
        let a = (f.l * 2.).min(1.);
        let (x, y) = ((f.x - v.cx) * v.s, (f.y - v.cy) * v.s);
        let dim = measure_text(&f.t, Some(font), fs, 1.);
        let tx = x - dim.width / 2. - if f.icon.is_some() { 4. * v.s } else { 0. };
        draw_text_ex(&f.t, tx + 2., y + 2., TextParams { font: Some(font), font_size: fs, color: Color::new(0., 0., 0., a), ..Default::default() });
        let mut c = f.c;
        c.a = a;
        draw_text_ex(&f.t, tx, y, TextParams { font: Some(font), font_size: fs, color: c, ..Default::default() });
        if let Some(ic) = f.icon {
            let r = sp.icons[ic];
            draw_texture_ex(&sp.tex, tx + dim.width + v.s, y - 7. * v.s, Color::new(1., 1., 1., a), DrawTextureParams { source: Some(r), dest_size: Some(vec2(8. * v.s, 8. * v.s)), ..Default::default() });
        }
    }
    if g.hurt_flash > 0. {
        draw_rectangle(0., 0., screen_width(), screen_height(), Color::new(1., 0., 0., g.hurt_flash * 0.5));
    }
}

// ---------- UI helpers ----------
/// ยกวรรณยุกต์ขึ้นเมื่อมีสระบนนำหน้า (หรือตามด้วยสระอำ) เพราะ renderer ไม่มี text shaping
/// ใช้ glyph ที่เพิ่มไว้ในฟอนต์ที่ U+F70A..U+F70E
fn thai(s: &str) -> String {
    let cs: Vec<char> = s.chars().collect();
    let mut o = String::with_capacity(s.len());
    for (i, &c) in cs.iter().enumerate() {
        if ('\u{0E48}'..='\u{0E4C}').contains(&c) {
            let prev_upper = i > 0 && matches!(cs[i - 1], '\u{0E31}' | '\u{0E34}'..='\u{0E37}' | '\u{0E47}' | '\u{0E4D}');
            let next_am = cs.get(i + 1) == Some(&'\u{0E33}');
            if prev_upper || next_am {
                o.push(char::from_u32(0xF70A + (c as u32 - 0x0E48)).unwrap());
                continue;
            }
        }
        o.push(c);
    }
    o
}
fn ui_scale() -> f32 {
    (screen_width().min(screen_height()) / 520.).clamp(1., 3.)
}
fn text(font: &Font, s: &str, x: f32, y: f32, size: f32, c: Color) {
    let s = &thai(s);
    let fs = size as u16;
    let o = (size / 14.).max(1.);
    draw_text_ex(s, x + o, y + o, TextParams { font: Some(font), font_size: fs, color: Color::new(0., 0., 0., c.a * 0.8), ..Default::default() });
    draw_text_ex(s, x, y, TextParams { font: Some(font), font_size: fs, color: c, ..Default::default() });
}
fn text_c(font: &Font, s: &str, cx: f32, y: f32, size: f32, c: Color) {
    let d = measure_text(s, Some(font), size as u16, 1.);
    text(font, s, cx - d.width / 2., y, size, c);
}
fn icon(sp: &Sprites, i: usize, x: f32, y: f32, sz: f32) {
    draw_texture_ex(&sp.tex, x, y, WHITE, DrawTextureParams { source: Some(sp.icons[i]), dest_size: Some(vec2(sz, sz)), ..Default::default() });
}
fn button(font: &Font, r: Rect, label: &str, enabled: bool, u: f32) -> bool {
    let (mx, my) = mouse_position();
    let hover = r.contains(vec2(mx, my));
    let bg = if !enabled { hx(0x555555) } else if hover { hx(0xffdd88) } else { hx(0xffcc66) };
    draw_rectangle(r.x, r.y, r.w, r.h, bg);
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., BLACK);
    let col = if enabled { hx(0x222222) } else { hx(0x999999) };
    let label = &thai(label);
    let d = measure_text(label, Some(font), (15. * u) as u16, 1.);
    draw_text_ex(label, r.x + r.w / 2. - d.width / 2., r.y + r.h / 2. + 5. * u, TextParams { font: Some(font), font_size: (15. * u) as u16, color: col, ..Default::default() });
    enabled && hover && is_mouse_button_pressed(MouseButton::Left)
}

fn draw_hud(g: &Game, sp: &Sprites, font: &Font, touch: bool) {
    let u = ui_scale();
    let (x, mut y) = (10. * u, 10. * u);
    let p = &g.p;
    let hr = ((g.time / DAYLEN * 24.) + 6.) % 24.;
    let day = (g.time / DAYLEN) as i32 + 1;
    let night = !(6. ..18.).contains(&hr);
    text(font, &format!("{} {} วันที่ {} · {:02}:{:02} · {}", SEASON_N[season(g) as usize], if night { "คืน" } else { "วัน" }, day, hr as i32, ((hr.fract()) * 60.) as i32, if g.indoors() { format!("ในบ้าน ({})", TN[g.w.biome_at(tfloor(p.x), tfloor(p.y)) as usize]) } else { TN[g.w.biome_at(tfloor(p.x), tfloor(p.y)) as usize].to_string() }), x, y + 13. * u, 13. * u, if night { hx(0xaabbff) } else { hx(0xffee99) });
    y += 20. * u;
    if let Some(st) = g.need_status() { text(font, st, x, y + 10. * u, 12. * u, hx(0xffbb77)); y += 16. * u; }
    // v27 หนังสัตว์/เสื้อขนสัตว์ โชว์เฉพาะหน้าหนาว (ตอนที่ต้องใช้) · ค่าเกราะดูได้ในเมนูตัวละคร
    if season(g) == 3 && (p.hides > 0 || p.coat) {
        text(font, &format!("หนังสัตว์ {}{}", p.hides, if p.coat { " · ใส่เสื้อขนสัตว์" } else { "" }), x, y + 10. * u, 11. * u, hx(0xe8c890));
        y += 14. * u;
    }
    y += 8. * u;
    if p.sp > 0 {
        text(font, &format!("มีแต้มสเตตัส {} แต้ม — {}", p.sp, if touch { "เมนู → ตัวละคร" } else { "กด P" }), x, y + 26. * u, 12. * u, hx(0x88ff88));
        y += 16. * u;
    }
    if let Some(q) = &g.quest {
        let done = q.prog >= q.goal;
        text(font, &format!("เควสต์: {} ({}/{}){}", quest_title(q), q.prog, q.goal, if done { " — ส่งที่แคมป์" } else { "" }), x, y + 26. * u, 12. * u, if done { hx(0x88ff88) } else { hx(0xffdd88) });
        y += 16. * u;
    }
    if p.buff_spd > 0. || p.buff_crit > 0. {
        let mut t = String::from("บัฟ:");
        if p.buff_spd > 0. { t += &format!(" วิ่งเร็ว {}:{:02}", p.buff_spd as i32 / 60, p.buff_spd as i32 % 60); }
        if p.buff_crit > 0. { t += &format!(" คริ+20% {}:{:02}", p.buff_crit as i32 / 60, p.buff_crit as i32 % 60); }
        text(font, &t, x, y + 26. * u, 12. * u, hx(0x88e0ff));
        y += 16. * u;
    }
    if g.in_safe(p.x, p.y) {
        let s = match g.home_comfort() { Some(c) => format!("ในบ้าน · น่าอยู่ {} · ปลอดภัย", stars(c)), None => "เขตปลอดภัย (แสงคบเพลิง)".to_string() };
        text(font, &s, x, y + 26. * u, 12. * u, hx(0x88ff88));
        y += 16. * u;
    }
    if g.w.dun.is_some() {
        text(font, "ดันเจี้ยนลับ — หาทางไปห้องบอส · หนาม/ไฟ ระวัง!", x, y + 26. * u, 12. * u, hx(0xc890ff));
    }
    if !touch {
        let r = if p.dash_cd > 0. { format!("พุ่ง (Shift) {:.1}s", p.dash_cd) } else { "พุ่ง (Shift) พร้อม".to_string() };
        text(font, &r, x, screen_height() - 34. * u, 12. * u, if p.dash_cd > 0. { hx(0x999999) } else { hx(0x88e0ff) });
    }
    // ข้อความแจ้งเตือน: เรียงชิดซ้ายใต้ HUD เหมือนกล่องข้อความ (เก่าอยู่บน ใหม่อยู่ล่าง) ไม่บังกลางจอ
    let (sw, sh) = (screen_width(), screen_height());
    {
        let fs = 13. * u;
        let lh = 16. * u;
        let maxw = hud::layout().feed_w;
        let mut ty = y + 40. * u;
        let limit = sh * 0.62;
        let feed = g.tips.iter().map(|t| (t, true)).chain(g.toasts.iter().map(|t| (t, false)));
        for ((t, l), is_tip) in feed {
            let a = (l * 2.).min(1.);
            let acc = if is_tip { Color::new(0.45, 0.85, 1., a) } else { Color::new(1., 0.8, 0.4, a) };
            let lines = menu::wrap_th(font, t, fs, maxw - 18. * u);
            let h = lines.len() as f32 * lh + 8. * u;
            if ty + h > limit { break; }
            let w = lines.iter().map(|s| measure_text(s, Some(font), fs as u16, 1.).width).fold(0., f32::max) + 18. * u;
            draw_rectangle(x, ty, w, h, Color::new(0.05, 0.06, 0.1, 0.72 * a));
            draw_rectangle(x, ty, 3. * u, h, acc);
            for (k, s) in lines.iter().enumerate() { text(font, s, x + 10. * u, ty + 4. * u + lh * (k as f32 + 0.78), fs, Color::new(1., 1., 0.93, a)); }
            ty += h + 4. * u;
        }
    }
    draw_mc_hud(g, sp, font);
    draw_boss_bar(g, font);
    let hint_y = sh - bottom_hud_h(g, u) - 34. * u;
    if let Some(f) = &g.fish {
        let s = if f.bite_t > 0. { if touch { "ปลากิน! กดปุ่ม \"ใช้\" เลย" } else { "ปลากิน! กด Q เลย" } } else { "รอปลากินเบ็ด... (ขยับ = เลิก)" };
        let d = measure_text(s, Some(font), (15. * u) as u16, 1.);
        draw_rectangle(sw / 2. - d.width / 2. - 12. * u, hint_y, d.width + 24. * u, 28. * u, Color::new(0., 0., 0., 0.65));
        text_c(font, s, sw / 2., hint_y + 20. * u, 15. * u, if f.bite_t > 0. { hx(0xffee44) } else { hx(0xbfe8ff) });
    } else if g.interact_target().is_none() && g.water_ahead().is_some() {
        let thirsty = g.survival_mode() > 0 && g.p.thirst < 90.;
        let s = if thirsty { if touch { "ปุ่ม \"ใช้\": ดื่มน้ำ" } else { "Q: ดื่มน้ำ" } } else if touch { "ปุ่ม \"ใช้\": ตกปลา" } else { "Q: ตกปลา" };
        let d = measure_text(s, Some(font), (15. * u) as u16, 1.);
        draw_rectangle(sw / 2. - d.width / 2. - 12. * u, hint_y, d.width + 24. * u, 28. * u, Color::new(0., 0., 0., 0.65));
        text_c(font, s, sw / 2., hint_y + 20. * u, 15. * u, hx(0xbfe8ff));
    }
    if let Some(s) = g.npc_hint(touch) {
        let d = measure_text(&s, Some(font), (15. * u) as u16, 1.);
        draw_rectangle(sw / 2. - d.width / 2. - 12. * u, hint_y, d.width + 24. * u, 28. * u, Color::new(0., 0., 0., 0.65));
        text_c(font, &s, sw / 2., hint_y + 20. * u, 15. * u, hx(0xffd0e0));
    } else if let Some((_, o)) = g.interact_target() {
        let s = interact_hint(o, touch);
        let d = measure_text(&s, Some(font), (15. * u) as u16, 1.);
        draw_rectangle(sw / 2. - d.width / 2. - 12. * u, hint_y, d.width + 24. * u, 28. * u, Color::new(0., 0., 0., 0.65));
        text_c(font, &s, sw / 2., hint_y + 20. * u, 15. * u, hx(0xffee99));
    } else if let Some(s) = g.farm_hint(touch) {
        let d = measure_text(&s, Some(font), (15. * u) as u16, 1.);
        draw_rectangle(sw / 2. - d.width / 2. - 12. * u, hint_y, d.width + 24. * u, 28. * u, Color::new(0., 0., 0., 0.65));
        text_c(font, &s, sw / 2., hint_y + 20. * u, 15. * u, hx(0xb8ffb0));
    }
    if g.sleep_fx > 0. {
        draw_rectangle(0., 0., sw, sh, Color::new(0., 0., 0.05, g.sleep_fx.min(1.)));
        if g.sleep_fx > 0.5 { text_c(font, "Zzz...", sw / 2., sh / 2., 30. * u, Color::new(1., 1., 1., g.sleep_fx.min(1.))); }
    }
    if g.moving_on() {
        let s = match &g.rs.carry { None => format!("ย้ายของ: แตะของที่จะย้าย 2 ครั้ง (ผนัง/ประตู/พื้นบ้าน = ทั้งหลัง) · {}", if touch { "ปุ่มแดง = เลิก" } else { "R = ยก · T = เลิก" }), Some(c) => format!("กำลังย้าย: {} · แตะที่ใหม่ 2 ครั้ง = วาง{} · {}", c.name, if c.building { " · หมุน = หันบ้าน" } else { "" }, if touch { "ปุ่มแดง = วางคืนที่เดิม" } else { "T = วางคืนที่เดิม" }) };
        build_line(font, &s, hint_y - 34. * u, u, hx(0x9adcff));
    } else if let Some(sel) = g.build_sel {
        let k = if touch { "ปุ่มสร้าง" } else { "R" };
        build_line(font, &format!("โหมดสร้าง: {} · {} = วาง · {} · {}", PIECES[sel].name, k, if touch { "แตะพื้นเพื่อเลือกช่อง" } else { "คลิก/เมาส์ = เลือกช่อง" }, if touch { "ปุ่มแดง = เลิกสร้าง" } else { "T = ยกเลิก" }), hint_y - 34. * u, u, hx(0x88ff88));
    }
    let _ = x;
}
/// v28 บรรทัดบอกโหมดสร้าง: กลางจอเหนือคำใบ้ "ใช้" (เดิมอยู่ชิดล่างซ้าย ทับแถบไอเทม)
fn build_line(font: &Font, s: &str, y: f32, u: f32, c: Color) {
    let fs = 13. * u;
    let d = measure_text(s, Some(font), fs as u16, 1.);
    let cx = screen_width() / 2.;
    draw_rectangle(cx - d.width / 2. - 10. * u, y, d.width + 20. * u, 24. * u, Color::new(0., 0., 0., 0.6));
    text_c(font, s, cx, y + 17. * u, fs, c);
}

fn minimap_rect(big: bool) -> Rect {
    let u = ui_scale();
    let (sw, sh) = (screen_width(), screen_height());
    if big {
        let s = sw.min(sh) * 0.86;
        Rect::new(sw / 2. - s / 2., sh / 2. - s / 2., s, s)
    } else {
        let s = 120. * u;
        Rect::new(sw - s - 10. * u, 10. * u, s, s)
    }
}
fn map_pt(tx: i32, ty: i32, ctr: (i32, i32), sc: i32, mr: Rect) -> (f32, f32, bool) {
    let i = 60. + (tx - ctr.0) as f32 / sc as f32;
    let j = 60. + (ty - ctr.1) as f32 / sc as f32;
    (mr.x + i / 120. * mr.w, mr.y + j / 120. * mr.h, (3.0..117.0).contains(&i) && (3.0..117.0).contains(&j))
}
fn mm_sc(g: &Game) -> i32 { if g.w.dun.is_some() { 1 } else if g.big_map { 4 } else { 2 } }
fn update_minimap(g: &Game, img: &mut Image, tex: &Texture2D) {
    let (c, sc) = ((tfloor(g.p.x), tfloor(g.p.y)), mm_sc(g));
    update_minimap_rows(g, img, tex, c, sc, 0..120);
    finish_minimap(g, img, tex, c, sc);
}
/// วาดแผนที่เล็กทีละช่วงแถว (กระจายงานหลายเฟรม ไม่ให้เฟรมเดียวหนัก)
fn update_minimap_rows(g: &Game, img: &mut Image, tex: &Texture2D, ctr: (i32, i32), sc: i32, rows: std::ops::Range<i32>) {
    let indun = g.w.dun.is_some();
    let (ptx, pty) = ctr;
    for j in rows {
        for i in 0..120 {
            let (tx, ty) = (ptx + (i - 60) * sc, pty + (j - 60) * sc);
            let t = g.w.tile_at(tx, ty);
            let mut c = hx(TCOL[t as usize]);
            if t >= 3 && t != 6 && t != 7 && OSOLID[g.w.obj_at(tx, ty) as usize] { c.r *= 0.8; c.g *= 0.8; c.b *= 0.8; }
            img.set_pixel(i as u32, j as u32, c);
        }
    }
    let _ = tex;
}
/// ใส่สัญลักษณ์ (สิ่งก่อสร้าง ดันเจี้ยน เสาวาร์ป บอส ฯลฯ) แล้วอัปโหลดภาพแผนที่
fn finish_minimap(g: &Game, img: &mut Image, tex: &Texture2D, ctr: (i32, i32), sc: i32) {
    let indun = g.w.dun.is_some();
    let (ptx, pty) = ctr;
    for (&(fx, fy), &v) in g.w.placed.iter().filter(|_| !indun) {
        let col = match v { 17 => 0x8a909c, 18 => 0xffb040, _ => 0 };
        let _ = col;
        let col = match v { 9 => 0xff9020, 10 | 12 | 13 => 0xa06a30, 11 => 0xc0c0cc, 14 => 0xffd84a, 17 => 0xc8d0e0, 18 => 0xffb040, _ => continue };
        let (i, j) = (60 + (fx - ptx) / sc, 60 + (fy - pty) / sc);
        for dy in -1..=1 { for dx in -1..=1 {
            let (a, b) = (i + dx, j + dy);
            if (0..120).contains(&a) && (0..120).contains(&b) { img.set_pixel(a as u32, b as u32, hx(col)); }
        } }
    }
    for &c in g.dun_found.iter().filter(|_| !indun) {
        let Some((cx, cy)) = g.w.dun_in_cell(c.0, c.1) else { continue };
        let (i, j) = (60 + (cx - ptx) / sc, 60 + (cy - pty) / sc);
        for dy in -2..=2 { for dx in -2..=2 {
            let (a, b) = (i + dx, j + dy);
            if (0..120).contains(&a) && (0..120).contains(&b) { img.set_pixel(a as u32, b as u32, hx(0xb060ff)); }
        } }
    }
    for (wx, wy, _) in g.wp_list().into_iter().filter(|_| !indun) {
        let (i, j) = (60 + (wx - ptx) / sc, 60 + (wy - pty) / sc);
        for dy in -2..=2 { for dx in -2..=2 {
            let (a, b) = (i + dx, j + dy);
            if (0..120).contains(&a) && (0..120).contains(&b) && (dx.abs() + dy.abs() < 4) { img.set_pixel(a as u32, b as u32, hx(0x6ae8ff)); }
        } }
    }
    for b in &g.bosses {
        let (i, j) = (60 + (tfloor(b.x) - ptx) / sc, 60 + (tfloor(b.y) - pty) / sc);
        for dy in -2..=2 { for dx in -2..=2 {
            let (a, c) = (i + dx, j + dy);
            if (0..120).contains(&a) && (0..120).contains(&c) && (dx.abs() + dy.abs() < 4) { img.set_pixel(a as u32, c as u32, hx(0xff3030)); }
        } }
    }
    for (&(cx, cy), &k) in g.marks_found.iter().filter(|_| !indun) {
        let (i, j) = (60 + (cx - ptx) / sc, 60 + (cy - pty) / sc);
        for dy in -1..=1 { for dx in -1..=1 {
            let (a, b) = (i + dx, j + dy);
            if (0..120).contains(&a) && (0..120).contains(&b) { img.set_pixel(a as u32, b as u32, hx(mark_letter(k).1)); }
        } }
    }
    for &(cx, cy) in g.camps_found.iter().filter(|_| !indun) {
        let (i, j) = (60 + (cx - ptx) / sc, 60 + (cy - pty) / sc);
        for dy in -2..=2 { for dx in -2..=2 {
            let (a, b) = (i + dx, j + dy);
            if (0..120).contains(&a) && (0..120).contains(&b) { img.set_pixel(a as u32, b as u32, hx(0xffd84a)); }
        } }
    }
    let (si, sj) = (60 + (tfloor(g.spawn.0) - ptx) / sc, 60 + (tfloor(g.spawn.1) - pty) / sc);
    for k in -2..=2 {
        if indun { break; }
        if (0..120).contains(&(si + k)) && (0..120).contains(&sj) { img.set_pixel((si + k) as u32, sj as u32, WHITE); }
        if (0..120).contains(&si) && (0..120).contains(&(sj + k)) { img.set_pixel(si as u32, (sj + k) as u32, WHITE); }
    }
    for j in 58..62 { for i in 58..62 { img.set_pixel(i, j, hx(0xff2222)); } }
    img.set_pixel(59, 59, WHITE); img.set_pixel(60, 60, WHITE); img.set_pixel(59, 60, WHITE); img.set_pixel(60, 59, WHITE);
    tex.update(img);
}

// ---------- touch controls ----------
struct TouchUI { active: bool, joy_id: Option<u64>, joy_c: Vec2, joy_v: Vec2, atk_id: Option<u64>, aim: Vec2, flash: [f32; 12], place_id: Option<u64>, atk_start: Vec2, build_id: Option<u64> }
/// 0 โจมตี/เล็ง · 1 กิน · 2 กองไฟ · 3 เมนู · 4 แผนที่ · 5 สลับอาวุธ · 6 ใช้ · 7 วางสิ่งปลูกสร้าง
/// หน่วยขนาดปุ่มทัช: อิงความสูงจอ (ปุ่มโจมตี ≈ 16 มม. บนมือถือทั่วไป) × ตัวเลือกขนาดปุ่มในตั้งค่า
fn tunit() -> f32 {
    (screen_height() / 450.).clamp(1., 3.2) * prefs::touch_mul()
}
fn touch_buttons() -> [(Vec2, f32, usize); 12] {
    let t = tunit();
    let (sw, sh) = (screen_width(), screen_height());
    let mm = minimap_rect(false);
    let ty = mm.y + 4. * t + 26. * t;
    [
        (vec2(sw - 80. * t, sh - 84. * t), 50. * t, 0),   // โจมตี (ใหญ่สุด หัวแม่มือขวา)
        (vec2(sw - 238. * t, sh - 54. * t), 28. * t, 1),  // กิน
        (vec2(sw - 238. * t, sh - 124. * t), 28. * t, 2), // กองไฟ
        (vec2(mm.x - 36. * t, ty), 26. * t, 3),           // เมนู (มุมขวาบน ไม่ปนกับปุ่มเล่น)
        (vec2(mm.x - 36. * t - 64. * t, ty), 26. * t, 4), // แผนที่
        (vec2(sw - 80. * t, sh - 184. * t), 28. * t, 5),  // สลับอาวุธ
        (vec2(sw - 156. * t, sh - 140. * t), 32. * t, 6), // ใช้
        (vec2(sw - 80. * t, sh - 244. * t), 28. * t, 7),  // วางสิ่งปลูกสร้าง
        (vec2(sw - 170. * t, sh - 56. * t), 32. * t, 8),  // พุ่ง
        (vec2(sw - 238. * t, sh - 194. * t), 26. * t, 9), // ขึ้น/ลงม้า (มีม้าแล้วเท่านั้น)
        (vec2(sw - 156. * t, sh - 226. * t), 26. * t, 10), // v25 จัดแถว (เฉพาะตอนยกทัพ)
        (vec2(sw - 80. * t, sh - 250. * t), 28. * t, 11),  // v25 เป่าแตร (เฉพาะตอนยกทัพ · ตำแหน่งเดียวกับปุ่มวางของซึ่งมีแค่ตอนสร้าง)
    ]
}
/// รัศมีแสงที่ "หายใจ" เข้าออกอย่างนุ่มนวล (ph = เฟสของแหล่งแสง) + ประกายเล็กน้อยสำหรับไฟ
fn pulse_r(base: f32, t: f32, ph: f32, flick: f32) -> f32 {
    base * (1. + 0.08 * (t * 1.7 + ph).sin()) + flick * (t * 19. + ph * 2.7).sin() * (t * 6.3 + ph).sin()
}
fn tile_of(v: &View, p: Vec2) -> (i32, i32) {
    (((p.x / v.s + v.cx) / 16.).floor() as i32, ((p.y / v.s + v.cy) / 16.).floor() as i32)
}
fn handle_touch(tu: &mut TouchUI, inp: &mut Input, build: bool, horse: bool, army: bool, block: &[Rect]) {
    let ts = touches();
    if !ts.is_empty() { tu.active = true; }
    let btns = touch_buttons();
    let u = tunit();
    for f in tu.flash.iter_mut() { *f = (*f - get_frame_time()).max(0.); }
    for t in ts {
        match t.phase {
            TouchPhase::Started => {
                let mut on_btn = false;
                // ปุ่มลัดเหนือหัวชาวบ้าน / แผนที่เล็ก — ผู้เรียกจัดการเอง
                if block.iter().any(|r| r.contains(t.position)) { continue; }
                if let Some(k) = hotbar_hit(t.position) { inp.hb = Some(k); continue; }
                // เลือกปุ่มที่ใกล้ที่สุด (สัดส่วนต่อรัศมี) ปุ่มเดียวต่อการแตะ กันกดซ้อนปุ่มติดกัน
                let mut best: Option<(f32, usize)> = None;
                for (c, r, id) in btns {
                    if id == 7 && !build { continue; }
                    if id == 9 && !horse { continue; }
                    if (id == 10 || id == 11) && (!army || build) { continue; }
                    let q = t.position.distance(c) / r;
                    if q < 1.3 && best.map_or(true, |(bq, _)| q < bq) { best = Some((q, id)); }
                }
                if let Some((_, id)) = best {
                    on_btn = true;
                    tu.flash[id] = 0.16;
                    match id {
                        0 => { tu.atk_id = Some(t.id); tu.aim = Vec2::ZERO; tu.atk_start = t.position; }
                        1 => inp.eat = true,
                        2 => if build { inp.rot = true; } else { inp.fire = true; },
                        3 => inp.menu = Some(0),
                        4 => inp.map = true,
                        5 => if build { inp.cancel = true; } else { inp.swap = true; },
                        6 => inp.act = true,
                        8 => inp.dash = true,
                        9 => inp.ride = true,
                        10 => inp.form = true,
                        11 => inp.horn = true,
                        _ => { inp.place = true; tu.place_id = Some(t.id); }
                    }
                }
                // โหมดสร้าง: แตะ/ลากตรงไหนก็วางตรงนั้น (ยกเว้นมุมซ้ายล่าง = จอยเดิน)
                let joy_zone = if build { t.position.x < screen_width() * 0.34 && t.position.y > screen_height() * 0.42 } else { t.position.x < screen_width() * 0.5 && t.position.y > screen_height() * 0.3 };
                if !on_btn && build && !joy_zone && tu.build_id.is_none() {
                    tu.build_id = Some(t.id);
                    inp.build_tap = Some(t.position);
                    inp.build_start = true;
                }
                if !on_btn && tu.joy_id.is_none() && joy_zone {
                    tu.joy_id = Some(t.id);
                    tu.joy_c = t.position;
                    tu.joy_v = Vec2::ZERO;
                }
            }
            TouchPhase::Moved | TouchPhase::Stationary => {
                if tu.build_id == Some(t.id) { inp.build_tap = Some(t.position); }
                if tu.joy_id == Some(t.id) {
                    let m = 52. * u;
                    let d = t.position - tu.joy_c;
                    tu.joy_v = if d.length() > m { d.normalize() } else { d / m };
                }
                if tu.atk_id == Some(t.id) {
                    // ลากนิ้วออกจากจุดที่แตะ "ชัดๆ" เท่านั้นถึงจะเล็งเอง — นิ้วขยับเล็กน้อยบนปุ่มไม่นับ
                    // (เดิมวัดจากกลางปุ่ม แตะไม่ตรงกลางก็กลายเป็นเล็งทิศมั่ว ตัวละครหันหนีศัตรู)
                    let d = t.position - tu.atk_start;
                    tu.aim = if d.length() > 40. * u { d.normalize() } else { Vec2::ZERO };
                }
            }
            TouchPhase::Ended | TouchPhase::Cancelled => {
                if tu.joy_id == Some(t.id) { tu.joy_id = None; tu.joy_v = Vec2::ZERO; }
                if tu.atk_id == Some(t.id) { tu.atk_id = None; tu.aim = Vec2::ZERO; }
                if tu.place_id == Some(t.id) { tu.place_id = None; }
                if tu.build_id == Some(t.id) { tu.build_id = None; inp.build_end = true; }
            }
        }
    }
    inp.mv += tu.joy_v;
    if tu.place_id.is_some() && build { inp.place_hold = true; } else if !build { tu.place_id = None; }
    if !build { tu.build_id = None; }
    if tu.atk_id.is_some() {
        inp.atk = true;
        if tu.aim.length() > 0.5 { inp.aim = Some(tu.aim); }
    }
}
fn draw_touch(tu: &TouchUI, sp: &Sprites, font: &Font, build: bool, weapon: u8, dash_cd: f32, use_lab: Option<&str>, horse: (bool, bool), army: Option<(u8, f32, bool)>) {
    if !tu.active { return; }
    let u = tunit();
    const TA: f32 = 0.62;
    let wh = Color::new(1., 1., 1., TA + 0.2);
    let ic = |i: usize, x: f32, y: f32, z: f32| draw_texture_ex(&sp.tex, x, y, wh, DrawTextureParams { source: Some(sp.icons[i]), dest_size: Some(vec2(z, z)), ..Default::default() });
    let tc = |s: &str, x: f32, y: f32, z: f32, c: Color| text_c(font, s, x, y, z, Color::new(c.r, c.g, c.b, 0.9));
    let m = 52. * u;
    if tu.joy_id.is_some() {
        draw_circle(tu.joy_c.x, tu.joy_c.y, m + 12. * u, Color::new(1., 1., 1., 0.10));
        draw_circle_lines(tu.joy_c.x, tu.joy_c.y, m + 12. * u, 3., Color::new(1., 1., 1., 0.35));
        let k = tu.joy_c + tu.joy_v * m;
        draw_circle(k.x, k.y, 24. * u, Color::new(1., 1., 1., 0.4));
    } else {
        // จอยสติ๊กลอย: แสดงวงจางๆ ที่มุมซ้ายล่างให้รู้ว่าแตะค้างตรงไหนก็ได้ครึ่งจอซ้าย
        let c = vec2(110. * u, screen_height() - 100. * u);
        draw_circle(c.x, c.y, m, Color::new(1., 1., 1., 0.07));
        draw_circle_lines(c.x, c.y, m, 3., Color::new(1., 1., 1., 0.22));
        draw_circle(c.x, c.y, 20. * u, Color::new(1., 1., 1., 0.14));
    }
    for (c, r, id) in touch_buttons() {
        if id == 7 && !build { continue; }
        if id == 9 && !horse.0 { continue; }
        if (id == 10 || id == 11) && (army.is_none() || build) { continue; }
        let held = (id == 0 && tu.atk_id.is_some()) || tu.flash[id] > 0.;
        let base = if id == 0 { Color::new(0.85, 0.25, 0.15, 0.38) } else if id == 7 { Color::new(0.1, 0.55, 0.25, 0.4) } else { Color::new(0.05, 0.05, 0.1, 0.42) };
        draw_circle(c.x, c.y, r, base);
        if held { draw_circle(c.x, c.y, r, Color::new(1., 1., 1., 0.28)); }
        draw_circle_lines(c.x, c.y, r, 3., Color::new(1., 1., 1., if held { 0.8 } else { 0.4 }));
        let lab = |s: &str| tc(s, c.x, c.y + r * 0.88, 11. * u, hx(0xdddddd));
        match id {
            0 => {
                if weapon == 3 { draw_fist(c.x, c.y, r * 0.5); } else { ic([ICON_SWORD, IC_BOW, IC_STAFF][weapon as usize], c.x - r * 0.55, c.y - r * 0.55, r * 1.1); }
                if held && tu.aim.length() > 0.5 {
                    let e = c + tu.aim * r * 1.8;
                    draw_line(c.x, c.y, e.x, e.y, 3. * u, Color::new(1., 1., 0.6, 0.6));
                    draw_circle(e.x, e.y, 6. * u, Color::new(1., 1., 0.6, 0.7));
                }
            }
            1 => { ic(BERRY, c.x - r * 0.5, c.y - r * 0.6, r); lab("กิน"); }
            2 if build => {
                // โหมดสร้าง: ปุ่มกองไฟกลายเป็น "หมุน" (ลูกศรวนรอบ)
                draw_circle(c.x, c.y, r, Color::new(0.15, 0.4, 0.75, 0.5));
                draw_circle_lines(c.x, c.y - 3. * u, r * 0.42, 3. * u, WHITE);
                draw_triangle(vec2(c.x + r * 0.42 - 6. * u, c.y - 3. * u), vec2(c.x + r * 0.42 + 6. * u, c.y - 3. * u), vec2(c.x + r * 0.42, c.y + 5. * u), WHITE);
                tc("หมุน", c.x, c.y + r * 0.86, 11. * u, WHITE);
            }
            2 => { ic(FIRE, c.x - r * 0.5, c.y - r * 0.6, r); lab("ไฟ"); }
            3 => tc("เมนู", c.x, c.y + 5. * u, 14. * u, WHITE),
            4 => tc("แผนที่", c.x, c.y + 5. * u, 13. * u, WHITE),
            9 => tc(if horse.1 { "ลงม้า" } else { "ม้า" }, c.x, c.y + 5. * u, 13. * u, hx(0xffd8a0)),
            10 => {
                // ไอคอนรูปแถว: จุดเรียงตามแบบที่ใช้อยู่ + ชื่อแถว
                let f = army.map_or(0, |a| a.0);
                draw_circle(c.x, c.y, r, Color::new(0.15, 0.3, 0.6, 0.5));
                let dots: Vec<(f32, f32)> = match f { 1 => vec![(0., -8.), (-5., -2.), (5., -2.), (-10., 4.), (10., 4.)], 2 => (0..6).map(|i| { let a = i as f32 * 1.047; (a.cos() * 9., a.sin() * 9. - 2.) }).collect(), _ => vec![(-10., -6.), (-5., -6.), (0., -6.), (5., -6.), (10., -6.), (-5., 2.), (5., 2.)] };
                for (dx, dy) in dots { draw_circle(c.x + dx * u, c.y + dy * u, 2.4 * u, WHITE); }
                if f == 2 { draw_circle(c.x, c.y - 2. * u, 2.4 * u, hx(0xffd84a)); }
                tc(war::FORM_N[f as usize], c.x, c.y + r * 0.9, 10. * u, WHITE);
            }
            11 => {
                let (cdf, on) = army.map_or((0., false), |a| (a.1, a.2));
                draw_circle(c.x, c.y, r, if on { Color::new(0.85, 0.6, 0.1, 0.6) } else { Color::new(0.45, 0.3, 0.05, 0.5) });
                // แตรเขาสัตว์
                draw_triangle(vec2(c.x - 10. * u, c.y - 2. * u), vec2(c.x + 9. * u, c.y - 10. * u), vec2(c.x + 9. * u, c.y + 6. * u), hx(0xf0d080));
                draw_circle(c.x - 10. * u, c.y - 2. * u, 3. * u, hx(0xc8a050));
                if cdf > 0. {
                    // คูลดาวน์: ทับสีเทาตามเวลาที่เหลือ
                    draw_circle(c.x, c.y, r * cdf, Color::new(0.1, 0.1, 0.1, 0.55));
                }
                tc("แตร", c.x, c.y + r * 0.9, 11. * u, if cdf > 0. { hx(0x999999) } else { hx(0xffe8a0) });
            }
            5 if build => {
                // โหมดสร้าง: ปุ่มสลับกลายเป็น "เลิกสร้าง" สีแดง
                draw_circle(c.x, c.y, r, Color::new(0.8, 0.15, 0.15, 0.55));
                tc("เลิก", c.x, c.y - 2. * u, 14. * u, WHITE);
                tc("สร้าง", c.x, c.y + 13. * u, 11. * u, WHITE);
            }
            5 => tc("สลับ", c.x, c.y + 5. * u, 14. * u, WHITE),
            6 => match use_lab {
                Some(l) => {
                    // มีของให้ใช้ตรงหน้า: วงเรืองแสงเต้นเบาๆ + บอกคำกริยา
                    let pr = 1. + 0.08 * (get_time() as f32 * 6.).sin();
                    draw_circle_lines(c.x, c.y, r * pr + 4. * u, 4., Color::new(1., 0.9, 0.3, 0.9));
                    let n = l.chars().count().max(1) as f32;
                    tc(l, c.x, c.y + 6. * u, (r * 1.6 / n).clamp(11. * u, 19. * u), hx(0xffee99));
                }
                None => tc("ใช้", c.x, c.y + 6. * u, 18. * u, hx(0xaaaaaa)),
            },
            8 => {
                if dash_cd > 0. { draw_circle(c.x, c.y, r * (1. - dash_cd.min(1.)), Color::new(0.5, 0.5, 0.5, 0.3)); }
                tc("พุ่ง", c.x, c.y + 6. * u, 17. * u, if dash_cd > 0. { hx(0x999999) } else { hx(0x88e0ff) });
            }
            _ => tc("สร้าง", c.x, c.y + 5. * u, 14. * u, WHITE),
        }
    }
}

// ======================= main =======================
fn window_conf() -> Conf {
    // Android: เต็มจอ (ซ่อนแถบสถานะ/แถบนำทาง — ปัดขอบจอเพื่อเรียกกลับชั่วคราว)
    Conf { window_title: "Pixel Realm".to_owned(), window_width: 1280, window_height: 720, high_dpi: true, window_resizable: true, fullscreen: cfg!(target_os = "android"), ..Default::default() }
}

enum State { Title, Play, Menu }

/// ปุ่มลัดสำหรับทดสอบ (มีเฉพาะ debug build)
#[cfg(debug_assertions)]
fn debug_keys(g: &mut Game) {
    let shift = is_key_down(KeyCode::LeftShift);
    if shift && is_key_pressed(KeyCode::F1) { g.p.inv[WOOD] += 60; g.p.inv[STONE] += 20; g.build_sel = PIECES.iter().position(|p| p.obj == OBJ_HOUSE_S); }
    if shift && is_key_pressed(KeyCode::F2) { g.dbg_caravan(); }
    if shift && is_key_pressed(KeyCode::F10) { g.build_sel = PIECES.iter().position(|p| p.obj == OBJ_MOVE); }
    if shift && is_key_pressed(KeyCode::F4) { for (j, k) in [EK::Scorp, EK::Wolf, EK::Crab, EK::Imp].into_iter().enumerate() { let (x, y) = (g.p.x - 60. + j as f32 * 40., g.p.y + 50.); g.make_enemy(k, x, y, 1., 0, false); if let Some(e) = g.en.last_mut() { e.stun = 6.; } } }
    if shift && is_key_pressed(KeyCode::F5) { let r = g.w.realm.clone(); if !g.rs.inited { g.rs.init(&r); } for st in 0..3u8 { let (x, y) = (g.p.x - 70. + st as f32 * 70., g.p.y - 50.); g.spawn_champion(st as usize % r.ks.len(), x, y); if let Some(b) = g.bosses.last_mut() { b.style = st; b.cd = 99.; } } }
    if shift && is_key_pressed(KeyCode::F9) { for b in g.bosses.iter_mut() { b.hp = b.maxhp * 0.45; b.cd = 0.3; } }
    if shift && is_key_pressed(KeyCode::F6) { for n in g.npcs.iter_mut() { n.role = R_SOLDIER; n.mood = 80; } g.army_call(None); }
    if shift && is_key_pressed(KeyCode::F8) { for j in 0..40 { let q = j as f32 * 0.7; let (x, y) = (g.p.x + 90. + q.cos() * (20. + j as f32), g.p.y + q.sin() * (20. + j as f32)); if !collide(&g.w, x, y) { g.make_enemy(EK::Soldier, x, y, 1.5, 99, false); } } }
    if shift && is_key_pressed(KeyCode::F7) { let n = g.en.len(); for i in (0..n).rev() { g.damage_enemy(i, 999, false, 1., 0.); } }
    if shift && is_key_pressed(KeyCode::F3) { let d = day_of(g); let e = (d % 4) as u8; g.start_event(e, d); }
    if shift && is_key_pressed(KeyCode::F12) { g.saga.end_day = day_of(g); g.rs.ending = Some(0.); }
    if !shift && is_key_pressed(KeyCode::F1) {
        for i in [WOOD, STONE, GEM, GEL, CORE] { g.p.inv[i] += 50; }
        g.p.inv[COIN] += 300;
        g.p.inv[ARROW] += 30;
        g.p.inv[SEED_CORN] += 5; g.p.inv[SEED_CARROT] += 5; g.p.can = true; g.p.water = WATER_MAX;
        g.toast("[debug] ได้ของ");
    }
    if !shift && is_key_pressed(KeyCode::F12) {
        // วนสกิน: เกราะทุกชิ้นระดับ 0..4 แล้วสลับชุดชาวไร่/นักล่า
        g.p.outfits = 3;
        if g.p.armor[0] < 4 && g.p.outfit != 1 && g.p.outfit != 2 { let a = g.p.armor[0] + 1; g.p.armor = [a; 3]; }
        else if g.p.armor[0] >= 4 { g.p.armor = [0; 3]; g.p.outfit = 1; }
        else if g.p.outfit == 1 { g.p.outfit = 2; } else { g.p.outfit = 0; g.p.armor = [1; 3]; }
    }
    if !shift && is_key_pressed(KeyCode::F2) {
        let (pcx, pcy) = (tfloor(g.p.x).div_euclid(CELL), tfloor(g.p.y).div_euclid(CELL));
        'find: for r in 0..12i32 {
            for cy in -r..=r {
                for cx in -r..=r {
                    if cx.abs().max(cy.abs()) != r { continue; }
                    if let Some((mx, my)) = g.w.camp_in_cell(pcx + cx, pcy + cy) {
                        g.p.x = mx as f32 * 16. + 8.;
                        g.p.y = (my + 1) as f32 * 16. + 10.;
                        g.p.dir = 1;
                        g.toast("[debug] วาร์ปไปแคมป์");
                        break 'find;
                    }
                }
            }
        }
    }
    if is_key_pressed(KeyCode::Home) {
        // วาร์ปไปหาคนหลงทางที่ใกล้ที่สุด (ที่ยังไม่ได้ช่วย)
        let (pcx, pcy) = (tfloor(g.p.x).div_euclid(CELL), tfloor(g.p.y).div_euclid(CELL));
        'h: for r in 0..14i32 {
            for cy in -r..=r { for cx in -r..=r {
                if cx.abs().max(cy.abs()) != r || g.npc_seen.contains(&(pcx + cx, pcy + cy)) { continue; }
                if let Some((x, y, _, _)) = g.w.lost_in_cell(pcx + cx, pcy + cy) {
                    g.p.x = x + 10.; g.p.y = y;
                    g.toast("[debug] วาร์ปไปหาคนหลงทาง");
                    break 'h;
                }
            } }
        }
    }
    if is_key_pressed(KeyCode::End) {
        // สร้างหมู่บ้านทดสอบรอบตัวผู้เล่น: เตียง 5 + หีบอาหาร + ชาวบ้าน 4 คน (เพื่อดูภาพ/ระบบ)
        let (tx, ty) = (tfloor(g.p.x), tfloor(g.p.y));
        let mut n = 0;
        for k in 0..5 {
            let b = (tx + 3 + k * 2, ty - 3);
            if g.w.solid_at(b.0, b.1) || g.w.obj_at(b.0, b.1) != 0 { continue; }
            g.w.placed.insert(b, 33);
            if n < 4 {
                let id = g.npc_next; g.npc_next += 1;
                g.npcs.push(npc::Npc { id, name: (id % 12) as u8, look: (id % npc::LOOKS as u32) as u8, x: b.0 as f32 * 16. + 8., y: b.1 as f32 * 16. + 18., state: npc::ST_HOME, hearts: 3, mood: 70, married: false, child: false, role: 0, bed: b, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id % 12) as u8) });
                n += 1;
            }
        }
        let mut c = [0; NI]; c[CORN] = 20; c[CARROT] = 20;
        g.chests.insert((tx + 4, ty - 1), c);
        g.p.inv[COIN] += 200; g.p.inv[GEM] += 10;
        for it in [BERRY, CORN, CARROT, FISH, MEAT, HERB] { g.p.inv[it] += 3; }
        g.assign_roles_pub();
        if g.npcs.len() > 1 { g.npcs[1].soc.anger = 60; g.npcs[1].soc.grudge = 35; } // ให้เห็นไอคอนโกรธ
        g.toast(format!("[debug] หมู่บ้านทดสอบ: {} คน", n));
    }
    if !shift && is_key_pressed(KeyCode::F3) { let (x, y) = (g.p.x + 90., g.p.y); g.spawn_boss(0, x, y); }
    if !shift && is_key_pressed(KeyCode::F4) { let (x, y) = (g.p.x + 90., g.p.y); g.spawn_boss(1, x, y); }
    if is_key_pressed(KeyCode::F11) { let (x, y) = (g.p.x + 90., g.p.y); g.spawn_boss_hp(FINAL_KIND, x, y, FINAL_HP); }
    if !shift && is_key_pressed(KeyCode::F5) { g.time = DAYLEN * 0.7; }
    if !shift && is_key_pressed(KeyCode::F7) {
        g.p.inv[SHARD] += 30; g.p.sp += 6; g.p.elems = 7; g.p.rod = true; if g.p.staff == 0 { g.p.staff = 1; } for i in [MEAT, SHROOM, HERB] { g.p.inv[i] += 10; } g.p.inv[COIN] += 500;
        for i in [WOOD, STONE, GEM, GEL, CORE] { g.p.inv[i] += 50; }
        g.toast("[debug] เศษดาว+แต้ม");
    }
    if is_key_pressed(KeyCode::F8) {
        let (tx, ty) = (tfloor(g.p.x), tfloor(g.p.y));
        'f8: for r in 0..40i32 {
            for cy in -r..=r { for cx in -r..=r {
                if cx.abs().max(cy.abs()) != r { continue; }
                if let Some(e) = g.w.dun_in_cell(tx.div_euclid(DCELL) + cx, ty.div_euclid(DCELL) + cy) {
                    g.p.x = e.0 as f32 * 16. + 8.; g.p.y = (e.1 + 2) as f32 * 16.;
                    g.toast("[debug] วาร์ปไปทางเข้าดันเจี้ยน");
                    break 'f8;
                }
            } }
        }
    }
    if !shift && is_key_pressed(KeyCode::F10) {
        let (tx, ty) = (tfloor(g.p.x), tfloor(g.p.y));
        'f10: for r in 0..30i32 {
            for cy in -r..=r { for cx in -r..=r {
                if cx.abs().max(cy.abs()) != r { continue; }
                if let Some(e) = g.w.wp_in_cell(tx.div_euclid(WPC) + cx, ty.div_euclid(WPC) + cy) {
                    g.p.x = e.0 as f32 * 16. + 8.; g.p.y = (e.1 + 3) as f32 * 16.;
                    g.toast("[debug] วาร์ปไปเสาวาร์ปใกล้สุด");
                    break 'f10;
                }
            } }
        }
    }
    if is_key_pressed(KeyCode::F9) { g.rain_on = !g.rain_on; g.rain_t = 90.; }
    if !shift && is_key_pressed(KeyCode::F6) { g.p.maxhp = 400; g.p.hp = 400; g.toast("[debug] เลือด 400"); }
}

#[macroquad::main(window_conf)]
async fn main() {
    macroquad::rand::srand((miniquad::date::now() * 1000.) as u64);
    let sp_ov = assets::load_sprite_overrides().await;
    let sp = build_sprites(&sp_ov);
    let font = load_ttf_font_from_bytes(include_bytes!("../assets/PixelRealmThai-Bold.ttf")).expect("font");
    let mut mini_img = Image::gen_image_color(120, 120, BLACK);
    let mut mini_job: Option<((i32, i32), i32, i32)> = None;
    let mini_tex = Texture2D::from_image(&mini_img);
    mini_tex.set_filter(FilterMode::Nearest);
    let mut sfx_slot: Option<Sfx> = Some(load_sfx().await);
    let mut bgm = BgmMgr::new();
    let mut amb = AmbMgr::new();
    let mut game: Option<Game> = None;
    prefs::load();
    let mut save_sum = prefs::save_summary(DAYLEN);
    let mut bak_msg = String::new();
    let mut tview = 0u8;
    let mut new_body = Body::default();
    let mut rep_t = 0f32;
    let mut bg = Game::new_world_placeholder(Sfx::default());
    bg.w = World::new(gen_range(1, i32::MAX));
    bg.p.invul = 0.07; // ซ่อนผู้เล่นบนหน้าเมนู
    let mut state = State::Title;
    let mut tu = TouchUI { active: std::env::var("PIXEL_TOUCH").is_ok(), joy_id: None, joy_c: Vec2::ZERO, joy_v: Vec2::ZERO, atk_id: None, aim: Vec2::ZERO, flash: [0.; 12], place_id: None, atk_start: Vec2::ZERO, build_id: None };
    let mut ui = UiState::new();
    let mut t_now = 0f32;
    let mut mouse_armed = false;
    let mut map_ctr = (0i32, 0i32);
    let mut map_sc = 2i32;

    loop {
        let frame_t0 = get_time();
        prof::begin();
        let dt = get_frame_time().min(0.05);
        t_now += dt;
        clear_background(BLACK);
        let u = ui_scale();
        let (sw, sh) = (screen_width(), screen_height());
        // ระหว่างเล่นใช้ touches() เองทั้งหมด ส่วนเมนู/ไตเติลให้นิ้วจำลองเป็นเมาส์
        simulate_mouse_with_touch(!matches!(state, State::Play));

        match state {
            State::Title => {
                // พื้นหลัง: โลกสุ่มที่เลื่อนช้าๆ
                bg.p.x = t_now * 20.;
                bg.p.y = 30.;
                let v = make_view(bg.p.x, bg.p.y);
                render_world(&bg, &sp, &v, t_now, &font);
                draw_rectangle(0., 0., sw, sh, Color::new(0., 0., 0., 0.5));
                let u = u.min(sh / 470.).max(0.8);
                if touches().iter().any(|_| true) { tu.active = true; }
                let mut start: Option<bool> = None;
                let bw = (300. * u).min(sw * 0.9);
                let bh = 48. * u;
                let gap = 12. * u;
                let bx = sw / 2. - bw / 2.;
                let panel = |x: f32, y: f32, w: f32, h: f32| {
                    draw_rectangle(x, y, w, h, Color::new(0.11, 0.12, 0.17, 0.95));
                    draw_rectangle_lines(x, y, w, h, 3., hx(0xffcc66));
                };
                match tview {
                    0 => {
                        // โลโก้ (เงาซ้อน + เด้งเบาๆ)
                        let ly = sh * 0.17 + (t_now * 1.6).sin() * 3. * u;
                        text_c(&font, "PIXEL REALM", sw / 2. + 3. * u, ly + 3. * u, 58. * u, Color::new(0., 0., 0., 0.7));
                        text_c(&font, "PIXEL REALM", sw / 2., ly, 58. * u, hx(0xffcc66));
                        text_c(&font, "สำรวจ · สร้างฐาน · ล่าบอส · ตั้งรกราก", sw / 2., ly + 30. * u, 15. * u, hx(0xdddddd));
                        let n = 6.;
                        let mut y = (sh * 0.30).max(ly + 50. * u).min(sh - n * (bh + gap) - 26. * u);
                        // ช่องเซฟ 3 ช่อง (แยกโลกกันเล่นหลายคนในเครื่องเดียว)
                        let sbw = (bw - 2. * gap) / 3.;
                        for sl in 0..SLOTS {
                            let cur = prefs::slot() == sl;
                            let has = std::fs::metadata(slot_path(sl)).is_ok();
                            let r = Rect::new(bx + sl as f32 * (sbw + gap), y, sbw, bh * 0.8);
                            if cur { draw_rectangle(r.x - 3., r.y - 3., r.w + 6., r.h + 6., hx(0xffffff)); }
                            if button(&font, r, &format!("ช่อง {}{}", sl + 1, if has { "" } else { " (ว่าง)" }), true, u) && !cur {
                                prefs::set_slot(sl);
                                save_sum = prefs::save_summary(DAYLEN);
                            }
                        }
                        y += bh * 0.8 + gap;
                        let sum = save_sum;
                        let lab = match sum { Some((d, l)) => format!("เล่นต่อ  ·  วันที่ {} · Lv {}", d, l), None => "เล่นต่อ".to_string() };
                        if button(&font, Rect::new(bx, y, bw, bh), &lab, sum.is_some(), u) || (sum.is_some() && is_key_pressed(KeyCode::Enter)) { start = Some(false); }
                        y += bh + gap;
                        if button(&font, Rect::new(bx, y, bw, bh), "เกมใหม่", true, u) || (sum.is_none() && is_key_pressed(KeyCode::Enter)) {
                            if sum.is_some() { tview = 3; } else { tview = 4; new_body = Body::default(); }
                        }
                        y += bh + gap;
                        if button(&font, Rect::new(bx, y, bw, bh), "ตั้งค่า", true, u) { tview = 1; }
                        y += bh + gap;
                        let hw = (bw - gap) / 2.;
                        if button(&font, Rect::new(bx, y, hw, bh), "วิธีเล่น", true, u) { tview = 2; }
                        if button(&font, Rect::new(bx + hw + gap, y, hw, bh), "สำรองเซฟ", true, u) { tview = 5; bak_msg.clear(); }
                        y += bh + gap;
                        if button(&font, Rect::new(bx, y, bw, bh), "ออกจากเกม", true, u) { std::process::exit(0); }
                        text_c(&font, "v12 · เกมวาดด้วยโค้ดทั้งหมด", sw / 2., sh - 10. * u, 11. * u, hx(0x999999));
                    }
                    5 => {
                        let (w, h) = ((460. * u).min(sw * 0.97), (360. * u).min(sh * 0.97));
                        let (x, y) = (sw / 2. - w / 2., sh / 2. - h / 2.);
                        panel(x, y, w, h);
                        let sl = prefs::slot();
                        let sp_ = save_path();
                        text_c(&font, &format!("สำรองเซฟ — ช่อง {}", sl + 1), sw / 2., y + 34. * u, 22. * u, hx(0xffcc66));
                        text_c(&font, &format!("เซฟสำรองอัตโนมัติ {} ชุด (สร้างทุกครั้งที่เริ่มเล่น)", backup_count(&sp_)), sw / 2., y + 58. * u, 12. * u, hx(0xaaaaaa));
                        let bw2 = w - 60. * u;
                        let mut ry = y + 74. * u;
                        if button(&font, Rect::new(x + 30. * u, ry, bw2, 40. * u), "กู้คืนเซฟสำรองล่าสุด", backup_count(&sp_) > 0, u) {
                            bak_msg = match restore_backup(&sp_) { Ok(()) => "กู้คืนแล้ว".to_string(), Err(e) => e };
                            save_sum = prefs::save_summary(DAYLEN);
                        }
                        ry += 48. * u;
                        if button(&font, Rect::new(x + 30. * u, ry, bw2, 40. * u), "ส่งออกเซฟเป็นไฟล์", save_sum.is_some(), u) {
                            bak_msg = match export_save(&sp_, sl, &stamp_now()) { Ok(p) => format!("บันทึกแล้ว: {}", p), Err(e) => e };
                        }
                        ry += 48. * u;
                        if button(&font, Rect::new(x + 30. * u, ry, bw2, 40. * u), "นำเข้าไฟล์เซฟล่าสุด (ทับช่องนี้)", true, u) {
                            bak_msg = match import_latest(&sp_) { Ok(f) => format!("นำเข้าแล้ว: {}", f), Err(e) => e };
                            save_sum = prefs::save_summary(DAYLEN);
                        }
                        ry += 52. * u;
                        let note = if bak_msg.is_empty() { format!("ไฟล์ส่งออกอยู่ที่ {}", export_dir()) } else { bak_msg.clone() };
                        for (k, l) in menu::wrap_th(&font, &note, 12. * u, w - 40. * u).iter().take(3).enumerate() { text_c(&font, l, sw / 2., ry + k as f32 * 15. * u, 12. * u, hx(0x9ad0ff)); }
                        if button(&font, Rect::new(sw / 2. - 70. * u, y + h - 50. * u, 140. * u, 38. * u), "กลับ", true, u) || is_key_pressed(KeyCode::Escape) { tview = 0; }
                    }
                    1 => {
                        let (w, h) = ((420. * u).min(sw * 0.97), (350. * u).min(sh * 0.97));
                        let (x, y) = (sw / 2. - w / 2., sh / 2. - h / 2.);
                        panel(x, y, w, h);
                        text_c(&font, "ตั้งค่า", sw / 2., y + 34. * u, 24. * u, hx(0xffcc66));
                        let rh = 40. * u;
                        let mut ry = y + 56. * u;
                        let rows: [(&str, String); 6] = [
                            ("เฟรมเรต", prefs::fps_name().to_string()),
                            ("เอฟเฟกต์ฝน/หิมะ", if prefs::weather_fx() { "เปิด" } else { "ปิด" }.to_string()),
                            ("แสงไดนามิก", prefs::light_name().to_string()),
                            ("ขนาดปุ่มทัช", prefs::touch_name().to_string()),
                            ("กล้องสั่น", if prefs::shake() { "เปิด" } else { "ปิด" }.to_string()),
                            ("แสดง FPS", if prefs::show_fps() { "เปิด" } else { "ปิด" }.to_string()),
                        ];
                        for (i, (n, val)) in rows.iter().enumerate() {
                            text(&font, n, x + 18. * u, ry + 26. * u, 15. * u, hx(0xeeeeee));
                            if button(&font, Rect::new(x + w - 150. * u, ry + 4. * u, 132. * u, 32. * u), val, true, u) {
                                match i { 0 => prefs::cycle_fps(), 1 => prefs::toggle(&prefs::WEATHER_FX), 2 => prefs::cycle_light(), 3 => prefs::cycle_touch(), 4 => prefs::toggle(&prefs::SHAKE), _ => prefs::toggle(&prefs::SHOW_FPS) }
                            }
                            ry += rh;
                        }
                        if button(&font, Rect::new(sw / 2. - 70. * u, y + h - 52. * u, 140. * u, 38. * u), "กลับ", true, u) || is_key_pressed(KeyCode::Escape) { tview = 0; }
                    }
                    2 => {
                        let (w, h) = ((640. * u).min(sw * 0.97), (380. * u).min(sh * 0.97));
                        let (x, y) = (sw / 2. - w / 2., sh / 2. - h / 2.);
                        panel(x, y, w, h);
                        text_c(&font, "วิธีเล่น", sw / 2., y + 34. * u, 24. * u, hx(0xffcc66));
                        let lines: Vec<&str> = if tu.active {
                            vec![
                                "ซ้าย: แตะค้างตรงไหนก็ได้เพื่อเดิน (จอยลอย)",
                                "ขวา: ปุ่มแดงใหญ่ = โจมตี · ลากนิ้วจากปุ่มเพื่อเล็งธนู/คทา",
                                "พุ่ง = หลบ · ใช้ = ประตู/กล่อง/พ่อค้า/เตียง · สลับ = อาวุธ",
                                "กิน = ใช้ยา/อาหาร · ไฟ = วางกองไฟ · เมนู (ขวาบน) = คราฟต์/สร้าง/กระเป๋า/ตั้งค่า",
                            ]
                        } else {
                            vec![
                                "WASD/ลูกศร = เดิน · Space/J/เมาส์ซ้าย = โจมตี · Shift = พุ่ง",
                                "E = กิน · F = กองไฟ · Q = ใช้ · Tab = สลับอาวุธ · M = แผนที่",
                                "C = คราฟต์ · B = สร้างฐาน (R = วาง) · I = กระเป๋า · P = สเตตัส · O = ตั้งค่า",
                            ]
                        };
                        let mut ly = y + 70. * u;
                        for l in lines.iter().chain(["ตีต้นไม้ได้ไม้ ตีหินได้หิน หินแร่สีฟ้าให้อัญมณี · กลางคืนมีโครงกระดูกและค้างคาว", "บอสซ่อนอยู่ในป่าทึบและเขตหินผา · แคมป์พ่อค้ามีเควสต์และจ้างผู้ติดตาม", "ปลูกผัก (จอบ+เมล็ด+กระป๋องรดน้ำ) · นอนเตียงตอนกลางคืน · ถ้ำและดันเจี้ยนมีแร่และหีบหายาก"].iter()) {
                            text_c(&font, l, sw / 2., ly, 13. * u, hx(0xcccccc));
                            ly += 27. * u;
                        }
                        if button(&font, Rect::new(sw / 2. - 70. * u, y + h - 52. * u, 140. * u, 38. * u), "กลับ", true, u) || is_key_pressed(KeyCode::Escape) { tview = 0; }
                    }
                    3 => {
                        let (w, h) = ((460. * u).min(sw * 0.97), (190. * u).min(sh * 0.97));
                        let (x, y) = (sw / 2. - w / 2., sh / 2. - h / 2.);
                        panel(x, y, w, h);
                        text_c(&font, "เริ่มเกมใหม่?", sw / 2., y + 42. * u, 24. * u, hx(0xffcc66));
                        text_c(&font, "เซฟเดิมจะถูกแทนที่", sw / 2., y + 76. * u, 15. * u, hx(0xdddddd));
                        if button(&font, Rect::new(sw / 2. - 160. * u, y + h - 62. * u, 150. * u, 42. * u), "เริ่มใหม่", true, u) { tview = 4; new_body = Body::default(); }
                        if button(&font, Rect::new(sw / 2. + 10. * u, y + h - 62. * u, 150. * u, 42. * u), "ยกเลิก", true, u) || is_key_pressed(KeyCode::Escape) { tview = 0; }
                    }
                    _ => {
                        // สร้างตัวละคร: สีผิว/ผม · ทรงผม · สีเสื้อ · ส่วนสูง · น้ำหนัก
                        let (w, h) = ((660. * u).min(sw * 0.97), (430. * u).min(sh * 0.97));
                        let (x, y) = (sw / 2. - w / 2., sh / 2. - h / 2.);
                        panel(x, y, w, h);
                        text_c(&font, "สร้างตัวละคร", sw / 2., y + 34. * u, 24. * u, hx(0xffcc66));
                        // พรีวิว: หมุนดูหน้า/ข้าง/หลัง
                        let (px, py, pw, ph) = (x + 16. * u, y + 54. * u, 190. * u, 214. * u);
                        draw_rectangle(px, py, pw, ph, hx(0x2a2d3f));
                        draw_rectangle_lines(px, py, pw, ph, 2., hx(0x555a75));
                        let pdir = ((t_now / 1.6) as usize) % 3;
                        let sc = 8. * u;
                        draw_figure(&sp, &new_body, [0, 0, 0], px + pw / 2. - 6. * sc, py + ph - 20. * u - 15. * sc, sc, pdir, 0, false, WHITE);
                        text_c(&font, &format!("สูง {} ซม. · หนัก {} กก.", new_body.cm, new_body.kg), px + pw / 2., py + ph + 20. * u, 14. * u, hx(0xdddddd));
                        text_c(&font, &format!("BMI {:.1} · {}", new_body.bmi(), new_body.build_word()), px + pw / 2., py + ph + 38. * u, 12. * u, hx(0xaabbcc));
                        // แถวปรับค่า
                        let rx = x + 226. * u;
                        let rw = w - 226. * u - 16. * u;
                        let rows: [(&str, String, Option<u32>); 9] = [
                            ("เชื้อสาย", ETH[new_body.eth as usize].0.to_string(), None),
                            ("เพศ", if new_body.fem { "หญิง" } else { "ชาย" }.to_string(), None),
                            ("สีผิว", SKIN_NAME[new_body.skin as usize].to_string(), Some(SKIN_TONES[new_body.skin as usize])),
                            ("สีผม", HAIR_NAME[new_body.hair as usize].to_string(), Some(HAIR_COLS[new_body.hair as usize])),
                            ("ทรงผม", STYLE_NAME[new_body.style as usize].to_string(), None),
                            ("สีเสื้อ", CLOTH_NAME[new_body.cloth as usize].to_string(), Some(CLOTH_COLS[new_body.cloth as usize])),
                            ("ส่วนสูง", format!("{} ซม.", new_body.cm), None),
                            ("น้ำหนัก", format!("{} กก.", new_body.kg), None),
                            ("กล้ามเนื้อ", format!("{} ({})", new_body.mus, mus_word(new_body.mus)), None),
                        ];
                        let mut ry = y + 56. * u;
                        for (i, (label, val, sw_col)) in rows.iter().enumerate() {
                            text(&font, label, rx, ry + 24. * u, 15. * u, hx(0xeeeeee));
                            let (lb, rb) = (Rect::new(rx + 76. * u, ry + 2. * u, 34. * u, 32. * u), Rect::new(rx + rw - 34. * u, ry + 2. * u, 34. * u, 32. * u));
                            let mut dir = 0i32;
                            let hold_a = is_mouse_button_down(MouseButton::Left);
                            let (mx, my) = mouse_position();
                            let (hl, hr) = (lb.contains(vec2(mx, my)), rb.contains(vec2(mx, my)));
                            let cl = button(&font, lb, "<", true, u);
                            let cr = button(&font, rb, ">", true, u);
                            if cl { dir = -1; }
                            if cr { dir = 1; }
                            // กดค้าง = เลื่อนต่อเนื่อง (ส่วนสูง/น้ำหนักช่วงกว้าง)
                            if (hl || hr) && hold_a && !cl && !cr {
                                rep_t += get_frame_time();
                                if rep_t > 0.4 { rep_t = 0.36; dir = if hl { -1 } else { 1 }; }
                            } else if !hold_a { rep_t = 0.; }
                            if dir != 0 {
                                let b = &mut new_body;
                                let wrap = |v: u8, n: usize| ((v as i32 + dir + n as i32) % n as i32) as u8;
                                // เปลี่ยนเชื้อสาย/เพศ = ตั้งสีผิว ส่วนสูง น้ำหนัก เป็นค่าเฉลี่ยของกลุ่ม (ปรับต่อเองได้)
                                let typical = |b: &mut Body| {
                                    let e = ETH[b.eth as usize];
                                    b.skin = (e.3 .0 + e.3 .1 + 1) / 2;
                                    b.cm = (if b.fem { e.2 } else { e.1 }).round() as u16;
                                    let m = b.cm as f64 / 100.;
                                    b.kg = ((22.6 + e.4 - if b.fem { 0.6 } else { 0. }) * m * m).round() as u16;
                                };
                                match i {
                                    0 => { b.eth = wrap(b.eth, ETH_N); typical(b); }
                                    1 => { b.fem = !b.fem; typical(b); }
                                    2 => b.skin = wrap(b.skin, SKIN_N),
                                    3 => b.hair = wrap(b.hair, HAIR_N),
                                    4 => b.style = wrap(b.style, STYLE_N),
                                    5 => b.cloth = wrap(b.cloth, CLOTH_N),
                                    6 => b.cm = (b.cm as i32 + dir).clamp(CM_MIN, CM_MAX) as u16,
                                    7 => b.kg = (b.kg as i32 + dir).clamp(KG_MIN, KG_MAX) as u16,
                                    _ => b.mus = (b.mus as i32 + dir * 5).clamp(0, 100) as u8,
                                }
                            }
                            let cx = rx + 76. * u + 34. * u + (rw - 76. * u - 68. * u) / 2.;
                            if let Some(c) = sw_col {
                                draw_rectangle(cx - 62. * u, ry + 9. * u, 18. * u, 18. * u, hx(*c));
                                draw_rectangle_lines(cx - 62. * u, ry + 9. * u, 18. * u, 18. * u, 2., BLACK);
                                text(&font, val, cx - 38. * u, ry + 24. * u, 14. * u, WHITE);
                            } else {
                                text_c(&font, val, cx, ry + 24. * u, 14. * u, WHITE);
                            }
                            ry += 33. * u;
                        }
                        text_c(&font, "ตัวสูงก้าวยาว · ร่างใหญ่หิวเร็ว · กล้ามเยอะตีแรงขึ้นนิดหน่อยแต่หิวเร็วขึ้น · อ้วนเดินช้าลงนิดหน่อย", x + w / 2., y + h - 62. * u, 11. * u, hx(0x9aa0b8));
                        let bw3 = 128. * u;
                        if button(&font, Rect::new(x + 16. * u, y + h - 48. * u, bw3, 38. * u), "กลับ", true, u) || is_key_pressed(KeyCode::Escape) { tview = 0; }
                        if button(&font, Rect::new(x + w / 2. - bw3 / 2., y + h - 48. * u, bw3, 38. * u), "สุ่ม", true, u) { new_body = Body::random_player(); }
                        if button(&font, Rect::new(x + w - 16. * u - 160. * u, y + h - 48. * u, 160. * u, 38. * u), "เริ่มเกม", true, u) { start = Some(true); }
                    }
                }
                if let Some(fresh) = start {
                    let sfx = sfx_slot.take().unwrap_or_default();
                    // เซฟสำรองอัตโนมัติก่อนเริ่มเล่นทุกครั้ง (กู้คืนได้ถ้ากดเกมใหม่ทับโดยไม่ตั้งใจ)
                    rotate_backups(&save_path());
                    let mut g = if fresh { Game::new_world(sfx) } else {
                        match Game::load(sfx) { Ok(g) => g, Err(sfx) => Game::new_world(sfx) }
                    };
                    if fresh { g.p.body = new_body.clamp(); }
                    let day = (g.time / DAYLEN) as i32 + 1;
                    g.toast(format!("วันที่ {} — ออกสำรวจกันเลย!", day));
                    g.save();
                    game = Some(g);
                    state = State::Play;
                    tview = 0;
                    mouse_armed = false;
                }
            }
            State::Play | State::Menu => {
                let g = game.as_mut().unwrap();
                let mut inp = Input::default();
                let in_play = matches!(state, State::Play);
                let v0 = make_view(g.p.x, g.p.y);
                if in_play {
                    let k = |a: KeyCode, b: KeyCode| if is_key_down(a) || is_key_down(b) { 1. } else { 0. };
                    inp.mv = vec2(k(KeyCode::D, KeyCode::Right) - k(KeyCode::A, KeyCode::Left), k(KeyCode::S, KeyCode::Down) - k(KeyCode::W, KeyCode::Up));
                    inp.atk = is_key_down(KeyCode::Space) || is_key_down(KeyCode::J);
                    inp.eat = is_key_pressed(KeyCode::E);
                    inp.fire = is_key_pressed(KeyCode::F);
                    inp.map = is_key_pressed(KeyCode::M);
                    inp.act = is_key_pressed(KeyCode::Q);
                    inp.place = is_key_pressed(KeyCode::R);
                    inp.place_hold = is_key_down(KeyCode::R);
                    inp.rot = is_key_pressed(KeyCode::Y) && g.build_sel.is_some();
                    inp.form = is_key_pressed(KeyCode::V);
                    inp.horn = is_key_pressed(KeyCode::X);
                    inp.swap = is_key_pressed(KeyCode::Tab);
                    inp.ride = is_key_pressed(KeyCode::G);
                    inp.dash = is_key_pressed(KeyCode::LeftShift) || is_key_pressed(KeyCode::K);
                    inp.potion = is_key_pressed(KeyCode::H);
                    for (k, kc) in [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4, KeyCode::Key5, KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9].iter().enumerate() { if is_key_pressed(*kc) { inp.hb = Some(k); } }
                    inp.cancel = is_key_pressed(KeyCode::T) || (is_key_pressed(KeyCode::Escape) && g.build_sel.is_some());
                    inp.menu = if is_key_pressed(KeyCode::C) { Some(0) } else if is_key_pressed(KeyCode::B) { Some(1) } else if is_key_pressed(KeyCode::I) { Some(2) } else if is_key_pressed(KeyCode::P) { Some(6) } else if is_key_pressed(KeyCode::O) { Some(7) } else if is_key_pressed(KeyCode::L) && g.rs.inited { Some(11) } else if is_key_pressed(KeyCode::U) { Some(10) } else { None };
                    #[cfg(debug_assertions)]
                    debug_keys(g);
                    #[cfg(debug_assertions)]
                    if is_key_pressed(KeyCode::F11) { tu.active = !tu.active; }
                    // ปุ่มลัดเหนือหัวชาวบ้าน + แผนที่เล็ก (แตะเปิดแผนที่ใหญ่)
                    let qb = v19::quick_btns(g, &v0);
                    let mut block: Vec<Rect> = qb.map(|q| q.1.to_vec()).unwrap_or_default();
                    let adv = advisor::adv_rects(g);
                    if let Some((r, _, _)) = adv { block.push(r); }
                    if !g.big_map { block.push(minimap_rect(false)); }
                    handle_touch(&mut tu, &mut inp, g.build_sel.is_some(), g.rs.horse, g.rs.war.march, &block);
                    // เมาส์ซ้าย = โจมตี/ยิงไปทางเมาส์ (เริ่มทำงานหลังปล่อยปุ่มครั้งแรก กันคลิกจากเมนูหลุดมา)
                    let mp: Vec2 = mouse_position().into();
                    if !is_mouse_button_down(MouseButton::Left) { mouse_armed = true; }
                    let over_map = minimap_rect(g.big_map).contains(mp);
                    let mut tap_used = false;
                    let tap: Option<Vec2> = if tu.active { touches().iter().find(|t| t.phase == TouchPhase::Started).map(|t| t.position) } else if is_mouse_button_pressed(MouseButton::Left) { Some(mp) } else { None };
                    if let (Some(tp), Some((r, go, close))) = (tap, adv) {
                        if close.contains(tp) { g.adv_dismiss(); tap_used = true; }
                        else if go.map_or(false, |b| b.contains(tp)) { let a = g.rs.adv_show.take().map(|x| x.0.act); if let Some(a) = a { g.adv_do(a); } tap_used = true; }
                        else if r.contains(tp) { tap_used = true; }
                    }
                    if let (Some(tp), false, false) = (tap, g.big_map, tap_used) {
                        if tu.active && minimap_rect(false).contains(tp) { inp.map = true; tap_used = true; }
                        else if let Some((id, rs)) = qb {
                            if rs[0].contains(tp) { g.quick_talk(id); tap_used = true; }
                            else if rs[1].contains(tp) { g.quick_gift_toggle(id); tap_used = true; }
                            else if rs[2].contains(tp) { g.req = Some(Req::Npc(id)); tap_used = true; }
                        }
                    }
                    if let (Some(tp), true) = (tap, g.big_map) {
                        let mr = minimap_rect(true);
                        let uu = ui_scale();
                        let mut hit = None;
                        for (tx, ty, spn) in g.wp_list() {
                            let (mx, my, ok) = map_pt(tx, ty, map_ctr, map_sc, mr);
                            if ok && (tp.x - mx).hypot(tp.y - my) < 16. * uu { hit = Some((tx, ty, spn)); break; }
                        }
                        if let Some((tx, ty, spn)) = hit {
                            g.warp_to(tx, ty, spn);
                            g.big_map = false;
                            g.mini_t = 99.;
                            tap_used = true;
                        } else if g.map_tap(tp, mr, map_ctr, map_sc, 11. * uu) {
                            tap_used = true;
                        } else if tu.active && !mr.contains(tp) {
                            inp.map = true;
                            tap_used = true;
                        }
                    }
                    if !tu.active && !tap_used {
                        if is_mouse_button_pressed(MouseButton::Left) && over_map {
                            inp.map = true;
                        } else if g.build_sel.is_some() {
                            if !over_map { g.build_cur = Some(tile_of(&v0, mp)); g.cur_pos = (g.p.x, g.p.y); }
                            // คลิก = วาง · กดค้างลาก = วางต่อเนื่อง (กำแพง/พื้น)
                            if mouse_armed && !over_map && hotbar_hit(mp).is_none() {
                                if is_mouse_button_pressed(MouseButton::Left) { g.build_drag(tile_of(&v0, mp), true); }
                                else if is_mouse_button_down(MouseButton::Left) { g.build_drag(tile_of(&v0, mp), false); }
                            }
                            if is_mouse_button_released(MouseButton::Left) { g.rs.drag_last = None; }
                        } else if !tu.active && hotbar_hit(mp).is_some() {
                            if is_mouse_button_pressed(MouseButton::Left) { inp.hb = hotbar_hit(mp); }
                        } else if mouse_armed && !over_map && is_mouse_button_down(MouseButton::Left) {
                            inp.atk = true;
                            inp.aim = Some(vec2(mp.x / v0.s + v0.cx, mp.y / v0.s + v0.cy) - vec2(g.p.x, g.p.y - 6.));
                        }
                    }
                    if g.build_sel.is_some() {
                        if let Some(tp) = inp.build_tap { g.build_drag(tile_of(&v0, tp), inp.build_start); }
                        if inp.build_end { g.rs.drag_last = None; }
                    }
                    if inp.map { g.big_map = !g.big_map; g.mini_t = 99.; }
                    let sdt = if g.hitstop > 0. { g.hitstop -= dt; dt * 0.08 } else { dt };
                    if g.rs.ending.is_none() { prof::t("update", || g.update(sdt, &inp)); }
                    if let Some(rq) = g.req.take() {
                        ui.reset_context();
                        match rq {
                            Req::Chest(k) => { ui.chest = Some(k); ui.open(3); }
                            Req::Shop => { ui.shop = true; ui.greet = rnd(0, 3) as usize; ui.open(4); }
                            Req::Forge => { ui.forge = true; ui.open(5); }
                            Req::Cook => { ui.cook = true; ui.open(8); }
                            Req::Adv => { ui.adv = true; ui.open(9); }
                            Req::Npc(id) => { ui.npc = Some(id); ui.open(10); }
                            Req::City(ci, kind) => { ui.city = Some((ci, kind)); ui.open(12); }
                            Req::Tab(t) => { ui.open(t as usize); }
                        }
                        state = State::Menu;
                    } else if let Some(t) = inp.menu {
                        ui.reset_context();
                        ui.open(t);
                        state = State::Menu;
                    }
                    if matches!(state, State::Menu) {
                        tu.atk_id = None;
                        tu.joy_id = None;
                        tu.joy_v = Vec2::ZERO;
                        tu.aim = Vec2::ZERO;
                    }
                } else {
                    // ล้าง touch ที่ค้างระหว่างเปิดเมนู
                    let _ = touches();
                }
                // แผนที่เล็ก: วาดใหม่ทุก 1 วิ แต่กระจายทีละ 15 แถวต่อเฟรม (เดิมทำรวดเดียว 120 แถว = เฟรมสะดุด)
                g.mini_t += dt;
                if g.mini_t > 50. { mini_job = None; }
                if g.mini_t > 1. && mini_job.is_none() {
                    g.mini_t = 0.;
                    mini_job = Some(((tfloor(g.p.x), tfloor(g.p.y)), mm_sc(g), 0));
                }
                if let Some((c, sc, r)) = mini_job {
                    // ทำทีละแถวจนครบงบ ~3 ms ต่อเฟรม (พื้นที่ใหม่ที่ต้องสร้างแผนที่จะช้ากว่า ก็แค่ใช้หลายเฟรม)
                    let t0 = std::time::Instant::now();
                    let mut n = 0;
                    while r + n < 120 && (n < 2 || t0.elapsed().as_secs_f32() < 0.003) {
                        update_minimap_rows(g, &mut mini_img, &mini_tex, c, sc, (r + n)..(r + n + 1));
                        n += 1;
                    }
                    if r + n >= 120 {
                        prof::t("minimap_fin", || finish_minimap(g, &mut mini_img, &mini_tex, c, sc));
                        map_ctr = c;
                        map_sc = sc;
                        mini_job = None;
                    } else { mini_job = Some((c, sc, r + n)); }
                }
                let mut v = make_view(g.p.x, g.p.y);
                let so = g.shake_off();
                v.cx += so.0.round();
                v.cy += so.1.round();
                prof::t("render_world", || render_world(g, &sp, &v, t_now, &font));
                prof::lap("L:pre_world");
                draw_nav_world(g, &v, &font, t_now);
                draw_siege_hud(g, &font, ui_scale());
                war::draw_war_hud(g, &font, ui_scale());
                hud::set_touch(tu.active || cfg!(target_os = "android"));
                prof::t("hud", || draw_hud(g, &sp, &font, tu.active));
                if matches!(state, State::Play) { v19::draw_quick(g, &make_view(g.p.x, g.p.y), &font); advisor::draw_advice(g, &font); }

                // บอสอยู่ใกล้ = แถบเลือดบอสใช้พื้นที่ด้านบนแทนบทเรียน/เป้าหมาย
                if !g.bosses.iter().any(|b| (b.x - g.p.x).hypot(b.y - g.p.y) < 420.) {
                    draw_tut(g, &font, &v0, ui_scale());
                    draw_adv(g, &font, ui_scale());
                }
                draw_ending(g, &font);
                let mr = minimap_rect(g.big_map);
                draw_rectangle(mr.x - 2., mr.y - 2., mr.w + 4., mr.h + 4., BLACK);
                draw_texture_ex(&mini_tex, mr.x, mr.y, WHITE, DrawTextureParams { dest_size: Some(vec2(mr.w, mr.h)), ..Default::default() });
                // สัญลักษณ์ M = แคมป์พ่อค้าที่ค้นพบแล้ว
                for &(cx, cy) in g.camps_found.iter().filter(|_| g.w.dun.is_none()) {
                    let i = 60. + (cx - map_ctr.0) as f32 / map_sc as f32;
                    let j = 60. + (cy - map_ctr.1) as f32 / map_sc as f32;
                    if (3.0..117.0).contains(&i) && (3.0..117.0).contains(&j) {
                        let (mx, my) = (mr.x + i / 120. * mr.w, mr.y + j / 120. * mr.h);
                        let rr = if g.big_map { 11. * u } else { 7. * u };
                        draw_circle(mx, my, rr, hx(0x1d1f2b));
                        draw_circle_lines(mx, my, rr, 2., hx(0xffd84a));
                        text_c(&font, "M", mx, my + rr * 0.5, rr * 1.5, hx(0xffd84a));
                    }
                }
                if g.w.dun.is_none() && g.big_map {
                    for (&(tx, ty), &k) in g.marks_found.iter() {
                        let (mx, my, ok) = map_pt(tx, ty, map_ctr, map_sc, mr);
                        if !ok { continue; }
                        let (l, c) = mark_letter(k);
                        let rr = 10. * u;
                        draw_circle(mx, my, rr, hx(0x1d1f2b));
                        draw_circle_lines(mx, my, rr, 2., hx(c));
                        text_c(&font, l, mx, my + rr * 0.5, rr * 1.4, hx(c));
                    }
                }
                if g.w.dun.is_none() {
                    for (tx, ty, spn) in g.wp_list() {
                        let (mx, my, ok) = map_pt(tx, ty, map_ctr, map_sc, mr);
                        if !ok { continue; }
                        let rr = if g.big_map { 11. * u } else { 6. * u };
                        draw_circle(mx, my, rr, hx(0x10202b));
                        draw_circle_lines(mx, my, rr, 2., if spn { hx(0xffffff) } else { hx(0x6ae8ff) });
                        if spn { text_c(&font, "เริ่ม", mx, my + rr * 0.35, rr * 0.85, hx(0xffffff)); } else { text_c(&font, "W", mx, my + rr * 0.5, rr * 1.4, hx(0x6ae8ff)); }
                    }
                    draw_realm_map(g, &font, mr, map_ctr, map_sc, g.big_map, u);
                    draw_nav_map(g, &font, mr, map_ctr, map_sc, g.big_map, u);
                    prof::lap("L:map");
                }
                if g.big_map {
                    // คำอธิบาย: แถบดำโปร่งด้านล่างในกรอบแผนที่ (ไม่ทับแถบไอเทม)
                    draw_rectangle(mr.x, mr.y + mr.h - 40. * u, mr.w, 40. * u, Color::new(0., 0., 0., 0.62));
                    let lf = (mr.w / 50.).min(14. * u);
                    text_c(&font, "รูปบ้าน = บ้าน (แตะ = วาร์ปกลับ) · W = เสาวาร์ป · เริ่ม = จุดเริ่มเกม · M = แคมป์ · จุดแดง = คุณ", mr.x + mr.w / 2., mr.y + mr.h - 24. * u, lf, WHITE);
                    text_c(&font, "แตะที่ว่าง = ปักหมุด+นำทาง · แตะหมุด = นำทาง · แตะซ้ำ = ลบ · แตะนอกแผนที่ = ปิด", mr.x + mr.w / 2., mr.y + mr.h - 7. * u, lf, hx(0x90f4ff));
                }
                if in_play {
                    prof::lap("L:before_touch");
                    draw_touch(&tu, &sp, &font, g.build_sel.is_some(), if g.p.bare || (g.p.weapon == 0 && g.p.sword == 0) { 3 } else { g.p.weapon }, g.p.dash_cd, g.use_label().as_deref(), (g.rs.horse, g.rs.riding), if g.rs.war.march { Some((g.rs.war.form, g.rs.war.horn_cd / war::HORN_CD, g.horn_on())) } else { None });
                    if !tu.active && g.p.weapon > 0 {
                        let (mx, my) = mouse_position();
                        let c = Color::new(1., 1., 0.6, 0.9);
                        draw_circle_lines(mx, my, 7. * u, 2., c);
                        draw_line(mx - 11. * u, my, mx - 4. * u, my, 2., c);
                        draw_line(mx + 4. * u, my, mx + 11. * u, my, 2., c);
                        draw_line(mx, my - 11. * u, mx, my - 4., 2., c);
                        draw_line(mx, my + 4. * u, mx, my + 11. * u, 2., c);
                    }
                } else {
                    let mut close = draw_menu(g, &sp, &font, &mut ui) || is_key_pressed(KeyCode::Escape);
                    for (key, tab) in [(KeyCode::C, 0usize), (KeyCode::B, 1), (KeyCode::I, 2)] {
                        if is_key_pressed(key) {
                            if ui.tab == tab { close = true; } else { ui.tab = tab; ui.scroll = 0.; }
                        }
                    }
                    if close {
                        ui.reset_context();
                        state = State::Play;
                        mouse_armed = false;
                    }
                    if g.to_title {
                        g.to_title = false;
                        g.save();
                        let mut gg = game.take().unwrap();
                        sfx_slot = Some(std::mem::take(&mut gg.sfx));
                        save_sum = prefs::save_summary(DAYLEN);
                        ui.reset_context();
                        state = State::Title;
                        tview = 0;
                        continue;
                    }
                }
            }
        }
        let want = match (&state, &game) {
            (State::Title, _) => 5,
            (_, Some(g)) => g.bgm_scene(),
            _ => -1,
        };
        // ฉากจบ (รวมแผ่นดินสำเร็จ) — วาดทับทุกอย่าง เกมหยุดจนกด "เล่นต่อ"
        if let (State::Play, Some(g)) = (&state, game.as_mut()) {
            if let Some(t) = g.rs.ending {
                g.rs.ending = Some(t + get_frame_time().min(0.1));
                if saga::draw_saga_end(g, &font, t) { g.rs.ending = None; g.save(); }
            }
        }
        prof::lap("L:ui");
        bgm.set(want).await;
        prof::lap("L:bgm");
        match (&state, &game) {
            (State::Title, _) | (_, None) => amb.stop_all(),
            (_, Some(g)) => amb.update(g.amb, get_frame_time().min(0.1), 1.0).await,
        }
        prof::lap("L:amb");
        if prefs::show_fps() {
            text(&font, &format!("{} FPS", get_fps()), 8., sh - 8., 14. * ui_scale(), hx(0x66ff66));
        }
        if let Some(tg) = prefs::frame_target() {
            // เผื่อเวลา vsync ~4ms: ถ้าเครื่องช้ากว่าเป้า จะไม่หน่วงเพิ่ม
            let rem = tg - (get_time() - frame_t0) - 0.004;
            if rem > 0.001 { std::thread::sleep(std::time::Duration::from_secs_f64(rem)); }
        }
        prof::lap("L:tail");
        prof::end_frame(((get_time() - frame_t0) * 1000.) as f32);
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game(seed: i32) -> Game {
        Game::new_world_seed(seed, Sfx::default())
    }

    #[test]
    fn camps_are_deterministic_and_exist() {
        let a = game(1234);
        let b = game(1234);
        let mut found = 0;
        for cy in -6..6 {
            for cx in -6..6 {
                let (ca, cb) = (a.w.camp_in_cell(cx, cy), b.w.camp_in_cell(cx, cy));
                assert_eq!(ca, cb);
                if let Some((x, y)) = ca {
                    found += 1;
                    assert_eq!(a.w.obj_at(x, y), 15, "พ่อค้าต้องอยู่กลางแคมป์");
                    assert!(!SOLIDT[a.w.tile_at(x, y + 1) as usize]);
                }
            }
        }
        assert!(found >= 10, "พบแคมป์น้อยเกินไป: {}", found);
    }

    #[test]
    fn chest_move_and_save_roundtrip() {
        let mut g = game(99);
        g.p.inv[WOOD] = 30;
        g.p.inv[GEL] = 4;
        g.w.placed.insert((5, 5), 14);
        g.chests.insert((5, 5), [0; NI]);
        g.chest_move((5, 5), WOOD, 20, true);
        g.chest_move((5, 5), GEL, 3, true);
        g.chest_move((5, 5), GEL, 1, false);
        assert_eq!(g.p.inv[WOOD], 10);
        assert_eq!(g.chests[&(5, 5)][WOOD], 20);
        assert_eq!(g.chests[&(5, 5)][GEL], 2);
        assert_eq!(g.p.inv[GEL], 2);
        g.p.armor = [1, 2, 3];
        g.p.bow = true;
        g.p.staff = 2;
        g.p.weapon = 2;
        g.p.sword = 4;
        g.p.inv[BP_ARM] = 1;
        g.w.placed.insert((6, 5), 13);
        g.camps_found.insert((10, 20));
        let s = g.save_string();
        let h = Game::load_str(&s, Sfx::default()).ok().expect("load");
        assert_eq!(h.chests[&(5, 5)][WOOD], 20);
        assert_eq!(h.chests[&(5, 5)][GEL], 2);
        assert_eq!(h.p.inv, g.p.inv);
        assert_eq!(h.p.armor, [1, 2, 3]);
        assert_eq!((h.p.weapon, h.p.bow, h.p.staff, h.p.sword), (2, true, 2, 4));
        assert_eq!(h.w.placed.get(&(6, 5)), Some(&13));
        assert!(h.camps_found.contains(&(10, 20)));
        assert_eq!(h.w.seed, g.w.seed);
    }

    #[test]
    fn armor_reduces_and_speeds() {
        let mut g = game(7);
        assert_eq!(g.p.reduction(), 0.);
        g.p.armor = [3, 3, 3];
        assert!((g.p.reduction() - 0.54).abs() < 1e-4);
        assert!(g.p.speed_mul() > 1.2);
        g.p.hp = 100;
        g.p.maxhp = 100;
        let mut tot = 0;
        for _ in 0..200 {
            g.p.hp = 100;
            g.p.invul = 0.;
            g.hurt(10, 1., 0.);
            tot += 100 - g.p.hp;
        }
        let avg = tot as f32 / 200.;
        assert!(avg > 3.5 && avg < 5.5, "avg {}", avg);
    }

    #[test]
    fn crafting_gates() {
        let mut g = game(3);
        // ล็อกจนกว่าจะมีพิมพ์เขียวและของบอส
        let top = RECIPES.iter().position(|r| r.name == "เกราะอกโกเลม").unwrap();
        g.p.inv[CORE] = 5;
        g.p.inv[GEL] = 5;
        assert_eq!(recipe_state(&g.p, &RECIPES[top]), 2);
        g.craft(top);
        assert_eq!(g.p.armor[1], 0);
        g.p.inv[BP_ARM] = 1;
        assert_eq!(recipe_state(&g.p, &RECIPES[top]), 0);
        g.craft(top);
        assert_eq!(g.p.armor[1], 3);
        assert_eq!(g.p.inv[CORE], 2);
        assert_eq!(g.p.inv[BP_ARM], 1, "พิมพ์เขียวไม่ถูกใช้หมด");
        // ธนู + ลูกธนู
        g.p.inv[WOOD] = 30;
        let bow = RECIPES.iter().position(|r| r.name == "ธนู").unwrap();
        g.craft(bow);
        assert!(g.p.bow && g.p.weapon == 1);
        let ar = RECIPES.iter().position(|r| r.name == "ลูกธนู x10").unwrap();
        g.p.inv[STONE] = 5;
        g.craft(ar);
        assert_eq!(g.p.inv[ARROW], 10);
    }

    #[test]
    fn shop_buy() {
        let mut g = game(5);
        g.p.inv[COIN] = 100;
        let bp = OFFERS.iter().position(|o| o.item == BP_ARM).unwrap();
        g.buy(bp);
        assert_eq!(g.p.inv[BP_ARM], 1);
        assert_eq!(g.p.inv[COIN], 30);
        g.buy(bp); // ซื้อซ้ำไม่ได้
        assert_eq!(g.p.inv[BP_ARM], 1);
        assert_eq!(g.p.inv[COIN], 30);
    }

    #[test]
    fn build_door_and_chest() {
        let mut g = game(11);
        g.p.inv[WOOD] = 50;
        g.p.inv[STONE] = 20;
        // หาที่โล่ง
        let (mut tx, mut ty) = (0, 0);
        'f: for y in -40..40 { for x in -40..40 {
            if g.w.obj_at(x, y) == 0 && !SOLIDT[g.w.tile_at(x, y) as usize] && g.w.obj_at(x + 1, y) == 0 && !SOLIDT[g.w.tile_at(x + 1, y) as usize] && g.w.obj_at(x + 2, y) == 0 {
                tx = x; ty = y; break 'f;
            }
        } }
        g.p.x = tx as f32 * 16. + 8.;
        g.p.y = ty as f32 * 16. + 10.;
        g.p.dir = 2;
        g.p.flip = false;
        g.build_sel = PIECES.iter().position(|p| p.obj == 12); // ประตู
        g.place_piece();
        let (dx, dy) = g.build_target();
        assert_eq!((dx, dy), (tx + 1, ty));
        assert_eq!(g.w.obj_at(dx, dy), 12);
        assert!(g.w.solid_at(dx, dy));
        g.interact();
        assert_eq!(g.w.obj_at(dx, dy), 13);
        assert!(!g.w.solid_at(dx, dy));
        g.interact();
        assert_eq!(g.w.obj_at(dx, dy), 12);
        // กล่อง: วางไม่ทับของเดิม
        g.build_sel = PIECES.iter().position(|p| p.obj == 14);
        g.place_piece();
        assert_eq!(g.w.obj_at(dx, dy), 12, "ห้ามวางทับ");
        g.w.placed.remove(&(dx, dy));
        g.place_piece();
        assert_eq!(g.w.obj_at(dx, dy), 14);
        assert!(g.chests.contains_key(&(dx, dy)));
        g.interact();
        assert_eq!(g.req, Some(Req::Chest((dx, dy))));
        // ไม่ให้รื้อกล่องที่มีของ
        g.chest_move((dx, dy), WOOD, 1, true);
        let hit = g.hit_object(dx, dy, 99);
        assert!(hit && g.w.obj_at(dx, dy) == 14);
    }

    #[test]
    fn bosses_attack_and_die() {
        for kind in 0..2u8 {
            let mut g = game(21);
            g.p.maxhp = 100000;
            g.p.hp = 100000;
            let (x, y) = (g.p.x + 60., g.p.y);
            g.spawn_boss(kind, x, y);
            g.bosses[0].cd = 0.;
            let mut saw_attack = false;
            let mut saw_proj_or_aoe = false;
            for _ in 0..1800 {
                g.p.invul = 0.;
                g.upd_bosses(0.016);
                g.upd_proj(0.016);
                if let Some(b) = g.bosses.first() { if b.atk != 0 || b.dash > 0. { saw_attack = true; } }
                if !g.proj.is_empty() || g.p.hp < 100000 { saw_proj_or_aoe = true; }
                for p in g.proj.iter_mut() { p.life = p.life.min(0.01); }
            }
            assert!(saw_attack, "บอส {} ไม่โจมตี", kind);
            assert!(saw_proj_or_aoe);
            // ฆ่าบอส
            g.bosses[0].hp = 1.;
            g.hit_boss(0, 5);
            assert!(g.bosses[0].dead);
            if kind == 0 { assert!(g.p.inv[GEL] >= 3); } else { assert!(g.p.inv[CORE] >= 2); }
            assert!(g.boss_cd > 100.);
        }
    }

    #[test]
    fn arrows_and_staff_hit() {
        let mut g = game(31);
        g.p.bow = true;
        g.p.weapon = 1;
        g.p.inv[ARROW] = 2;
        g.p.x = 0.; g.p.y = 0.;
        // ศัตรูทางขวา
        g.en.push(Enemy { path: Path::default(), night: false, burn: 0., slow: 0., stun: 0., btick: 0., sk: 0., fl: false, k: EK::Skel, x: 60., y: 6., hp: 4., spd: 0., dmg: 1, xp: 1, coin: 2, vx: 0., vy: 0., kb: 0., flash: 0., wt: 0., wdx: 0., wdy: 0., anim: 0., pal: 0, dead: false });
        // กันกำแพง: ล้างวัตถุในแนวยิง
        for tx in -2..8 { for ty in -2..2 { g.w.removed.insert((tx, ty)); } }
        for _ in 0..5 { g.w.tmod.insert((0, 0), 3); }
        g.shoot(Some(vec2(1., 0.)));
        assert_eq!(g.p.inv[ARROW], 1);
        assert_eq!(g.proj.len(), 1);
        for _ in 0..120 { g.upd_proj(0.01); }
        assert!(g.en[0].hp < 4. || g.en[0].dead, "ลูกธนูต้องโดนศัตรู");
        g.p.staff = 2; g.p.weapon = 2; g.p.atk_cd = 0.;
        g.shoot(Some(vec2(0., 1.)));
        assert_eq!(g.proj.len(), 3, "คทาราชา ยิง 3 ทิศ");
        g.p.weapon = 1; g.p.inv[ARROW] = 0; g.proj.clear();
        g.shoot(None);
        assert!(g.proj.is_empty(), "ไม่มีลูกธนูต้องไม่ยิง");
    }

    fn find_dun(g: &Game) -> (i32, i32) {
        for cy in -12..12 { for cx in -12..12 { if let Some(e) = g.w.dun_in_cell(cx, cy) { if !g.w.is_cave_cell(cx, cy) { return e; } } } }
        panic!("ไม่พบทางเข้าดันเจี้ยน");
    }
    fn find_cave(g: &Game) -> (i32, i32) {
        for cy in -12..12 { for cx in -12..12 { if let Some(e) = g.w.dun_in_cell(cx, cy) { if g.w.is_cave_cell(cx, cy) { return e; } } } }
        panic!("ไม่พบปากถ้ำ");
    }

    #[test]
    fn dungeon_enter_reach_boss_exit() {
        let mut g = game(777);
        let e = find_dun(&g);
        assert_eq!(g.w.obj_at(e.0, e.1), 19);
        g.p.x = e.0 as f32 * 16.; g.p.y = e.1 as f32 * 16.;
        let before = (g.p.x, g.p.y);
        g.enter_dungeon(e);
        let d = g.w.dun.as_ref().expect("in dungeon");
        let (en, bs) = (d.entry, d.boss_at);
        // BFS: ทางเดินจากจุดเกิดถึงห้องบอสต้องเชื่อมถึงกัน (หนามเดินผ่านได้)
        let mut seen = std::collections::HashSet::new();
        let mut q = vec![en];
        seen.insert(en);
        while let Some((x, y)) = q.pop() {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let n = (x + dx, y + dy);
                if seen.contains(&n) { continue; }
                if SOLIDT[g.w.tile_at(n.0, n.1) as usize] { continue; }
                let o = g.w.obj_at(n.0, n.1);
                if o != 0 && o != 21 && o != 22 && o != 20 && OSOLID[o as usize] { continue; }
                seen.insert(n);
                q.push(n);
            }
        }
        assert!(seen.contains(&bs), "ห้องบอสต้องเดินถึงได้");
        assert!(g.w.camp_in_cell(0, 0).is_none());
        // เซฟในดันเจี้ยน = บันทึกตำแหน่งข้างนอก
        let s = g.save_string();
        let h = Game::load_str(&s, Sfx::default()).ok().expect("load");
        assert!(h.w.dun.is_none());
        assert!((h.p.x - before.0).abs() < 0.1 && (h.p.y - before.1).abs() < 0.1);
        g.exit_dungeon();
        assert!(g.w.dun.is_none());
        assert_eq!((g.p.x, g.p.y), before);
    }

    #[test]
    fn forge_stats_and_save() {
        let mut g = game(5);
        for i in [STONE, GEM, GEL, CORE, SHARD] { g.p.inv[i] = 99; }
        for _ in 0..5 { g.forge(0); }
        assert_eq!(g.p.wplus[0], 5);
        g.forge(0);
        assert_eq!(g.p.wplus[0], 5, "สูงสุด +5");
        g.p.armor = [1, 1, 1];
        g.forge(4);
        assert_eq!(g.p.aplus[1], 1);
        let r0 = { let mut h = game(5); h.p.armor = [1, 1, 1]; h.p.reduction() };
        assert!(g.p.reduction() > r0);
        g.p.sp = 3;
        let mhp = g.p.maxhp;
        g.spend_stat(0); g.spend_stat(3); g.spend_stat(3);
        assert_eq!((g.p.sp, g.p.st[0], g.p.st[3]), (0, 1, 2));
        assert_eq!(g.p.maxhp, mhp + 3);
        g.spend_stat(1);
        assert_eq!(g.p.st[1], 0, "ไม่มีแต้มต้องไม่เพิ่ม");
        g.sfx_on = false; g.auto_aim = false;
        g.comp = Some(Comp { kind: 1, x: 0., y: 0., atk_cd: 0., anim: 0., left: false });
        g.dun_found.insert((3, 4)); g.dun_clear.insert((3, 4)); g.dun_chests.insert((3, 4, 1));
        g.quest = Some(Quest { camp: (1, 2), kind: 5, goal: 4, prog: 2, coin: 50, shard: 2, bp: 1 });
        g.qn.insert((1, 2), 3);
        let s = g.save_string();
        let h = Game::load_str(&s, Sfx::default()).ok().expect("load");
        assert_eq!((h.p.wplus, h.p.aplus, h.p.st, h.p.sp), (g.p.wplus, g.p.aplus, g.p.st, g.p.sp));
        assert!(!h.sfx_on && !h.auto_aim && h.bgm_on);
        assert_eq!(h.comp.as_ref().map(|c| c.kind), Some(1));
        assert!(h.dun_found.contains(&(3, 4)) && h.dun_clear.contains(&(3, 4)) && h.dun_chests.contains(&(3, 4, 1)));
        let q = h.quest.as_ref().unwrap();
        assert_eq!((q.kind, q.goal, q.prog, q.bp), (5, 4, 2, 1));
        assert_eq!(h.qn[&(1, 2)], 3);
        SFX_ON.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    #[test]
    fn quest_and_loot_flow() {
        let mut g = game(11);
        let camp = (10, 10);
        g.accept_quest(camp);
        let q = g.quest.clone().unwrap();
        for _ in 0..q.goal {
            match q.kind { 5 => g.quest_item(GEM, 1), 6 => g.quest_item(STONE, 1), 7 => g.quest_item(WOOD, 1), 1 => g.quest_kill(EK::Skel, false), 2 => g.quest_kill(EK::Slime, false), 3 => g.quest_kill(EK::Bat, false), 4 => g.quest_kill(EK::Slime, true), _ => g.quest_kill(EK::Slime, false) }
        }
        assert_eq!(g.quest.as_ref().unwrap().prog, q.goal);
        let c0 = g.p.inv[COIN];
        g.claim_quest();
        assert!(g.quest.is_none());
        assert_eq!(g.p.inv[COIN], c0 + q.coin);
        assert_eq!(g.qn[&camp], 1);
        // ลูต: เหรียญที่ดรอปต้องถูกเก็บเมื่อเดินเข้าไป
        g.spawn_loot(g.p.x, g.p.y, COIN, 7);
        let c1 = g.p.inv[COIN];
        g.upd_drops(0.1);
        assert_eq!(g.p.inv[COIN], c1 + 7);
        // แดช: มีอมตะและคูลดาวน์
        g.try_dash(vec2(1., 0.));
        assert!(g.p.dash_t > 0. && g.p.invul > 0. && g.p.dash_cd > 0.);
    }

    #[test]
    fn waypoints_discover_and_warp() {
        let mut g = game(2024);
        let mut found = vec![];
        for cy in -4..4 { for cx in -4..4 { if let Some(e) = g.w.wp_in_cell(cx, cy) { found.push(e); } } }
        assert!(found.len() >= 8, "เสาวาร์ปน้อยเกินไป {}", found.len());
        let e = found[0];
        assert_eq!(g.w.obj_at(e.0, e.1), 25);
        assert!(!SOLIDT[g.w.tile_at(e.0, e.1) as usize]);
        g.p.x = e.0 as f32 * 16. + 8.; g.p.y = (e.1 + 3) as f32 * 16.;
        g.discover_wps();
        assert!(g.wp_found.contains(&e));
        g.p.x = 5000.; g.p.y = 5000.;
        g.warp_to(e.0, e.1, false);
        assert!((tfloor(g.p.x) - e.0).abs() <= 2 && (tfloor(g.p.y) - e.1).abs() <= 2);
        assert!(!g.w.solid_at(tfloor(g.p.x), tfloor(g.p.y)));
        g.warp_to(0, 0, true);
        assert_eq!((g.p.x, g.p.y), g.spawn);
        let s = g.save_string();
        let h = Game::load_str(&s, Sfx::default()).ok().expect("load");
        assert!(h.wp_found.contains(&e));
    }

    #[test]
    fn cooking_and_buffs() {
        let mut g = game(3);
        g.cook(0);
        assert_eq!(g.p.inv[DISH_STEW], 0, "ไม่มีวัตถุดิบต้องทำไม่ได้");
        for i in [MEAT, SHROOM, HERB, BERRY] { g.p.inv[i] = 6; }
        for i in 0..4 { g.cook(i); }
        assert_eq!((g.p.inv[DISH_STEW], g.p.inv[DISH_SPEED], g.p.inv[DISH_CRIT]), (1, 1, 1));
        let sp0 = g.p.speed_mul();
        let cr0 = g.p.crit_rate();
        g.use_dish(DISH_SPEED);
        g.use_dish(DISH_CRIT);
        assert!((g.p.speed_mul() - sp0 - 0.25).abs() < 1e-4);
        assert!((g.p.crit_rate() - cr0 - 0.2).abs() < 1e-4);
        g.p.hp = 3;
        g.use_dish(DISH_STEW);
        assert_eq!(g.p.hp, g.p.maxhp.min(17));
        let s = g.save_string();
        let h = Game::load_str(&s, Sfx::default()).ok().expect("load");
        assert!(h.p.buff_spd > 100. && h.p.buff_crit > 100.);
        g.upd_buffs(200.);
        assert_eq!(g.p.speed_mul(), sp0);
    }

    #[test]
    fn elements_status_effects() {
        let mut g = game(8);
        g.p.staff = 1; g.p.weapon = 2;
        assert_eq!(g.p.elems, 1);
        for i in [GEM, GEL, SHARD] { g.p.inv[i] = 20; }
        let ice = RECIPES.iter().position(|r| matches!(r.eff, Eff::Elem(1))).unwrap();
        let ltn = RECIPES.iter().position(|r| matches!(r.eff, Eff::Elem(2))).unwrap();
        g.craft(ice); g.craft(ltn);
        assert_eq!(g.p.elems, 7);
        // สลับธาตุด้วย Tab ตอนถือคทา: ฟ้า → (ออกจากคทา)
        g.p.elem = 0;
        g.swap_weapon(); assert_eq!((g.p.weapon, g.p.elem), (2, 1));
        g.swap_weapon(); assert_eq!((g.p.weapon, g.p.elem), (2, 2));
        g.swap_weapon(); assert_eq!(g.p.elem, 0);
        assert_ne!(g.p.weapon, 2);
        let mk = |g: &mut Game, k: u8| {
            g.en.clear();
            g.make_enemy(EK::Skel, g.p.x + 20., g.p.y, 1., 0, false);
            g.en[0].hp = 999.;
            g.proj.push(Proj { x: g.p.x + 20., y: g.p.y - 6., vx: 0., vy: 0., kind: k, dmg: 3, life: 1., mine: true, dead: false });
            g.upd_proj(0.01);
        };
        mk(&mut g, 7); assert!(g.en[0].burn > 0.);
        let hp = g.en[0].hp;
        g.upd_enemies(1.0);
        assert!(g.en[0].hp < hp, "เผาไหม้ต้องลดเลือด");
        mk(&mut g, 8); assert!(g.en[0].slow > 0.);
        let x0 = g.en[0].x; g.en[0].spd = 40.; g.en[0].kb = 0.;
        g.upd_enemies(0.1);
        let dslow = (g.en[0].x - x0).abs();
        g.en[0].slow = 0.; g.en[0].x = x0; g.en[0].burn = 0.;
        g.upd_enemies(0.1);
        assert!(dslow < (g.en[0].x - x0).abs(), "ต้องเดินช้ากว่าเมื่อโดนน้ำแข็ง");
        g.en[0].stun = 1.; g.en[0].kb = 0.; let x1 = g.en[0].x;
        g.upd_enemies(0.1);
        assert_eq!(g.en[0].x, x1, "สตันต้องหยุดนิ่ง");
    }

    #[test]
    fn fishing_flow_and_sell() {
        let mut g = game(31);
        g.p.rod = true;
        g.p.last_hit = 10.;
        let mut spot = None;
        'o: for ty in -300..300 { for tx in -300..300 {
            if !SOLIDT[g.w.tile_at(tx, ty) as usize] && g.w.obj_at(tx, ty) == 0 && g.w.tile_at(tx + 1, ty) < 2 && g.w.tile_at(tx + 2, ty) < 2 { spot = Some((tx, ty)); break 'o; }
        } }
        let (tx, ty) = spot.expect("ไม่พบริมน้ำ");
        g.p.x = tx as f32 * 16. + 8.; g.p.y = ty as f32 * 16. + 11.;
        g.p.dir = 2; g.p.flip = false;
        assert!(g.water_ahead().is_some());
        assert!(g.start_fish());
        g.fish_press(); // เร็วไป
        assert!(g.fish.is_none() && g.p.inv[FISH] == 0);
        let mut got = 0;
        for _ in 0..30 {
            assert!(g.start_fish());
            g.fish.as_mut().unwrap().bite_at = 0.;
            g.upd_fish(0.05);
            assert!(g.fish.as_ref().unwrap().bite_t > 0.);
            g.fish_press();
            got += 1;
        }
        assert!(g.p.inv[FISH] >= 15, "ควรได้ปลาเป็นส่วนใหญ่ {}", g.p.inv[FISH]);
        // ปลาหลุดถ้าไม่ดึง
        g.start_fish(); g.fish.as_mut().unwrap().bite_at = 0.;
        g.upd_fish(0.05); g.upd_fish(1.2);
        assert!(g.fish.is_none());
        let n = g.p.inv[FISH]; let c = g.p.inv[COIN];
        g.sell_fish();
        assert_eq!((g.p.inv[FISH], g.p.inv[COIN]), (0, c + n * FISH_PRICE));
        g.p.inv[FISH] = 2; g.cook(DISHES.iter().position(|d| d.item == DISH_FISH).unwrap());
        assert_eq!(g.p.inv[DISH_FISH], 1);
        let _ = got;
    }

    #[test]
    fn torch_safe_zone() {
        let mut g = game(4);
        let (tx, ty) = (tfloor(g.p.x) + 1, tfloor(g.p.y));
        g.w.placed.insert((tx, ty), 18);
        g.refresh_torches();
        let (cx, cy) = (tx as f32 * 16. + 8., ty as f32 * 16. + 8.);
        assert!(g.in_safe(cx + 40., cy));
        assert!(!g.in_safe(cx + 120., cy));
        g.time = DAYLEN * 0.7; g.darkv = 0.8;
        for _ in 0..2000 {
            g.try_spawn();
            assert!(g.en.iter().all(|e| !g.in_safe(e.x, e.y)), "มอนเกิดในเขตปลอดภัย");
            if g.en.len() > 8 { g.en.clear(); }
        }
        // มอนที่อยู่ในเขตถูกผลักออก
        g.en.clear();
        g.make_enemy(EK::Slime, cx + 20., cy, 1., 0, false);
        for _ in 0..120 { g.upd_enemies(0.05); }
        assert!((g.en[0].x - cx).hypot(g.en[0].y - cy) > SAFE_R - 8., "ต้องถูกผลักออกจากเขต");
    }
    /// หาจุดทุ่งหญ้าที่ว่าง 3 ช่องแนวนอน
    fn grass_spot(g: &Game) -> (i32, i32) {
        for ty in -200..200 { for tx in -200..200 {
            if (0..3).all(|i| g.w.tile_at(tx + i, ty) == 3 && g.w.obj_at(tx + i, ty) == 0) && g.w.tile_at(tx, ty + 1) == 3 { return (tx, ty); }
        } }
        panic!("ไม่พบทุ่งหญ้า");
    }

    #[test]
    fn farming_till_plant_water_harvest() {
        let mut g = game(11);
        let (tx, ty) = grass_spot(&g);
        g.p.x = tx as f32 * 16. + 8.; g.p.y = ty as f32 * 16. + 11.;
        g.p.dir = 2; g.p.flip = false; // หันขวา → ไทล์ (tx+1)
        let k = g.facing_tile();
        assert_eq!(k.1, ty);
        // ไถดิน ผ่านชิ้นส่วนสร้าง
        let till = PIECES.iter().position(|p| p.obj == OBJ_TILL).unwrap();
        g.build_sel = Some(till);
        g.place_piece();
        assert_eq!(g.w.tile_at(k.0, k.1), T_SOIL);
        // ไม่มีเมล็ด
        assert!(g.try_farm());
        assert!(g.w.obj_at(k.0, k.1) == 0);
        g.p.inv[SEED_CORN] = 1;
        assert!(g.try_farm());
        assert_eq!(g.p.inv[SEED_CORN], 0);
        assert_eq!(g.w.obj_at(k.0, k.1), 27);
        assert!(g.crops.contains_key(&k));
        // ไม่รดน้ำ = ไม่โต
        g.upd_crops(0.6); g.upd_crops(5.);
        assert_eq!(g.crops[&k].grown, 0.);
        // ไม่มีบัวรดน้ำ
        assert!(g.try_farm()); assert_eq!(g.crops[&k].wet, 0.);
        let can = RECIPES.iter().position(|r| matches!(r.eff, Eff::Can)).unwrap();
        g.p.inv[WOOD] = 10; g.p.inv[STONE] = 10;
        g.craft(can);
        assert!(g.p.can && g.p.water == WATER_MAX);
        assert!(g.try_farm());
        assert_eq!(g.p.water, WATER_MAX - 1);
        assert_eq!(g.w.tile_at(k.0, k.1), T_WET);
        // โตเมื่อเปียก
        for _ in 0..40 { g.upd_crops(1.); }
        assert!(g.crops[&k].grown >= 39.);
        assert_eq!(g.w.obj_at(k.0, k.1), 28, "ต้องอยู่ระยะกลาง");
        // ดินแห้งหลังหมดเวลา และโตเฉพาะช่วงที่เปียก (สะสมได้ไม่เกิน WET_T)
        for _ in 0..120 { g.upd_crops(1.); }
        assert_eq!(g.w.tile_at(k.0, k.1), T_SOIL);
        assert!((g.crops[&k].grown - WET_T).abs() < 2., "โตเฉพาะตอนเปียก {}", g.crops[&k].grown);
        assert_eq!(g.w.obj_at(k.0, k.1), 29, "สุกแล้ว");
        assert_eq!(g.farm_hint(false).unwrap(), "Q: เก็บเกี่ยว");
        assert!(g.try_farm());
        assert!(g.p.inv[CORN] >= 1);
        assert_eq!(g.w.obj_at(k.0, k.1), 0);
        assert!(g.crops.is_empty());
        // ไม่อนุญาตให้อาวุธทำลายพืช
        g.p.inv[SEED_CARROT] = 1; g.seed_sel = 1;
        assert!(g.try_farm());
        assert_eq!(g.w.obj_at(k.0, k.1), 30);
        assert!(g.hit_object(k.0, k.1, 99));
        assert_eq!(g.w.obj_at(k.0, k.1), 30);
        // ขาย/กิน
        g.p.inv[CORN] = 2; g.p.inv[CARROT] = 1;
        let c0 = g.p.inv[COIN];
        g.p.hp = 1; g.eat_crop(CORN);
        assert_eq!(g.p.hp, 1 + CROP_HEAL[0]);
        g.sell_crops();
        assert_eq!(g.p.inv[COIN], c0 + CROP_PRICE[0] + CROP_PRICE[1]);
        // เติมน้ำ + ไถซ้ำคืนหญ้า
        g.p.water = 0;
        g.till_at(k);
        assert_eq!(g.w.tile_at(k.0, k.1), 3);
        assert!(g.crops.is_empty());
    }

    #[test]
    fn farm_save_load_roundtrip() {
        let mut g = game(12);
        g.crops.insert((3, 4), Crop { kind: 1, grown: 12.5, wet: 30. });
        g.w.placed.insert((3, 4), 30);
        g.w.tmod.insert((3, 4), T_WET);
        g.p.can = true; g.p.water = 5; g.seed_sel = 1; g.bed = Some((7, 8));
        g.p.sword = 5; g.p.staff = 3; g.p.armor = [4, 4, 4];
        let h = Game::load_str(&g.save_string(), Sfx::default()).ok().expect("load");
        let c = h.crops[&(3, 4)];
        assert_eq!((c.kind, c.grown, c.wet), (1, 12.5, 30.));
        assert_eq!((h.p.can, h.p.water, h.seed_sel, h.bed), (true, 5, 1, Some((7, 8))));
        assert_eq!((h.p.sword, h.p.staff, h.p.armor), (5, 3, [4, 4, 4]));
        assert_eq!(h.w.tile_at(3, 4), T_WET);
        // เซฟเก่า (21 ไอเทม) ยังโหลดได้
        let old = "seed 5\nspawn 0 0\ntime 0\np 0 0 10 10 0 1 0 0 0 0 0 0 0 0 0\ninv 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21\n";
        let o = Game::load_str(old, Sfx::default()).ok().expect("old save");
        assert_eq!(o.p.inv[FISH], 21);
        assert_eq!(o.p.inv[SEED_CORN], 0);
    }

    #[test]
    fn bed_sleep_and_respawn() {
        let mut g = game(13);
        let k = (tfloor(g.p.x) + 2, tfloor(g.p.y));
        g.w.placed.insert(k, 33);
        g.time = DAYLEN * 2. + DAYLEN * 0.3; // กลางวัน
        g.p.hp = 3;
        g.sleep(k);
        assert_eq!(g.p.hp, 3, "กลางวันนอนไม่ได้");
        assert!(g.bed.is_none());
        g.time = DAYLEN * 2. + DAYLEN * 0.6; // กลางคืน (hr>=18)
        assert!(g.is_night_hr());
        g.make_enemy(EK::Slime, g.p.x + 50., g.p.y, 1., 0, false);
        g.crops.insert((1, 1), Crop { kind: 0, grown: 0., wet: 20. });
        g.sleep(k);
        assert_eq!(g.p.hp, g.p.maxhp);
        assert!(g.en.is_empty());
        assert_eq!(g.time, DAYLEN * 3.);
        assert!(!g.is_night_hr() && g.hour() < 7.);
        assert_eq!(g.bed, Some(k));
        assert!(g.crops[&(1, 1)].grown >= 40.);
        // ตายแล้วเกิดที่เตียง
        g.p.x = 5000.; g.p.y = 5000.;
        let rp = g.respawn_pt();
        assert!((tfloor(rp.0) - k.0).abs() <= 1 && (tfloor(rp.1) - k.1).abs() <= 1);
        g.hurt(999, 1., 0.);
        assert_eq!((tfloor(g.p.x) - k.0).abs() <= 1, true);
        // เตียงพัง → กลับจุดเกิด
        g.w.placed.remove(&k);
        assert_eq!(g.respawn_pt(), g.spawn);
        // ตั้งเตียงข้ามวัน: นอนตอนเช้ามืด (hr<5) ก็ได้
        g.w.placed.insert(k, 33);
        g.time = DAYLEN * 5. + DAYLEN * 0.8;
        assert!(g.is_night_hr());
        g.sleep(k);
        assert_eq!(g.time, DAYLEN * 6.);
    }

    #[test]
    fn advanced_table_tier2_gating() {
        let mut g = game(14);
        for i in [CORE, SHARD, GEM, GEL] { g.p.inv[i] = 50; }
        let st = |g: &Game, i: usize| recipe_state(&g.p, &ADV_RECIPES[i]);
        assert_eq!(st(&g, 0), 2, "ดาบขั้น 2 ต้องมีดาบแกนโกเลมก่อน");
        assert_eq!(st(&g, 1), 2);
        assert_eq!(st(&g, 2), 2);
        g.craft_from(&ADV_RECIPES, 0);
        assert_eq!(g.p.sword, 0);
        g.p.sword = 4; g.p.staff = 2; g.p.armor = [3, 2, 3];
        assert_eq!((st(&g, 0), st(&g, 1), st(&g, 2), st(&g, 3), st(&g, 4)), (0, 0, 0, 2, 0));
        g.craft_from(&ADV_RECIPES, 0); g.craft_from(&ADV_RECIPES, 1); g.craft_from(&ADV_RECIPES, 2); g.craft_from(&ADV_RECIPES, 4);
        assert_eq!((g.p.sword, g.p.staff, g.p.armor), (5, 3, [4, 2, 4]));
        assert!(SWD[g.p.sword] > SWD[4]);
        assert!(g.p.reduction() > 0.3 - 1e-3 && g.p.reduction() <= 0.75);
        // คทาขั้น 3 ยิง 5 ทิศ
        g.p.weapon = 2; g.p.atk_cd = 0.; g.proj.clear();
        g.shoot(Some(vec2(1., 0.)));
        assert_eq!(g.proj.len(), 5);
        // วางโต๊ะแล้วโต้ตอบ
        let k = (tfloor(g.p.x) + 1, tfloor(g.p.y));
        g.w.placed.insert(k, 34);
        assert!(matches!(g.interact_target(), Some((_, 34))));
        g.interact();
        assert_eq!(g.req, Some(Req::Adv));
    }

    #[test]
    fn grid_snap_targets_cursor_in_reach() {
        let mut g = game(15);
        let (tx, ty) = grass_spot(&g);
        g.p.x = tx as f32 * 16. + 8.; g.p.y = ty as f32 * 16. + 11.;
        g.p.dir = 2; g.p.flip = false;
        let face = g.facing_tile();
        assert_eq!(g.build_target(), face);
        g.build_cur = Some((tx + 2, ty));
        assert_eq!(g.build_target(), (tx + 2, ty));
        g.build_cur = Some((tx + 40, ty));
        g.p.inv[WOOD] = 10;
        g.build_sel = PIECES.iter().position(|p| p.obj == 10);
        g.place_piece();
        assert_eq!(g.w.obj_at(face.0, face.1), 0, "นอกระยะ: ไม่วาง (และไม่แอบวางหน้าตัว)");
        assert_eq!(g.w.obj_at(tx + 40, ty), 0);
        // วางกำแพงตรงช่องที่เลือก (ไม่ใช่หน้าตัว)
        g.p.inv[WOOD] = 10;
        g.build_sel = PIECES.iter().position(|p| p.obj == 10);
        g.build_cur = Some((tx + 2, ty));
        g.cur_pos = (g.p.x, g.p.y);
        g.place_piece();
        assert_eq!(g.w.obj_at(tx + 2, ty), 10);
        assert_eq!(g.w.obj_at(face.0, face.1), 0);
        // ขยับห่างเกิน 20px → เคอร์เซอร์หาย
        g.p.x += 30.;
        g.update(0.016, &Input::default());
        assert!(g.build_cur.is_none() || g.build_sel.is_some());
        g.build_sel = None;
        g.update(0.016, &Input::default());
        assert!(g.build_cur.is_none());
    }

    #[test]
    fn snow_slows_and_desert_has_no_rain() {
        let mut g = game(16);
        let find = |g: &Game, t: u8| -> (i32, i32) {
            for ty in (-3000..3000).step_by(7) { for tx in (-3000..3000).step_by(7) { if g.w.tile_at(tx, ty) == t { return (tx, ty); } } }
            panic!("ไม่พบไทล์ {}", t)
        };
        let s = find(&g, 7);
        g.p.x = s.0 as f32 * 16. + 8.; g.p.y = s.1 as f32 * 16. + 8.;
        g.rain_on = true; g.rain_t = 1000.;
        for _ in 0..400 { g.upd_weather(0.1); }
        assert_eq!(g.weather_kind, 2);
        assert!(g.rain > 0.9);
        let m = g.weather_mul();
        assert!(m < 0.84 && m > 0.7, "หิมะต้องทำให้ช้าลง ~18% ได้ {}", m);
        let d = find(&g, 8);
        g.p.x = d.0 as f32 * 16. + 8.; g.p.y = d.1 as f32 * 16. + 8.;
        for _ in 0..400 { g.upd_weather(0.1); }
        assert_eq!(g.weather_kind, 0);
        assert!(g.rain < 0.05);
        assert_eq!(g.weather_mul(), 1.);
        let gr = grass_spot(&g);
        g.p.x = gr.0 as f32 * 16. + 8.; g.p.y = gr.1 as f32 * 16. + 8.;
        for _ in 0..400 { g.upd_weather(0.1); }
        assert_eq!(g.weather_kind, 1);
        assert!(g.weather_mul() < 1. && g.weather_mul() > 0.9);
        // ฝนทำให้ดินเปียกเอง
        let k = (gr.0 + 1, gr.1);
        g.w.tmod.insert(k, T_SOIL);
        g.w.placed.insert(k, 27);
        g.crops.insert(k, Crop { kind: 0, grown: 0., wet: 0. });
        g.upd_crops(1.);
        assert!(g.crops[&k].grown > 0.);
    }
    #[test]
    fn cave_generation_is_connected_and_minable() {
        let opened = HashSet::new();
        for (seed, cx, cy) in [(1, 0, 0), (777, 3, -2), (42, -5, 7), (9, 11, 11), (31337, -9, -4), (5, 2, 2)] {
            let d = Dungeon::generate_cave(seed, (cx, cy), false, &opened, 0);
            assert!(d.cave);
            assert_eq!(d.tile(d.entry.0, d.entry.1), T_CFLOOR);
            assert_eq!(d.tile(d.boss_at.0, d.boss_at.1), T_CFLOOR);
            // ผนังรอบนอกต้องปิด
            for x in 0..DW { assert_eq!(d.tile(x, 0), T_CWALL); assert_eq!(d.tile(x, DH - 1), T_CWALL); }
            // เดินจากทางเข้าไปห้องบอสและหีบทุกใบได้ แม้แร่เป็นของตัน
            let dm = dun_dist(&d, d.entry, false);
            let ix = |x: i32, y: i32| (y * DW + x) as usize;
            assert!(dm[ix(d.boss_at.0, d.boss_at.1)] > 20, "ห้องบอสต้องไกลพอและไปถึง seed {}", seed);
            assert!(d.chests.len() >= 3, "หีบ 2 ใบ + หีบมินิบอส");
            for &(x, y) in &d.chests {
                assert!([(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|&(a, b)| dm[ix(x + a, y + b)] >= 0), "หีบไปไม่ถึง");
            }
            let ores = d.objs.iter().filter(|&&o| o == 4 || o == 5).count();
            assert!(ores >= 10, "ต้องมีแร่/หินให้ขุด {} seed {}", ores, seed);
            assert!(d.objs.iter().any(|&o| o == 5), "ต้องมีแร่อัญมณี");
            assert!(d.objs.iter().any(|&o| o == 18) && d.objs.iter().filter(|&&o| o == 20).count() == 1);
            assert!(d.spawns.len() >= 8, "มอนสเตอร์น้อยไป {}", d.spawns.len());
            for &(_, x, y) in &d.spawns { assert_eq!(d.tile(tfloor(x), tfloor(y)), T_CFLOOR); }
            // deterministic
            let d2 = Dungeon::generate_cave(seed, (cx, cy), false, &opened, 0);
            assert_eq!((d.tiles == d2.tiles, d.objs == d2.objs), (true, true));
            // หีบที่เปิดแล้วไม่โผล่อีก
            let mut op = HashSet::new(); op.insert((cx, cy, 0));
            let d3 = Dungeon::generate_cave(seed, (cx, cy), false, &op, 0);
            assert!(d3.objs.iter().filter(|&&o| o == 24).count() < d.objs.iter().filter(|&&o| o == 24).count() + 1);
        }
    }

    #[test]
    fn cave_entrance_enter_mine_exit() {
        let mut g = game(2024);
        let e = find_cave(&g);
        assert_eq!(g.w.obj_at(e.0, e.1), 36);
        assert!(g.w.is_cave_cell(e.0.div_euclid(DCELL), e.1.div_euclid(DCELL)));
        g.p.x = e.0 as f32 * 16. + 8.; g.p.y = e.1 as f32 * 16. + 20.;
        let before = (g.p.x, g.p.y);
        g.w.placed.insert((900, 900), 10);
        let (plc, rem) = (g.w.placed.len(), g.w.removed.len());
        g.interact_target().filter(|t| t.1 == 36).expect("ปากถ้ำต้องโต้ตอบได้");
        g.interact();
        let d = g.w.dun.as_ref().expect("เข้าถ้ำ");
        assert!(d.cave);
        g.update(0.016, &Input::default());
        assert!((g.darkv - 0.8).abs() < 1e-4, "ถ้ำมืดกว่าดันเจี้ยนห้อง {}", g.darkv);
        assert!(g.en.len() >= 8);
        assert!(g.en.iter().any(|e| e.night), "ต้องมีมอนสเตอร์ดุ");
        // ขุดแร่ในถ้ำ
        let ore = (0..DH).flat_map(|y| (0..DW).map(move |x| (x, y))).find(|&(x, y)| g.w.obj_at(x, y) == 5).expect("ไม่มีแร่");
        let (gem0, st0) = (g.p.inv[GEM], g.p.inv[STONE]);
        for _ in 0..6 { g.hit_object(ore.0, ore.1, 1); }
        assert_eq!(g.w.obj_at(ore.0, ore.1), 0, "แร่ต้องหายหลังขุด");
        assert!(g.p.inv[GEM] > gem0 && g.p.inv[STONE] > st0);
        assert_eq!((g.w.placed.len(), g.w.removed.len()), (plc, rem), "การขุดในถ้ำห้ามแก้โลกข้างนอก");
        // วัตถุอื่นในถ้ำ (คบเพลิง) ห้ามตีพัง
        let tch = (0..DH).flat_map(|y| (0..DW).map(move |x| (x, y))).find(|&(x, y)| g.w.obj_at(x, y) == 18).unwrap();
        assert!(!g.hit_object(tch.0, tch.1, 99));
        assert_eq!(g.w.obj_at(tch.0, tch.1), 18);
        // เซฟกลางถ้ำ → โหลดกลับมาอยู่ที่ปากถ้ำข้างนอก
        let h = Game::load_str(&g.save_string(), Sfx::default()).ok().expect("load");
        assert!(h.w.dun.is_none());
        assert!((h.p.x - before.0).abs() < 0.01 && (h.p.y - before.1).abs() < 0.01);
        g.exit_dungeon();
        assert!(g.w.dun.is_none());
        assert_eq!((g.p.x, g.p.y), before);
        // ถ้ำดุกว่า: มอนสเตอร์เลือดเยอะกว่าดันเจี้ยนห้องที่ระยะเดียวกัน (mul x1.4)
        let hp_cave = { g.enter_dungeon(e); let m = g.en.iter().map(|e| e.hp).fold(0., f32::max); g.exit_dungeon(); m };
        assert!(hp_cave >= 4.);
    }

    #[test]
    fn room_dungeons_and_caves_both_exist() {
        let g = game(777);
        let (mut rooms, mut caves) = (0, 0);
        for cy in -12..12 { for cx in -12..12 {
            if let Some(e) = g.w.dun_in_cell(cx, cy) {
                let o = g.w.obj_at(e.0, e.1);
                if g.w.is_cave_cell(cx, cy) { assert_eq!(o, 36); caves += 1; } else { assert_eq!(o, 19); rooms += 1; }
            }
        } }
        assert!(rooms >= 3 && caves >= 3, "rooms {} caves {}", rooms, caves);
    }

    #[test]
    fn light_pulse_is_smooth_and_asset_fit_resizes() {
        let mut prev = pulse_r(66., 0., 1., 0.);
        let (mut lo, mut hi) = (f32::MAX, 0f32);
        for i in 1..2000 {
            let r = pulse_r(66., i as f32 * 0.01, 1., 0.);
            assert!((r - prev).abs() < 0.5, "แสงกระโดด");
            prev = r; lo = lo.min(r); hi = hi.max(r);
        }
        assert!(hi - lo > 8. && lo > 55. && hi < 77., "ต้องหายใจเข้าออก {} {}", lo, hi);
        let mut im = Image::gen_image_color(64, 64, Color::new(1., 0., 0., 1.));
        im.set_pixel(0, 0, Color::new(0., 1., 0., 1.));
        let f = assets::fit(&im, 16, 16);
        assert_eq!((f.width, f.height), (16, 16));
        assert!(f.get_pixel(8, 8).r > 0.9);
        let up = assets::fit(&Image::gen_image_color(2, 2, Color::new(0., 0., 1., 1.)), 8, 8);
        assert_eq!((up.width, up.get_pixel(7, 7).b), (8, 1.));
    }
}

#[cfg(test)]
mod prefs_tests {
    use super::*;
    #[test]
    fn frame_target_modes() {
        prefs::FPS_MODE.store(0, std::sync::atomic::Ordering::Relaxed);
        assert!((prefs::frame_target().unwrap() - 1. / 30.).abs() < 1e-9);
        prefs::FPS_MODE.store(2, std::sync::atomic::Ordering::Relaxed);
        assert!(prefs::frame_target().is_none());
        prefs::FPS_MODE.store(1, std::sync::atomic::Ordering::Relaxed);
    }
    #[test]
    fn touch_buttons_dont_overlap() {
        // ปุ่มหลักต้องไม่ซ้อนกันที่ขนาดจอมือถือทั่วไป (ใช้ค่ากลางของ screen ใน test ไม่ได้ จึงตรวจเรขาคณิตด้วย t=2.4)
        let t = 2.4f32;
        let (sw, sh) = (2400f32, 1080f32);
        let b = [(sw - 80. * t, sh - 84. * t, 50. * t), (sw - 238. * t, sh - 54. * t, 28. * t), (sw - 238. * t, sh - 124. * t, 28. * t), (sw - 80. * t, sh - 184. * t, 28. * t), (sw - 156. * t, sh - 140. * t, 32. * t), (sw - 170. * t, sh - 56. * t, 32. * t), (sw - 238. * t, sh - 194. * t, 26. * t)];
        for i in 0..b.len() { for j in i + 1..b.len() {
            let d = ((b[i].0 - b[j].0).powi(2) + (b[i].1 - b[j].1).powi(2)).sqrt();
            assert!(d > b[i].2 + b[j].2, "ปุ่ม {} ซ้อน {}", i, j);
        } }
    }
}

#[cfg(test)]
mod guide_tests {
    use super::*;
    #[test]
    fn tutorial_advances_and_finishes() {
        let mut g = Game::new_world_seed(12345, Sfx::default());
        assert_eq!(g.tut, 0);
        assert!(g.tut_text().is_some());
        g.tut_update(0.016);
        assert_eq!(g.tut, 0);
        g.p.inv[WOOD] = 3;
        g.tut_update(0.016);
        assert_eq!(g.tut, 1);
        g.p.inv[STONE] = 3;
        g.tut_update(0.016);
        assert_eq!(g.tut, 2);
        for k in 0..4 { g.ev[k] += 1; g.tut_update(0.016); }
        assert_eq!(g.tut, TUT_DONE);
        let coin = g.p.inv[COIN];
        assert!(coin >= 30);
        assert!(g.tut_text().is_none());
        // เซฟ/โหลดเก็บสถานะบทเรียน
        let txt = g.save_string();
        assert!(txt.contains("tut 99"));
    }
    #[test]
    fn old_save_without_tut_line_skips_tutorial() {
        let g = Game::new_world_seed(777, Sfx::default());
        let txt: String = g.save_string().lines().filter(|l| !l.starts_with("tut")).map(|l| format!("{}\n", l)).collect();
        let l = Game::load_str(&txt, Sfx::default()).ok().unwrap();
        assert_eq!(l.tut, TUT_DONE);
    }
    #[test]
    fn use_label_none_in_open_field_and_tree_target_found() {
        let g = Game::new_world_seed(2024, Sfx::default());
        // ไม่มีอะไรให้ใช้ตรงจุดเกิด (ยกเว้นน้ำ/ฟาร์ม) → label เป็น None หรือ ตกปลา เท่านั้น
        if let Some(l) = g.use_label() { assert!(l == "ตกปลา" || !l.is_empty()); }
        assert!(g.tut_target().is_some(), "ควรมีต้นไม้ในรัศมี 18 ช่องจากจุดเกิด");
    }
}

#[cfg(test)]
mod skin_tests {
    use super::*;
    #[test]
    fn skin_parts_armor_overrides_outfit() {
        let mut p = Player::new(0., 0.);
        assert_eq!(skin_parts(&p), [0, 0, 0]);
        p.outfit = 1;
        assert_eq!(skin_parts(&p), [5, 5, 5]);
        p.armor = [2, 0, 4];
        assert_eq!(skin_parts(&p), [2, 5, 4]);
        p.outfit = 2;
        assert_eq!(skin_parts(&p), [2, 6, 4]);
        p.armor = [4; 3];
        assert!(full_meteor(&p));
    }
    #[test]
    fn outfit_craft_and_save_roundtrip() {
        let mut g = Game::new_world_seed(99, Sfx::default());
        g.p.inv[WOOD] = 10; g.p.inv[CORN] = 5;
        let i = RECIPES.iter().position(|r| matches!(r.eff, Eff::Outfit(1))).unwrap();
        assert_eq!(recipe_state(&g.p, &RECIPES[i]), 0);
        g.craft(i);
        assert_eq!((g.p.outfit, g.p.outfits), (1, 1));
        assert_eq!(recipe_state(&g.p, &RECIPES[i]), 1);
        let txt = g.save_string();
        let l = Game::load_str(&txt, Sfx::default()).ok().unwrap();
        assert_eq!((l.p.outfit, l.p.outfits), (1, 1));
    }
}

#[cfg(test)]
mod npc_tests {
    use super::*;

    fn mk(id: u32, state: u8, bed: (i32, i32)) -> npc::Npc {
        npc::Npc { id, name: (id % 12) as u8, look: 0, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state, hearts: 2, mood: 60, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id % 12) as u8) }
    }
    fn home(g: &mut Game, n: i32) {
        for k in 0..n {
            let b = (500 + k * 2, 500);
            g.w.placed.insert(b, 33);
            let mut m = mk(k as u32 + 1, npc::ST_HOME, b);
            m.soc.love = GEM as u8; m.soc.hate = FISH as u8; // รสนิยมกลาง ๆ สำหรับข้าวโพด
            g.npcs.push(m);
        }
        g.w.placed.insert((498, 498), 39); // บ่อน้ำ
        g.npc_next = n as u32 + 1;
    }
    fn chest_with_food(g: &mut Game, near: (i32, i32), n: i32) {
        let mut c = [0; NI];
        c[CORN] = n;
        g.chests.insert((near.0 + 1, near.1), c);
    }
    /// หาเซลล์ที่มีคนหลงทางใกล้จุดเกิด
    fn find_lost(g: &Game) -> (i32, i32, f32, f32) {
        for cy in -12..=12 { for cx in -12..=12 {
            if let Some((x, y, _, _)) = g.w.lost_in_cell(cx, cy) { return (cx, cy, x, y); }
        } }
        panic!("ไม่มีคนหลงทางเลยในรัศมี 12 เซลล์");
    }

    #[test]
    fn lost_npc_is_deterministic_and_on_walkable_ground() {
        let g = Game::new_world_seed(4242, Sfx::default());
        let (cx, cy, x, y) = find_lost(&g);
        assert_eq!(g.w.lost_in_cell(cx, cy).map(|t| (t.0, t.1)), Some((x, y)));
        assert!(!g.w.solid_at(tfloor(x), tfloor(y - 4.)));
        let g2 = Game::new_world_seed(4242, Sfx::default());
        assert_eq!(g2.w.lost_in_cell(cx, cy).map(|t| (t.0, t.1)), Some((x, y)));
    }

    #[test]
    fn rescue_follow_then_invite_needs_free_bed() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (cx, cy, x, y) = find_lost(&g);
        g.p.x = x; g.p.y = y;
        assert_eq!(g.lost_near(), Some((cx, cy)));
        assert!(g.npc_interact());
        assert_eq!(g.npcs.len(), 1);
        assert_eq!(g.npcs[0].state, npc::ST_FOLLOW);
        assert!(g.npc_seen.contains(&(cx, cy)));
        assert!(g.lost_near().is_none(), "ช่วยแล้วต้องไม่โผล่ซ้ำ");
        let id = g.npcs[0].id;
        // ไม่มีเตียง → ยังไม่ย้ายเข้า
        g.npc_invite(id);
        assert_eq!(g.npcs[0].state, npc::ST_FOLLOW);
        assert_eq!(g.pop(), 0);
        // มีเตียง → ย้ายเข้า กลายเป็นค่าย
        g.w.placed.insert((int_near(x), int_near(y)), 33);
        g.npc_invite(id);
        assert_eq!(g.npcs[0].state, npc::ST_HOME);
        assert_eq!(g.pop(), 1);
        assert_eq!(g.tier(), 1);
        // เตียงเดียวใช้ได้คนเดียว
        assert!(g.free_beds().is_empty());
    }
    fn int_near(v: f32) -> i32 { tfloor(v) + 2 }

    #[test]
    fn tier_thresholds_and_kingdom_needs_castle_and_walls() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        assert_eq!(g.tier(), 0);
        home(&mut g, 3);
        assert_eq!(g.tier(), 1);
        g.w.placed.insert((520, 520), 33);
        g.npcs.push(mk(4, npc::ST_HOME, (520, 520)));
        assert_eq!(g.tier(), 2, "4 คน = หมู่บ้าน");
        for k in 5..=11 {
            let b = (530 + k, 520);
            g.w.placed.insert(b, 33);
            g.npcs.push(mk(k as u32, npc::ST_HOME, b));
        }
        assert_eq!(g.pop(), 11);
        assert_eq!(g.tier(), 2, ">10 คนแต่ยังไม่มีปราสาท/กำแพง");
        g.w.placed.insert((600, 600), 37);
        assert_eq!(g.tier(), 2, "มีปราสาทแต่ยังไม่มีกำแพง");
        for k in 0..12 { g.w.placed.insert((610 + k, 600), 38); }
        assert_eq!(g.tier(), 3);
        // คนที่ตามอยู่ (ยังไม่ย้ายเข้า) ไม่นับเป็นประชากร
        g.npcs.push(mk(99, npc::ST_FOLLOW, (0, 0)));
        assert_eq!(g.pop(), 11);
    }

    #[test]
    fn dawn_mood_depends_on_bed_and_food() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        home(&mut g, 1);
        let bed = g.npcs[0].bed;
        chest_with_food(&mut g, bed, 5);
        g.npcs[0].mood = 60;
        g.npc_dawn();
        assert_eq!(g.npcs[0].mood, 65, "มีเตียง(+3) + อิ่ม(+5) − นอนกลางแจ้ง(3)");
        assert_eq!(g.chests[&(bed.0 + 1, bed.1)][CORN], 4, "กินอาหารจากหีบ 1 ชิ้น");
        // อาหารหมด → ความสุขลด
        g.chests.clear();
        g.npcs[0].mood = 60;
        g.npc_dawn();
        assert_eq!(g.npcs[0].mood, 40, "เตียง +3, หิว -20, กลางแจ้ง -3");
        // เตียงหาย + หิว → ลดหนัก
        g.w.placed.remove(&bed);
        g.npcs[0].mood = 60;
        g.npc_dawn();
        assert_eq!(g.npcs[0].mood, 15, "ไม่มีเตียง -25, หิว -20");
    }

    #[test]
    fn unhappy_resident_leaves_at_zero_mood() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        home(&mut g, 2);
        // อาหารมี 1 ชิ้นพอแค่คนเดียว: id=2 อยู่หน้าลิสต์จึงได้กินก่อน, id=1 หิว (10+3-20 → 0) → ย้ายออก
        g.npcs.swap(0, 1);
        g.npcs[0].mood = 100; // id=2
        g.npcs[1].mood = 10;  // id=1
        let bed = g.npcs[0].bed;
        chest_with_food(&mut g, bed, 1);
        g.npc_dawn();
        let ids: Vec<u32> = g.npcs.iter().map(|n| n.id).collect();
        assert_eq!(ids, vec![2], "id=1 ความสุขหมดต้องย้ายออก เหลือ id=2");
        assert_eq!(g.pop(), 1);
    }

    #[test]
    fn marriage_requirements_and_single_spouse() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        home(&mut g, 2);
        g.p.inv[COIN] = 500; g.p.inv[GEM] = 20;
        g.npcs[0].hearts = 7;
        g.npc_marry(1);
        assert!(!g.npcs[0].married, "ความสนิทไม่ถึง 8");
        g.npcs[0].hearts = 8; g.npcs[0].mood = 40;
        g.npc_marry(1);
        assert!(!g.npcs[0].married, "ความสุขต่ำ");
        g.npcs[0].mood = 80;
        g.p.inv[COIN] = 50;
        g.npc_marry(1);
        assert!(!g.npcs[0].married, "เหรียญไม่พอ");
        g.p.inv[COIN] = 500;
        g.npc_marry(1);
        assert!(g.npcs[0].married);
        assert_eq!((g.p.inv[COIN], g.p.inv[GEM]), (500 - npc::MARRY_COIN, 20 - npc::MARRY_GEM));
        g.npcs[1].hearts = 10; g.npcs[1].mood = 90;
        g.npc_marry(2);
        assert!(!g.npcs[1].married, "แต่งได้คนเดียว");
    }

    #[test]
    fn child_born_after_wedding_when_bed_free() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        home(&mut g, 1);
        g.w.placed.insert((700, 700), 33); // เตียงว่างสำหรับลูก
        g.npcs[0].married = true;
        g.wed_day = 0;
        g.time = DAYLEN * 5.5; // วันที่ 5
        g.npc_dawn();
        assert_eq!(g.npcs.iter().filter(|n| n.child).count(), 1);
        assert_eq!(g.last_child, 5);
        g.npc_dawn(); // วันเดิม → ห้ามมีลูกซ้ำ
        assert_eq!(g.npcs.iter().filter(|n| n.child).count(), 1);
    }

    #[test]
    fn tax_only_in_kingdom_and_collectable() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        home(&mut g, 11);
        for n in g.npcs.iter_mut() { n.mood = 90; }
        g.npc_dawn();
        assert_eq!(g.tax_pool, 0, "ยังไม่ใช่อาณาจักร");
        g.w.placed.insert((600, 600), 37);
        for k in 0..12 { g.w.placed.insert((610 + k, 600), 38); }
        for n in g.npcs.iter_mut() { n.mood = 90; }
        for b in g.beds() { chest_with_food(&mut g, b, 1); }
        g.npc_dawn();
        assert!(g.tax_pool > 0);
        let c0 = g.p.inv[COIN];
        let pool = g.tax_pool;
        g.collect_tax();
        assert_eq!(g.p.inv[COIN], c0 + pool);
        assert_eq!(g.tax_pool, 0);
    }

    #[test]
    fn save_roundtrip_keeps_npcs_and_old_seen_line_still_works() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        home(&mut g, 2);
        g.npcs[0].married = true; g.npcs[0].hearts = 9; g.npcs[1].mood = 33;
        g.npc_seen.insert((3, -4));
        g.tax_pool = 17; g.wed_day = 2; g.last_child = 3;
        g.seen.insert(0); // "seen" เดิม (ของ discovery) ต้องไม่ถูกชนกับของ NPC
        let txt = g.save_string();
        let l = Game::load_str(&txt, Sfx::default()).ok().unwrap();
        assert_eq!(l.npcs.len(), 2);
        assert!(l.npcs[0].married);
        assert_eq!((l.npcs[0].hearts, l.npcs[1].mood), (9, 33));
        assert_eq!(l.npcs[0].bed, g.npcs[0].bed);
        assert!(l.npc_seen.contains(&(3, -4)));
        assert_eq!((l.tax_pool, l.wed_day, l.last_child), (17, 2, 3));
        assert!(l.seen.contains(&0));
        assert_eq!(l.npc_next, g.npc_next);
    }

    #[test]
    fn npc_tick_followers_walk_toward_player() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        g.p.x = 400.; g.p.y = 400.;
        let mut f = mk(1, npc::ST_FOLLOW, (0, 0));
        f.x = 340.; f.y = 400.;
        g.npcs.push(f);
        let d0 = (g.npcs[0].x - g.p.x).abs();
        for _ in 0..60 { g.npc_tick(0.05); }
        let d1 = (g.npcs[0].x - g.p.x).abs();
        assert!(d1 < d0, "คนที่ตามต้องเดินเข้าหาผู้เล่น ({} → {})", d0, d1);
    }
}

#[cfg(test)]
mod social_tests {
    use super::*;

    fn mk(id: u32, bed: (i32, i32)) -> npc::Npc {
        npc::Npc { id, name: (id % 12) as u8, look: 0, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: npc::ST_HOME, hearts: 3, mood: 70, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id % 12) as u8) }
    }
    fn town(g: &mut Game, n: u32) {
        for k in 0..n { let b = (500 + k as i32 * 2, 500); g.w.placed.insert(b, 33); g.npcs.push(mk(k + 1, b)); }
        g.w.placed.insert((498, 498), 39); // บ่อน้ำ
        g.npc_next = n + 1;
    }

    #[test]
    fn soc_generate_is_deterministic_and_varied() {
        let a = Soc::generate(5, 3);
        let b = Soc::generate(5, 3);
        assert_eq!((a.tr, a.love, a.hate), (b.tr, b.love, b.hate));
        assert_ne!(a.love, a.hate);
        assert!(a.tr.iter().all(|&t| (10..=95).contains(&t)));
        // คนละ id ต้องต่างกันพอสมควร (50 คน ต้องมีนิสัยเด่นอย่างน้อย 4 แบบ)
        let mut seen = HashSet::new();
        for id in 1..=50 { seen.insert(Soc::generate(id, (id % 12) as u8).top_trait()); }
        assert!(seen.len() >= 4, "{:?}", seen);
        let loves: HashSet<u8> = (1..=50).map(|id| Soc::generate(id, 0).love).collect();
        assert!(loves.len() >= 5);
    }

    #[test]
    fn loved_gift_beats_neutral_and_hated_gift_angers() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        town(&mut g, 2);
        let (love, hate) = (g.npcs[0].soc.love as usize, g.npcs[0].soc.hate as usize);
        let neutral = GIFTS.iter().copied().find(|&i| i != love && i != hate).unwrap();
        for it in [love, hate, neutral] { g.p.inv[it] = 5; }
        // ชอบ
        let h0 = g.npcs[0].hearts;
        g.npc_gift_item(1, love);
        assert_eq!(g.npcs[0].hearts, h0 + 3);
        assert!(g.npcs[0].gifted);
        g.npc_gift_item(1, love); // วันละครั้ง
        assert_eq!(g.p.inv[love], 4);
        // ไม่ชอบ → โกรธ
        let hate2 = g.npcs[1].soc.hate as usize;
        g.p.inv[hate2] = 5;
        let h = g.npcs[1].hearts;
        g.npc_gift_item(2, hate2);
        assert_eq!(g.npcs[1].hearts, h.saturating_sub(1));
        assert!(g.npcs[1].soc.anger >= 15);
        // กลาง ๆ ได้ตามค่าพื้นฐาน
        let mut g2 = Game::new_world_seed(1, Sfx::default());
        town(&mut g2, 2);
        let nl = g2.npcs[0].soc.love as usize; let nh = g2.npcs[0].soc.hate as usize;
        let neu = GIFTS.iter().copied().find(|&i| i != nl && i != nh).unwrap();
        g2.p.inv[neu] = 2;
        g2.npc_gift_item(1, neu);
        assert_eq!(g2.npcs[0].hearts as i32, 3 + gift_base(neu));
    }

    #[test]
    fn repeated_same_gift_gets_boring() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        town(&mut g, 1);
        let neu = GIFTS.iter().copied().find(|&i| i != g.npcs[0].soc.love as usize && i != g.npcs[0].soc.hate as usize && gift_base(i) == 2).unwrap();
        g.p.inv[neu] = 9;
        g.npc_gift_item(1, neu);
        let h1 = g.npcs[0].hearts;
        for _ in 0..2 { g.npcs[0].gifted = false; g.npc_gift_item(1, neu); }
        let h3 = g.npcs[0].hearts;
        // ครั้งที่ 3 (bore>=2) ได้แค่ครึ่งเดียว: +1 แทน +2
        assert_eq!(h3 as i32 - (h1 as i32 + 2), 1, "ครั้งที่ 2 ได้เต็ม ครั้งที่ 3 ได้ครึ่ง");
    }

    #[test]
    fn angry_npc_refuses_then_apology_and_time_heal() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        town(&mut g, 1);
        g.npcs[0].x = g.p.x; g.npcs[0].y = g.p.y;
        let id = 1;
        let mut c = [0; NI]; c[CORN] = 9; g.chests.insert((501, 500), c); // มีอาหาร จะได้ไม่โกรธเพราะหิวเพิ่ม
        g.npc_hit(0);
        assert!(g.npcs[0].soc.anger >= 45 && g.npcs[0].soc.grudge >= 12);
        assert_eq!(g.npcs[0].hearts, 1);
        assert!(refuses(&g.npcs[0]));
        g.npc_talk(id);
        assert!(!g.npcs[0].talked, "คนที่โกรธไม่ยอมคุย");
        g.p.inv[GEM] = 3;
        g.npc_gift_item(id, GEM);
        assert!(!g.npcs[0].gifted, "ไม่รับของขวัญตอนโกรธ");
        // รอ 2 รุ่งเช้า ความโกรธ (ระยะสั้น) หาย แต่ความแค้นยังเหลือ
        for d in 1..=3 { g.time = DAYLEN * (d as f32 + 0.5); g.npc_dawn(); }
        assert_eq!(g.npcs[0].soc.anger, 0, "โกรธ 45 หายใน 3 วัน");
        let gr = g.npcs[0].soc.grudge;
        assert!(gr >= 3 && gr < 12, "แค้นจางช้ากว่าโกรธ: {}", gr);
        g.npcs[0].talked = false;
        g.npc_talk(id);
        assert!(g.npcs[0].talked, "หายโกรธแล้วคุยได้");
    }

    #[test]
    fn apology_chance_depends_on_kindness_and_once_a_day() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        town(&mut g, 1);
        g.npcs[0].soc.anger = 60; g.npcs[0].soc.grudge = 20; g.npcs[0].soc.tr[T_KIND] = 95; g.npcs[0].hearts = 6;
        g.time = DAYLEN * 3.1;
        g.npc_apologize(1);
        let a1 = g.npcs[0].soc.anger;
        assert!(a1 < 60);
        g.npc_apologize(1);
        assert_eq!(g.npcs[0].soc.anger, a1, "ขอโทษซ้ำวันเดิมไม่ได้ผล");
        // ไม่มีเรื่องค้างคาใจ → ไม่ทำอะไร
        let mut g2 = Game::new_world_seed(1, Sfx::default());
        town(&mut g2, 1);
        g2.npc_apologize(1);
        assert_eq!(g2.npcs[0].soc.apol_day, -1);
    }

    #[test]
    fn opinion_labels_and_price_follow_feelings() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        town(&mut g, 1);
        g.npcs[0].hearts = 2; assert_eq!(opinion_label(opinion(&g.npcs[0])), "คุ้นเคย");
        g.npcs[0].hearts = 5; assert_eq!(opinion_label(opinion(&g.npcs[0])), "เพื่อน");
        g.npcs[0].hearts = 8; assert_eq!(opinion_label(opinion(&g.npcs[0])), "เพื่อนสนิท");
        g.npcs[0].soc.tr[T_GREED] = 50;
        let p_friend = g.npc_price(1, 100).unwrap();
        g.npcs[0].hearts = 1; g.npcs[0].soc.grudge = 0;
        let p_stranger = g.npc_price(1, 100).unwrap();
        assert!(p_friend < 100 && p_stranger > p_friend, "{} {}", p_friend, p_stranger);
        g.npcs[0].hearts = 0; g.npcs[0].soc.grudge = 40;
        assert!(g.npc_price(1, 100).is_none(), "ไม่ขายให้คนที่ตัวเองเกลียด");
        assert!(!Game::willing(&{ let mut n = g.npcs[0].clone(); n.soc.grudge = 50; n }), "ยาม/ชาวไร่หยุดงานเมื่อเกลียดผู้นำ");
    }

    #[test]
    fn friendship_fades_without_contact_but_talking_resets() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        town(&mut g, 1);
        g.npcs[0].hearts = 5; g.npcs[0].soc.last_talk = 1;
        g.time = DAYLEN * 3.5; g.npc_dawn();
        assert_eq!(g.npcs[0].hearts, 5, "ยังไม่ถึง 6 วัน");
        g.time = DAYLEN * 8.5; g.npc_dawn();
        assert_eq!(g.npcs[0].hearts, 4, "ไม่คุยเกิน 6 วัน สนิทลดลง");
        // ฝ่ายคู่สมรสที่ถูกทิ้งไว้เสียอารมณ์แทน
        g.npcs[0].married = true; g.npcs[0].soc.last_talk = 1; let m0 = g.npcs[0].mood;
        g.time = DAYLEN * 12.5; g.npc_dawn();
        assert!(g.npcs[0].mood < m0 + 8, "คู่สมรสที่ไม่ได้คุยด้วยเศร้า: {} → {}", m0, g.npcs[0].mood);
    }

    #[test]
    fn marriage_triggers_jealousy_or_joy_by_closeness_and_kindness() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        town(&mut g, 4);
        g.p.inv[COIN] = 500; g.p.inv[GEM] = 20;
        g.npcs[0].hearts = 9; g.npcs[0].mood = 90;
        // id2: สนิทมาก+ไม่ใจดี → อิจฉา · id3: สนิทมาก+ใจดี → ยอมรับ · id4: สนิทปานกลาง → ยินดี
        g.npcs[1].hearts = 9; g.npcs[1].soc.tr[T_KIND] = 20;
        g.npcs[2].hearts = 9; g.npcs[2].soc.tr[T_KIND] = 90;
        g.npcs[3].hearts = 5;
        let (m2, m3, m4) = (g.npcs[1].mood, g.npcs[2].mood, g.npcs[3].mood);
        g.npc_marry(1);
        assert!(g.npcs[0].married);
        assert!(g.npcs[1].soc.anger >= 30 && g.npcs[1].soc.grudge >= 15 && g.npcs[1].hearts == 6 && g.npcs[1].mood < m2);
        assert!(g.npcs[2].soc.anger == 0 && g.npcs[2].hearts == 9 && g.npcs[2].mood < m3);
        assert!(g.npcs[3].mood > m4 && g.npcs[3].hearts == 6);
        assert!(g.news.iter().any(|(_, t)| t.contains("อิจฉา")));
        // คนที่ยังมีเรื่องค้างคาใจ แต่งไม่ได้
        let mut g2 = Game::new_world_seed(1, Sfx::default());
        town(&mut g2, 1);
        g2.p.inv[COIN] = 500; g2.p.inv[GEM] = 20;
        g2.npcs[0].hearts = 9; g2.npcs[0].mood = 90; g2.npcs[0].soc.grudge = 30;
        g2.npc_marry(1);
        assert!(!g2.npcs[0].married);
    }

    #[test]
    fn quarrels_happen_between_incompatible_neighbours_and_logged() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        town(&mut g, 2);
        // บังคับให้ไม่ถูกกัน + ใจร้อนทั้งคู่
        for n in g.npcs.iter_mut() { n.soc.tr[T_TEMPER] = 95; n.soc.tr[T_KIND] = 10; n.mood = 90; }
        g.add_rel(1, 2, -60);
        let r0 = g.rel_between(1, 2);
        assert!(r0 <= -30, "{}", r0);
        let mut quarrels = 0;
        for d in 1..40 {
            g.time = DAYLEN * (d as f32 + 0.5);
            let before = g.news.len();
            g.social_dawn();
            if g.news.len() > before && g.news.last().unwrap().1.contains("ทะเลาะ") { quarrels += 1; }
            if g.npcs.len() < 2 { break; }
        }
        assert!(quarrels >= 1 || g.npcs.len() < 2, "คู่ที่ไม่ถูกกันและใจร้อนต้องทะเลาะ/แยกทางใน 40 วัน");
    }

    #[test]
    fn deep_hatred_makes_one_leave_with_news() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        town(&mut g, 2);
        for n in g.npcs.iter_mut() { n.soc.tr[T_TEMPER] = 95; n.soc.tr[T_KIND] = 5; n.mood = 100; }
        g.npcs[0].soc.tr[T_BRAVE] = 10; g.npcs[0].soc.tr[T_SOCIAL] = 10;  // คนแรกอ่อนแอกว่า
        g.npcs[1].soc.tr[T_BRAVE] = 90; g.npcs[1].soc.tr[T_SOCIAL] = 90;
        g.add_rel(1, 2, -150);
        for d in 1..60 {
            g.time = DAYLEN * (d as f32 + 0.5);
            g.social_dawn();
            if g.npcs.len() < 2 { break; }
        }
        assert_eq!(g.npcs.len(), 1);
        assert_eq!(g.npcs[0].id, 2, "คนที่กล้า/เข้าสังคมน้อยกว่าเป็นฝ่ายย้ายออก");
        assert!(g.news.iter().any(|(_, t)| t.contains("ทนอยู่ร่วมกับ")));
    }

    #[test]
    fn hunger_makes_residents_resent_the_leader_hot_tempered_faster() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        town(&mut g, 2);
        for n in g.npcs.iter_mut() { n.hearts = 0; n.soc.tr[T_KIND] = 10; }
        g.npcs[0].soc.tr[T_TEMPER] = 95; // id1 ใจร้อน
        g.npcs[1].soc.tr[T_TEMPER] = 5;  // id2 ใจเย็น
        for d in 1..5 {
            g.time = DAYLEN * (d as f32 + 0.5);
            g.npc_dawn(); // ไม่มีอาหารเลย
            for n in g.npcs.iter_mut() { n.mood = 100; }
        }
        let (hot, calm) = (g.npcs[0].soc.grudge, g.npcs[1].soc.grudge);
        assert!(calm > 0 && hot > calm, "ใจร้อนแค้นเร็วกว่า: {} vs {}", hot, calm);
    }

    #[test]
    fn roles_follow_aptitude_in_a_village() {
        let mut g = Game::new_world_seed(1, Sfx::default());
        town(&mut g, 6);
        for (i, n) in g.npcs.iter_mut().enumerate() { n.soc.tr[T_BRAVE] = 20 + i as u8 * 10; n.soc.tr[T_GREED] = 30; n.soc.tr[T_SOCIAL] = 30; }
        g.npcs[1].soc.tr[T_GREED] = 95; g.npcs[1].soc.tr[T_SOCIAL] = 95; // พ่อค้าโดยธรรมชาติ
        g.assign_roles_pub();
        assert_eq!(g.npcs[1].role, 3);
        let guards: Vec<u32> = g.npcs.iter().filter(|n| n.role == 2).map(|n| n.id).collect();
        assert_eq!(guards.len(), 1 + 0, "{:?}", guards);
        assert_eq!(guards[0], 6, "ยามคือคนที่กล้าที่สุด");
        assert!(g.npcs.iter().filter(|n| n.role == 1).count() >= 3);
    }

    #[test]
    fn rescue_never_duplicates_a_name_and_news_wrap_state_roundtrips() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        town(&mut g, 3);
        g.npcs[0].soc.anger = 33; g.npcs[0].soc.grudge = 21; g.npcs[0].soc.last_gift = 4; g.npcs[0].soc.bore = 2; g.npcs[0].soc.tr = [11, 22, 33, 44, 55];
        g.add_rel(1, 2, -17);
        g.news_add("ทดสอบ ข่าว มีช่องว่าง".to_string());
        let txt = g.save_string();
        let l = Game::load_str(&txt, Sfx::default()).ok().unwrap();
        assert_eq!(l.npcs[0].soc.tr, [11, 22, 33, 44, 55]);
        assert_eq!((l.npcs[0].soc.anger, l.npcs[0].soc.grudge, l.npcs[0].soc.last_gift, l.npcs[0].soc.bore), (33, 21, 4, 2));
        assert_eq!((l.npcs[0].soc.love, l.npcs[0].soc.hate), (g.npcs[0].soc.love, g.npcs[0].soc.hate));
        assert_eq!(l.rel_between(1, 2), g.rel_between(1, 2));
        assert!(l.news.iter().any(|(_, t)| t == "ทดสอบ ข่าว มีช่องว่าง"));
        // เซฟรุ่นก่อน v7 (ไม่มีบรรทัด nsoc) โหลดได้ และสร้างนิสัยให้ตามเดิม
        let old: String = txt.lines().filter(|l| !l.starts_with("nsoc") && !l.starts_with("nrel") && !l.starts_with("nnews")).map(|l| format!("{}\n", l)).collect();
        let lo = Game::load_str(&old, Sfx::default()).ok().unwrap();
        assert_eq!(lo.npcs.len(), 3);
        assert_eq!(lo.npcs[1].soc.tr, Soc::generate(2, (2 % 12) as u8).tr);
        // ชื่อไม่ซ้ำ: ทุกชื่อในรายการมีเพศกำกับ และไม่มีชื่อซ้ำกัน
        let set: HashSet<&&str> = NPC_NAMES.iter().collect();
        assert_eq!(set.len(), NPC_NAMES.len());
        assert_eq!(NPC_FEM.len(), NPC_NAMES.len());
    }
}

#[cfg(test)]
mod needs_tests {
    use super::*;

    fn g0() -> Game { Game::new_world_seed(777, Sfx::default()) }
    fn stand_on(g: &mut Game, tile: u8) {
        // หาไทล์ชนิดที่ต้องการใกล้จุดเกิด แล้วเอาผู้เล่นไปยืน
        for r in 0..200i32 { for dy in -r..=r { for dx in -r..=r {
            if dx.abs().max(dy.abs()) != r { continue; }
            let (tx, ty) = (tfloor(g.spawn.0) + dx, tfloor(g.spawn.1) + dy);
            if g.w.tile_at(tx, ty) == tile && g.w.obj_at(tx, ty) == 0 { g.p.x = tx as f32 * 16. + 8.; g.p.y = ty as f32 * 16. + 12.; return; }
        } } }
        panic!("ไม่พบไทล์ {}", tile);
    }

    #[test]
    fn thirst_drains_faster_than_hunger_and_rest_slowest() {
        let mut g = g0();
        g.time = DAYLEN * 15.2; // ฤดูใบไม้ผลิ = อัตราพื้นฐาน
        for _ in 0..240 { g.needs_tick(1., 1); } // 1 วันเกม
        let (h, t, r) = (100. - g.p.hunger, 100. - g.p.thirst, 100. - g.p.rest);
        assert!(t > h && h > r, "หิว {:.1} กระหาย {:.1} เหนื่อย {:.1}", h, t, r);
        assert!((t - 66.7).abs() < 3. && (h - 33.3).abs() < 3. && (r - 25.).abs() < 3., "{} {} {}", t, h, r);
    }

    #[test]
    fn desert_is_hotter_and_snow_burns_food_unless_warm() {
        let mut g = g0();
        stand_on(&mut g, 8);
        assert_eq!(g.climate().0, 2.2);
        let mut b = g0();
        stand_on(&mut b, 3);
        assert_eq!(b.climate(), (1., 1.));
        for _ in 0..60 { g.needs_tick(1., 1); b.needs_tick(1., 1); }
        assert!(g.p.thirst < b.p.thirst - 10., "ทะเลทรายกระหายเร็วกว่า: {} vs {}", g.p.thirst, b.p.thirst);
        let mut s = g0();
        stand_on(&mut s, 7);
        assert_eq!(s.climate().1, 1.5);
        s.w.placed.insert((tfloor(s.p.x), tfloor(s.p.y)), 9); // กองไฟใต้เท้า
        assert_eq!(s.climate().1, 1.0, "ผิงไฟกันหนาว");
    }

    #[test]
    fn survival_off_means_no_drain() {
        let mut g = g0();
        for _ in 0..500 { g.needs_tick(1., 0); }
        assert_eq!((g.p.hunger, g.p.thirst, g.p.rest), (100., 100., 100.));
        assert_eq!(g.need_speed_mul_mode(0), 1.);
    }

    #[test]
    fn low_needs_slow_you_down_and_stop_regen() {
        let mut g = g0();
        assert_eq!(g.need_speed_mul_mode(1), 1.);
        assert!(g.can_regen());
        g.p.thirst = 25.; assert_eq!(g.need_speed_mul_mode(1), 0.95);
        g.p.thirst = 10.; assert_eq!(g.need_speed_mul_mode(1), 0.85);
        g.p.hunger = 0.;  assert_eq!(g.need_speed_mul_mode(1), 0.7);
        assert!(!g.can_regen());
        g.p.hunger = 100.; g.p.thirst = 100.; g.p.sick = 10.;
        assert!(!g.can_regen(), "ท้องเสียไม่ฟื้นเลือด");
        assert!((g.need_speed_mul_mode(1) - 0.9).abs() < 1e-6);
    }

    #[test]
    fn starvation_drains_hp_but_normal_mode_never_kills_hard_mode_faints() {
        let mut g = g0();
        g.p.hp = 5; g.p.maxhp = 10; g.p.thirst = 0.; g.p.hunger = 100.;
        for _ in 0..200 { g.needs_tick(1., 1); g.p.thirst = 0.; }
        assert_eq!(g.p.hp, 1, "โหมดปกติ: เหลือ 1 ไม่ตาย");
        // โหมดยาก: หมดสติ ฟื้นที่จุดเกิดพร้อมน้ำ/อาหารขั้นต่ำ เสียเหรียญครึ่งหนึ่ง
        g.p.inv[COIN] = 100;
        let mut fainted = false;
        for _ in 0..60 { g.needs_tick(1., 2); if g.died { fainted = true; break; } g.p.thirst = 0.; }
        assert!(fainted, "โหมดยาก: อดน้ำจนหมดสติ");
        assert_eq!(g.p.hp, g.p.maxhp, "ฟื้นเลือดเต็มหลังหมดสติ");
        assert_eq!(g.p.inv[COIN], 50);
        assert!(g.p.thirst >= 35. && g.p.hunger >= 35., "ฟื้นพร้อมน้ำ/อาหารขั้นต่ำ {} {}", g.p.thirst, g.p.hunger);
        // อัตราเร็วขึ้นเมื่อขาดทั้งอาหารและน้ำ
        let mut a = g0(); a.p.hp = 9; a.p.maxhp = 10; a.p.hunger = 0.; a.p.thirst = 0.;
        let mut b = g0(); b.p.hp = 9; b.p.maxhp = 10; b.p.hunger = 0.; b.p.thirst = 50.;
        for _ in 0..8 { a.p.hunger = 0.; a.p.thirst = 0.; a.needs_tick(1., 1); b.p.hunger = 0.; b.p.thirst = 50.; b.needs_tick(1., 1); }
        assert!(a.p.hp < b.p.hp, "ขาดทั้งคู่เลือดลดเร็วกว่า {} vs {}", a.p.hp, b.p.hp);
    }

    #[test]
    fn eating_restores_hunger_raw_food_can_make_you_sick_herb_cures() {
        let mut g = g0();
        g.p.hunger = 40.; g.p.inv[BERRY] = 5; g.p.inv[MEAT] = 5; g.p.inv[FISH] = 5; g.p.inv[HERB] = 3; g.p.inv[DISH_STEW] = 2; g.p.inv[SHROOM] = 2;
        assert!(g.eat_generic_roll(BERRY, 0.9));
        assert!((g.p.hunger - 52.).abs() < 0.01);
        g.p.hunger = 40.;
        g.use_dish(DISH_STEW);
        assert!((g.p.hunger - 100.).abs() < 0.01, "สตูว์ +60 ทำให้เกือบเต็ม: {}", g.p.hunger);
        // เนื้อดิบ: roll ต่ำกว่า 0.30 → ท้องเสีย
        g.p.hunger = 30.; g.p.hp = 8;
        assert!(g.eat_generic_roll(MEAT, 0.1));
        assert!(g.p.sick > 0. && g.p.hp == 7);
        assert!(g.p.hunger < 30. + 30., "อาเจียนบางส่วน");
        // สมุนไพรแก้ท้องเสีย
        assert!(g.eat_generic_roll(HERB, 0.9));
        assert_eq!(g.p.sick, 0.);
        // โชคดี roll สูง ไม่ป่วย
        g.p.hunger = 30.;
        assert!(g.eat_generic_roll(MEAT, 0.9));
        assert_eq!(g.p.sick, 0.);
        // เห็ด: ก็เสี่ยงเหมือนกัน
        g.p.hunger = 30.;
        g.eat_generic_roll(SHROOM, 0.01);
        assert!(g.p.sick > 0.);
        // อิ่มและเลือดเต็ม → ไม่กิน ไม่เสียของ
        let mut f = g0(); f.p.inv[BERRY] = 2; f.p.hunger = 100.; f.p.hp = f.p.maxhp;
        assert!(!f.eat_generic_roll(BERRY, 0.9));
        assert_eq!(f.p.inv[BERRY], 2);
        // ปรุงสุกไม่เสี่ยง
        let mut c = g0(); c.p.hunger = 20.; c.p.inv[DISH_FISH] = 1;
        c.use_dish(DISH_FISH);
        assert!((c.p.hunger - 65.).abs() < 0.01 && c.p.sick == 0.);
    }

    /// เอาผู้เล่นไปยืนริมน้ำตื้น หันหน้าเข้าหาน้ำ — salty=true ชายทะเล (ติดทะเลลึก) · false บึง/ทะเลสาบน้ำจืด
    fn stand_facing_water(g: &mut Game, salty: bool) -> (i32, i32) {
        for r in 5..350i32 { for dy in -r..=r { for dx in -r..=r {
            if dx.abs().max(dy.abs()) != r { continue; }
            let (tx, ty) = (tfloor(g.spawn.0) + dx, tfloor(g.spawn.1) + dy);
            if g.w.tile_at(tx, ty) != 1 || g.is_salt(tx, ty) != salty { continue; }
            // ยืนที่ช่องข้างน้ำ แล้วหันเข้าหาน้ำ (DV: 0 ลง, 1 ขึ้น, 2 ขวา, 3 ซ้าย)
            for (ox, oy, dir, flip) in [(0, -1, 0usize, false), (0, 1, 1, false), (-1, 0, 2, false), (1, 0, 2, true)] {
                let (sx, sy) = (tx + ox, ty + oy);
                if g.w.solid_at(sx, sy) || g.w.tile_at(sx, sy) < 2 { continue; }
                g.p.x = sx as f32 * 16. + 8.; g.p.y = sy as f32 * 16. + 8. + 3.;
                g.p.dir = dir; g.p.flip = flip;
                if let Some((wx, wy)) = g.water_ahead() { if g.w.tile_at(wx, wy) == 1 && g.is_salt(wx, wy) == salty { return (wx, wy); } }
            }
        } } }
        panic!("ไม่พบจุดยืนริมน้ำ salty={}", salty);
    }

    #[test]
    fn salt_depends_on_deep_sea_nearby() {
        let g = g0();
        let (mut sea, mut pond) = (None, None);
        'o: for y in -250..250 { for x in -250..250 {
            if g.w.tile_at(x, y) != 1 { continue; }
            if g.is_salt(x, y) { if sea.is_none() { sea = Some((x, y)); } } else if pond.is_none() { pond = Some((x, y)); }
            if sea.is_some() && pond.is_some() { break 'o; }
        } }
        let (sx, sy) = sea.expect("มีชายทะเล");
        let (px, py) = pond.expect("มีบึงน้ำจืด");
        assert!((-3..=3).any(|dy| (-3..=3).any(|dx| g.w.tile_at(sx + dx, sy + dy) == 0)));
        assert!(!(-3..=3).any(|dy| (-3..=3).any(|dx| g.w.tile_at(px + dx, py + dy) == 0)));
        let deep = (-250..250).flat_map(|y| (-250..250).map(move |x| (x, y))).find(|&(x, y)| g.w.tile_at(x, y) == 0).unwrap();
        assert!(g.is_salt(deep.0, deep.1));
        assert!(!g.is_salt(tfloor(g.spawn.0), tfloor(g.spawn.1)), "พื้นดินไม่ใช่น้ำ");
    }

    #[test]
    fn drink_fresh_pond_helps_and_requires_thirst() {
        let mut g = g0();
        stand_facing_water(&mut g, false);
        g.p.thirst = 20.;
        assert!(g.drink_ahead_roll(0.9));
        assert!((g.p.thirst - 50.).abs() < 0.01 && g.p.sick == 0.);
        assert!(g.drink_ahead_roll(0.05), "ดื่มซ้ำ น้ำจากธรรมชาติไม่สะอาดเสมอไป");
        assert!(g.p.sick > 0.);
        g.p.thirst = 95.;
        assert!(!g.drink_ahead_roll(0.9), "ไม่กระหายแล้ว ไม่ดื่ม (ไปตกปลาแทน)");
    }

    #[test]
    fn sea_water_is_salty_and_makes_thirst_worse() {
        let mut g = g0();
        stand_facing_water(&mut g, true);
        g.p.thirst = 50.;
        assert!(g.drink_ahead_roll(0.9));
        assert!(g.p.thirst < 50., "น้ำทะเลทำให้กระหายขึ้น: {}", g.p.thirst);
        assert_eq!(g.p.sick, 0.);
    }

    #[test]
    fn well_gives_clean_water_and_supplies_nearby_village() {
        let mut g = g0();
        g.p.thirst = 30.;
        g.drink_well();
        assert!((g.p.thirst - 75.).abs() < 0.01 && g.p.sick == 0.);
        // ชาวบ้านต้องมีน้ำ: ไม่มีแหล่งน้ำ/บ่อ → ความสุขลด · มีบ่อ → ปกติ
        let mut a = g0();
        let mut bed = (900, 900);
        'd: for oy in 0..30 { for ox in 0..30 {
            let c = (900 + ox * 45, 900 + oy * 45);
            if !a.water_within(c.0, c.1, 20) && a.w.tile_at(c.0, c.1) >= 3 && !a.w.solid_at(c.0, c.1) { bed = c; break 'd; }
        } }
        assert!(!a.water_within(bed.0, bed.1, 20), "จุดทดสอบต้องห่างน้ำจืด");
        a.w.placed.insert(bed, 33);
        let mut n = npc::Npc { id: 1, name: 1, look: 0, x: 0., y: 0., state: npc::ST_HOME, hearts: 2, mood: 60, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(1, 1) };
        n.soc.love = GEM as u8; n.soc.hate = FISH as u8;
        a.npcs.push(n);
        let mut c = [0; NI]; c[CORN] = 5; a.chests.insert((bed.0 + 1, bed.1), c);
        a.npc_dawn();
        let dry = a.npcs[0].mood;
        a.w.placed.insert((bed.0 - 2, bed.1 - 2), 39);
        a.npcs[0].mood = 60;
        a.npc_dawn();
        let wet = a.npcs[0].mood;
        assert_eq!(wet - dry, 15, "ไม่มีน้ำ −15 ความสุข ({} vs {})", dry, wet);
        assert!(a.news.iter().any(|(_, t)| t.contains("น้ำดื่ม")));
    }

    #[test]
    fn npcs_eat_favourites_and_avoid_hated_food() {
        let mut g = g0();
        let bed = (900, 900);
        g.w.placed.insert(bed, 33);
        g.w.placed.insert((898, 898), 39);
        let mut n = npc::Npc { id: 1, name: 1, look: 0, x: 0., y: 0., state: npc::ST_HOME, hearts: 2, mood: 50, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(1, 1) };
        n.soc.love = CORN as u8; n.soc.hate = BERRY as u8;
        g.npcs.push(n);
        let mut c = [0; NI]; c[BERRY] = 3; c[CORN] = 3; c[CARROT] = 3;
        g.chests.insert((901, 900), c);
        g.npc_dawn();
        assert_eq!(g.chests[&(901, 900)][CORN], 2, "กินของโปรดก่อน");
        assert_eq!(g.chests[&(901, 900)][BERRY], 3);
        // 50 + 3(เตียง) + 5(อิ่ม) + 4(ของโปรด) − 3(นอนกลางแจ้ง)
        assert_eq!(g.npcs[0].mood, 59);
        // เหลือแต่ของที่เกลียด → ยอมกินแต่ไม่สบอารมณ์
        g.chests.get_mut(&(901, 900)).unwrap().iter_mut().for_each(|v| *v = 0);
        g.chests.get_mut(&(901, 900)).unwrap()[BERRY] = 2;
        g.npcs[0].mood = 50;
        g.npc_dawn();
        assert_eq!(g.chests[&(901, 900)][BERRY], 1);
        assert_eq!(g.npcs[0].mood, 50 + 3 + 5 - 4 - 3);
    }

    #[test]
    fn sleeping_restores_rest_and_costs_food_but_never_kills() {
        let mut g = g0();
        g.p.rest = 10.; g.p.sick = 30.; g.p.hunger = 20.; g.p.thirst = 5.;
        g.sleep_needs(120.);
        assert_eq!(g.p.rest, 100.);
        assert_eq!(g.p.sick, 0.);
        assert!(g.p.hunger >= 3. && g.p.thirst >= 3., "ไม่ตายตอนหลับ");
        let mut h = g0();
        h.p.hunger = 90.; h.p.thirst = 90.;
        h.sleep_needs(120.);
        assert!(h.p.thirst < h.p.hunger, "น้ำลดมากกว่าอาหาร: {} {}", h.p.thirst, h.p.hunger);
        assert!(h.p.hunger > 80.);
    }

    #[test]
    fn needs_roundtrip_in_save_and_old_saves_start_full() {
        let mut g = g0();
        g.p.hunger = 41.5; g.p.thirst = 22.; g.p.rest = 63.; g.p.sick = 12.;
        let txt = g.save_string();
        assert!(txt.contains("need 41.5 22.0 63.0 12.0"));
        let l = Game::load_str(&txt, Sfx::default()).ok().unwrap();
        assert_eq!((l.p.hunger, l.p.thirst, l.p.rest, l.p.sick), (41.5, 22., 63., 12.));
        let old: String = txt.lines().filter(|x| !x.starts_with("need")).map(|x| format!("{}\n", x)).collect();
        let lo = Game::load_str(&old, Sfx::default()).ok().unwrap();
        assert_eq!((lo.p.hunger, lo.p.thirst, lo.p.rest), (100., 100., 100.));
    }

    #[test]
    fn rain_and_warning_toasts() {
        let mut g = g0();
        g.p.thirst = 20.; g.rain_on = true; g.rain = 1.;
        for _ in 0..10 { g.needs_tick(1., 1); }
        assert!(g.p.thirst > 25., "ฝนตกช่วยดับกระหาย {}", g.p.thirst);
        let mut w = g0();
        w.p.thirst = 30.2;
        w.needs_tick(1., 1);
        assert!(w.toasts.iter().any(|t| format!("{:?}", t).contains("กระหาย")), "ข้ามเกณฑ์ 30 ต้องเตือน");
    }

    #[test]
    fn well_is_buildable_solid_and_listed() {
        assert!(PIECES.iter().any(|p| p.obj == 39));
        assert!(OSOLID[39] && OHP[39] > 0);
        let mut g = g0();
        let t = (tfloor(g.p.x) + 3, tfloor(g.p.y));
        g.w.placed.insert(t, 39);
        g.p.x = t.0 as f32 * 16. + 8.; g.p.y = t.1 as f32 * 16. + 20.;
        assert_eq!(g.interact_target().map(|x| x.1), Some(39));
        g.p.thirst = 10.;
        g.interact();
        assert!(g.p.thirst > 50.);
    }
}

#[cfg(test)]
mod body_tests {
    use super::*;

    fn g0() -> Game { Game::new_world_seed(777, Sfx::default()) }

    #[test]
    fn generated_bodies_are_deterministic_and_in_range() {
        for seed in 0..LOOKS as u32 {
            for fem in [false, true] {
                let a = Body::generate(seed, fem);
                assert_eq!(a, Body::generate(seed, fem), "เมล็ดเดียวกันต้องได้คนเดิม");
                assert!((CM_MIN..=CM_MAX).contains(&(a.cm as i32)), "สูง {}", a.cm);
                assert!((KG_MIN..=KG_MAX).contains(&(a.kg as i32)), "หนัก {}", a.kg);
                assert!(a.bmi() >= 16.5 && a.bmi() <= 37., "BMI {}", a.bmi());
                assert!((18..=76).contains(&a.age));
                assert!((a.skin as usize) < SKIN_N && (a.hair as usize) < HAIR_N && (a.style as usize) < STYLE_N && (a.cloth as usize) < CLOTH_N);
            }
        }
    }

    #[test]
    fn population_is_varied_and_realistic() {
        let men: Vec<Body> = (0..LOOKS as u32).map(|s| Body::generate(s, false)).collect();
        let women: Vec<Body> = (0..LOOKS as u32).map(|s| Body::generate(s, true)).collect();
        let avg = |v: &Vec<Body>| v.iter().map(|b| b.cm as f32).sum::<f32>() / v.len() as f32;
        let (hm, hw) = (avg(&men), avg(&women));
        assert!(hm - hw > 8. && hm - hw < 20., "ผู้ชายสูงกว่าผู้หญิงเฉลี่ย {:.1} vs {:.1}", hm, hw);
        assert!((hm - 175.).abs() < 5. && (hw - 162.).abs() < 5.);
        let all: Vec<Body> = men.iter().chain(women.iter()).copied().collect();
        let distinct = |f: &dyn Fn(&Body) -> u8| { let mut s: Vec<u8> = all.iter().map(|b| f(b)).collect(); s.sort(); s.dedup(); s.len() };
        assert_eq!(distinct(&|b| b.skin), SKIN_N, "ครบทุกโทนผิว");
        assert!(distinct(&|b| b.hair) >= 7);
        assert_eq!(distinct(&|b| b.style), STYLE_N, "ครบทุกทรงผม");
        assert!(distinct(&|b| b.cloth) >= 9);
        let heights = { let mut h: Vec<u16> = all.iter().map(|b| b.cm).collect(); h.sort(); h };
        assert!(heights[heights.len() - 1] - heights[0] >= 40, "ส่วนสูงกระจายกว้าง");
        let kgs = { let mut h: Vec<u16> = all.iter().map(|b| b.kg).collect(); h.sort(); h };
        assert!(kgs[kgs.len() - 1] - kgs[0] >= 40, "น้ำหนักกระจายกว้าง");
        // ผู้หญิงผมยาว/หางม้ามากกว่าผู้ชาย · ผู้ชายล้านมากกว่า
        let long = |v: &Vec<Body>| v.iter().filter(|b| b.style == 2 || b.style == 3).count();
        assert!(long(&women) > long(&men) + 40);
        let bald = |v: &Vec<Body>| v.iter().filter(|b| b.style == 4).count();
        assert!(bald(&men) > 5 && bald(&women) == 0);
    }

    #[test]
    fn elders_go_grey_and_older_men_go_bald_more() {
        let all: Vec<Body> = (0..LOOKS as u32).flat_map(|s| [Body::generate(s, false), Body::generate(s, true)]).collect();
        let elders: Vec<&Body> = all.iter().filter(|b| b.age >= 60).collect();
        assert!(elders.len() > 10);
        assert!(elders.iter().all(|b| b.hair == 6 || b.hair == 7), "อายุ 60+ ผมหงอกทุกคน");
        let young: Vec<&Body> = all.iter().filter(|b| b.age < 30 && b.style != 4).collect();
        assert!(young.iter().all(|b| b.hair < 6), "คนหนุ่มสาวไม่ผมหงอก");
        let old_m: Vec<Body> = (0..LOOKS as u32).map(|s| Body::generate(s, false)).filter(|b| b.age >= 50).collect();
        let young_m: Vec<Body> = (0..LOOKS as u32).map(|s| Body::generate(s, false)).filter(|b| b.age < 30).collect();
        let rate = |v: &Vec<Body>| v.iter().filter(|b| b.style == 4).count() as f32 / v.len().max(1) as f32;
        assert!(rate(&old_m) > rate(&young_m));
    }

    #[test]
    fn dark_skin_mostly_dark_hair() {
        let all: Vec<Body> = (0..LOOKS as u32).flat_map(|s| [Body::generate(s, false), Body::generate(s, true)]).filter(|b| b.age < 45).collect();
        let dark: Vec<&Body> = all.iter().filter(|b| b.skin >= 5).collect();
        assert!(dark.len() > 20);
        assert!(dark.iter().all(|b| b.hair <= 2), "ผิวเข้มผมสีเข้มตามธรรมชาติ");
        assert!(all.iter().filter(|b| b.skin <= 1).any(|b| b.hair == 4 || b.hair == 5), "ผิวขาวมีผมบลอนด์/แดงได้");
    }

    #[test]
    fn children_mix_parents_skin_and_are_small() {
        let (mut a, mut b) = (Body::default(), Body::default());
        a.skin = 0; b.skin = 6;
        for id in 1..60u32 {
            for fem in [false, true] {
                let c = Body::child_of(id, fem, &a, &b);
                assert!((3 - 1..=3 + 1).contains(&(c.skin as i32)), "ผสมผิว 0 กับ 6 ได้ราว 3: {}", c.skin);
                assert!((5..=12).contains(&c.age) && c.is_child());
                assert!(c.cm >= 95 && c.cm <= 165, "{}", c.cm);
                assert!(c.hair < 6, "เด็กไม่ผมหงอก");
                assert!(c.scale().1 < 0.9, "เด็กตัวเล็กกว่าผู้ใหญ่"); if c.age <= 7 { assert!(c.scale().1 < 0.72, "เด็กเล็กตัวเล็กมาก"); }
            }
        }
        let same = Body::child_of(7, true, &a, &a);
        assert!(same.skin <= 1, "พ่อแม่ผิวเดียวกัน ลูกใกล้เคียง");
        assert_eq!(Body::child_of(9, true, &a, &b), Body::child_of(9, true, &a, &b));
    }

    #[test]
    fn scale_follows_height_and_weight() {
        let d = Body::default();
        let (bw, bh) = d.scale();
        assert!((bh - 1.).abs() < 1e-6 && (bw - 1.).abs() < 0.03, "{} {}", bw, bh);
        let tall = Body { cm: 195, ..d };
        let short = Body { cm: 150, ..d };
        assert!(tall.scale().1 > 1.1 && short.scale().1 < 0.9);
        let heavy = Body { kg: 120, ..d };
        let thin = Body { kg: 50, ..d };
        assert!(heavy.scale().0 > d.scale().0 + 0.1 && thin.scale().0 < d.scale().0 - 0.05);
        assert_eq!(heavy.scale().1, d.scale().1, "น้ำหนักไม่เปลี่ยนความสูง");
    }

    #[test]
    fn body_affects_hunger_and_speed_mildly() {
        let d = Body::default();
        assert_eq!(d.speed_mul(), 1.0, "ค่าเริ่มต้นต้องไม่เปลี่ยนความเร็วเดิม");
        assert!((d.hunger_mul() - 1.0).abs() < 1e-6);
        let big = Body { cm: 190, kg: 110, ..d };
        let small = Body { cm: 155, kg: 48, ..d };
        assert!(big.hunger_mul() > 1.15 && small.hunger_mul() < 0.9);
        assert!(big.hunger_mul() <= 1.25 && small.hunger_mul() >= 0.85);
        let obese = Body { cm: 170, kg: 130, ..d };
        assert!(obese.speed_mul() < 0.96 && obese.speed_mul() >= 0.92);
        assert!(Body { cm: 195, kg: 80, ..d }.speed_mul() > 1.01);
        assert!(Body { cm: 150, kg: 50, ..d }.speed_mul() < 1.0);
        let mut p = Player::new(0., 0.);
        let base = p.speed_mul();
        p.body = obese;
        assert!(p.speed_mul() < base);
        // หิวเร็วต่างกันจริง
        let (mut a, mut b) = (g0(), g0());
        a.p.body = big; b.p.body = small;
        for _ in 0..120 { a.needs_tick(1., 1); b.needs_tick(1., 1); }
        assert!(a.p.hunger < b.p.hunger - 2., "ร่างใหญ่หิวเร็วกว่า {} vs {}", a.p.hunger, b.p.hunger);
        assert!((a.p.thirst - b.p.thirst).abs() < 1e-3, "ความกระหายไม่ขึ้นกับรูปร่าง");
    }

    #[test]
    fn body_saves_and_old_saves_default() {
        let mut g = g0();
        g.p.body = Body { skin: 6, hair: 7, style: 3, cloth: 8, cm: 181, kg: 92, age: 25, eth: 7, mus: 70, fem: true };
        let txt = g.save_string();
        assert!(txt.contains("body 6 7 3 8 181 92 25"));
        let l = Game::load_str(&txt, Sfx::default()).ok().unwrap();
        assert_eq!(l.p.body, g.p.body);
        let old: String = txt.lines().filter(|x| !x.starts_with("body")).map(|x| format!("{}\n", x)).collect();
        assert_eq!(Game::load_str(&old, Sfx::default()).ok().unwrap().p.body, Body::default());
        // ค่าเพี้ยนในเซฟถูกบีบกลับเข้าช่วง
        let bad = txt.replace("body 6 7 3 8 181 92 25", "body 99 99 99 99 9999 1 25");
        let b = Game::load_str(&bad, Sfx::default()).ok().unwrap().p.body;
        assert!((b.skin as usize) < SKIN_N && (b.style as usize) < STYLE_N && b.cm as i32 <= CM_MAX && b.kg as i32 >= KG_MIN);
    }

    #[test]
    fn npc_body_is_stable_and_child_inherits_from_player_and_spouse() {
        let mut g = g0();
        let mk = |id: u32, name: u8, look: u8, child: bool, married: bool| npc::Npc { id, name, look, x: 0., y: 0., state: npc::ST_HOME, hearts: 3, mood: 70, married, child, role: 0, bed: (0, 0), anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, name) };
        g.npcs.push(mk(1, 0, 40, false, true));
        g.npcs.push(mk(2, 5, 41, true, false));
        let a = g.npc_body(&g.npcs[0]);
        assert_eq!(a, g.npc_body(&g.npcs[0]));
        assert_eq!(a, Body::generate(40, NPC_FEM[0]), "เพศตามชื่อ");
        g.p.body.skin = 0;
        let c = g.npc_body(&g.npcs[1]);
        let mix = (0 + a.skin as i32 + 1) / 2;
        assert!((c.skin as i32 - mix).abs() <= 1, "ลูกผสมผิวพ่อแม่: พ่อแม่ 0,{} ลูก {}", a.skin, c.skin);
        assert!(c.is_child());
        // เปลี่ยนคู่สมรส ลูกยังอยู่ในช่วงที่ผสมได้ (ไม่ crash เมื่อไม่มีคู่)
        g.npcs[0].married = false;
        let c2 = g.npc_body(&g.npcs[1]);
        assert!(c2.is_child());
    }

    #[test]
    fn hair_styles_keep_sprite_grid_and_change_silhouette() {
        let pd = ["...hhhhhh...", "..hhhhhhhh..", "..hhhhhhhh..", "..hssssssh..", "..sesssses..", "..ssssssss..", "...ssmmss..."];
        let pu = ["...hhhhhh...", "..hhhhhhhh..", "..hhhhhhhh..", "..hhhhhhhh..", "..hhhhhhhh..", "..shhhhhhs..", "...ssssss..."];
        let pr = ["...hhhhhh...", "..hhhhhhhh..", "..hhhhhhhh..", "..hhhsssss..", "..hhhssses..", "..hhssssss..", "...sssss...."];
        let heads = [&pd, &pu, &pr];
        let count = |v: &Vec<String>, c: char| v.iter().map(|r| r.chars().filter(|&x| x == c).count()).sum::<usize>();
        for (dir, h) in heads.iter().enumerate() {
            let base = skin::hair_style(&h[..], dir, 0);
            for st in 0..STYLE_N {
                let v = skin::hair_style(&h[..], dir, st);
                assert_eq!(v.len(), 7);
                assert!(v.iter().all(|r| r.chars().count() == 12), "ทรง {} มุม {}: {:?}", st, dir, v);
            }
            assert_eq!(count(&skin::hair_style(&h[..], dir, 4), 'h'), 0, "ล้านไม่มีผมเลย");
            assert!(count(&skin::hair_style(&h[..], dir, 1), 'h') <= count(&base, 'h'), "เกรียนสั้นกว่า");
        }
        assert!(count(&skin::hair_style(&pd, 0, 2), 'h') > count(&skin::hair_style(&pd, 0, 0), 'h'), "ผมยาวมีผมมากกว่า");
        assert!(count(&skin::hair_style(&pu, 1, 2), 'h') > count(&skin::hair_style(&pu, 1, 0), 'h'));
        assert_ne!(skin::hair_style(&pu, 1, 3), skin::hair_style(&pu, 1, 0), "หางม้าเห็นด้านหลัง");
        assert_eq!(skin::hair_style(&pd, 0, 3), skin::hair_style(&pd, 0, 0), "หางม้าด้านหน้าเหมือนผมสั้น");
    }

    #[test]
    fn layers_split_pixels_without_overlap() {
        let mut at = Atlas { img: Image::gen_image_color(64, 64, BLANK), cx: 0, cy: 0, rowh: 0 };
        let pal: &[(char, u32)] = &[('h', 0x111111), ('s', 0x222222), ('e', 0x333333), ('r', 0x444444), ('R', 0x555555), ('b', 0x666666)];
        let rows: Vec<String> = vec!["hsesr".into(), "RbRrh".into()];
        let p = skin::split_layers(&mut at, &rows, pal);
        assert!(p.skin.is_some() && p.hair.is_some() && p.cloth.is_some() && p.hi.is_some() && p.base.is_some());
        let pixels = |r: Rect| -> usize {
            let mut n = 0;
            for y in 0..r.h as u32 { for x in 0..r.w as u32 { if at.img.get_pixel(r.x as u32 + x, r.y as u32 + y).a > 0. { n += 1; } } }
            n
        };
        let total: usize = [p.base, p.skin, p.hair, p.cloth, p.hi].iter().map(|r| pixels(r.unwrap())).sum();
        assert_eq!(total, 10, "ทุกพิกเซลอยู่ในชั้นเดียวเท่านั้น");
        assert_eq!(pixels(p.skin.unwrap()), 2);
        assert_eq!(pixels(p.hair.unwrap()), 2);
        let none = skin::split_layers(&mut at, &["bb".to_string()], pal);
        assert!(none.skin.is_none() && none.hair.is_none() && none.cloth.is_none() && none.base.is_some());
    }

    #[test]
    fn role_outfits_cover_all_four_roles() {
        assert_eq!(skin::ROLE_PARTS.len(), 4);
        assert_eq!(skin::ROLE_PARTS[0], [0, 0, 0]);
        assert!(skin::ROLE_PARTS.iter().all(|p| p.iter().all(|&v| v < skin::NV)));
    }
}

#[cfg(test)]
mod tree_tests {
    use super::*;

    fn g0() -> Game { Game::new_world_seed(777, Sfx::default()) }

    #[test]
    fn species_tables_are_complete_and_biome_appropriate() {
        for tile in [3u8, 4, 5, 7, 8] {
            let mut seen = std::collections::BTreeSet::new();
            for i in 0..1000 { seen.insert(pick_species(tile, i as f64 / 1000.)); }
            assert!(seen.iter().all(|&o| is_tree(o)), "ไบโอม {} ได้วัตถุที่ไม่ใช่ต้นไม้ {:?}", tile, seen);
            match tile {
                3 => { assert!(seen.len() >= 5, "ทุ่งหญ้าต้องหลากหลาย {:?}", seen); assert!(!seen.contains(&T_SPRUCE) && !seen.contains(&T_PALM) && !seen.contains(&T_DEAD)); }
                4 => { assert!(seen.len() >= 6 && seen.contains(&T_SPRUCE) && seen.contains(&T_OLD) && seen.contains(&T_MAPLE)); assert!(!seen.contains(&T_PALM)); }
                5 => assert_eq!(seen, [T_SPRUCE, T_DEAD].into_iter().collect()),
                7 => assert_eq!(seen, [8, T_SPRUCE].into_iter().collect()),
                _ => assert_eq!(seen, [T_DEAD].into_iter().collect()),
            }
        }
        assert_eq!(pick_species(1, 0.5), 1, "ไทล์อื่นๆ ตกกลับเป็นโอ๊ก");
        let share = |tile: u8, id: u8| (0..1000).filter(|&i| pick_species(tile, i as f64 / 1000.) == id).count();
        assert!(share(4, T_SPRUCE) > 200 && share(3, T_APPLE) < 120, "ป่าเต็มไปด้วยสปรูซ · ต้นแอปเปิลหายากในทุ่ง");
    }

    #[test]
    fn world_has_varied_trees_per_biome() {
        let g = g0();
        let mut by_tile: HashMap<u8, std::collections::BTreeMap<u8, i32>> = HashMap::new();
        for y in -220..220 { for x in -220..220 {
            let o = g.w.obj_at(x, y);
            if is_tree(o) { *by_tile.entry(g.w.tile_at(x, y)).or_default().entry(o).or_insert(0) += 1; }
        } }
        let kinds = |t: u8| by_tile.get(&t).map_or(0, |m| m.len());
        assert!(kinds(4) >= 6, "ป่าทึบหลากหลาย: {:?}", by_tile.get(&4));
        assert!(kinds(3) >= 5, "ทุ่งหญ้า: {:?}", by_tile.get(&3));
        assert!(by_tile.get(&2).map_or(false, |m| m.contains_key(&T_PALM)), "ชายหาดมีมะพร้าว");
        assert!(by_tile.get(&8).map_or(false, |m| m.contains_key(&T_DEAD)), "ทะเลทรายมีต้นไม้ตาย");
        assert!(by_tile.keys().all(|&t| matches!(t, 2 | 3 | 4 | 5 | 7 | 8 | 15 | 16 | 17)), "ต้นไม้ขึ้นบนน้ำ/ภูเขาไม่ได้ {:?}", by_tile.keys());
        // ในบรรดาต้นไม้ของป่า โอ๊กยังเป็นส่วนใหญ่ไม่ได้ทั้งหมด
        let f = by_tile.get(&4).unwrap();
        let total: i32 = f.values().sum();
        assert!(*f.get(&1).unwrap() * 100 / total < 40);
        // มะพร้าวในทะเลทรายอยู่ติดน้ำเท่านั้น
        for y in -300..300 { for x in -300..300 {
            if g.w.tile_at(x, y) == 8 && g.w.obj_at(x, y) == T_PALM { assert!(g.w.water_near2(x, y), "มะพร้าวกลางทะเลทรายต้องติดโอเอซิส ({}, {})", x, y); }
        } }
    }

    #[test]
    fn tree_world_is_deterministic() {
        let (a, b) = (g0(), g0());
        for y in -60..60 { for x in -60..60 { assert_eq!(a.w.obj_at(x, y), b.w.obj_at(x, y)); } }
    }

    #[test]
    fn tree_stats_tables_and_sprites_are_consistent() {
        for o in 40..=47usize { assert!(OSOLID[o] && OHP[o] > 0, "ต้นไม้ {} ต้องชนได้และตีได้", o); }
        assert!(OHP[T_OLD as usize] > OHP[T_BIRCH as usize] && OHP[T_SAPLING as usize] == 1);
        assert_eq!(OSOLID.len(), 86);
        for o in 40..=47u8 { let (a, b) = wood_range(o); assert!(a >= 1 && b >= a, "{}", o); assert!(!tree_name(o).is_empty()); }
        assert!(wood_range(T_OLD).0 >= 5 && wood_range(T_SAPLING).1 == 1 && wood_range(T_SPRUCE).1 >= 4);
        // สไปรต์ทุกชิ้นกว้าง 16 พิกเซล วางเข้าแอตลาสได้ ไม่ซ้ำกัน
        let mut at = Atlas { img: Image::gen_image_color(512, 1024, BLANK), cx: 0, cy: 0, rowh: 0 };
        let mut obj = vec![Rect::new(0., 0., 1., 1.); 86];
        trees::build_trees(&mut at, &mut obj);
        for o in 40..=49usize { assert!(obj[o].w == 16. && obj[o].h >= 14. && obj[o].h <= 22., "{} {:?}", o, obj[o]); }
        let mut pos: Vec<(i32, i32)> = (40..=49).map(|o| (obj[o].x as i32, obj[o].y as i32)).collect();
        pos.sort(); pos.dedup();
        assert_eq!(pos.len(), 10);
        let px = |r: Rect| (0..r.h as u32).flat_map(|y| (0..r.w as u32).map(move |x| (x, y))).filter(|&(x, y)| at.img.get_pixel(r.x as u32 + x, r.y as u32 + y).a > 0.).count();
        assert!(px(obj[T_OLD as usize]) > px(obj[T_SAPLING as usize]) * 5, "ต้นเก่าแก่ใหญ่กว่าต้นกล้ามาก");
        assert!(px(obj[T_APPLE as usize]) == px(obj[T_APPLE_BARE as usize]), "ต้นไม่มีผลเป็นรูปทรงเดียวกัน");
    }

    fn put(g: &mut Game, o: u8) -> (i32, i32) {
        let k = (tfloor(g.p.x) + 1, tfloor(g.p.y));
        g.w.removed.remove(&k);
        g.w.placed.insert(k, o);
        g.p.x = k.0 as f32 * 16. + 8. - 14.; g.p.y = k.1 as f32 * 16. + 12.;
        g.p.dir = 2; g.p.flip = false;
        k
    }
    fn chop(g: &mut Game, k: (i32, i32)) { for _ in 0..12 { if g.w.obj_at(k.0, k.1) == 0 { break; } g.hit_object(k.0, k.1, 1); } }

    #[test]
    fn chopping_each_species_gives_wood_scaled_to_tree_size() {
        for o in [T_BIRCH, T_SPRUCE, T_DEAD, T_MAPLE, T_OLD, T_SAPLING, T_PALM, T_APPLE] {
            let mut g = g0();
            g.p.x = 200.; g.p.y = 200.;
            let k = put(&mut g, o);
            let before = g.p.inv[WOOD];
            let hits = (0..20).take_while(|_| { g.hit_object(k.0, k.1, 1); g.w.obj_at(k.0, k.1) != 0 }).count() + 1;
            assert_eq!(hits as i32, OHP[o as usize], "{} ต้องตี {} ครั้ง", tree_name(o), OHP[o as usize]);
            let got = g.p.inv[WOOD] - before;
            let (a, b) = wood_range(o);
            assert!(got >= a && got <= b, "{}: ได้ไม้ {} (ช่วง {}..{})", tree_name(o), got, a, b);
        }
    }

    #[test]
    fn apple_tree_gives_fruit_without_felling_and_regrows() {
        let mut g = g0();
        g.p.x = 200.; g.p.y = 200.;
        let k = put(&mut g, T_APPLE);
        assert!(g.fruit_ready(k));
        assert_eq!(g.display_obj(T_APPLE, k), T_APPLE);
        assert_eq!(g.interact_target().map(|t| t.1), Some(T_APPLE));
        assert_eq!(g.use_label().as_deref(), Some("เก็บผล"));
        let b0 = g.p.inv[BERRY];
        assert!(g.pick_fruit(k, T_APPLE));
        let got = g.p.inv[BERRY] - b0;
        assert!((2..=4).contains(&got), "{}", got);
        assert_eq!(g.w.obj_at(k.0, k.1), T_APPLE, "ต้นยังอยู่");
        assert!(!g.fruit_ready(k));
        assert_eq!(g.display_obj(T_APPLE, k), T_APPLE_BARE);
        assert!(g.interact_target().is_none(), "ผลยังไม่สุก ไม่ขึ้นปุ่มใช้");
        assert!(!g.pick_fruit(k, T_APPLE));
        assert_eq!(g.p.inv[BERRY] - b0, got);
        g.time += DAYLEN * 1.1;
        assert!(!g.fruit_ready(k), "ผ่านไป 1 วัน ยังไม่สุก");
        g.time += DAYLEN * 1.1;
        assert!(g.fruit_ready(k) && g.pick_fruit(k, T_APPLE));
        // โค่นตอนผลยังสุก → ได้ผลไม้เพิ่ม · โค่นตอนยังไม่สุก → ได้แต่ไม้
        g.time += DAYLEN * 3.;
        let (w1, b1) = (g.p.inv[WOOD], g.p.inv[BERRY]);
        chop(&mut g, k);
        assert!(g.p.inv[WOOD] > w1 && g.p.inv[BERRY] > b1, "โค่นต้นที่มีผลได้ทั้งไม้และผลไม้");
        assert!(!g.fruit.contains_key(&k));
        let mut h = g0();
        h.p.x = 200.; h.p.y = 200.;
        let k2 = put(&mut h, T_APPLE);
        h.pick_fruit(k2, T_APPLE);
        let b2 = h.p.inv[BERRY];
        chop(&mut h, k2);
        assert_eq!(h.p.inv[BERRY], b2, "ไม่มีผลก็ไม่ได้ผลไม้");
    }

    #[test]
    fn coconut_palm_is_clean_drinking_water_and_does_not_work_when_full() {
        let mut g = g0();
        g.p.x = 200.; g.p.y = 200.;
        let k = put(&mut g, T_PALM);
        g.p.thirst = 20.; g.p.hunger = 50.;
        assert_eq!(g.use_label().as_deref(), Some("ดื่มมะพร้าว"));
        assert!(g.pick_fruit(k, T_PALM));
        assert!((g.p.thirst - 48.).abs() < 0.01 && (g.p.hunger - 56.).abs() < 0.01 && g.p.sick == 0.);
        assert_eq!(g.display_obj(T_PALM, k), T_PALM_BARE);
        assert_eq!(g.w.obj_at(k.0, k.1), T_PALM);
        let mut f = g0();
        f.p.x = 200.; f.p.y = 200.;
        let k = put(&mut f, T_PALM);
        f.p.thirst = 100.; f.p.hunger = 100.;
        assert!(!f.pick_fruit(k, T_PALM) && f.fruit_ready(k), "อิ่มและไม่กระหาย ไม่เปลืองผล");
        assert!(!f.pick_fruit(k, T_SPRUCE), "ต้นอื่นไม่มีผล");
    }

    #[test]
    fn fruit_timers_save_and_old_saves_load() {
        let mut g = g0();
        g.fruit.insert((5, -7), 3);
        g.fruit.insert((12, 40), 1);
        let txt = g.save_string();
        assert!(txt.contains("fruit 12,40,1 5,-7,3") || txt.contains("fruit 5,-7,3 12,40,1"));
        let l = Game::load_str(&txt, Sfx::default()).ok().unwrap();
        assert_eq!(l.fruit, g.fruit);
        let old: String = txt.lines().filter(|x| !x.starts_with("fruit")).map(|x| format!("{}\n", x)).collect();
        assert!(Game::load_str(&old, Sfx::default()).ok().unwrap().fruit.is_empty());
        let bad = txt.replace("fruit 12,40,1 5,-7,3", "fruit 1,2 x,y,z 3,4,5").replace("fruit 5,-7,3 12,40,1", "fruit 1,2 x,y,z 3,4,5");
        assert_eq!(Game::load_str(&bad, Sfx::default()).ok().unwrap().fruit.len(), 1);
    }

    #[test]
    fn tutorial_arrow_accepts_every_wood_tree() {
        let mut g = g0();
        g.tut = 0;
        // หาจุดที่ไม่มีต้นไม้สายไม้เลยในรัศมี 18 ช่อง (เช่นกลางทะเล) แล้ววางเบิร์ชไว้ต้นเดียว
        let wood_tree = |o: u8| matches!(o, 1 | 8 | 40 | 41 | 42 | 43 | 45 | 46);
        let spot = (-60..60).flat_map(|y| (-60..60).map(move |x| (x * 5, y * 5))).find(|&(x, y)| (-18..=18).all(|d| (-18..=18).all(|e| !wood_tree(g.w.obj_at(x + d, y + e)))));
        let (x, y) = spot.expect("ต้องมีที่ว่างจากต้นไม้ในโลกทดสอบ");
        g.p.x = x as f32 * 16. + 8.; g.p.y = y as f32 * 16. + 8.;
        assert!(g.tut_target().is_none());
        g.w.placed.insert((x + 5, y), T_BIRCH);
        let t = g.tut_target().expect("ลูกศรบทเรียนต้องชี้ต้นเบิร์ชด้วย");
        assert_eq!(t, ((x + 5) as f32 * 16. + 8., y as f32 * 16. + 8.));
    }
}

#[cfg(test)]
mod land_tests {
    use super::*;

    fn find_volcano(g: &Game) -> Option<(i32, i32)> {
        for cy in -3..=3 { for cx in -3..=3 { if let Some(v) = g.w.volcano_in_cell(cx, cy) { return Some(v); } } }
        None
    }

    #[test]
    fn new_biomes_appear_in_world() {
        let g = Game::new_world_seed(777, Sfx::default());
        let mut cnt = [0; 23];
        for y in (-500..500).step_by(3) { for x in (-500..500).step_by(3) { cnt[g.w.tile_at(x, y) as usize] += 1; } }
        assert!(cnt[T_SWAMP as usize] > 50, "หนองน้ำ {:?}", cnt);
        assert!(cnt[T_SAVANNA as usize] > 50, "สะวันนา {:?}", cnt);
        assert!(cnt[3] > cnt[T_SAVANNA as usize] && cnt[3] > cnt[T_SWAMP as usize], "ทุ่งหญ้ายังเป็นหลัก {:?}", cnt);
    }

    #[test]
    fn volcano_has_lava_core_rim_and_ash_and_no_sea() {
        let mut found = 0;
        for seed in 1..12 {
            let g = Game::new_world_seed(seed, Sfx::default());
            let Some((vx, vy)) = find_volcano(&g) else { continue };
            found += 1;
            assert_eq!(g.w.tile_at(vx, vy), T_LAVA, "ปล่องกลางเป็นลาวา");
            assert!(g.w.solid_at(vx, vy), "ลาวาเดินทับไม่ได้");
            let mut ash = 0;
            let mut lava = 0;
            for dy in -VR..=VR { for dx in -VR..=VR {
                let t = g.w.tile_at(vx + dx, vy + dy);
                if t == T_ASH { ash += 1; }
                if t == T_LAVA { lava += 1; }
                if dx * dx + dy * dy <= 100 { assert!(t >= 2, "ไม่มีทะเลในเขตภูเขาไฟ ({},{})={}", dx, dy, t); }
            } }
            assert!(ash > 120 && lava >= 12, "เถ้า {} ลาวา {}", ash, lava);
            // มีหินออบซิเดียนบนเถ้า
            let obs = (-VR..=VR).flat_map(|dy| (-VR..=VR).map(move |dx| (dx, dy))).filter(|&(dx, dy)| g.w.obj_at(vx + dx, vy + dy) == O_OBSIDIAN).count();
            assert!(obs >= 1, "ต้องมีหินออบซิเดียน");
        }
        assert!(found >= 2, "ภูเขาไฟต้องพบได้ในหลายโลก ({})", found);
    }

    fn all_marks(g: &Game) -> Vec<(i32, i32, u8)> {
        let mut v = vec![];
        for cy in -6..6 { for cx in -6..6 { if let Some(m) = g.w.mark_in_cell(cx, cy) { v.push(m); } } }
        v
    }

    #[test]
    fn landmarks_fit_biome_and_have_walkable_layout() {
        let g = Game::new_world_seed(777, Sfx::default());
        let ms = all_marks(&g);
        let kinds: std::collections::BTreeSet<u8> = ms.iter().map(|m| m.2).collect();
        assert!(ms.len() >= 10 && kinds.len() >= 3, "จุดสำคัญ {} ชนิด {:?}", ms.len(), kinds);
        for &(x, y, k) in &ms {
            let t = g.w.tile_at(x, y);
            match k {
                MK_GIANT => { assert_eq!(t, 4, "ต้นไม้ยักษ์อยู่ในป่า"); assert_eq!(g.w.obj_at(x, y), O_GIANT); assert!(g.w.solid_at(x - 1, y) && g.w.solid_at(x + 1, y)); }
                MK_RUINS => { assert!(matches!(t, 3 | 4 | 8 | 15 | 16)); assert_eq!(g.w.obj_at(x, y), O_ACHEST); assert_eq!(g.w.obj_at(x, y + 4), 0, "มีทางเข้าซาก"); }
                MK_CIRCLE => { assert!(matches!(t, 3 | 5 | 7 | 15 | 16)); assert_eq!(g.w.obj_at(x, y), O_ALTAR); assert_eq!(g.w.obj_at(x, y - 3), O_PILLAR); }
                _ => panic!("ชนิดผิด"),
            }
            for dy in -4..=4 { for dx in -4..=4 { assert!(!SOLIDT[g.w.tile_at(x + dx, y + dy) as usize]); } }
            // ไม่มีต้นไม้ธรรมชาติขึ้นทับลาน
            assert!((-1..=1).all(|d| !is_tree(g.w.obj_at(x + d, y + 2))));
        }
    }

    #[test]
    fn camps_vary_by_biome_and_merchant_stays_reachable() {
        let mut kinds = std::collections::BTreeSet::new();
        for seed in [777, 3, 9] {
            let g = Game::new_world_seed(seed, Sfx::default());
            for cy in -8..8 { for cx in -8..8 {
                let Some(c) = g.w.camp_in_cell(cx, cy) else { continue };
                let k = g.w.camp_kind(c);
                kinds.insert(k);
                let t = g.w.tile_at(c.0, c.1);
                if t == 7 { assert_eq!(k, 3); }
                if t == 8 { assert_eq!(k, 2); }
                assert_eq!(g.w.obj_at(c.0, c.1), 15);
                assert!(!g.w.solid_at(c.0, c.1 + 1), "หน้าพ่อค้าต้องเดินได้");
                if k > 0 {
                    assert!(matches!(g.w.obj_at(c.0 - 2, c.1 - 1), O_HUT_FISH | O_HUT_SAND | O_HUT_LOG));
                    assert!(g.w.solid_at(c.0 - 3, c.1 - 1) && g.w.solid_at(c.0 - 1, c.1 - 1), "บ้านกว้าง 3 ช่องชนได้ทั้งหลัง");
                }
                assert_eq!(camp_offers(k).len(), OFFERS.len() + if k > 0 { 2 } else { 0 });
            } }
        }
        assert!(kinds.len() >= 3, "แคมป์ต้องมีหลายแบบ {:?}", kinds);
    }

    fn walk_spot(g: &Game) -> (i32, i32) {
        for r in 0..80 { for dx in -r..=r { for dy in [-r, r] {
            let (x, y) = (dx, dy);
            if g.w.tile_at(x, y) == 3 && !g.w.solid_at(x, y) && !g.w.solid_at(x, y + 1) { return (x, y); }
        } } }
        panic!("ไม่มีทุ่งหญ้า")
    }

    #[test]
    fn shrine_and_giant_tree_once_per_day() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        g.time = DAYLEN * 1.3;
        g.p.hp = 1;
        g.p.rest = 10.;
        g.p.sick = 30.;
        assert!(g.shrine_pray((5, 5)));
        assert_eq!(g.p.hp, g.p.maxhp);
        assert!(g.p.sick == 0. && g.p.rest >= 59.);
        g.p.hp = 1;
        assert!(!g.shrine_pray((5, 5)), "วันเดียวกันขอซ้ำไม่ได้");
        assert_eq!(g.p.hp, 1);
        let h0 = g.p.inv[HERB];
        assert!(g.giant_gather((9, 9)));
        assert!(!g.giant_gather((9, 9)));
        g.time += DAYLEN;
        assert!(g.shrine_pray((5, 5)) && g.giant_gather((9, 9)), "วันใหม่ใช้ได้อีก");
        let picked: i32 = g.drops.iter().filter(|d| d.item == HERB).map(|d| d.n).sum::<i32>() + g.p.inv[HERB] - h0;
        assert!(picked >= 2, "ได้สมุนไพร {}", picked);
        let s = g.save_string();
        let mut h = Game::new_world_seed(777, Sfx::default());
        for line in s.lines() {
            let mut it = line.split_whitespace();
            if let Some(tag) = it.next() { if tag == "bless" || tag == "marks" { let rest: Vec<&str> = it.collect(); h.marks_load_line(tag, &rest); } }
        }
        assert_eq!(h.blessed, g.blessed);
    }

    #[test]
    fn swamp_slows_and_lava_burns() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        let (x, y) = walk_spot(&g);
        g.p.x = x as f32 * 16. + 8.;
        g.p.y = y as f32 * 16. + 12.;
        assert_eq!(g.ground_speed_mul(), 1.);
        g.w.tmod.insert((x, y), T_SWAMP);
        assert!(g.ground_speed_mul() < 0.8);
        g.w.tmod.insert((x, y), 3);
        let hp = g.p.hp;
        g.w.tmod.insert((x + 1, y), T_LAVA);
        g.p.invul = 0.;
        g.land_tick(1.);
        assert!(g.p.hp < hp, "ยืนติดลาวาต้องเสียเลือด");
        assert!(g.p.kbx < 0., "กระเด็นออกจากลาวา");
    }

    #[test]
    fn discovering_ruins_records_and_wakes_guards() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        let m = all_marks(&g).into_iter().find(|m| m.2 == MK_RUINS).expect("ต้องมีซากปรักหักพัง");
        g.p.x = m.0 as f32 * 16. + 8.;
        g.p.y = (m.1 + 6) as f32 * 16. + 12.;
        g.en.clear();
        g.discover_marks();
        assert_eq!(g.marks_found.get(&(m.0, m.1)), Some(&MK_RUINS));
        assert!(g.en.iter().filter(|e| e.k == EK::Skel).count() >= 2, "ผู้พิทักษ์ตื่น");
        let n = g.en.len();
        g.discover_marks();
        assert_eq!(g.en.len(), n, "ค้นพบซ้ำไม่เกิดมอนสเตอร์เพิ่ม");
        // หีบโบราณให้ของดี
        let (c0, g0) = (g.p.inv[COIN], g.p.inv[GEM]);
        g.drops.clear();
        assert!(g.hit_object(m.0, m.1, 5));
        let got = |it: usize| g.drops.iter().filter(|d| d.item == it).map(|d| d.n).sum::<i32>();
        assert!(got(COIN) + g.p.inv[COIN] - c0 >= 20 && got(GEM) + g.p.inv[GEM] - g0 >= 2);
        assert_eq!(g.w.obj_at(m.0, m.1), 0, "หีบเปิดแล้วหายไป");
    }

    #[test]
    fn special_caves_have_crystals_or_lava_and_boss_is_reachable() {
        let op = HashSet::new();
        for (cx, cy) in [(1, 2), (3, -1), (-2, 5)] {
            let n = Dungeon::generate_cave(777, (cx, cy), false, &op, 0);
            let c = Dungeon::generate_cave(777, (cx, cy), false, &op, 1);
            let l = Dungeon::generate_cave(777, (cx, cy), false, &op, 2);
            let count = |d: &Dungeon, f: &dyn Fn(&Dungeon, i32, i32) -> bool| (0..DH).flat_map(|y| (0..DW).map(move |x| (x, y))).filter(|&(x, y)| f(d, x, y)).count();
            let gems = |d: &Dungeon| count(d, &|d, x, y| matches!(d.obj(x, y), 5 | O_CRYSTAL));
            assert!(count(&c, &|d, x, y| d.obj(x, y) == O_CRYSTAL) >= 5, "ถ้ำคริสตัลมีคริสตัล");
            assert!(gems(&c) > gems(&n), "ถ้ำคริสตัลมีแร่มากกว่า");
            assert_eq!(count(&n, &|d, x, y| d.tile(x, y) == T_LAVA), 0);
            assert!(count(&l, &|d, x, y| d.tile(x, y) == T_LAVA) >= 20, "ถ้ำลาวามีลาวา");
            for d in [&c, &l] {
                let dist = dun_dist(d, d.entry, false);
                assert!(dist[(d.boss_at.1 * DW + d.boss_at.0) as usize] >= 0, "ไปถึงห้องบอสได้");
            }
        }
    }

    #[test]
    fn land_sprites_fit_atlas() {
        let mut at = Atlas { img: Image::gen_image_color(512, 1024, BLANK), cx: 0, cy: 0, rowh: 0 };
        let mut obj = vec![Rect::new(0., 0., 0., 0.); 86];
        land::build_land(&mut at, &mut obj);
        for o in 50..70usize { assert!(obj[o].w >= 1. && obj[o].h >= 1., "{}", o); }
        assert!(obj[O_GIANT as usize].w > 40. && obj[O_HUT_LOG as usize].w == 40.);
        for o in 50..70usize { if o != O_FILL as usize { assert!(OSOLID[o] || matches!(o as u8, O_REED | O_SHROOM | O_DRYGRASS | O_BONES)); } }
        for o in [O_ALTAR, O_GIANT, O_FILL, O_HUT_FISH] { assert!(land_fixed(o)); }
    }
}

#[cfg(test)]
mod season_tests {
    use super::*;

    fn at_day(g: &mut Game, d: i32) { g.time = DAYLEN * (d as f32 + 0.3); }
    fn dawn_to(g: &mut Game, d: i32) { while day_of(g) < d { g.time += DAYLEN; g.season_dawn(); } }

    #[test]
    fn year_cycle_starts_in_summer() {
        let names: Vec<u8> = [0, 4, 5, 9, 10, 14, 15, 19, 20].iter().map(|&d| season_of_day(d)).collect();
        assert_eq!(names, vec![1, 1, 2, 2, 3, 3, 0, 0, 1]);
        assert_eq!(SEASON_N[season_of_day(0) as usize], "ฤดูร้อน");
    }

    #[test]
    fn seasons_change_climate_crops_and_weather() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        at_day(&mut g, 0);
        assert!(g.season_climate().0 > 1., "ร้อน = กระหายเร็ว");
        at_day(&mut g, 11);
        assert!(g.season_climate().1 > 1., "หนาว = หิวเร็ว");
        assert!(g.crop_season_mul() < 0.5, "หนาว พืชแทบไม่โต");
        assert_eq!(g.season_weather(1, 3), 2, "หนาว ทุ่งหญ้าหิมะตก");
        assert_eq!(g.season_weather(1, 2), 1, "ชายหาดยังเป็นฝน");
        assert_eq!(g.season_weather(0, 8), 0, "ทะเลทรายไม่มีฝน/หิมะ");
        at_day(&mut g, 16);
        assert!(g.crop_season_mul() > 1.);
        assert!(g.rain_len_mul().1 < 1., "ใบไม้ผลิฝนบ่อย");
        at_day(&mut g, 1);
        assert!(g.rain_len_mul().1 > 1., "ร้อนฝนน้อย");
        assert_eq!(g.season_weather(1, 3), 1);
    }

    fn find_tree(g: &Game, sp: u8) -> (i32, i32) {
        for r in 0..200 { for dx in -r..=r { for dy in [-r, r] {
            if g.w.obj_at(dx, dy) == sp && !g.near_build((dx, dy)) { return (dx, dy); }
        } } }
        panic!("ไม่พบต้นไม้ {}", sp)
    }

    #[test]
    fn apples_only_in_summer_and_autumn() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        let k = find_tree(&g, T_APPLE);
        at_day(&mut g, 2);
        assert!(g.fruit_ready(k));
        at_day(&mut g, 7);
        assert!(g.fruit_ready(k));
        at_day(&mut g, 12);
        assert!(!g.fruit_ready(k), "หนาวไม่มีแอปเปิล");
        assert_eq!(g.display_obj(T_APPLE, k), T_DEAD, "หนาวผลัดใบหมด");
        at_day(&mut g, 17);
        assert!(!g.fruit_ready(k), "ใบไม้ผลิยังไม่ออกผล");
        let p = find_tree(&g, T_PALM);
        assert!(g.fruit_ready(p), "มะพร้าวออกตลอดปี");
    }

    #[test]
    fn felled_tree_regrows_as_sapling_then_original_species() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        at_day(&mut g, 0);
        let k = find_tree(&g, T_BIRCH);
        g.p.x = -9999.;
        for _ in 0..10 { if g.w.obj_at(k.0, k.1) == 0 { break; } g.hit_object(k.0, k.1, 1); }
        assert_eq!(g.w.obj_at(k.0, k.1), 0, "โค่นแล้ว");
        assert!(g.felled.contains_key(&k));
        dawn_to(&mut g, 2);
        assert_eq!(g.w.obj_at(k.0, k.1), 0, "ยังไม่งอก");
        dawn_to(&mut g, 3);
        assert_eq!(g.w.obj_at(k.0, k.1), T_SAPLING, "งอกเป็นต้นกล้า");
        // ตัดต้นกล้าที่งอก → เริ่มนับใหม่ ไม่โผล่เป็นต้นใหญ่ทันที
        g.hit_object(k.0, k.1, 5);
        assert_eq!(g.w.obj_at(k.0, k.1), 0);
        assert_eq!(g.felled.get(&k), Some(&(0, 0)));
        dawn_to(&mut g, 6);
        assert_eq!(g.w.obj_at(k.0, k.1), T_SAPLING);
        dawn_to(&mut g, 9);
        assert_eq!(g.w.obj_at(k.0, k.1), T_BIRCH, "โตกลับเป็นชนิดเดิม");
        assert!(!g.felled.contains_key(&k));
    }

    #[test]
    fn no_regrowth_in_winter_or_inside_base() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        at_day(&mut g, 10); // ต้นหนาว
        let k = find_tree(&g, 1);
        g.p.x = -9999.;
        for _ in 0..10 { if g.w.obj_at(k.0, k.1) == 0 { break; } g.hit_object(k.0, k.1, 1); }
        dawn_to(&mut g, 15);
        assert_eq!(g.w.obj_at(k.0, k.1), 0, "หนาวทั้งฤดู ไม่งอก");
        dawn_to(&mut g, 18);
        assert_eq!(g.w.obj_at(k.0, k.1), T_SAPLING, "ใบไม้ผลิแล้วงอก");
        // ต้นที่อยู่ติดสิ่งก่อสร้าง ไม่งอกทับฐาน
        let k2 = find_tree(&g, T_SPRUCE);
        for _ in 0..10 { if g.w.obj_at(k2.0, k2.1) == 0 { break; } g.hit_object(k2.0, k2.1, 1); }
        g.w.placed.insert((k2.0 + 2, k2.1), 10);
        dawn_to(&mut g, 25);
        assert_eq!(g.w.obj_at(k2.0, k2.1), 0, "ในเขตฐานไม่งอก");
    }

    #[test]
    fn planted_sapling_grows_by_biome_and_old_saves_regrow() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        at_day(&mut g, 0);
        let mut spot = None;
        'a: for r in 0..150 { for dx in -r..=r { for dy in [-r, r] {
            if g.w.tile_at(dx, dy) == 2 && g.w.obj_at(dx, dy) == 0 && !g.near_build((dx, dy)) { spot = Some((dx, dy)); break 'a; }
        } } }
        let k = spot.expect("ชายหาด");
        assert!(g.plant_sapling(k));
        assert_eq!(g.w.obj_at(k.0, k.1), T_SAPLING);
        dawn_to(&mut g, 3);
        assert_eq!(g.w.obj_at(k.0, k.1), T_SAPLING);
        dawn_to(&mut g, 4);
        assert_eq!(g.w.obj_at(k.0, k.1), T_PALM, "ปลูกที่หาดได้มะพร้าว");
        // ตัดต้นที่ปลูกเอง → หายไป ไม่งอกเอง
        for _ in 0..10 { if g.w.obj_at(k.0, k.1) == 0 { break; } g.hit_object(k.0, k.1, 1); }
        assert_eq!(g.w.obj_at(k.0, k.1), 0);
        assert!(!g.felled.contains_key(&k));
        // ปลูกบนน้ำไม่ได้
        let mut wat = (0, 0);
        'b: for r in 0..150 { for dx in -r..=r { if g.w.tile_at(dx, r) == 1 { wat = (dx, r); break 'b; } } }
        assert!(!g.plant_sapling(wat));
        // เซฟเก่า: ต้นไม้ที่ถูกโค่นไว้ (มีแค่ใน removed) งอกใหม่ได้
        let t = find_tree(&g, 1);
        g.w.removed.insert(t);
        g.time += DAYLEN; g.season_dawn();
        assert!(g.felled.contains_key(&t));
        // เซฟ/โหลด
        g.planted.insert((5, 5), 2);
        let s = g.seasons_save_str();
        let mut h = Game::new_world_seed(777, Sfx::default());
        for line in s.lines() { let mut it = line.split_whitespace(); let tag = it.next().unwrap(); let rest: Vec<&str> = it.collect(); h.seasons_load_line(tag, &rest); }
        assert_eq!(h.felled, g.felled);
        assert_eq!(h.planted, g.planted);
    }

    #[test]
    fn tints_only_touch_leafy_trees_and_green_ground() {
        assert_eq!(tree_tint(2, 8), WHITE, "สนไม่เปลี่ยนสี");
        assert_ne!(tree_tint(2, 1), WHITE, "โอ๊กใบส้มในฤดูใบไม้ร่วง");
        assert_eq!(tree_tint(1, 1), WHITE);
        assert_eq!(ground_tint(3, 8), WHITE, "ทะเลทรายไม่เปลี่ยน");
        assert_ne!(ground_tint(2, 3), WHITE);
        assert!(PIECES.iter().any(|p| p.obj == T_SAPLING && p.cost.contains(&(BERRY, 1))));
    }
}

#[cfg(test)]
mod animal_tests {
    use super::*;

    fn open_spot(g: &mut Game, tile: u8, r: i32) -> (f32, f32) {
        for rr in 0..400 { for dx in -rr..=rr { for dy in [-rr, rr] {
            if g.w.tile_at(dx, dy) != tile { continue; }
            if (-r..=r).all(|j| (-r..=r).all(|i| g.w.tile_at(dx + i, dy + j) == tile)) {
                for j in -r..=r { for i in -r..=r { if g.w.obj_at(dx + i, dy + j) != 0 { g.w.removed.insert((dx + i, dy + j)); } } }
                return (dx as f32 * 16. + 8., dy as f32 * 16. + 12.);
            }
        } } }
        panic!("ไม่พบที่โล่ง {}", tile)
    }
    fn setup(tile: u8, r: i32) -> (Game, f32, f32) {
        let mut g = Game::new_world_seed(777, Sfx::default());
        g.time = DAYLEN * 0.3;
        let (x, y) = open_spot(&mut g, tile, r);
        g.p.x = x; g.p.y = y;
        g.an_spawn_t = 999.;
        g.animals.clear();
        (g, x, y)
    }
    fn run(g: &mut Game, secs: f32) { let n = (secs / 0.05) as i32; for _ in 0..n { g.an_spawn_t = 999.; g.animals_tick(0.05); if g.p.invul > 0. { g.p.invul -= 0.05; } } }

    #[test]
    fn spawn_tables_follow_biome_and_time() {
        let g = Game::new_world_seed(777, Sfx::default());
        let all = |t: u8, night: bool| (0..100).filter_map(|i| g.animal_for(t, night, i as f32 / 100.)).collect::<std::collections::BTreeSet<u8>>();
        assert!(!all(3, false).contains(&A_WOLF), "กลางวันทุ่งหญ้าไม่มีหมาป่า");
        assert!(all(3, true).contains(&A_WOLF));
        assert!(all(T_SWAMP, false).contains(&A_CROC));
        for t in [3u8, 4, 7, 8, 2, T_SAVANNA] { assert!(!all(t, false).contains(&A_CROC) && !all(t, true).contains(&A_CROC), "จระเข้อยู่แค่หนองน้ำ"); }
        assert!(all(8, false).is_empty() && all(5, false).is_empty(), "ทะเลทราย/หินผาไม่มีสัตว์");
        assert!(all(T_SAVANNA, false).contains(&A_ANTE));
        assert!(all(7, false).contains(&A_RABBIT));
    }

    #[test]
    fn deer_flees_from_player() {
        let (mut g, x, y) = setup(3, 7);
        let d = g.new_animal(A_DEER, x + 40., y);
        g.animals.push(d);
        g.animals[0].bold = 0.5;
        run(&mut g, 0.6);
        let a = &g.animals[0];
        assert_eq!(a.st, AS::Flee);
        assert!(a.x > x + 60., "กวางวิ่งหนีออกไป {}", a.x - x);
    }

    #[test]
    fn wolf_hunts_deer_and_is_then_fed() {
        let (mut g, x, y) = setup(3, 7);
        g.p.x = x - 600.; // ผู้เล่นอยู่ไกล
        let w = g.new_animal(A_WOLF, x - 40., y);
        let mut d = g.new_animal(A_RABBIT, x + 10., y);
        d.hp = 1.;
        g.animals.push(w);
        g.animals.push(d);
        // ให้กระต่ายหนีไม่ได้ (ล้อมด้วยกำแพง)
        let (tx, ty) = (tfloor(x + 10.), tfloor(y));
        for (dx, dy) in [(1, 0), (0, 1), (0, -1), (1, 1), (1, -1)] { g.w.placed.insert((tx + dx, ty + dy), 11); }
        run(&mut g, 4.);
        assert!(g.animals.iter().all(|a| a.k != A_RABBIT), "หมาป่ากินกระต่าย");
        let w = g.animals.iter().find(|a| a.k == A_WOLF).unwrap();
        assert!(w.fed > 0., "กินอิ่มแล้ว");
        assert_ne!(w.st, AS::Chase);
    }

    #[test]
    fn boar_charges_when_close_and_hurts() {
        let (mut g, x, y) = setup(3, 7);
        let b = g.new_animal(A_BOAR, x + 30., y);
        g.animals.push(b);
        let hp = g.p.hp;
        run(&mut g, 1.5);
        assert!(g.p.hp < hp, "หมูป่าชน");
        // หมูป่าบาดเจ็บหนักจะหนี
        g.animals[0].hp = 1.;
        g.animals[0].x = x + 30.;
        run(&mut g, 0.4);
        assert_eq!(g.animals[0].st, AS::Flee);
    }

    #[test]
    fn hunting_gives_meat_and_xp() {
        let (mut g, x, y) = setup(3, 7);
        let d = g.new_animal(A_DEER, x + 12., y);
        g.animals.push(d);
        g.drops.clear();
        let xp = g.p.xp + g.p.lvl * 1000;
        for _ in 0..4 { if g.animals.is_empty() { break; } g.hit_animal(0, 1, 1., 0.); }
        assert!(g.animals.is_empty(), "กวางตาย");
        let meat: i32 = g.drops.iter().filter(|d| d.item == MEAT).map(|d| d.n).sum::<i32>() + g.p.inv[MEAT];
        assert!(meat >= 2);
        assert!(g.p.xp + g.p.lvl * 1000 > xp);
        // ฟันโดนสัตว์ด้วยดาบ
        let r = g.new_animal(A_RABBIT, x + 12., y);
        g.animals.push(r);
        g.p.dir = 2; g.p.flip = false;
        let (dx, dy) = g.p.facing();
        assert!(g.melee_animals(g.p.x + dx * 12., g.p.y - 6. + dy * 12., 2, dx, dy));
        assert!(g.animals.is_empty());
    }

    #[test]
    fn birds_fly_off_and_croc_stays_in_swamp() {
        let (mut g, x, y) = setup(3, 7);
        let b = g.new_animal(A_BIRD, x + 30., y);
        g.animals.push(b);
        run(&mut g, 0.2);
        assert_eq!(g.animals[0].st, AS::Fly);
        run(&mut g, 3.5);
        assert!(g.animals.is_empty(), "นกบินหายไป");
        let (mut g, x, y) = setup(T_SWAMP, 3);
        let c = g.new_animal(A_CROC, x, y);
        g.animals.push(c);
        g.p.x = x + 1000.;
        run(&mut g, 20.);
        for a in &g.animals { assert!(matches!(g.w.tile_at(tfloor(a.x), tfloor(a.y)), T_SWAMP | 2)); }
    }

    #[test]
    fn spawner_respects_cap_and_dungeon_clears() {
        let (mut g, _, _) = setup(4, 3);
        for _ in 0..400 { g.an_spawn_t = 0.; g.animals_tick(0.01); }
        let n = g.animals.iter().filter(|a| a.k != A_BIRD).count();
        assert!(n > 0 && n <= 13, "จำนวนสัตว์ {}", n);
        let mut at = Atlas { img: Image::gen_image_color(512, 512, BLANK), cx: 0, cy: 0, rowh: 0 };
        let r = animals::build_animals(&mut at);
        assert_eq!(r.len(), 16);
        assert!(r[A_CROC as usize * 2].w > r[A_RABBIT as usize * 2].w);
        g.w.dun = Some(Box::new(Dungeon::generate_cave(777, (1, 1), false, &HashSet::new(), 0)));
        g.animals_tick(0.1);
        assert!(g.animals.is_empty());
    }
}

#[cfg(test)]
mod quest_tests {
    use super::*;

    fn mk(id: u32, bed: (i32, i32)) -> npc::Npc {
        npc::Npc { id, name: (id % 12) as u8, look: 0, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: npc::ST_HOME, hearts: 3, mood: 70, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id % 12) as u8) }
    }
    fn town(n: u32) -> Game {
        let mut g = Game::new_world_seed(1, Sfx::default());
        for k in 0..n { let b = (500 + k as i32 * 2, 500); g.w.placed.insert(b, 33); g.npcs.push(mk(k + 1, b)); }
        g.npc_next = n + 1;
        // ทุกคนนิสัยกลางๆ (ไม่ขอ/ไม่ขโมยเอง) — แต่ละเทสต์ตั้งเองตามต้องการ
        for nn in g.npcs.iter_mut() { nn.soc.tr = [50, 50, 50, 50, 50]; }
        g
    }
    /// เดินวันไปจนเกิดคำขอที่ต้องการ
    fn until(g: &mut Game, f: &dyn Fn(&NReq) -> bool) -> NReq {
        for d in 1..80 {
            g.time = DAYLEN * (d as f32 + 0.2);
            g.nreqs.retain(|r| f(r));
            g.quest_dawn();
            if let Some(r) = g.nreqs.iter().find(|r| f(r)) { return r.clone(); }
        }
        panic!("ไม่เกิดคำขอ");
    }

    #[test]
    fn hungry_villager_asks_for_food_and_pays() {
        let mut g = town(2);
        g.npcs[0].mood = 30;
        let r = until(&mut g, &|r| r.from == 1 && matches!(r.k, QK::Fetch { .. }));
        let QK::Fetch { item, qty } = r.k else { unreachable!() };
        assert!(nutrition(item).is_some(), "คนหิวขอของกิน");
        let ri = g.nreqs.iter().position(|x| *x == r).unwrap();
        assert!(!g.req_actions(ri, 1)[0].2, "ของไม่พอ กดไม่ได้");
        g.p.inv[item] = qty;
        let (c0, h0) = (g.p.inv[COIN], g.npcs[0].hearts);
        assert!(g.req_do(ri, QA::Hand).is_some());
        assert_eq!(g.p.inv[item], 0);
        assert!(g.p.inv[COIN] > c0 && g.npcs[0].hearts == h0 + 1);
        assert!(g.nreqs.iter().all(|x| x.from != 1));
    }

    #[test]
    fn coward_asks_to_hunt_and_kills_count() {
        let mut g = town(2);
        g.npcs[0].soc.tr[T_BRAVE] = 15;
        let r = until(&mut g, &|r| r.from == 1 && matches!(r.k, QK::Hunt { .. }));
        let QK::Hunt { animal, n, .. } = r.k else { unreachable!() };
        let ri = g.nreqs.iter().position(|x| *x == r).unwrap();
        assert!(g.req_do(ri, QA::Report).is_none(), "ยังล่าไม่ครบ");
        g.quest_hunt(if animal == A_DEER { A_RABBIT } else { A_DEER });
        for _ in 0..n { g.quest_hunt(animal); }
        let ri = g.nreqs.iter().position(|x| x.from == 1).unwrap();
        assert!(matches!(g.nreqs[ri].k, QK::Hunt { done, n, .. } if done == n));
        assert!(g.req_do(ri, QA::Report).is_some());
    }

    #[test]
    fn kind_villager_asks_to_mediate_and_talking_to_both_reconciles() {
        let mut g = town(3);
        g.npcs[0].soc.tr[T_KIND] = 90;
        g.set_rel(2, 3, -40);
        assert_eq!(g.rel_between(2, 3), -40);
        let r = until(&mut g, &|r| r.from == 1 && matches!(r.k, QK::Mediate { .. }));
        assert!(matches!(r.k, QK::Mediate { a: 2, b: 3, .. }));
        let before = g.rel_between(2, 3);
        g.quest_talked(2);
        assert!(g.nreqs.iter().any(|x| x.from == 1), "ยังไม่ครบสองคน");
        g.quest_reset_daily(); // ข้ามวัน = ต้องคุยใหม่ทั้งคู่
        g.quest_talked(3);
        assert!(g.nreqs.iter().any(|x| x.from == 1));
        g.quest_talked(2);
        assert!(g.nreqs.iter().all(|x| x.from != 1), "ไกล่เกลี่ยสำเร็จ");
        assert!(g.rel_between(2, 3) >= before + 35);
    }

    fn dispute() -> (Game, usize) {
        let mut g = town(2);
        g.add_rel(1, 2, -300);
        let r = until(&mut g, &|r| matches!(r.k, QK::Dispute { .. }));
        let ri = g.nreqs.iter().position(|x| *x == r).unwrap();
        (g, ri)
    }
    #[test]
    fn dispute_sides_and_compromise_need_trust() {
        let (g, ri) = dispute();
        assert_eq!(g.req_actions(ri, 2).len(), 3, "คู่กรณีทั้งสองเห็นตัวเลือก");
        // ประนีประนอมไม่ได้เพราะยังไม่สนิท
        let (mut h, ri) = dispute();
        let msg = h.req_do(ri, QA::Compromise).unwrap();
        assert!(msg.contains("ไม่มีใครฟัง"));
        // สนิทกับทั้งคู่ → ประนีประนอมได้
        let (mut h2, ri) = dispute();
        for n in h2.npcs.iter_mut() { n.hearts = 6; }
        let r0 = h2.rel_between(1, 2);
        h2.req_do(ri, QA::Compromise);
        assert!(h2.rel_between(1, 2) >= r0 + 45);
        // เข้าข้างคนที่ 1 → คนที่ 2 แค้น
        let (mut g, ri) = dispute();
        let g2 = g.npcs[1].soc.grudge;
        let h1 = g.npcs[0].hearts;
        g.req_do(ri, QA::Side(1));
        assert!(g.npcs[1].soc.grudge >= g2 + 25 && g.npcs[1].soc.anger >= 30);
        assert_eq!(g.npcs[0].hearts, h1 + 1);
    }

    fn theft() -> (Game, usize, NReq) {
        let mut g = town(3);
        g.npcs[0].soc.tr = [10, 50, 95, 50, 50]; // โลภ ใจดำ
        g.npcs[0].mood = 40;
        g.npcs[2].soc.tr[T_SOCIAL] = 95; // ช่างพูด → พยาน
        g.w.placed.insert((503, 503), 14);
        let mut c = [0; NI];
        c[COIN] = 30;
        g.chests.insert((503, 503), c);
        let r = until(&mut g, &|r| matches!(r.k, QK::Theft { .. }));
        let ri = g.nreqs.iter().position(|x| *x == r).unwrap();
        (g, ri, r)
    }
    #[test]
    fn thief_steals_from_chest_witness_reports_and_choices_matter() {
        let (g, _, r) = theft();
        let QK::Theft { thief, item, qty } = r.k.clone() else { unreachable!() };
        assert_eq!(thief, 1);
        assert_eq!(r.from, 3, "คนช่างพูดเป็นพยาน");
        assert_eq!(g.chests[&(503, 503)][item], 30 - qty);
        // ไล่ออก: ได้ของคืนและคนขโมยหายไป
        let (mut h, ri, _) = theft();
        let c0 = h.p.inv[item];
        h.req_do(ri, QA::Expel);
        assert!(h.idx(1).is_none() && h.p.inv[item] == c0 + qty);
        // ยกโทษ: ความโลภลดลง
        let (mut h2, ri, _) = theft();
        h2.req_do(ri, QA::Forgive);
        assert!(h2.npcs[0].soc.tr[T_GREED] < 95);
        // ทวงคืน: คนขโมยโกรธ
        let (mut g, ri, _) = theft();
        g.req_do(ri, QA::Reclaim);
        assert!(g.npcs[0].soc.anger >= 30);
    }

    #[test]
    fn ignored_requests_expire_with_disappointment_and_save_roundtrip() {
        let mut g = town(2);
        g.npcs[0].mood = 30;
        let r = until(&mut g, &|r| r.from == 1);
        let s = g.quests_save_str();
        let mut h = town(2);
        for line in s.lines() { let v: Vec<&str> = line.split_whitespace().collect(); h.quests_load_line(&v[1..]); }
        assert_eq!(h.nreqs, g.nreqs);
        let m0 = g.npcs[0].mood;
        g.npcs[0].soc.tr = [50, 50, 50, 50, 50];
        g.npcs[0].mood = 80; // ไม่ให้ขอเรื่องใหม่
        let m0 = m0.max(80);
        g.time = DAYLEN * (r.day + REQ_DAYS + 1) as f32 + 1.;
        g.quest_dawn();
        assert!(g.nreqs.iter().all(|x| *x != r), "หมดเวลา");
        assert!(g.npcs[0].mood < m0);
        assert!(g.news.iter().any(|(_, t)| t.contains("ผิดหวัง")));
        // ปฏิเสธ
        let r = until(&mut h, &|r| r.from == 1);
        let ri = h.nreqs.iter().position(|x| *x == r).unwrap();
        if !matches!(r.k, QK::Theft { .. } | QK::Dispute { .. }) { assert!(h.req_do(ri, QA::Decline).is_some()); assert!(h.nreqs.iter().all(|x| *x != r)); }
    }
}

#[cfg(test)]
mod balance_tests {
    use super::*;

    #[test]
    fn difficulty_tables_scale_monotonically() {
        for t in [DMG, EMUL, NMUL, AMUL] { assert!(t[0] < t[1] && t[1] < t[2] && t[1] == 1.0); }
        assert_eq!(diff(), 1, "ค่าเริ่มต้น = ปกติ");
    }

    #[test]
    fn grace_period_softens_first_two_days() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        g.time = DAYLEN * 0.5;
        assert!(g.in_grace() && g.needs_drain_mul() < 1.);
        // ป่ากลางคืนช่วงผ่อนแรง: ไม่มีสัตว์ดุ
        let mut spot = (0, 0);
        'a: for r in 0..300 { for dx in -r..=r { if g.w.tile_at(dx, r) == 4 { spot = (dx, r); break 'a; } } }
        g.p.x = spot.0 as f32 * 16.; g.p.y = spot.1 as f32 * 16.;
        g.darkv = 0.9;
        for _ in 0..600 { g.try_spawn_animal(); g.animals.retain(|a| prey(a.k) && false); }
        let mut preds = 0;
        for _ in 0..600 { g.try_spawn_animal(); preds += g.animals.iter().filter(|a| predator(a.k)).count(); g.animals.clear(); }
        assert_eq!(preds, 0, "ช่วงผ่อนแรงไม่มีสัตว์ดุ");
        g.time = DAYLEN * 3.5;
        assert!(!g.in_grace() && g.needs_drain_mul() == 1.);
        for _ in 0..600 { g.try_spawn_animal(); preds += g.animals.iter().filter(|a| predator(a.k)).count(); g.animals.clear(); }
        assert!(preds > 0, "พ้นช่วงผ่อนแรงแล้วมีหมาป่า/หมูป่า");
    }

    #[test]
    fn winter_is_announced_two_days_ahead() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        g.time = DAYLEN * 8.1;
        g.winter_warn();
        assert!(g.toasts.iter().any(|t| t.0.contains("อีก 2 วัน")));
        assert!(g.tips_seen & (1 << TIP_WINTER) != 0);
        g.toasts.clear();
        g.time = DAYLEN * 9.1;
        g.winter_warn();
        assert!(g.toasts.iter().any(|t| t.0.contains("พรุ่งนี้")));
        g.toasts.clear();
        g.time = DAYLEN * 3.1;
        g.winter_warn();
        assert!(g.toasts.is_empty());
        g.time = DAYLEN * 28.1; // ปีถัดไป
        g.winter_warn();
        assert!(g.toasts.iter().any(|t| t.0.contains("อีก 2 วัน")));
    }

    #[test]
    fn hunting_gives_hides_and_coat_keeps_you_warm() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        g.p.x = 100.; g.p.y = 100.;
        let d = g.new_animal(A_DEER, 110., 100.);
        g.animals.push(d);
        for _ in 0..5 { if g.animals.is_empty() { break; } g.hit_animal(0, 1, 1., 0.); }
        assert_eq!(g.p.hides, 1);
        assert!(g.an_hunted & (1 << A_DEER) != 0);
        let ri = RECIPES.iter().position(|r| r.name == "เสื้อขนสัตว์").unwrap();
        g.p.inv[WOOD] = 10;
        assert_eq!(recipe_state(&g.p, &RECIPES[ri]), 2, "หนังไม่พอ = ล็อก");
        g.p.hides = 6;
        g.time = DAYLEN * 11.; // หน้าหนาว
        g.p.x = -99999.; // ไกลกองไฟ
        let cold = g.season_climate().1;
        g.craft(ri);
        assert!(g.p.coat && g.p.hides == 1);
        assert!(g.season_climate().1 < cold, "เสื้อขนสัตว์กันหนาว");
        let s = g.balance_save_str();
        let mut h = Game::new_world_seed(777, Sfx::default());
        h.balance_load_line(&s.split_whitespace().skip(1).collect::<Vec<_>>());
        assert!(h.p.coat && h.p.hides == 1 && h.an_hunted == g.an_hunted);
    }

    #[test]
    fn tips_show_once_and_codex_rewards_once() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        g.tip(TIP_NIGHT);
        g.tip(TIP_NIGHT);
        assert_eq!(g.tips.len(), 1);
        for &t in CODEX_BIOMES.iter() { g.seen.insert(t); }
        let hp = g.p.maxhp;
        g.codex_check();
        g.codex_check();
        assert_eq!(g.p.maxhp, hp + 4, "รางวัลครบไบโอมได้ครั้งเดียว");
        assert_eq!(g.codex_progress(0).0, CODEX_BIOMES.len());
        assert_eq!(g.codex_progress(2).0, 0);
    }
}

#[cfg(test)]
mod v11_tests {
    use super::*;

    fn water_edge(g: &Game) -> (i32, i32) {
        for r in 0..300 { for dx in -r..=r { for dy in [-r, r] {
            if g.w.tile_at(dx, dy) == 1 && g.w.tile_at(dx, dy - 1) >= 2 && !g.w.solid_at(dx, dy - 1) && g.w.obj_at(dx, dy) == 0 { return (dx, dy); }
        } } }
        panic!("ไม่พบชายน้ำ")
    }

    #[test]
    fn hotbar_eats_and_swaps() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        g.p.inv = [0; NI];
        g.p.inv[BERRY] = 3;
        g.p.inv[WOOD] = 5;
        g.p.hunger = 40.;
        let items = hotbar_items(&g);
        let k = items.iter().position(|&i| i == BERRY).unwrap();
        g.hotbar_use(k);
        assert_eq!(g.p.inv[BERRY], 2);
        assert!(g.p.hunger > 40.);
        let kw = items.iter().position(|&i| i == WOOD).unwrap();
        g.hotbar_use(kw);
        assert_eq!(g.p.inv[WOOD], 5, "วัสดุกดแล้วไม่หาย");
        g.p.bow = true;
        g.hotbar_use(9);
        assert_eq!(g.p.weapon, 1, "ช่องอาวุธ = สลับอาวุธ");
        assert_eq!(NMUL[3], 0., "สันติไม่หิว");
    }

    #[test]
    fn boat_lets_you_sail_and_counts_goal() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        let (x, y) = water_edge(&g);
        let (wx, wy) = (x as f32 * 16. + 8., y as f32 * 16. + 10.);
        assert!(collide_p(&g.w, wx, wy, false), "ไม่มีเรือลงน้ำไม่ได้");
        assert!(!collide_p(&g.w, wx, wy, true), "มีเรือลงน้ำได้");
        let ri = RECIPES.iter().position(|r| r.name == "เรือ").unwrap();
        g.p.inv[WOOD] = 20;
        g.craft(ri);
        assert!(g.p.boat);
        g.p.x = wx; g.p.y = wy;
        assert!(g.on_boat());
        let inp = Input { mv: vec2(0., 0.), ..Default::default() };
        g.update(0.016, &inp);
        assert!(g.boated);
        g.adv_check();
        assert!(g.adv_done & (1 << 9) != 0, "เป้าหมายล่องเรือสำเร็จ");
    }

    #[test]
    fn taming_a_wolf_makes_a_saved_pet_that_follows() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        g.p.x = 200.; g.p.y = 200.;
        g.p.dir = 2; g.p.flip = false;
        let w = g.new_animal(A_WOLF, 210., 201.);
        g.animals.push(w);
        g.p.inv[MEAT] = 0;
        assert!(g.try_tame(), "มีสัตว์ตรงหน้า (บอกให้หาเนื้อ)");
        assert!(!g.animals[0].tame);
        g.p.inv[MEAT] = 60;
        for _ in 0..60 { if g.animals[0].tame { break; } g.try_tame(); }
        assert!(g.animals[0].tame, "ป้อนหลายครั้งต้องเชื่อง");
        // ตีสัตว์เลี้ยงไม่โดน
        assert!(!g.melee_animals(210., 196., 5, 1., 0.));
        // วิ่งไกล → สัตว์เลี้ยงตาม
        g.p.x = 600.;
        for _ in 0..200 { g.pets_tick(0.05); }
        assert!((g.animals[0].x - g.p.x).abs() < 60., "สัตว์เลี้ยงเดินตาม");
        let s = g.pets_save_str();
        let mut h = Game::new_world_seed(777, Sfx::default());
        h.pets_load_line(&s.split_whitespace().skip(1).collect::<Vec<_>>());
        assert!(h.animals.iter().any(|a| a.tame && a.k == A_WOLF));
        g.adv_check();
        assert!(g.adv_done & (1 << 10) != 0);
    }

    #[test]
    fn goals_progress_and_tool_shows_in_hand() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        assert_eq!(g.adv_next(), Some(0));
        g.p.sword = 1;
        g.adv_check();
        assert!(g.adv_done & 0b11 == 0b11);
        assert_eq!(g.adv_next(), Some(2));
        let c0 = g.p.inv[COIN];
        g.adv_check();
        assert_eq!(g.p.inv[COIN], c0, "รางวัลได้ครั้งเดียว");
        g.p.pick = true;
        let (tx, ty) = g.facing_tile();
        g.w.placed.insert((tx, ty), 4);
        assert_eq!(tool_in_hand(&g), 1, "หันเข้าหินถืออีเต้อ");
        g.w.placed.remove(&(tx, ty));
        g.w.removed.insert((tx, ty));
        assert_eq!(tool_in_hand(&g), 0);
    }
}

#[cfg(test)]
mod v12_tests {
    use super::*;

    #[test]
    fn slots_backups_export_import() {
        let dir = std::env::temp_dir().join(format!("pr_saves_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let p = dir.join("s.txt").to_string_lossy().to_string();
        assert!(slot_path(1).ends_with("_2.txt") && slot_path(2).ends_with("_3.txt"));
        write_atomic(&p, "time 100\np 1 2\n").unwrap();
        assert!(!std::path::Path::new(&format!("{}.tmp", p)).exists());
        rotate_backups(&p);
        write_atomic(&p, "time 999\n").unwrap();
        rotate_backups(&p);
        assert_eq!(backup_count(&p), 2);
        write_atomic(&p, "พัง").unwrap();
        restore_backup(&p).unwrap();
        assert!(std::fs::read_to_string(&p).unwrap().contains("time 999"), "กู้คืนเซฟสำรองล่าสุด");
        let ex = dir.join("ex");
        std::env::set_var("PIXEL_REALM_EXPORT", &ex);
        let out = export_save(&p, 0, "20261121-1300").unwrap();
        assert!(out.contains("pixel_realm_slot1_20261121-1300.txt"));
        write_atomic(&p, "time 5\n").unwrap();
        import_latest(&p).unwrap();
        assert!(std::fs::read_to_string(&p).unwrap().contains("time 999"), "นำเข้าไฟล์ส่งออก");
        std::fs::write(ex.join("pixel_realm_bad.txt"), "hello").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        let _ = std::fs::OpenOptions::new().append(true).open(ex.join("pixel_realm_bad.txt"));
        assert!(stamp_now().len() == 13);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn final_boss_needs_preparation_and_ends_the_game() {
        let mut g = Game::new_world_seed(777, Sfx::default());
        g.time = DAYLEN * 0.6; // กลางคืน
        g.darkv = 0.8;
        assert!(!g.final_ready().0);
        assert!(!g.try_summon_final((10, 10)), "ยังไม่พร้อม = ขอพรตามปกติ");
        g.darkv = 0.;
        assert!(!g.try_summon_final((10, 10)), "กลางวันไม่อัญเชิญ");
        for k in 0..3 { g.dun_clear.insert((k, k)); }
        for &(it, n) in FINAL_COST.iter() { g.p.inv[it] = n; }
        g.darkv = 0.8;
        assert!(g.try_summon_final((10, 10)));
        assert!(g.bosses.iter().any(|b| b.kind == FINAL_KIND && b.hp >= 300.));
        assert!(FINAL_COST.iter().all(|&(it, _)| g.p.inv[it] == 0), "ใช้วัตถุดิบแล้ว");
        let b = g.bosses.iter().find(|b| b.kind == FINAL_KIND).unwrap().clone();
        g.boss_die(&b);
        assert!(g.final_won && g.ending > 0.);
        g.adv_check();
        assert!(g.adv_done & (1 << 15) != 0, "เป้าหมายบอสสุดท้าย");
        // ฉากจบหยุดเกม แล้วกดปิดได้หลัง 3 วินาที
        let t0 = g.time;
        let tap = Input { atk: true, ..Default::default() };
        g.update(0.5, &tap);
        assert!(g.ending > 0. && g.time == t0, "เกมหยุดระหว่างฉากจบ");
        for _ in 0..8 { g.update(0.5, &Input::default()); }
        g.update(0.1, &tap);
        assert_eq!(g.ending, 0.);
        let s = g.balance_save_str();
        let mut h = Game::new_world_seed(777, Sfx::default());
        h.balance_load_line(&s.lines().next().unwrap().split_whitespace().skip(1).collect::<Vec<_>>());
        assert!(h.final_won);
    }
}

#[cfg(test)]
mod body2_tests {
    use super::*;
    #[test]
    fn height_ethnicity_muscle_are_visibly_different() {
        let d = Body::default();
        let a = Body { cm: 170, ..d };
        let b = Body { cm: 205, ..d };
        let (ha, hb) = (a.scale().1 * 15., b.scale().1 * 15.);
        assert!(hb - ha >= 4., "205 กับ 170 ต้องต่างกันอย่างน้อย 4 พิกเซลสไปรต์ ({} vs {})", hb, ha);
        // คนสูงหัวเล็กเมื่อเทียบตัว · เด็กหัวโตเมื่อเทียบตัว
        let (_, hh_b, _, _, rest_b) = b.part_scale();
        assert!(rest_b > hh_b);
        let kid = Body { cm: 110, age: 7, kg: 20, ..d };
        let (_, hh_k, _, _, rest_k) = kid.part_scale();
        assert!(hh_k > rest_k);
        // กล้ามทำให้ลำตัวกว้าง ไม่เปลี่ยนความสูง
        let m = Body { mus: 90, ..d };
        assert!(m.part_scale().2 > d.part_scale().2 * 1.2);
        assert_eq!(m.scale().1, d.scale().1);
        // อ้วนกว้างกว่าผอมชัดเจน
        let fat = Body { kg: 110, ..d };
        let thin = Body { kg: 52, ..d };
        assert!(fat.part_scale().2 / thin.part_scale().2 > 1.5);
        // ค่าเฉลี่ยส่วนสูงตามเชื้อสาย (สุ่ม 400 คนต่อเพศ)
        let avg = |eth: u8, fem: bool| {
            let v: Vec<f64> = (0..5000u32).map(|s| Body::generate(s, fem)).filter(|x| x.eth == eth && x.age < 50).map(|x| x.cm as f64).collect();
            (v.iter().sum::<f64>() / v.len().max(1) as f64, v.len())
        };
        let (sea, n1) = avg(0, false);
        let (neu, n2) = avg(3, false);
        assert!(n1 > 50 && n2 > 20, "{} {}", n1, n2);
        assert!((sea - 169.).abs() < 3. && (neu - 181.).abs() < 4., "เอเชียตะวันออกเฉียงใต้ {:.1} · ยุโรปเหนือ {:.1}", sea, neu);
        assert!(neu - sea > 8.);
        let (seaf, _) = avg(0, true);
        assert!(sea - seaf > 8., "ชายสูงกว่าหญิงโดยเฉลี่ย");
        // สีผิวอยู่ในช่วงของกลุ่ม
        for s in 0..2000u32 { let x = Body::generate(s, s % 2 == 0); let e = ETH[x.eth as usize]; assert!(x.skin >= e.3 .0 && x.skin <= e.3 .1); }
        let s = Body { eth: 9, mus: 77, fem: true, ..d };
        let t = s.save_str();
        let f: Vec<f32> = t.split_whitespace().skip(1).map(|v| v.parse().unwrap()).collect();
        assert_eq!(Body::load(&f), s);
    }
}

#[cfg(test)]
mod house_tests {
    use super::*;
    fn lot() -> Game {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (ox, oy) = (1000, 1000);
        for y in oy - 2..oy + 9 { for x in ox - 2..ox + 9 { g.w.tmod.insert((x, y), 3); g.w.removed.insert((x, y)); } }
        // กำแพง 5x5 (ภายใน 3x3) ประตูด้านล่างกลาง
        for i in 0..5 {
            for &(x, y) in &[(ox + i, oy), (ox + i, oy + 4), (ox, oy + i), (ox + 4, oy + i)] { g.w.placed.insert((x, y), 10); }
        }
        g.w.placed.insert((ox + 2, oy + 4), 12);
        g.w.placed.insert((ox + 1, oy + 1), 33);
        g.p.x = (ox + 2) as f32 * 16. + 8.;
        g.p.y = (oy + 2) as f32 * 16. + 12.;
        g
    }
    #[test]
    fn closed_room_detected_and_open_room_not() {
        let mut g = lot();
        let r = g.w.room_from((1002, 1002)).expect("ห้องปิดต้องเจอ");
        assert_eq!(r.tiles.len(), 9);
        assert!(r.has_bed);
        assert_eq!(r.doors.len(), 1);
        assert!(r.walls.len() >= 15, "ผนัง+มุม {}", r.walls.len());
        // ประตูเปิดก็ยังนับเป็นผนังห้อง
        g.w.placed.insert((1002, 1004), 13);
        assert!(g.w.room_from((1002, 1002)).is_some());
        // เจาะกำแพง = ไม่ใช่ห้อง
        g.w.placed.remove(&(1004, 1002));
        assert!(g.w.room_from((1002, 1002)).is_none());
        // ข้างนอกไม่ใช่ห้อง (ทุ่งโล่งเกิน ROOM_MAX)
        assert!(lot().w.room_from((998, 998)).is_none());
    }
    #[test]
    fn indoors_effects() {
        let mut g = lot();
        g.rooms_refresh();
        assert_eq!(g.rooms.len(), 1);
        assert!(g.indoors());
        assert!(g.in_safe(1002. * 16. + 8., 1002. * 16. + 8.), "ในบ้านมอนไม่เกิด");
        assert_eq!(g.season_climate(), (1., 1.));
        // หลังคาจางลงเมื่ออยู่ข้างใน
        for _ in 0..30 { g.house_tick(1. / 30.); }
        assert!(g.rooms[0].alpha < 0.1);
        // ออกไปข้างนอก หลังคากลับมา
        g.p.y += 16. * 4.;
        assert!(!g.indoors());
        for _ in 0..30 { g.house_tick(1. / 30.); }
        assert!(g.rooms.is_empty() || g.rooms[0].alpha > 0.9);
        assert!(!g.in_safe(998. * 16., 998. * 16.) || !g.torches.is_empty());
    }
    #[test]
    fn floors_place_and_refund() {
        let mut g = lot();
        g.p.inv[WOOD] = 3;
        let k = (1003, 1003);
        g.w.tmod.remove(&k);
        g.w.tmod.remove(&(1002, 1002));
        if SOLIDT[g.w.tile_at(k.0, k.1) as usize] { return; }
        assert!(g.place_floor(k, OBJ_FLOOR_W));
        assert_eq!(g.w.tile_at(k.0, k.1), T_FLOORW);
        assert_eq!(g.p.inv[WOOD], 2);
        assert!(!SOLIDT[T_FLOORW as usize]);
        // พื้นไม่ทำให้ห้องรั่ว
        assert!(g.w.room_from((1002, 1002)).is_some());
        // วางซ้ำ = รื้อคืน
        assert!(g.place_floor(k, OBJ_FLOOR_W));
        assert_eq!(g.w.tile_at(k.0, k.1), g.w.gen_tile(k.0, k.1));
        assert_eq!(g.p.inv[WOOD], 3);
        // ไม้ → หิน เปลี่ยนชนิดแล้วคืนไม้
        g.p.inv[STONE] = 1;
        g.place_floor(k, OBJ_FLOOR_W);
        assert!(g.place_floor(k, OBJ_FLOOR_S));
        assert_eq!(g.w.tile_at(k.0, k.1), T_FLOORS);
        assert_eq!((g.p.inv[WOOD], g.p.inv[STONE]), (3, 0));
        // ไบโอมใต้พื้นยังเป็นของเดิม
        assert_eq!(g.w.biome_at(k.0, k.1), g.w.gen_tile(k.0, k.1));
    }
    #[test]
    fn floor_save_roundtrip() {
        let mut g = lot();
        g.p.inv[WOOD] = 2;
        g.w.tmod.remove(&(1002, 1002));
        if SOLIDT[g.w.tile_at(1002, 1002) as usize] { return; }
        assert!(g.place_floor((1002, 1002), OBJ_FLOOR_W));
        let s = g.save_string();
        let h = Game::load_str(&s, Sfx::default()).ok().expect("load");
        assert_eq!(h.w.tile_at(1002, 1002), T_FLOORW);
    }
    #[test]
    fn adv_house_needs_room() {
        let mut g = lot();
        assert!(g.adv_check_one(3));
        g.w.placed.remove(&(1004, 1002));
        assert!(!g.adv_check_one(3), "เตียงกลางแจ้งยังไม่นับเป็นบ้าน");
    }
}

#[cfg(test)]
mod house2_tests {
    use super::*;
    fn clear_lot(g: &mut Game, x0: i32, y0: i32, x1: i32, y1: i32) {
        for y in y0..=y1 { for x in x0..=x1 { g.w.tmod.insert((x, y), 3); g.w.removed.insert((x, y)); } }
    }
    #[test]
    fn prefab_small_house_is_a_real_room() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        clear_lot(&mut g, 990, 990, 1010, 1010);
        // tmod กันปูพื้น → ล้างช่องในบ้านให้เป็นหญ้าธรรมชาติ ด้วยการลบ tmod ภายในแล้วดูว่าเป็นพื้นแข็งไหม
        g.p.x = 1000. * 16. + 8.; g.p.y = 1002. * 16. + 12.;
        let k = (1000, 1001);
        assert_eq!(g.prefab_blocked(OBJ_HOUSE_S, k, 0), Some("ทับแปลงผัก/ดินที่ขุดไว้ — หาที่อื่น"), "tmod หญ้าที่ทดสอบใส่ไว้ถือว่าดัดแปลงแล้ว");
        for ((x, y), _) in g.prefab_cells(OBJ_HOUSE_S, k, 0) { g.w.tmod.remove(&(x, y)); }
        assert!(!g.prefab_cells(OBJ_HOUSE_S, k, 0).iter().any(|((x, y), _)| SOLIDT[g.w.tile_at(*x, *y) as usize]), "ที่ทดสอบต้องเป็นพื้นดิน");
        g.p.inv[WOOD] = 50; g.p.inv[STONE] = 10;
        let cost = PIECES.iter().find(|p| p.obj == OBJ_HOUSE_S).unwrap().cost;
        g.build_sel = Some(0);
        assert!(g.place_prefab(OBJ_HOUSE_S, k, 0, cost));
        assert_eq!(g.p.inv[WOOD], 10);
        assert!(g.build_sel.is_none(), "วางแล้วออกจากโหมดสร้างเอง");
        assert!(g.w.room_from((1000, 999)).is_none(), "ยังไม่เสร็จทันที ต้องใช้เวลาสร้าง");
        assert!(!g.can_place_at(999, 998), "พื้นที่ก่อสร้างถูกจองไว้");
        for _ in 0..(31 * 30) { g.sites_tick(1. / 30.); }
        assert!(g.sites.is_empty(), "30 วินาทีต้องเสร็จ");
        let r = g.w.room_from((1000, 999)).expect("บ้านสำเร็จรูปต้องเป็นห้องปิด");
        assert_eq!(r.tiles.len(), 9);
        assert!(r.has_bed);
        assert!(r.comfort >= 3, "เตียง+พื้นเต็ม+หน้าต่าง+โคม ได้ {} ดาว", r.comfort);
        assert_eq!(g.w.obj_at(1000, 1001), 12);
        assert!(g.torches.iter().any(|_| true), "โคมไฟนับเป็นแสง");
        // วางซ้ำทับไม่ได้
        assert!(g.prefab_blocked(OBJ_HOUSE_S, k, 0).is_some());
    }
    #[test]
    fn prefab_rotates_door_all_four_ways() {
        // บ้านเล็กหมุน 4 ทิศ: ประตูอยู่ที่จุดเลือกเสมอ ตัวบ้านยื่นไปฝั่งตรงข้าม และยังเป็นห้องปิดที่มีเตียง
        let mut ran = 0;
        for dir in 0..4u8 {
            let mut g = Game::new_world_seed(4242, Sfx::default());
            clear_lot(&mut g, 985, 985, 1015, 1015);
            let k = (1000, 1000);
            for ((x, y), _) in g.prefab_cells(OBJ_HOUSE_S, k, dir) { g.w.tmod.remove(&(x, y)); }
            if g.prefab_cells(OBJ_HOUSE_S, k, dir).iter().any(|((x, y), _)| SOLIDT[g.w.tile_at(*x, *y) as usize]) { continue; }
            let (ox, oy) = rot_off((0, 1), dir);
            g.p.x = (k.0 + ox * 2) as f32 * 16. + 8.; g.p.y = (k.1 + oy * 2) as f32 * 16. + 12.;
            g.p.inv[WOOD] = 50; g.p.inv[STONE] = 10;
            let cost = PIECES.iter().find(|p| p.obj == OBJ_HOUSE_S).unwrap().cost;
            assert!(g.place_prefab(OBJ_HOUSE_S, k, dir, cost), "ทิศ {}", dir);
            for _ in 0..(31 * 30) { g.sites_tick(1. / 30.); }
            assert!(g.sites.is_empty());
            assert_eq!(g.w.obj_at(k.0, k.1), 12, "ประตูที่จุดเลือก ทิศ {}", dir);
            // ด้านนอกประตู (ทางเข้า) ว่าง · ด้านในประตูเป็นห้อง
            assert_eq!(g.w.obj_at(k.0 + ox, k.1 + oy), 0);
            let inside = (k.0 - ox * 2, k.1 - oy * 2);
            let r = g.w.room_from(inside).unwrap_or_else(|| panic!("ทิศ {} ต้องเป็นห้องปิด", dir));
            assert_eq!(r.tiles.len(), 9);
            assert!(r.has_bed);
            assert!(r.doors.contains(&k));
            ran += 1;
        }
        assert_eq!(ran, 4, "ทดสอบครบ 4 ทิศ");
        // เซฟ/โหลดไซต์ที่หมุนแล้วจำทิศได้
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.sites.push(Site { kind: OBJ_HUT, k: (5, 6), prog: 1., done: 2, dir: 3 });
        let line = g.sites_save();
        g.sites.clear();
        let parts: Vec<&str> = line.trim().split(' ').skip(1).collect();
        g.sites_load(&parts);
        assert_eq!(g.sites[0].dir, 3);
        g.sites.clear();
        g.sites_load(&["249", "5", "6", "1.0", "2"]);
        assert_eq!(g.sites[0].dir, 0, "เซฟเก่าไม่มีทิศ = ประตูหันลง");
    }
    #[test]
    fn door_faces_town_center() {
        assert_eq!(door_toward((10, 0), (0, 0)), 1);
        assert_eq!(door_toward((-10, 0), (0, 0)), 3);
        assert_eq!(door_toward((0, 10), (0, 0)), 2);
        assert_eq!(door_toward((0, -10), (0, 0)), 0);
    }
    #[test]
    fn prefab_large_house_comfort_and_player_block() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let k = (1000, 1001);
        clear_lot(&mut g, 990, 990, 1010, 1010);
        for ((x, y), _) in g.prefab_cells(OBJ_HOUSE_L, k, 0) { g.w.tmod.remove(&(x, y)); }
        assert!(!g.prefab_cells(OBJ_HOUSE_L, k, 0).iter().any(|((x, y), _)| SOLIDT[g.w.tile_at(*x, *y) as usize]), "ที่ทดสอบต้องเป็นพื้นดิน");
        // ยืนตรงกำแพงหลังบ้าน → ห้ามวาง
        g.p.x = 1000. * 16. + 8.; g.p.y = 996. * 16. + 12.;
        assert!(g.prefab_blocked(OBJ_HOUSE_L, k, 0).is_some());
        g.p.y = 1003. * 16. + 12.;
        assert_eq!(g.prefab_blocked(OBJ_HOUSE_L, k, 0), None);
        g.p.inv[WOOD] = 50; g.p.inv[STONE] = 60; g.p.inv[BERRY] = 3;
        let cost = PIECES.iter().find(|p| p.obj == OBJ_HOUSE_L).unwrap().cost;
        assert!(g.place_prefab(OBJ_HOUSE_L, k, 0, cost));
        for _ in 0..(61 * 30) { g.sites_tick(1. / 30.); }
        assert!(g.sites.is_empty());
        let r = g.w.room_from((1000, 999)).unwrap();
        assert_eq!(r.tiles.len(), 20);
        assert!(r.stone);
        assert_eq!(r.comfort, 5);
        assert!(g.chests.contains_key(&(998, 1000)));
    }
    #[test]
    fn hold_to_paint_walls_behind_player() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        clear_lot(&mut g, 990, 990, 1010, 1010);
        g.p.inv[WOOD] = 20;
        g.build_sel = PIECES.iter().position(|p| p.obj == 10);
        g.p.x = 1000. * 16. + 8.; g.p.y = 1000. * 16. + 12.;
        g.paint_tick(true);
        for i in 1..=3 { g.p.x = (1000 + i) as f32 * 16. + 8.; g.paint_tick(true); }
        assert_eq!((1000..1003).filter(|&x| g.w.obj_at(x, 1000) == 10).count(), 3);
        assert_eq!(g.w.obj_at(1003, 1000), 0, "ช่องที่ยืนอยู่ไม่โดนวาง");
        assert_eq!(g.p.inv[WOOD], 14);
        // ปล่อยปุ่มแล้วเดิน = ไม่วาง
        g.paint_tick(false);
        g.p.x += 16.; g.paint_tick(false);
        assert_eq!(g.w.obj_at(1003, 1000), 0);
    }
    #[test]
    fn doors_open_and_close_automatically() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        clear_lot(&mut g, 990, 990, 1010, 1010);
        g.w.placed.insert((1000, 999), 12);
        g.p.x = 1000. * 16. + 8.; g.p.y = 1000. * 16. + 4.5;
        g.auto_door(vec2(0., -1.));
        assert_eq!(g.w.obj_at(1000, 999), 13, "เดินชนแล้วเปิดเอง");
        g.auto_door(Vec2::ZERO);
        assert_eq!(g.w.obj_at(1000, 999), 13, "ยังใกล้อยู่ ไม่ปิด");
        g.p.y += 48.;
        g.auto_door(Vec2::ZERO);
        assert_eq!(g.w.obj_at(1000, 999), 12, "เดินห่างแล้วปิดเอง");
    }
    #[test]
    fn furniture_tables_and_comfort() {
        for o in O_TABLE..=O_WARDROBE { assert!(OHP[o as usize] > 0); }
        assert!(!OSOLID[O_CHAIR as usize] && !OSOLID[O_RUG as usize] && OSOLID[O_TABLE as usize]);
        assert!(is_wall_obj(O_WINDOW) && !is_furniture(O_WINDOW));
        assert_eq!(stars(3), "3/5 ดาว");
    }
}

#[cfg(test)]
mod psy_tests {
    use super::*;
    fn villager(id: u32, bed: (i32, i32)) -> npc::Npc {
        npc::Npc { id, name: (id % 12) as u8, look: 0, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: npc::ST_HOME, hearts: 3, mood: 70, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id % 12) as u8) }
    }
    fn lot() -> Game {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        for y in 980..1020 { for x in 980..1020 { g.w.removed.insert((x, y)); } }
        g
    }
    #[test]
    fn personality_starts_consistent_with_old_traits() {
        for id in 1..40 {
            let n = villager(id, (0, 0));
            let p = Psy::from_npc(&n);
            let s = p.surface();
            for k in 0..5 { assert!((s[k] as i32 - n.soc.tr[k] as i32).abs() <= 1, "id {} trait {} {} vs {}", id, k, s[k], n.soc.tr[k]); }
            assert_eq!(p.mbti().len(), 4);
        }
    }
    #[test]
    fn personality_changes_with_life_and_saves() {
        let mut g = lot();
        g.npcs.push(villager(7, (1000, 1000)));
        g.psy_ensure();
        let n0 = g.psy[&7].t[PN];
        let a0 = g.psy[&7].t[PA];
        // ชีวิตดีต่อเนื่อง 20 วัน: อิ่ม มีคนคุยด้วย ได้ของขวัญ
        for d in 0..20 {
            g.time = (d + 1) as f32 * DAYLEN;
            g.npcs[0].mood = 90; g.npcs[0].talked = true; g.npcs[0].gifted = true;
            g.psy_dawn();
        }
        assert!(g.psy[&7].t[PN] < n0 - 4., "ใจเย็นขึ้น {} -> {}", n0, g.psy[&7].t[PN]);
        assert!(g.psy[&7].t[PA] > a0 + 4.);
        assert_eq!(g.npcs[0].soc.tr[T_KIND], g.psy[&7].surface()[T_KIND], "นิสัยเดิมเปลี่ยนตาม");
        // เซฟ/โหลด
        let s = g.psy_save();
        let mut h = lot();
        for l in s.lines() { let r: Vec<&str> = l.split_whitespace().skip(1).collect(); h.psy_load(&r); }
        assert!((h.psy[&7].t[PN] - g.psy[&7].t[PN]).abs() < 0.11);
        assert!((h.psy[&7].base[PA] - a0).abs() < 0.11);
        let info = g.psy_info(7);
        assert!(info.iter().any(|l| l.contains("เปลี่ยนไป")));
    }
    #[test]
    fn evening_villagers_sit_around_campfire_and_chat() {
        let mut g = lot();
        g.w.placed.insert((1000, 1000), 9);
        for (i, k) in [(-2, 0), (2, 0), (0, -2), (0, 2)].iter().enumerate() { g.w.placed.insert((1000 + k.0, 1000 + k.1), O_LOG); let _ = i; }
        for id in 1..=3 {
            let mut n = villager(id, (1005 + id as i32, 1004));
            n.soc.tr[T_SOCIAL] = 90;
            g.npcs.push(n);
        }
        g.psy_ensure();
        for p in g.psy.values_mut() { p.t[PE] = 90.; p.bat = 90.; }
        g.time = (19. - 6.) / 24. * DAYLEN; // 19:00
        assert!(g.is_night_hr());
        g.p.x = 1000. * 16.; g.p.y = 990. * 16.;
        g.psy_t = 0.;
        g.psy_tick(0.01);
        assert_eq!(g.gather.len(), 3, "เอ็กซ์โทรเวิร์ตทุกคนอยากไปนั่ง");
        // เดินไปนั่ง
        for _ in 0..(20 * 30) { g.npc_tick(1. / 30.); }
        g.psy_t = 0.;
        g.psy_tick(0.01);
        assert!(g.psy.values().all(|p| p.sitting), "ทุกคนนั่งแล้ว");
        // คุยกัน: ภายใน 10 วินาทีต้องมีคนพูด
        let mut said = false;
        for _ in 0..12 { g.time += 1.; g.psy_t = 0.; g.psy_tick(0.01); if g.psy.values().any(|p| p.bubble.is_some()) { said = true; } }
        assert!(said);
        // ผู้เล่นนั่งท่อนไม้ที่เหลือ
        let free = [(998, 1000), (1002, 1000), (1000, 998), (1000, 1002)].into_iter().find(|k| !g.npcs.iter().any(|n| (tfloor(n.x), tfloor(n.y - 3.)) == *k)).unwrap();
        g.sit_on(free);
        assert_eq!(g.sit, Some(free));
        for _ in 0..12 { g.time += 1.; g.psy_t = 0.; g.psy_tick(0.01); }
        assert!(g.npcs.iter().any(|n| n.talked), "นั่งในวงนับเป็นการคุย");
    }
    #[test]
    fn introvert_battery_drains_in_crowd() {
        let mut g = lot();
        for id in 1..=4 { g.npcs.push(villager(id, (1000, 1000))); }
        g.psy_ensure();
        g.psy.get_mut(&1).unwrap().t[PE] = 15.;
        g.psy.get_mut(&1).unwrap().bat = 60.;
        g.time = 0.3 * DAYLEN;
        for _ in 0..10 { g.psy_t = 0.; g.psy_tick(0.01); }
        assert!(g.psy[&1].bat < 50., "อินโทรเวิร์ตอยู่กับคนเยอะ แบตลด {}", g.psy[&1].bat);
    }
    #[test]
    fn sitting_heals_and_eating_is_better() {
        let mut g = lot();
        g.w.placed.insert((1000, 1000), O_CHAIR);
        g.sit_on((1000, 1000));
        g.p.hp = 3; g.p.maxhp = 10;
        for _ in 0..(7 * 30) { g.psy_tick(1. / 30.); }
        assert!(g.p.hp >= 5);
        g.p.hunger = 20.; g.p.inv[BERRY] = 2;
        assert!(g.eat_generic_roll(BERRY, 1.));
        let sat = g.p.hunger - 20.;
        g.sit = None;
        g.p.hunger = 20.;
        assert!(g.eat_generic_roll(BERRY, 1.));
        assert!(sat > (g.p.hunger - 20.) * 1.3, "นั่งกินอิ่มกว่า {} vs {}", sat, g.p.hunger - 20.);
    }
    #[test]
    fn tent_builds_fast_into_a_room() {
        let mut g = lot();
        let k = (1000, 1003);
        for ((x, y), _) in g.prefab_cells(OBJ_TENT, k, 0) { assert!(!SOLIDT[g.w.tile_at(x, y) as usize]); }
        g.p.x = 1000. * 16. + 8.; g.p.y = 1005. * 16.;
        g.p.inv[WOOD] = 8;
        assert!(g.place_prefab(OBJ_TENT, k, 0, &[(WOOD, 8)]));
        for _ in 0..(7 * 30) { g.sites_tick(1. / 30.); }
        assert!(g.sites.is_empty());
        let r = g.w.room_from((1000, 1001)).expect("เต็นท์เป็นห้อง");
        assert!(r.tent && r.has_bed);
        // ไซต์ที่อยู่ไกลไม่คืบ
        g.p.inv[WOOD] = 30;
        assert!(g.place_prefab(OBJ_HUT, (1010, 1003), 0, &[(WOOD, 22)]));
        g.p.x = 0.; g.p.y = 0.;
        for _ in 0..(20 * 30) { g.sites_tick(1. / 30.); }
        assert_eq!(g.sites[0].done, 0);
        // เซฟ/โหลดไซต์
        let s = g.sites_save();
        let mut h = lot();
        let r: Vec<&str> = s.split_whitespace().skip(1).collect();
        h.sites_load(&r);
        assert_eq!(h.sites[0].kind, OBJ_HUT);
    }
}

#[cfg(test)]
mod move_tests {
    use super::*;
    fn open_lot() -> Game {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        for y in 990..1010 { for x in 990..1010 { g.w.tmod.insert((x, y), 3); g.w.removed.insert((x, y)); } }
        g
    }
    #[test]
    fn can_slip_between_two_trees_one_gap_apart() {
        let mut g = open_lot();
        // ต้นไม้ 2 ต้นติดกันซ้าย-ขวา มีช่องว่างระหว่างลำต้นประมาณ 10 พิกเซล
        g.w.placed.insert((1000, 1000), 1);
        g.w.placed.insert((1001, 1000), 1);
        // เดินขึ้นผ่านรอยต่อระหว่างสองต้น
        let x = 1001. * 16.;
        assert!(!collide(&g.w, x, 1000. * 16. + 12.), "ช่องว่างระหว่างลำต้นต้องเดินผ่านได้");
        // แต่ชนลำต้นตรงๆ ยังชน
        assert!(collide(&g.w, 1000. * 16. + 8., 1000. * 16. + 12.));
        // ใบไม้ด้านบนของต้นไม่ชน (เดินลอดได้)
        assert!(!collide(&g.w, 1000. * 16. + 8., 1000. * 16. + 6.));
        // กำแพงยังเต็มช่อง
        g.w.placed.insert((1003, 1000), 10);
        assert!(collide(&g.w, 1003. * 16. + 1., 1000. * 16. + 6.));
    }
    #[test]
    fn corner_nudge_slides_past_tree_trunk() {
        let mut g = open_lot();
        g.w.placed.insert((1001, 1000), 1);
        // ยืนซ้ายของต้น ระดับเดียวกับขอบลำต้นนิดเดียว แล้วเดินขวา
        g.p.x = 1000. * 16. + 8.; g.p.y = 1000. * 16. + 9.;
        let y0 = g.p.y;
        let mut inp = Input::default();
        inp.mv = vec2(1., 0.);
        for _ in 0..60 { g.update(1. / 60., &inp); }
        assert!(g.p.x > 1001. * 16. + 4. || (g.p.y - y0).abs() > 0.5, "ต้องลื่นผ่านมุมหรืออย่างน้อยถูกดันเลี่ยง x={} y={}", g.p.x, g.p.y);
    }
    #[test]
    fn auto_aim_prefers_enemy_in_front_and_recent_target() {
        let mut g = open_lot();
        g.p.x = 1000. * 16.; g.p.y = 1000. * 16.;
        g.p.dir = 2; g.p.flip = false; // หันขวา
        g.en.clear();
        g.make_enemy(EK::Slime, g.p.x + 26., g.p.y + 5., 1., 0, false);
        g.make_enemy(EK::Slime, g.p.x - 22., g.p.y + 5., 1., 0, false);
        assert_eq!(g.en.len(), 2);
        let a = g.auto_target(40.).unwrap();
        assert!(a.x > 0., "ศัตรูข้างหน้าได้แต้มต่อ แม้อีกตัวด้านหลังจะใกล้กว่านิดหน่อย");
    }
}

#[cfg(test)]
mod world_step_tests {
    use super::*;
    fn villager(id: u32, x: f32, y: f32) -> npc::Npc {
        npc::Npc { id, name: 1, look: 0, x, y, state: npc::ST_HOME, hearts: 3, mood: 70, married: false, child: false, role: 0, bed: (0, 0), anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, 1) }
    }
    #[test]
    fn nearby_walkers_make_steps_far_ones_dont() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.p.x = 1000. * 16.; g.p.y = 1000. * 16.;
        g.npcs.push(villager(1, g.p.x + 40., g.p.y));
        g.world_steps_tick(0.016);
        g.step_budget = 4.;
        for _ in 0..20 { g.npcs[0].x += 2.; g.world_steps_tick(0.0); }
        assert!(g.step_budget < 4., "ชาวบ้านเดินใกล้ๆ ต้องมีเสียงก้าว");
        // ไกลเกินระยะได้ยิน
        let mut h = Game::new_world_seed(4242, Sfx::default());
        h.p.x = 1000. * 16.; h.p.y = 1000. * 16.;
        h.npcs.push(villager(1, h.p.x + 400., h.p.y));
        h.world_steps_tick(0.016);
        h.step_budget = 4.;
        for _ in 0..20 { h.npcs[0].x += 2.; h.world_steps_tick(0.0); }
        assert_eq!(h.step_budget, 4.);
    }
    #[test]
    fn crowd_is_throttled() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.p.x = 1000. * 16.; g.p.y = 1000. * 16.;
        for i in 0..30 { g.npcs.push(villager(i, g.p.x + 20., g.p.y)); }
        g.world_steps_tick(0.016);
        g.step_budget = 4.;
        for n in g.npcs.iter_mut() { n.x += 18.; }
        g.world_steps_tick(0.0);
        assert!(g.step_budget >= 0., "เล่นได้ไม่เกินโควตา แม้ 30 คนก้าวพร้อมกัน");
    }
}

#[cfg(test)]
mod nav_tests {
    use super::*;
    #[test]
    fn home_marker_warp_and_cooldown() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        for y in 995..1006 { for x in 995..1006 { g.w.tmod.insert((x, y), 3); g.w.removed.insert((x, y)); } }
        assert!(g.home_tile().is_none());
        assert!(!g.warp_home(), "ยังไม่มีบ้าน");
        g.w.placed.insert((1000, 1000), 33);
        g.bed = Some((1000, 1000));
        assert_eq!(g.home_tile(), Some(((1000, 1000), false)));
        g.p.x = 5000.; g.p.y = 5000.;
        assert!(g.warp_home());
        assert!((g.p.x - 1000. * 16.).abs() < 40. && (g.p.y - 1000. * 16.).abs() < 40.);
        g.p.x = 5000.;
        assert!(!g.warp_home(), "ติดรอใช้ซ้ำ");
        g.nav_tick(HOME_CD + 1.);
        assert!(g.warp_home());
        // ล้อมห้อง → เป็นบ้านมีหลังคา
        for i in 0..5 { for &(x, y) in &[(998 + i, 998), (998 + i, 1002), (998, 998 + i), (1002, 998 + i)] { g.w.placed.insert((x, y), 10); } }
        g.w.placed.insert((1000, 1002), 12);
        assert_eq!(g.home_tile(), Some(((1000, 1000), true)));
    }
    #[test]
    fn pins_place_navigate_delete_and_save() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.p.x = 0.; g.p.y = 0.;
        let mr = Rect::new(0., 0., 480., 480.);
        let ctr = (0, 0);
        // แตะตรงกลางค่อนขวา → หมุดที่ (+40 ช่อง ด้วยสเกล 4)
        assert!(g.map_tap(vec2(240. + 40., 240.), mr, ctr, 4, 11.));
        assert_eq!(g.pins.len(), 1);
        assert_eq!(g.pins[0], (40, 0));
        assert_eq!(g.pin_on, Some(0));
        let s = g.nav_save();
        let mut h = Game::new_world_seed(4242, Sfx::default());
        let r: Vec<&str> = s.split_whitespace().skip(1).collect();
        h.nav_load(&r);
        assert_eq!((h.pins.clone(), h.pin_on), (vec![(40, 0)], Some(0)));
        // แตะหมุดที่กำลังนำทาง = ลบ
        let (mx, my, _) = map_pt(40, 0, ctr, 4, mr);
        assert!(g.map_tap(vec2(mx, my), mr, ctr, 4, 11.));
        assert!(g.pins.is_empty() && g.pin_on.is_none());
        // ถึงหมุดแล้วเลิกนำทางเอง
        g.map_tap(vec2(240. + 10., 240.), mr, ctr, 4, 11.);
        g.p.x = 10. * 16. + 8.; g.p.y = 0. * 16. + 8.;
        g.nav_tick(0.016);
        assert!(g.pin_on.is_none());
        // หมุดเต็ม 8 แล้วแทนอันเก่า
        for k in 0..10 { g.map_tap(vec2(20. + k as f32 * 40., 400.), mr, ctr, 4, 11.); }
        assert_eq!(g.pins.len(), PIN_MAX);
    }
}

#[cfg(test)]
mod realm_tests {
    use super::*;
    fn villager(id: u32, bed: (i32, i32)) -> npc::Npc {
        npc::Npc { id, name: (id % 12) as u8, look: 0, x: bed.0 as f32 * 16. + 8., y: bed.1 as f32 * 16. + 18., state: npc::ST_HOME, hearts: 3, mood: 70, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(id, (id % 12) as u8) }
    }
    fn game() -> Game {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        g.realm_tick(0.01);
        g
    }
    /// ชุมชนเล็กๆ ที่จุดเกิด (เตียง+กล่อง)
    fn town(g: &mut Game, n: u32) {
        let (sx, sy) = (tfloor(g.spawn.0), tfloor(g.spawn.1));
        for i in 0..n {
            let b = (sx + 2 + i as i32 * 2, sy + 3);
            g.w.placed.insert(b, 33);
            g.npcs.push(villager(i + 1, b));
        }
        g.w.placed.insert((sx - 2, sy + 3), 14);
        g.chests.insert((sx - 2, sy + 3), [0; NI]);
        g.bed = Some((sx + 2, sy + 3));
    }
    #[test]
    fn kingdoms_and_cities_exist_with_playable_layout() {
        let g = game();
        let r = &g.w.realm;
        assert_eq!(r.ks.len(), 4);
        assert!(r.cities.len() >= 10, "เมือง {}", r.cities.len());
        for (ci, c) in r.cities.iter().enumerate() {
            assert!((c.pos.0.pow(2) + c.pos.1.pow(2)) as f32 > 140f32.powi(2), "เมืองไม่ทับจุดเกิด");
            assert_eq!(r.city_at(c.pos.0, c.pos.1), Some(ci));
            // ประตูทั้ง 4 ทิศและถนนเดินได้
            for (dx, dy) in [(0, c.r), (0, -c.r), (c.r, 0), (-c.r, 0), (0, 2), (2, 0)] { assert!(!g.w.solid_at(c.pos.0 + dx, c.pos.1 + dy), "เมือง {} ช่อง {:?}", c.name, (dx, dy)); }
            assert_eq!(g.w.obj_at(c.pos.0 + 1, c.pos.1 - c.r), 0);
            assert_eq!(g.w.obj_at(c.pos.0 + 3, c.pos.1 - c.r), 38, "กำแพงเมือง");
            assert_eq!(g.w.obj_at(c.pos.0, c.pos.1 - 4), 37, "ปราสาท/ศาลากลาง");
            assert_eq!(g.w.obj_at(c.pos.0 + 3, c.pos.1 + 2), O_STALL);
            assert_eq!(g.w.obj_at(c.pos.0 + 4, c.pos.1 + 4), O_COACH);
            // มีบ้านอย่างน้อย 1 หลังที่เป็นห้องจริง
            let mut rooms = 0;
            for y in c.pos.1 - c.r..c.pos.1 + c.r { for x in c.pos.0 - c.r..c.pos.0 + c.r { if g.w.obj_at(x, y) == 12 && g.w.room_from((x, y + 1)).is_some() || g.w.obj_at(x, y) == 12 && g.w.room_from((x, y - 1)).is_some() { rooms += 1; } } }
            assert!(rooms >= 1, "เมือง {} ไม่มีบ้าน", c.name);
        }
        // ไม่มีแคมป์ทับเมือง
        for cy in -12..12 { for cx in -12..12 { if let Some(p) = g.w.camp_in_cell(cx, cy) { assert!(!g.w.near_city(p.0, p.1, 5)); } } }
        assert!(g.rs.disc.len() >= 4, "เมืองหลวงรู้จักตั้งแต่แรก");
        let names: std::collections::HashSet<_> = r.cities.iter().map(|c| c.name).collect();
        assert_eq!(names.len(), r.cities.len(), "ชื่อเมืองไม่ซ้ำ");
    }
    #[test]
    fn market_prices_follow_specialty_and_supply() {
        let mut g = game();
        let ci = 0;
        let c = g.w.realm.cities[ci].clone();
        let gi_cheap = GOODS.iter().position(|x| x.0 == c.cheap[0]).unwrap();
        let gi_want = GOODS.iter().position(|x| x.0 == c.want[0]).unwrap();
        let base = |gi: usize| GOODS[gi].1;
        assert!(g.price(ci, gi_cheap, true) as f32 <= base(gi_cheap) as f32 * 0.8 + 1.);
        assert!(g.price(ci, gi_want, false) as f32 / base(gi_want) as f32 > g.price(ci, gi_cheap, false) as f32 / base(gi_cheap) as f32, "ของที่เมืองต้องการขายได้ราคาดีกว่า");
        g.p.inv[c.want[0]] = 20;
        let p0 = g.price(ci, gi_want, false);
        g.trade(ci, gi_want, false, 10);
        assert_eq!(g.p.inv[c.want[0]], 10);
        assert!(g.price(ci, gi_want, false) < p0, "ขายเยอะราคาตก");
        g.p.inv[COIN] = 0;
        g.trade(ci, gi_cheap, true, 1);
        assert_eq!(g.p.inv[c.cheap[0]], if c.cheap[0] == c.want[0] { 10 } else { 0 }, "ไม่มีเงินซื้อไม่ได้");
    }
    #[test]
    fn city_laws_fines_and_indestructible_property() {
        let mut g = game();
        let ci = (0..g.w.realm.cities.len()).find(|&ci| g.law_on(ci, L_WEAPON)).expect("มีเมืองห้ามอาวุธ");
        let c = g.w.realm.cities[ci].clone();
        let k = g.city_owner(ci) as usize;
        g.p.x = c.pos.0 as f32 * 16. + 8.; g.p.y = (c.pos.1 + 2) as f32 * 16. + 12.;
        g.p.inv[COIN] = 50;
        let rep0 = g.rs.ks[k].rep;
        g.law_on_attack(false);
        assert!(g.p.inv[COIN] < 50 && g.rs.ks[k].rep < rep0, "โดนปรับ");
        g.rs.crime_t = 0.;
        // ทุบกำแพงเมืองไม่พัง
        let wall = (c.pos.0 + 3, c.pos.1 - c.r);
        assert!(g.hit_object(wall.0, wall.1, 5));
        assert_eq!(g.w.obj_at(wall.0, wall.1), 38);
        // สร้างของในเมืองคนอื่นไม่ได้
        assert!(!g.can_place_at(c.pos.0 + 8, c.pos.1 + 1));
    }
    #[test]
    fn diplomacy_conditions_and_trade_pact_income() {
        let mut g = game();
        town(&mut g, 4);
        let k = 0;
        g.diplo_trade(k);
        assert_eq!(g.rs.ks[k].stance, 0, "ชื่อเสียงไม่พอ");
        g.rs.ks[k].rep = 20;
        g.diplo_trade(k);
        assert_eq!(g.rs.ks[k].stance, 1);
        g.diplo_ally(k);
        assert_eq!(g.rs.ks[k].stance, 1, "พันธมิตรต้องมีคน 5+ และชื่อเสียงสูง");
        // ขายของส่วนเกินในกล่อง
        let ck = *g.chests.keys().next().unwrap();
        g.chests.get_mut(&ck).unwrap()[WOOD] = 30;
        let pool = g.tax_pool;
        g.realm_dawn();
        assert!(g.tax_pool > pool && g.chests[&ck][WOOD] < 30, "สัญญาการค้าขายของให้");
        // ของขวัญ
        g.p.inv[COIN] = 100;
        let r0 = g.rs.ks[k].rep;
        g.diplo_gift(k);
        assert!(g.rs.ks[k].rep > r0);
        g.diplo_gift(k);
        assert_eq!(g.p.inv[COIN], 50, "วันละครั้ง");
    }
    #[test]
    fn war_sends_army_and_battle_resolves_when_away() {
        let mut g = game();
        town(&mut g, 6);
        if peaceful() { return; }
        g.diplo_war(1);
        assert_eq!(g.rs.ks[1].stance, 3);
        g.time += DAYLEN;
        g.realm_dawn();
        assert!(g.rs.ks[1].army.is_some(), "ยกทัพ");
        // เดินทัพจนถึง (ผู้เล่นอยู่ไกล → คิดผลเอง)
        g.p.x += 100. * 16.;
        let mut arrived = false;
        for _ in 0..(DAYLEN as i32 * 12) { g.realm_tick(1.); if g.rs.ks[1].army.is_none() { arrived = true; break; } }
        assert!(arrived, "ทัพมาถึง");
        assert!(g.rs.siege_wins == 1 || g.rs.ks[1].stance == 4, "ชนะหรือกลายเป็นเมืองขึ้น");
        assert!(!g.news.is_empty());
    }
    #[test]
    fn real_siege_spawns_soldiers_and_can_be_won() {
        let mut g = game();
        town(&mut g, 4);
        if peaceful() { return; }
        let tc = g.town_center().unwrap();
        g.p.x = tc.0 as f32 * 16.; g.p.y = tc.1 as f32 * 16.;
        g.rs.ks[2].army = Some(Army { x: tc.0 as f32 + 23., y: tc.1 as f32, n: 6, k: 2, vs: None });
        g.realm_tick(0.01);
        assert!(g.rs.siege.is_some(), "ผู้เล่นอยู่ = ศึกจริง");
        for _ in 0..200 { g.realm_tick(0.1); }
        let n = g.en.iter().filter(|e| e.k == EK::Soldier).count();
        assert!(n > 0, "ทหารศัตรูเกิด");
        // ฆ่าให้หมด
        for _ in 0..400 { for e in g.en.iter_mut() { if e.k == EK::Soldier { e.dead = true; } } g.en.retain(|e| !e.dead); g.realm_tick(0.1); if g.rs.siege.is_none() { break; } }
        assert!(g.rs.siege.is_none());
        assert_eq!(g.rs.siege_wins, 1);
    }
    #[test]
    fn ultimatum_pay_or_war_and_vassal_tribute() {
        let mut g = game();
        town(&mut g, 4);
        g.rs.ks[0].ult = Some((day_of(&g) + 3, 40));
        g.p.inv[COIN] = 10; g.tax_pool = 0;
        g.pay_ultimatum(0, true);
        assert!(g.rs.ks[0].ult.is_some(), "เงินไม่พอ คำขาดยังอยู่");
        g.p.inv[COIN] = 100;
        g.pay_ultimatum(0, true);
        assert_eq!(g.p.inv[COIN], 60);
        g.answer_vassal(1, true);
        assert_eq!(g.rs.ks[1].stance, 4);
        g.tax_pool = 200;
        g.time += 5. * DAYLEN;
        g.realm_dawn();
        assert!(g.tax_pool < 200, "ส่งบรรณาการ");
    }
    #[test]
    fn founding_requires_conditions() {
        let mut g = game();
        town(&mut g, 3);
        g.found_kingdom();
        assert!(!g.rs.founded);
        assert!(g.found_reqs().iter().any(|r| !r.1));
        assert_eq!(g.my_name(), "ชุมชนของคุณ");
    }
    #[test]
    fn migrants_arrive_and_nobles_need_comfy_home() {
        let mut g = game();
        town(&mut g, 2);
        let (sx, sy) = (tfloor(g.spawn.0), tfloor(g.spawn.1));
        for i in 0..4 { g.w.placed.insert((sx + 20 + i * 2, sy), 33); }
        let p0 = g.pop();
        for d in 0..40 { g.time = (d + 1) as f32 * DAYLEN; g.realm_dawn(); }
        assert!(g.pop() > p0 || !g.rs.petitions.is_empty(), "มีคนอยากย้ายมา");
        g.rs.petitions.push(Petition { rank: 2, perk: P_TREASURY, salary: 5, name: 1, look: 0, city: 0, day: 0 });
        let n = g.rs.petitions.len();
        g.accept_petition(n - 1, true);
        assert_eq!(g.rs.petitions.len(), n, "ไม่มีบ้าน 3 ดาว — คำขอยังค้าง");
    }
    #[test]
    fn jobs_produce_and_roles_have_conditions() {
        let mut g = game();
        town(&mut g, 3);
        g.rs.manual.insert(1, R_LUMBER);
        g.assign_roles_pub();
        assert_eq!(g.npcs[0].role, R_LUMBER);
        let ck = *g.chests.keys().next().unwrap();
        g.realm_dawn();
        assert!(g.chests[&ck][WOOD] >= 5, "คนตัดไม้ส่งไม้");
        let mut coward = g.npcs[1].clone();
        coward.soc.tr[T_BRAVE] = 10;
        assert!(role_ok(&coward, R_SOLDIER, &g.rs).is_err());
        assert!(role_ok(&coward, R_GENERAL, &g.rs).is_err());
        assert!(role_ok(&coward, R_LUMBER, &g.rs).is_ok());
    }
    #[test]
    fn realm_state_saves_and_loads() {
        let mut g = game();
        g.rs.ks[1].rep = 33; g.rs.ks[1].stance = 1; g.rs.ks[2].war[3] = true;
        g.rs.owner[1] = PLAYER_OWN; g.rs.horse = true; g.rs.visited.insert(3);
        g.rs.manual.insert(7, R_MINER);
        let s = g.save_string();
        let h = Game::load_str(&s, Sfx::default()).ok().expect("load");
        assert_eq!((h.rs.ks[1].rep, h.rs.ks[1].stance, h.rs.ks[2].war[3]), (33, 1, true));
        assert_eq!(h.rs.owner[1], PLAYER_OWN);
        assert!(h.rs.horse && h.rs.visited.contains(&3));
        assert_eq!(h.rs.manual.get(&7), Some(&R_MINER));
    }
    #[test]
    fn horse_and_coach() {
        let mut g = game();
        g.p.inv[COIN] = 500;
        g.buy_horse();
        assert!(g.rs.horse && g.rs.riding);
        assert!(g.ride_mul() > 1.5);
        g.rs.visited.insert(0); g.rs.visited.insert(1);
        let t0 = g.time;
        g.coach_go(0, 1);
        assert!(g.time > t0);
        assert_eq!(g.player_city(), Some(1));
    }
}

#[cfg(test)]
mod path_tests {
    use super::*;
    /// ลานโล่ง 40x40 รอบ (ox,oy)
    fn flat() -> (World, i32, i32) {
        let mut w = World::new(777);
        let (ox, oy) = (5000, 5000);
        for y in oy - 20..oy + 20 { for x in ox - 20..ox + 20 { w.tmod.insert((x, y), 3); w.removed.insert((x, y)); } }
        (w, ox, oy)
    }
    fn walk(w: &World, from: (i32, i32), to: (i32, i32), mode: Pass, secs: f32) -> (bool, f32) {
        let (mut x, mut y) = tile_foot(from);
        let goal = tile_foot(to);
        let mut p = Path::default();
        let dt = 1. / 30.;
        let mut t = 0.;
        while t < secs {
            let mut b = 2;
            let (gx, gy) = p.steer(w, x, y, goal, mode, dt, &mut b);
            let (dx, dy) = (gx - x, gy - y);
            let l = dx.hypot(dy).max(0.001);
            let st = (40. * dt).min(l);
            // ชาวบ้าน/ชาวเมืองชนแบบจุดที่เท้า (เดินผ่านประตูได้) · ทหารชนแบบกล่อง
            let blk = |x: f32, y: f32| if mode == Pass::Door { let (a, b) = (tfloor(x), tfloor(y - 3.)); w.obj_at(a, b) != 12 && w.solid_at(a, b) } else { collide(w, x, y) };
            if !blk(x + dx / l * st, y) { x += dx / l * st; }
            if !blk(x, y + dy / l * st) { y += dy / l * st; }
            if (x - goal.0).hypot(y - goal.1) < 6. { return (true, t); }
            t += dt;
        }
        (false, t)
    }
    #[test]
    fn path_goes_around_wall_without_cutting_corners() {
        let (mut w, ox, oy) = flat();
        for y in oy - 6..=oy + 6 { w.placed.insert((ox, y), 11); }
        let p = find_path(&w, (ox - 4, oy), (ox + 4, oy), Pass::Door, PATH_NODES);
        assert_eq!(*p.last().unwrap(), (ox + 4, oy));
        let mut prev = (ox - 4, oy);
        for &q in &p {
            assert!(!w.solid_at(q.0, q.1), "ทางผ่านช่องทึบ {:?}", q);
            if q.0 != prev.0 && q.1 != prev.1 { assert!(!w.solid_at(q.0, prev.1) && !w.solid_at(prev.0, q.1), "ตัดมุม"); }
            assert!((q.0 - prev.0).abs() <= 1 && (q.1 - prev.1).abs() <= 1);
            prev = q;
        }
        assert!(p.iter().any(|q| (q.1 - oy).abs() >= 7), "อ้อมปลายกำแพง");
        // เดินตามจริงด้วยการชนแบบเกม
        let (ok, t) = walk(&w, (ox - 4, oy), (ox + 4, oy), Pass::Door, 30.);
        assert!(ok, "เดินไปถึงอีกฝั่ง ({}s)", t);
    }
    #[test]
    fn walker_enters_house_through_door_and_escapes_u_trap() {
        let (mut w, ox, oy) = flat();
        // บ้าน 5x5 ประตูด้านล่าง
        for x in ox..=ox + 4 { w.placed.insert((x, oy), 11); w.placed.insert((x, oy + 4), if x == ox + 2 { 12 } else { 11 }); }
        for y in oy..=oy + 4 { w.placed.insert((ox, y), 11); w.placed.insert((ox + 4, y), 11); }
        // จากข้างบนบ้าน → เข้าไปข้างใน (ต้องอ้อมไปประตูด้านล่าง)
        let (ok, _) = walk(&w, (ox + 2, oy - 3), (ox + 2, oy + 2), Pass::Door, 40.);
        assert!(ok, "เข้าบ้านทางประตู");
        // กับดักรูปตัว U (ปากเปิดด้านหลัง) — เดินตรงจะติดก้นตัว U
        let (mut w, ox, oy) = flat();
        for y in oy - 4..=oy + 4 { w.placed.insert((ox + 3, y), 11); }
        for x in ox - 2..=ox + 3 { w.placed.insert((x, oy - 4), 11); w.placed.insert((x, oy + 4), 11); }
        let (ok, _) = walk(&w, (ox, oy), (ox + 8, oy), Pass::Door, 40.);
        assert!(ok, "ออกจากตัว U ไปอีกฝั่ง");
    }
    #[test]
    fn unreachable_goal_gives_closest_point_and_breach_mode_breaks_in() {
        let (mut w, ox, oy) = flat();
        for x in ox - 2..=ox + 2 { w.placed.insert((x, oy - 2), 11); w.placed.insert((x, oy + 2), 11); }
        for y in oy - 2..=oy + 2 { w.placed.insert((ox - 2, y), 11); w.placed.insert((ox + 2, y), 11); }
        let p = find_path(&w, (ox - 8, oy), (ox, oy), Pass::Door, PATH_NODES);
        let end = *p.last().unwrap();
        assert!((end.0 - ox).abs() <= 3 && (end.1 - oy).abs() <= 3 && !w.solid_at(end.0, end.1), "หยุดข้างกำแพงที่ใกล้สุด {:?}", end);
        let p = find_path(&w, (ox - 8, oy), (ox, oy), Pass::Breach, PATH_NODES);
        assert_eq!(*p.last().unwrap(), (ox, oy));
        assert_eq!(p.iter().filter(|q| w.solid_at(q.0, q.1)).count(), 1, "ทุบกำแพงชั้นเดียว");
        // กำแพงเมือง (ไม่ใช่ของผู้เล่น) ทุบไม่ได้
        assert_eq!(tile_cost(&w, ox - 30, oy, Pass::Breach), tile_cost(&w, ox - 30, oy, Pass::Door).filter(|_| !w.solid_at(ox - 30, oy)));
    }
    #[test]
    fn pathfinding_is_cheap_enough() {
        let g = Game::new_world_seed(4242, Sfx::default());
        let c = g.w.realm.cities[0].clone();
        let t0 = std::time::Instant::now();
        let mut n = 0;
        for i in 0..20 {
            let a = (c.pos.0 - c.r + 2 + i, c.pos.1 - c.r + 2);
            let b = (c.pos.0 + c.r - 2 - i, c.pos.1 + c.r - 2);
            n += find_path(&g.w, a, b, Pass::Door, PATH_NODES).len();
        }
        let ms = t0.elapsed().as_secs_f32() * 1000. / 20.;
        eprintln!("A* เฉลี่ย {:.2} ms (debug) · ความยาวรวม {}", ms, n);
        assert!(ms < 40., "ช้าเกินไป {} ms", ms);
    }
    #[test]
    fn citizens_reach_their_targets_in_city() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let c = g.w.realm.cities[0].clone();
        g.p.x = c.pos.0 as f32 * 16.; g.p.y = c.pos.1 as f32 * 16. + 40.;
        g.realm_tick(0.01);
        let n = g.rs.cit.iter().filter(|c| c.city == 0).count();
        assert!(n > 0);
        // นับจำนวนครั้งที่ถึงเป้าหมาย (เปลี่ยนเป้าหลังรอ) vs ยอมแพ้เพราะติด
        let mut arrive = 0;
        let mut give_up = 0;
        let mut last: Vec<(f32, f32)> = g.rs.cit.iter().map(|c| (c.tx, c.ty)).collect();
        for _ in 0..(60 * 30) {
            let before: Vec<(f32, f32, f32, f32)> = g.rs.cit.iter().map(|c| (c.x, c.y, c.tx, c.ty)).collect();
            g.time += 1. / 30.;
            g.realm_tick(1. / 30.);
            for (i, c) in g.rs.cit.iter().enumerate() {
                if (c.tx, c.ty) != last[i] {
                    let b = before[i];
                    if (b.0 - b.2).hypot(b.1 - b.3) < 4. { arrive += 1; } else { give_up += 1; }
                    last[i] = (c.tx, c.ty);
                }
                assert!(!g.w.solid_at(tfloor(c.x), tfloor(c.y - 3.)) || g.w.obj_at(tfloor(c.x), tfloor(c.y - 3.)) == 12, "ชาวเมืองทะลุของ");
            }
        }
        eprintln!("ถึงเป้า {} ยอมแพ้ {} คน {}", arrive, give_up, n);
        assert!(arrive >= n as i32 * 2 && give_up * 10 <= arrive, "ถึง {} ติด {}", arrive, give_up);
    }
    fn clear_ground(g: &mut Game, cx: i32, cy: i32, r: i32) {
        for y in cy - r..=cy + r { for x in cx - r..=cx + r { g.w.tmod.insert((x, y), 3); g.w.removed.insert((x, y)); } }
    }
    #[test]
    fn villager_walks_home_through_door_at_night_without_teleport() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let (sx, sy) = (tfloor(g.spawn.0) + 30, tfloor(g.spawn.1) + 30);
        clear_ground(&mut g, sx, sy, 14);
        // บ้าน 5x5 ประตูทิศใต้ เตียงข้างใน
        for x in sx..=sx + 4 { g.w.placed.insert((x, sy), 11); g.w.placed.insert((x, sy + 4), if x == sx + 2 { 12 } else { 11 }); }
        for y in sy..=sy + 4 { g.w.placed.insert((sx, y), 11); g.w.placed.insert((sx + 4, y), 11); }
        let bed = (sx + 2, sy + 1);
        g.w.placed.insert(bed, 33);
        let mut n = npc::Npc { id: 1, name: 1, look: 0, x: (sx + 2) as f32 * 16. + 8., y: (sy - 4) as f32 * 16. + 11., state: npc::ST_HOME, hearts: 3, mood: 70, married: false, child: false, role: 0, bed, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(1, 1) };
        n.role = 0;
        g.npcs.push(n);
        g.p.x = (sx - 10) as f32 * 16.; g.p.y = sy as f32 * 16.;
        while !g.is_night_hr() { g.time += 10.; }
        g.last_day = day_of(&g);
        let home = (bed.0 as f32 * 16. + 8., bed.1 as f32 * 16. + 18.);
        let mut prev = (g.npcs[0].x, g.npcs[0].y);
        let mut reached = false;
        for _ in 0..(30 * 30) {
            g.npc_tick(1. / 30.);
            let n = &g.npcs[0];
            assert!((n.x - prev.0).hypot(n.y - prev.1) < 6., "วาร์ป");
            prev = (n.x, n.y);
            if (n.x - home.0).hypot(n.y - home.1) < 8. { reached = true; break; }
        }
        assert!(reached, "เดินอ้อมเข้าประตูกลับถึงเตียง (อยู่ที่ {:?})", (tfloor(prev.0) - sx, tfloor(prev.1 - 3.) - sy));
    }
    #[test]
    fn siege_soldiers_break_through_player_walls_to_center() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        if peaceful() { return; }
        g.realm_tick(0.01);
        let (sx, sy) = (tfloor(g.spawn.0), tfloor(g.spawn.1));
        clear_ground(&mut g, sx, sy, 26);
        for i in 0..4 { g.w.placed.insert((sx - 1 + i, sy + 1), 33); g.npcs.push(npc::Npc { id: i as u32 + 1, name: 1, look: 0, x: 0., y: 0., state: npc::ST_HOME, hearts: 3, mood: 70, married: false, child: false, role: 0, bed: (sx - 1 + i, sy + 1), anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc: Soc::generate(i as u32 + 1, 1) }); }
        g.bed = Some((sx, sy + 1));
        let tc = g.town_center().unwrap();
        // กำแพงหินล้อมรอบ (ไม่มีช่อง)
        let r = 5;
        for d in -r..=r { for k in [(tc.0 + d, tc.1 - r), (tc.0 + d, tc.1 + r), (tc.0 - r, tc.1 + d), (tc.0 + r, tc.1 + d)] { g.w.placed.insert(k, 38); } }
        let walls0 = g.w.placed.values().filter(|&&o| o == 38).count();
        g.p.x = (tc.0 + 20) as f32 * 16.; g.p.y = tc.1 as f32 * 16.;
        g.rs.ks[2].army = Some(Army { x: tc.0 as f32 - 23., y: tc.1 as f32, n: 4, k: 2, vs: None });
        g.realm_tick(0.01);
        assert!(g.rs.siege.is_some());
        let mut inside = false;
        for _st in 0..(90 * 20) {
            g.realm_tick(0.05);
            g.upd_enemies(0.05);
            g.p.hp = g.p.maxhp;
            if g.en.iter().any(|e| e.k == EK::Soldier && (tfloor(e.x) - tc.0).abs() < r && (tfloor(e.y - 3.) - tc.1).abs() < r) { inside = true; break; }
        }
        let walls1 = g.w.placed.values().filter(|&&o| o == 38).count();
        assert!(walls1 < walls0, "ทุบกำแพง");
        assert!(inside, "ทหารเข้าถึงลานกลาง");
    }
}
/// ไอคอนกำปั้น (มือเปล่า) วาดด้วยสี่เหลี่ยม — cx,cy กึ่งกลาง · r ครึ่งความกว้าง (พิกเซลจอ)
fn draw_fist(cx: f32, cy: f32, r: f32) {
    let p = r / 4.;
    let sk = hx(0xf2c79a);
    let dk = hx(0xb07a50);
    draw_rectangle(cx - 3. * p, cy - 2. * p, 6. * p, 5. * p, sk);
    for k in 0..4 { draw_rectangle(cx - 3. * p + k as f32 * 1.5 * p, cy - 3. * p, 1.4 * p, 2. * p, sk); draw_rectangle(cx - 3. * p + k as f32 * 1.5 * p + 1.3 * p, cy - 3. * p, 0.2 * p, 2. * p, dk); }
    draw_rectangle(cx - 4. * p, cy - 1. * p, 1.5 * p, 3. * p, sk);
    draw_rectangle(cx - 3. * p, cy + 3. * p, 6. * p, 0.6 * p, dk);
}
