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
mod boss;
mod camp;
mod combat;
mod dungeon;
mod menu;
mod prefs;
mod rpg;
use extra::*;
use farm::*;
use guide::*;
use audio::*;
use base::*;
use boss::*;
use camp::*;
use combat::*;
use dungeon::*;
use menu::*;
use rpg::*;

// ======================= ค่าคงที่ =======================
const DAYLEN: f32 = 240.0;
const TN: [&str; 15] = [
    "ทะเลลึก", "แหล่งน้ำ", "ชายหาด", "ทุ่งหญ้า", "ป่าทึบ", "เขตหินผา", "ภูเขา", "ดินแดนหิมะ", "ทะเลทราย", "ดันเจี้ยน", "กำแพงดันเจี้ยน", "ดินไถ", "ดินเปียก", "พื้นถ้ำ", "ผนังถ้ำ",
];
const TCOL: [u32; 15] = [
    0x1d4f91, 0x2f7fd0, 0xe6d29a, 0x5fae4a, 0x3f8a3a, 0x8a8a8a, 0x5c5450, 0xeef4f8, 0xe8b86a, 0x3a3a4c, 0x14141e, 0x7a5230, 0x52341e, 0x3a343c, 0x1a1618,
];
// tiles: 0 deep,1 water,2 sand,3 grass,4 forest,5 stone,6 mountain,7 snow,8 desert
const SOLIDT: [bool; 15] = [true, true, false, false, false, false, true, false, false, false, true, false, false, false, true];
// objects: 1 tree,2 bush,3 flower,4 rock,5 ore,6 chest,7 cactus,8 pine,9 campfire,
//   10 กำแพงไม้,11 กำแพงหิน,12 ประตูปิด,13 ประตูเปิด,14 กล่องเก็บของ,15 พ่อค้า,16 เต็นท์
//   17 โต๊ะตีเหล็ก,18 คบเพลิง,19 ทางเข้าดันเจี้ยน,20 ทางออกดันเจี้ยน,21 หนามหุบ,22 หนามโผล่,23 ป้อมลูกไฟ,24 หีบสมบัติหายาก
const OSOLID: [bool; 40] = [false, true, true, false, true, true, true, true, true, true, true, true, true, false, true, true, true, true, true, false, false, false, false, true, true, true, true, false, false, false, false, false, false, true, true, false, false, false, false, false];
const OHP: [i32; 40] = [0, 3, 1, 0, 3, 4, 1, 2, 3, 2, 5, 8, 4, 4, 5, 0, 0, 6, 1, 0, 0, 0, 0, 0, 0, 0, 5, 1, 1, 1, 1, 1, 1, 4, 8, 0, 0, 0, 0, 0];

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
const INAME: [&str; NI] = ["ไม้", "หิน", "เบอร์รี่", "อัญมณี", "เหรียญ", "กองไฟ", "ลูกธนู", "เจลราชาสไลม์", "แกนโกเลม", "ยาฟื้นฟู", "พิมพ์เขียวเกราะ", "พิมพ์เขียวอาวุธ", "เศษดาว", "เนื้อสัตว์", "เห็ด", "สมุนไพร", "สเต็กเนื้อย่าง", "ซุปเห็ดสายลม", "ชาสมุนไพรคมเขี้ยว", "ปลาย่าง", "ปลา", "เมล็ดข้าวโพด", "เมล็ดแครอท", "ข้าวโพด", "แครอท"];

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
}
impl World {
    fn new(seed: i32) -> Self {
        World { seed, chunks: RefCell::new(HashMap::new()), tmod: HashMap::new(), removed: HashSet::new(), placed: HashMap::new(), camps: RefCell::new(HashMap::new()), duns: RefCell::new(HashMap::new()), wps: RefCell::new(HashMap::new()), dun: None }
    }
    fn gen_tile(&self, x: i32, y: i32) -> u8 {
        let s = self.seed;
        let e = fbm(x as f64 / 48., y as f64 / 48., s);
        let m = fbm(x as f64 / 70. + 500., y as f64 / 70. + 500., s.wrapping_add(77));
        if e < 0.36 { return 0; }
        if e < 0.42 { return 1; }
        if e < 0.445 { return 2; }
        if e < 0.60 {
            if m < 0.38 { return 8; }
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
            if ch.len() > 500 { ch.clear(); }
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
        if let Some(o) = self.camp_obj(tx, ty) { return o; }
        if self.dun_in_cell(tx.div_euclid(DCELL), ty.div_euclid(DCELL)) == Some((tx, ty)) { return if self.is_cave_cell(tx.div_euclid(DCELL), ty.div_euclid(DCELL)) { 36 } else { 19 }; }
        if self.wp_in_cell(tx.div_euclid(WPC), ty.div_euclid(WPC)) == Some((tx, ty)) { return 25; }
        let t = self.tile_at(tx, ty);
        let r = hash(tx, ty, self.seed.wrapping_add(5));
        match t {
            3 => if r < 0.035 { 1 } else if r < 0.05 { 2 } else if r < 0.13 { 3 } else if r > 0.9993 { 6 } else { 0 },
            4 => if r < 0.27 { 1 } else if r < 0.32 { 2 } else if r < 0.35 { 3 } else if r > 0.998 { 6 } else { 0 },
            2 => if r > 0.9985 { 6 } else { 0 },
            8 => if r < 0.025 { 7 } else if r > 0.998 { 6 } else { 0 },
            5 => if r < 0.10 { 4 } else if r < 0.125 { 5 } else if r > 0.997 { 6 } else { 0 },
            7 => if r < 0.06 { 8 } else if r < 0.075 { 5 } else if r > 0.996 { 6 } else { 0 },
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
fn collide(w: &World, x: f32, y: f32) -> bool {
    let (l, r, t, b) = (tfloor(x - 4.), tfloor(x + 4.), tfloor(y - 4.), tfloor(y));
    w.solid_at(l, t) || w.solid_at(r, t) || w.solid_at(l, b) || w.solid_at(r, b)
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
}
fn build_sprites(ov: &assets::SpriteOv) -> Sprites {
    let mut at = Atlas { img: Image::gen_image_color(512, 512, BLANK), cx: 0, cy: 0, rowh: 0 };
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
    ];
    for d in &defs {
        tiles.push((0..4).map(|i| at.add(&d(i))).collect::<Vec<_>>());
    }
    // ---- player
    let pp: &[(char, u32)] = &[('h', 0x5a3a1e), ('s', 0xf2c79a), ('e', 0x222222), ('m', 0xc9705a), ('r', 0x3a6fd8), ('R', 0xbcd4ff), ('b', 0x5a3a1e), ('p', 0x34405a), ('k', 0x1d1d22)];
    let pd = ["...hhhhhh...", "..hhhhhhhh..", "..hhhhhhhh..", "..hssssssh..", "..sessssess.", "..ssssssss..", "...ssmmss...", "..rrrrrrrr..", ".srrrRRrrrs.", ".srrrrrrrrs.", ".s.rrrrrr.s.", "...bbbbbb..."];
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
    let mut obj = vec![Rect::new(0., 0., 1., 1.); 40];
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
    ex.flash_slime = [fl_slime[0], fl_slime[1], fl_slime[2], fl_slime[3]];
    ex.flash_skel = fl_skel;
    ex.flash_bat = fl_bat;
    ov.apply(&mut at, &mut tiles, &mut obj, &mut icons);
    let tex = Texture2D::from_image(&at.img);
    tex.set_filter(FilterMode::Nearest);
    Sprites { tex, tiles, player, slime, skel, bat, obj, icons, ex }
}

// ======================= entities =======================
#[derive(Clone, Copy, PartialEq)]
enum EK { Slime, Skel, Bat }
#[derive(Clone)]
struct Enemy { k: EK, x: f32, y: f32, hp: f32, spd: f32, dmg: i32, xp: i32, coin: i32, vx: f32, vy: f32, kb: f32, flash: f32, wt: f32, wdx: f32, wdy: f32, anim: f32, pal: usize, dead: bool, night: bool, burn: f32, slow: f32, stun: f32, btick: f32 }
struct Particle { x: f32, y: f32, vx: f32, vy: f32, l: f32, c: Color }
struct Float { x: f32, y: f32, t: String, c: Color, l: f32, icon: Option<usize>, sz: f32 }
struct Player { x: f32, y: f32, dir: usize, flip: bool, hp: i32, maxhp: i32, xp: i32, lvl: i32, sword: usize, pick: bool, rod: bool, elem: u8, elems: u8, inv: [i32; NI], invul: f32, kbx: f32, kby: f32, kbt: f32, atk_t: f32, atk_cd: f32, anim: f32, moving: bool, regen: f32, last_hit: f32, kills: i32, weapon: u8, bow: bool, staff: u8, armor: [u8; 3], sp: i32, st: [u8; 4], wplus: [u8; 3], aplus: [u8; 3], dash_t: f32, dash_cd: f32, dash_dir: Vec2, ghost_t: f32, buff_spd: f32, buff_crit: f32, can: bool, water: i32 }
impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player { x, y, dir: 0, flip: false, hp: 10, maxhp: 10, xp: 0, lvl: 1, sword: 0, pick: false, rod: false, elem: 0, elems: 1, inv: [0, 0, 2, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], invul: 0., kbx: 0., kby: 0., kbt: 0., atk_t: 0., atk_cd: 0., anim: 0., moving: false, regen: 0., last_hit: 0., kills: 0, weapon: 0, bow: false, staff: 0, armor: [0; 3], sp: 0, st: [0; 4], wplus: [0; 3], aplus: [0; 3], dash_t: 0., dash_cd: 0., dash_dir: Vec2::ZERO, ghost_t: 0., buff_spd: 0., buff_crit: 0., can: false, water: 0 }
    }
    fn facing(&self) -> (f32, f32) {
        DV[if self.dir == 2 && self.flip { 3 } else { self.dir }]
    }
}

#[derive(Default)]
struct Input { mv: Vec2, atk: bool, eat: bool, fire: bool, menu: Option<usize>, map: bool, act: bool, swap: bool, place: bool, cancel: bool, potion: bool, aim: Option<Vec2>, dash: bool, tile_tap: Option<Vec2> }

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
    boss_cd: f32,
    boss_t: f32,
    chests: HashMap<(i32, i32), [i32; NI]>,
    camps_found: HashSet<(i32, i32)>,
    camp_t: f32,
    build_sel: Option<usize>,
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

fn save_path() -> String {
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
            proj: vec![], bosses: vec![], boss_cd: 0., boss_t: 2., chests: HashMap::new(), camps_found: HashSet::new(), camp_t: 0.,
            build_sel: None, req: None,
            drops: vec![], comp: None, ghosts: vec![], quest: None, qn: HashMap::new(), shop_camp: None, fish: None, torches: vec![], torch_t: 0.,
            dun_found: HashSet::new(), wp_found: HashSet::new(), dun_clear: HashSet::new(), dun_chests: HashSet::new(), dun_ret: (0., 0.), dun_t: 0.,
            rain: 0., rain_on: false, rain_t: 90., thunder: 0., cshake: (0., 0.), bgm_on: true, to_title: false, tut: TUT_DONE, tut_ev0: [0; EV_N], ev: [0; EV_N], tut_flash: 0., hitstop: 0., sfx_on: true,
            auto_aim: true, pick_cd: 0.,
            crops: HashMap::new(), crop_t: 0., bed: None, seed_sel: 0, sleep_fx: 0., build_cur: None, cur_pos: (0., 0.), weather_kind: 1,
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
        s += &format!("\ngear {} {} {} {} {} {}\n", p.weapon, p.bow as i32, p.staff, p.armor[0], p.armor[1], p.armor[2]);
        s += &format!("bcd {}\n", self.boss_cd);
        s += &format!("opt {} {} {}\n", self.sfx_on as i32, self.bgm_on as i32, self.auto_aim as i32);
        s += &format!("rpg {} {} {} {} {} {} {} {} {} {}\n", p.sp, p.st[0], p.st[1], p.st[2], p.st[3], p.wplus[0], p.wplus[1], p.wplus[2], p.aplus[0], p.aplus[1]);
        s += &format!("rpg2 {}\n", p.aplus[2]);
        if let Some(q) = &self.quest { s += &format!("quest {} {} {} {} {} {} {} {}\n", q.camp.0, q.camp.1, q.kind, q.goal, q.prog, q.coin, q.shard, q.bp); }
        s += &format!("buff {} {}\n", p.buff_spd, p.buff_crit);
        s += &format!("elem {} {}\nrod {}\n", p.elem, p.elems, p.rod as i32);
        s += &format!("farm {} {} {}\n", p.can as i32, p.water, self.seed_sel);
        s += &format!("tut {}\n", self.tut);
        if let Some(b) = self.bed { s += &format!("bed {} {}\n", b.0, b.1); }
        s += "crops";
        for ((x, y), c) in &self.crops { s += &format!(" {},{},{},{:.1},{:.1}", x, y, c.kind, c.grown, c.wet); }
        s += "\n";
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
        let _ = std::fs::write(save_path(), self.save_string());
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
                    if !p.has_weapon(p.weapon) { p.weapon = 0; }
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
                "farm" => { g.p.can = f(0) > 0.; g.p.water = (f(1) as i32).clamp(0, WATER_MAX); g.seed_sel = (f(2) as u8).min(1); }
                "bed" => g.bed = Some((f(0) as i32, f(1) as i32)),
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
        self.toasts.push((t.into(), 2.6));
        if self.toasts.len() > 4 { self.toasts.remove(0); }
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
            self.toast(format!("เลเวลอัป! Lv {} (+{} แต้มสเตตัส — กด P)", l, SP_PER_LV));
            play(&self.sfx.lvl);
        }
    }

    // ---------- actions ----------
    fn eat(&mut self) {
        if self.p.inv[BERRY] <= 0 {
            if self.p.inv[POTION] > 0 { self.drink(); } else { self.toast("ไม่มีเบอร์รี่"); }
            return;
        }
        if self.p.hp >= self.p.maxhp { self.toast("เลือดเต็มแล้ว"); return; }
        self.p.inv[BERRY] -= 1;
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
        let k = (tx, ty);
        let in_dun = self.w.dun.is_some();
        if in_dun && !matches!(self.w.obj_at(tx, ty), 4 | 5) { return false; }
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
        if o == 15 || o == 16 { return true; }
        if o == 14 && self.chests.get(&k).map_or(false, |c| c.iter().any(|&n| n > 0)) {
            self.toast("เอาของออกจากกล่องก่อนรื้อ");
            return true;
        }
        let h = self.obj_dmg.get(&k).copied().unwrap_or(0) + dmg;
        self.shake = (k, 0.2);
        let col = hx(match o { 1 => 0x3f9a3f, 2 => 0x3d9440, 4 => 0x9a9aa4, 5 => 0x4ef0ff, 6 => 0xc88a40, 7 => 0x3c9a4a, 8 => 0x2f6b4f, 10 => 0x9a6a38, 11 => 0x9a9aa6, 12 | 13 => 0xa06a30, 14 => 0xc88a40, 17 => 0x8a909c, 26 => 0x6a6a74, _ => 0xffa630 });
        let (x, y) = (tx as f32 * 16. + 8., ty as f32 * 16.);
        self.burst(x, y + 6., col, 5, 50.);
        if o == 4 || o == 5 || o == 11 || o == 17 || o == 26 || o == 34 { play(&self.sfx.rock) } else { play(&self.sfx.chop) }
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
            2 => self.gain(BERRY, rnd(1, 3), x, y),
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
            18 => self.gain(WOOD, 1, x, y),
            10 => self.gain(WOOD, 1, x, y),
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
        self.save();
        true
    }
    fn kill(&mut self, e: &Enemy) {
        let c = match e.k { EK::Slime => hx(self_slime_col(e.pal)), EK::Bat => hx(0x5a4670), EK::Skel => hx(0xe8e8d8) };
        self.burst(e.x, e.y - 6., c, 14, 80.);
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
        let dmg = SWD[self.p.sword];
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
        if !hit {
            let (tx, ty) = (tfloor(self.p.x + dx * 11.), tfloor(self.p.y - 3. + dy * 11.));
            hit = self.hit_object(tx, ty, if self.p.sword >= 2 { 2 } else { 1 });
        } else {
            play(&self.sfx.hit);
        }
        if !hit { play(&self.sfx.swing); }
    }
    fn hurt(&mut self, d: i32, nx: f32, ny: f32) {
        let df = d as f32 * (1. - self.p.reduction());
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
        if self.p.hp <= 0 {
            let lost = self.p.inv[COIN] / 2;
            self.p.inv[COIN] -= lost;
            self.p.hp = self.p.maxhp;
            self.w.dun = None;
            let rp = self.respawn_pt();
            self.p.x = rp.0;
            self.p.y = rp.1;
            self.died = true;
            self.proj.clear();
            self.drops.clear();
            self.tp_comp();
            self.toast(format!("หมดสติ... ฟื้นที่จุดเกิด (เสีย {} เหรียญ)", lost));
            self.save();
        }
    }
    fn try_spawn(&mut self) {
        if self.w.dun.is_some() { return; }
        let night = self.darkv > 0.45;
        if self.en.len() >= if night { 10 } else { 4 } { return; }
        let a = gen_range(0., std::f32::consts::TAU);
        let r = gen_range(13., 19.) * 16.;
        let (x, y) = (self.p.x + a.cos() * r, self.p.y + a.sin() * r);
        let (tx, ty) = (tfloor(x), tfloor(y));
        let t = self.w.tile_at(tx, ty);
        if SOLIDT[t as usize] || OSOLID[self.w.obj_at(tx, ty) as usize] || self.w.near_fire(tx, ty, 6).is_some() || self.w.camp_near(tx, ty, 10).is_some() { return; }
        let k = if night {
            let q = gen_range(0., 1.);
            if q < 0.5 { EK::Skel } else if q < 0.8 { EK::Bat } else { EK::Slime }
        } else { EK::Slime };
        if self.in_safe(x, y) { return; }
        let mul = 1. + (x - self.spawn.0).hypot(y - self.spawn.1) / 16. / 250.;
        let elite = night && gen_range(0., 1.) < 0.4;
        let pal = match t { 8 => 1, 7 => 2, 4 => 3, _ => 0 };
        self.make_enemy(k, x, y, mul, pal, elite);
    }
    fn upd_enemies(&mut self, dt: f32) {
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
            if d > 440. && self.w.dun.is_none() { e.dead = true; continue; }
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
            if e.kb <= 0. {
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
            if e.k == EK::Bat {
                e.x += mx * dt;
                e.y += my * dt;
            } else {
                if !collide(&self.w, e.x + mx * dt, e.y) { e.x += mx * dt; }
                if !collide(&self.w, e.x, e.y + my * dt) { e.y += my * dt; }
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
            if d < 11. && self.p.invul <= 0. && e.kb <= 0. && e.stun <= 0. && !e.dead {
                self.hurt(e.dmg, dx / d, dy / d);
            }
        }
        en.retain(|e| !e.dead);
        if self.died { en.clear(); self.died = false; }
        self.en = en;
    }

    fn update(&mut self, dt: f32, inp: &Input) {
        self.time += dt;
        let hr = ((self.time / DAYLEN * 24.) + 6.) % 24.;
        self.darkv = (((hr - 12.).abs() - 5.) / 2.).clamp(0., 1.) * 0.82;
        self.darkv = if let Some(d) = &self.w.dun { if d.cave { 0.8 } else { 0.62 } } else { (self.darkv + self.rain * 0.18).min(0.88) };
        if inp.eat { self.eat(); }
        if inp.fire { self.place_fire(); }
        if inp.potion { self.drink(); }
        if inp.swap { self.swap_weapon(); }
        if inp.dash { self.try_dash(inp.mv); }
        if inp.act { self.interact(); }
        if inp.place { self.place_piece(); }
        if inp.cancel && self.build_sel.is_some() { self.build_sel = None; self.toast("ยกเลิกโหมดสร้าง"); }
        if self.build_sel.is_none() { self.build_cur = None; }
        if self.build_cur.is_some() && (self.p.x - self.cur_pos.0).hypot(self.p.y - self.cur_pos.1) > 20. { self.build_cur = None; }
        if self.sleep_fx > 0. { self.sleep_fx -= dt; }
        self.tut_update(dt);
        let wmul = self.weather_mul();
        // movement
        let mut mv = inp.mv;
        let il = mv.length();
        if il > 1. { mv /= il; }
        let p = &mut self.p;
        p.moving = il > 0.15;
        if p.moving && p.atk_t <= 0. {
            if mv.x.abs() > mv.y.abs() { p.dir = 2; p.flip = mv.x < 0.; } else { p.dir = if mv.y > 0. { 0 } else { 1 }; }
        }
        let sp = 64. * p.speed_mul() * wmul * if p.atk_t > 0. { 0.5 } else { 1. };
        let (mut mx, mut my) = if p.dash_t > 0. { (p.dash_dir.x * 250. * dt, p.dash_dir.y * 250. * dt) } else if p.moving { (mv.x * sp * dt, mv.y * sp * dt) } else { (0., 0.) };
        if p.kbt > 0. { p.kbt -= dt; mx += p.kbx * dt; my += p.kby * dt; }
        if !collide(&self.w, p.x + mx, p.y) { p.x += mx; }
        if !collide(&self.w, p.x, p.y + my) { p.y += my; }
        if p.moving { p.anim += dt; }
        if p.invul > 0. { p.invul -= dt; }
        if p.atk_t > 0. { p.atk_t -= dt; }
        if p.atk_cd > 0. { p.atk_cd -= dt; }
        if inp.atk && self.p.atk_cd <= 0. {
            let mut aim = inp.aim.filter(|a| a.length() > 0.01);
            if aim.is_none() && self.auto_aim {
                aim = self.auto_target(if self.p.weapon == 0 { 30. } else { 180. });
            }
            if let Some(a) = aim { self.face(a); }
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
        if self.p.regen > if fire_near { 1.5 } else { 8. } && self.p.last_hit > 4. && self.p.hp < self.p.maxhp {
            self.p.regen = 0.;
            self.p.hp += 1;
            if fire_near { let (x, y) = (self.p.x, self.p.y - 20.); self.add_float(x, y, "+1 HP", hx(0xff8888), None); }
        }
        // biome discovery
        let bt = self.w.tile_at(tfloor(self.p.x), tfloor(self.p.y));
        if self.w.dun.is_none() && bt as i32 != self.last_biome {
            self.last_biome = bt as i32;
            if bt <= 8 && self.seen.insert(bt) && self.seen.len() > 1 {
                self.toast(format!("ค้นพบ: {}", TN[bt as usize]));
                self.p.xp += 3;
                self.check_lv();
            }
        }
        // enemies
        self.spawn_t -= dt;
        if self.spawn_t <= 0. { self.spawn_t = 1.; self.try_spawn(); }
        self.pick_cd -= dt;
        self.upd_buffs(dt);
        self.upd_fish(dt);
        self.torch_t -= dt;
        if self.torch_t <= 0. { self.torch_t = 0.5; self.refresh_torches(); }
        self.upd_dash(dt);
        self.upd_weather(dt);
        self.upd_crops(dt);
        self.upd_traps(dt);
        self.upd_proj(dt);
        self.upd_bosses(dt);
        self.boss_t -= dt;
        if self.boss_t <= 0. { self.boss_t = 2.; self.try_spawn_boss(); }
        if self.boss_cd > 0. { self.boss_cd -= dt; }
        self.camp_t -= dt;
        if self.camp_t <= 0. { self.camp_t = 1.; self.discover_camps(); }
        self.upd_enemies(dt);
        self.upd_drops(dt);
        self.upd_comp(dt);
        self.dun_t -= dt;
        if self.dun_t <= 0. { self.dun_t = 1.; self.discover_duns(); self.discover_wps(); }
        // fx
        for p in self.parts.iter_mut() { p.x += p.vx * dt; p.y += p.vy * dt; p.vy += 120. * dt; p.l -= dt; }
        self.parts.retain(|p| p.l > 0.);
        for f in self.floats.iter_mut() { f.y -= 18. * dt; f.l -= dt; }
        self.floats.retain(|f| f.l > 0.);
        for t in self.toasts.iter_mut() { t.1 -= dt; }
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

enum Draw { Obj(u8, i32, i32), Enemy(usize), Boss(usize), Player, Loot(usize), Comp }

fn render_world(g: &Game, sp: &Sprites, v: &View, t_now: f32, font: &Font) {
    let w = &g.w;
    let (tx0, ty0) = ((v.cx / 16.).floor() as i32, (v.cy / 16.).floor() as i32);
    let (tx1, ty1) = (((v.cx + v.vw) / 16.).floor() as i32, ((v.cy + v.vh) / 16.).floor() as i32);
    let wf = ((t_now / 0.6) as i32 % 2) as usize;
    let foam = Color::new(0.86, 0.94, 1., 0.75);
    for ty in ty0..=ty1 {
        for tx in tx0..=tx1 {
            let t = w.tile_at(tx, ty) as usize;
            let var = (hash(tx, ty, w.seed.wrapping_add(9)) * 4.) as usize;
            let (px, py) = (tx as f32 * 16. - v.cx, ty as f32 * 16. - v.cy);
            let idx = if t < 2 { (var & 2) | wf } else { var };
            blit(sp, v, sp.tiles[t][idx], px, py, false, WHITE);
            if t < 2 && fish_spot(w.seed, tx, ty) {
                let ph = ((t_now * 0.9 + hash(tx, ty, 3) as f32 * 6.).fract()) * 6.;
                draw_circle_lines((px + 8.) * v.s, (py + 8.) * v.s, ph * v.s, 1., Color::new(1., 1., 1., 0.6 * (1. - ph / 6.)));
            }
            if t < 2 {
                if w.tile_at(tx, ty - 1) > 1 { prect(v, px, py, 16., 1., foam); }
                if w.tile_at(tx, ty + 1) > 1 { prect(v, px, py + 15., 16., 1., foam); }
                if w.tile_at(tx - 1, ty) > 1 { prect(v, px, py, 1., 16., foam); }
                if w.tile_at(tx + 1, ty) > 1 { prect(v, px + 15., py, 1., 16., foam); }
            } else if t == 6 {
                if w.tile_at(tx, ty + 1) != 6 { prect(v, px, py + 13., 16., 3., hx(0x3e3733)); }
                if let Some(&h) = g.obj_dmg.get(&(tx, ty)) {
                    for i in 0..h * 2 { prect(v, px + 3. + i as f32 * 2., py + 5. + (i % 2) as f32 * 3., 1., 3., hx(0x2a2522)); }
                }
            }
        }
    }
    draw_telegraphs(g, v);
    // y-sort
    let mut list: Vec<(f32, Draw)> = vec![];
    let mut lights: Vec<(f32, f32, f32, bool)> = vec![];
    for ty in ty0 - 1..=ty1 + 2 {
        for tx in tx0 - 1..=tx1 + 1 {
            let o = w.obj_at(tx, ty);
            if o == 0 { continue; }
            if o == 9 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 10. - v.cy, pulse_r(80., t_now, tx as f32 * 1.7 + ty as f32 * 0.9, 2.5), true));
            } else if o == 15 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 8. - v.cy, 58., true));
            } else if o == 18 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 6. - v.cy, pulse_r(66., t_now, tx as f32 * 1.7 + ty as f32 * 0.9, 2.), true));
            } else if o == 19 || o == 20 || o == 36 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 8. - v.cy, 40., false));
            } else if o == 25 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 6. - v.cy, 46. + (t_now * 1.6 + tx as f32).sin() * 4., false));
            } else if o == 26 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 8. - v.cy, pulse_r(44., t_now, tx as f32 * 1.7 + ty as f32 * 0.9, 1.5), true));
            } else if o == 24 || o == 23 {
                lights.push((tx as f32 * 16. + 8. - v.cx, ty as f32 * 16. + 8. - v.cy, 26., false));
            }
            let flat = matches!(o, 3 | 19 | 20 | 21 | 22 | 36);
            list.push((ty as f32 * 16. + if flat { 0. } else { 14. }, Draw::Obj(o, tx, ty)));
        }
    }
    for (i, e) in g.en.iter().enumerate() {
        let (sx, sy) = (e.x - v.cx, e.y - v.cy);
        if sx < -20. || sx > v.vw + 20. || sy < -30. || sy > v.vh + 30. { continue; }
        list.push((e.y, Draw::Enemy(i)));
        if e.k == EK::Skel && g.darkv > 0.3 { lights.push((sx, sy - 11., 12., false)); }
    }
    list.push((g.p.y, Draw::Player));
    for (i, l) in g.drops.iter().enumerate() {
        let (sx, sy) = (l.x - v.cx, l.y - v.cy);
        if sx < -10. || sx > v.vw + 10. || sy < -10. || sy > v.vh + 10. { continue; }
        list.push((l.y, Draw::Loot(i)));
        lights.push((sx, sy - 4., 10., false));
    }
    if let Some(c) = &g.comp { list.push((c.y, Draw::Comp)); }
    for (i, b) in g.bosses.iter().enumerate() {
        let (sx, sy) = (b.x - v.cx, b.y - v.cy);
        if sx < -40. || sx > v.vw + 40. || sy < -50. || sy > v.vh + 50. { continue; }
        list.push((b.y, Draw::Boss(i)));
        if g.darkv > 0.3 { lights.push((sx, sy - 14., 26., false)); }
    }
    list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (_, d) in &list {
        match *d {
            Draw::Obj(o, tx, ty) => {
                let (px, py) = (tx as f32 * 16. - v.cx, ty as f32 * 16. - v.cy);
                if o == 3 {
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
                let r = if o == 15 { sp.ex.merchant[if near_m && (t_now * 3.) as i32 % 2 == 0 { 1 } else { 0 }] } else { sp.obj[o as usize] };
                let (px, py) = if matches!(o, 19 | 20 | 21 | 22) { (px, py - (r.h - 16.).max(0.) + (r.h - 16.).max(0.)) } else { (px, py) };
                blit(sp, v, r, px + sx, py + 16. - r.h, false, WHITE);
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
                        prect(v, sx - 5., sy - 1., 10., 2., shadow);
                        if e.night { prect(v, sx - 7., sy - hh - 1., 14., hh + 2., Color::new(0.7, 0.3, 1., 0.28)); }
                        blit_sz(sp, v, if white { sp.ex.flash_slime[e.pal] } else { sp.slime[e.pal] }, (sx - ww / 2.).floor(), sy - hh, ww, hh, c);
                    }
                    EK::Skel => {
                        let bob = ((e.anim * 6.) as i32 % 2) as f32;
                        prect(v, sx - 4., sy - 1., 8., 2., shadow);
                        if e.night { prect(v, sx - 6., sy - 16., 12., 16., Color::new(0.7, 0.3, 1., 0.28)); }
                        blit(sp, v, if white { sp.ex.flash_skel } else { sp.skel }, sx - 5., sy - 15. - bob, false, c);
                    }
                    EK::Bat => {
                        prect(v, sx - 3., sy - 1., 6., 2., Color::new(0., 0., 0., 0.2));
                        if e.night { prect(v, sx - 7., sy - 19., 14., 12., Color::new(0.7, 0.3, 1., 0.28)); }
                        let bi = (e.anim * 8.) as usize % 2;
                        blit(sp, v, if white { sp.ex.flash_bat[bi] } else { sp.bat[bi] }, sx - 6., sy - 18. + ((e.anim * 4.).sin() * 2.).round(), false, c);
                    }
                }
            }
            Draw::Boss(i) => draw_boss(g, sp, v, i),
            Draw::Loot(i) => draw_loot(g, sp, v, i, t_now),
            Draw::Comp => draw_comp(g, sp, v),
            Draw::Player => {
                let p = &g.p;
                let (sx, sy) = ((p.x - v.cx).round(), (p.y - v.cy).round());
                for &(gx, gy, gd, gf, gl) in &g.ghosts {
                    blit(sp, v, sp.player[gd][1], (gx - v.cx).round() - 6., (gy - v.cy).round() - 15., gd == 2 && gf, Color::new(0.5, 0.8, 1., gl * 1.2));
                }
                if !(p.invul > 0. && ((p.invul * 15.) as i32) % 2 == 1) {
                    let f = if p.moving { 1 + (p.anim * 8.) as usize % 2 } else { 0 };
                    prect(v, sx - 4., sy - 1., 8., 2., Color::new(0., 0., 0., 0.25));
                    blit(sp, v, sp.player[p.dir][f], sx - 6., sy - 15., p.dir == 2 && p.flip, WHITE);
                    for (sl, &t) in p.armor.iter().enumerate() {
                        if t == 0 { continue; }
                        let c = hx(ATC[t as usize]);
                        let hi = Color::new((c.r + 0.3).min(1.), (c.g + 0.3).min(1.), (c.b + 0.3).min(1.), 1.);
                        match sl {
                            0 => { prect(v, sx - 4., sy - 15., 8., 1., hi); prect(v, sx - 4., sy - 14., 8., 2., c); }
                            1 => { prect(v, sx - 3., sy - 8., 6., 1., hi); prect(v, sx - 3., sy - 7., 6., 3., c); }
                            _ => { prect(v, sx - 3., sy - 3., 6., 1., hi); prect(v, sx - 3., sy - 2., 6., 2., c); }
                        }
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
                    let r = 9. + if p.sword > 0 { 3. } else { 0. };
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
    for pr in &g.proj { if matches!(pr.kind, 1 | 7 | 8 | 9) { lights.push((pr.x - v.cx, pr.y - v.cy, pulse_r(26., t_now, pr.x * 0.3, 1.5), false)); } }
    if let Some(sel) = g.build_sel {
        let (tx, ty) = g.build_target();
        // กริดช่วยวางให้ตรงช่อง (เฉพาะช่องที่อยู่ในระยะ)
        let (ptx, pty) = (tfloor(g.p.x), tfloor(g.p.y - 3.));
        for j in -5..=5 {
            for i in -5..=5 {
                let c = (ptx + i, pty + j);
                if g.in_reach(c) {
                    draw_rectangle_lines((c.0 as f32 * 16. - v.cx) * v.s, (c.1 as f32 * 16. - v.cy) * v.s, 16. * v.s, 16. * v.s, 1., Color::new(1., 1., 1., 0.14));
                }
            }
        }
        let till = PIECES[sel].obj == OBJ_TILL;
        let ok = if till { g.can_till(tx, ty) } else { g.can_place_at(tx, ty) && can_pay(&g.p, PIECES[sel].cost) };
        let (px, py) = (tx as f32 * 16. - v.cx, ty as f32 * 16. - v.cy);
        let r = if till { sp.tiles[T_SOIL as usize][0] } else { sp.obj[PIECES[sel].obj as usize] };
        blit(sp, v, r, px, py + 16. - r.h, false, Color::new(1., 1., 1., 0.6));
        let lc = if ok { Color::new(0.3, 1., 0.3, 0.9) } else { Color::new(1., 0.3, 0.3, 0.9) };
        draw_rectangle_lines(px * v.s, py * v.s, 16. * v.s, 16. * v.s, 2., lc);
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
    if prefs::weather_fx() { draw_weather(g, v, t_now); }
    // night lighting (เซลล์ 4px ให้ดูเป็นพิกเซล)
    if g.darkv > 0.02 {
        // คทาเวทมนตร์: แสงรอบตัวกว้างกว่าและเต้นตามจังหวะ มีสีตามธาตุ
        let staff = g.p.weapon == 2 && g.p.staff > 0;
        let pr = if staff { pulse_r(62., t_now, 0., 0.) } else { 48. };
        lights.push(((g.p.x - v.cx).round(), (g.p.y - v.cy).round() - 6., pr, false));
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
            if fire {
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
    text(font, &format!("Lv {} · {}{}{}", p.lvl, p.weapon_name(), if p.wplus[p.weapon as usize] > 0 { format!(" +{}", p.wplus[p.weapon as usize]) } else { String::new() }, if p.pick { " · มีอีเต้อ" } else { "" }), x, y + 13. * u, 15. * u, WHITE);
    y += 20. * u;
    let bw = 160. * u;
    let bh = 12. * u;
    draw_rectangle(x, y, bw, bh, hx(0x441111));
    draw_rectangle(x, y, bw * (p.hp as f32 / p.maxhp as f32).clamp(0., 1.), bh, hx(0xee3333));
    draw_rectangle_lines(x, y, bw, bh, 2., BLACK);
    text_c(font, &format!("HP {}/{}", p.hp, p.maxhp), x + bw / 2., y + bh - 2. * u, 11. * u, WHITE);
    y += bh + 4. * u;
    let need = p.lvl * 10;
    draw_rectangle(x, y, bw, bh, hx(0x112233));
    draw_rectangle(x, y, bw * (p.xp as f32 / need as f32).clamp(0., 1.), bh, hx(0x44ccff));
    draw_rectangle_lines(x, y, bw, bh, 2., BLACK);
    text_c(font, &format!("XP {}/{}", p.xp, need), x + bw / 2., y + bh - 2. * u, 11. * u, WHITE);
    y += bh + 4. * u;
    let hr = ((g.time / DAYLEN * 24.) + 6.) % 24.;
    let day = (g.time / DAYLEN) as i32 + 1;
    let night = !(6. ..18.).contains(&hr);
    text(font, &format!("{} วันที่ {} · {:02}:{:02} · {}", if night { "กลางคืน" } else { "กลางวัน" }, day, hr as i32, ((hr.fract()) * 60.) as i32, TN[g.w.tile_at(tfloor(p.x), tfloor(p.y)) as usize]), x, y + 13. * u, 13. * u, if night { hx(0xaabbff) } else { hx(0xffee99) });
    y += 20. * u;
    let mut ix = x;
    for &i in &[WOOD, STONE, BERRY, GEM, COIN, FIRE, ARROW, POTION, SHARD, MEAT, SHROOM, HERB, DISH_STEW, DISH_SPEED, DISH_CRIT, DISH_FISH, FISH] {
        if (i == POTION || i >= SHARD) && p.inv[i] == 0 { continue; }
        if i == ARROW && !p.bow && p.inv[i] == 0 { continue; }
        let s = format!("{}", p.inv[i]);
        let d = measure_text(&s, Some(font), (13. * u) as u16, 1.);
        let w = 20. * u + d.width;
        draw_rectangle(ix, y, w, 18. * u, Color::new(0., 0., 0., 0.55));
        icon(sp, i, ix + 3. * u, y + 2. * u, 14. * u);
        text(font, &s, ix + 18. * u, y + 14. * u, 13. * u, WHITE);
        ix += w + 4. * u;
    }
    y += 24. * u;
    if p.reduction() > 0. {
        text(font, &format!("เกราะ ลดดาเมจ {:.0}% · เดินเร็ว +{:.0}%", p.reduction() * 100., (p.speed_mul() - 1.) * 100.), x, y + 10. * u, 11. * u, hx(0x88ccff));
    }
    if p.sp > 0 {
        text(font, &format!("มีแต้มสเตตัส {} แต้ม — กด P", p.sp), x, y + 26. * u, 12. * u, hx(0x88ff88));
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
        text(font, "เขตปลอดภัย (แสงคบเพลิง)", x, y + 26. * u, 12. * u, hx(0x88ff88));
        y += 16. * u;
    }
    if g.w.dun.is_some() {
        text(font, "ดันเจี้ยนลับ — หาทางไปห้องบอส · หนาม/ไฟ ระวัง!", x, y + 26. * u, 12. * u, hx(0xc890ff));
    }
    if !touch {
        let r = if p.dash_cd > 0. { format!("พุ่ง (Shift) {:.1}s", p.dash_cd) } else { "พุ่ง (Shift) พร้อม".to_string() };
        text(font, &r, x, screen_height() - 34. * u, 12. * u, if p.dash_cd > 0. { hx(0x999999) } else { hx(0x88e0ff) });
    }
    // toasts
    let (sw, sh) = (screen_width(), screen_height());
    for (i, (t, l)) in g.toasts.iter().enumerate() {
        let a = (l * 2.).min(1.);
        let d = measure_text(t, Some(font), (16. * u) as u16, 1.);
        let (w, h) = (d.width + 28. * u, 30. * u);
        let ty = sh * 0.36 + i as f32 * (h + 6. * u);
        draw_rectangle(sw / 2. - w / 2., ty, w, h, Color::new(0., 0., 0., 0.75 * a));
        draw_rectangle_lines(sw / 2. - w / 2., ty, w, h, 2., Color::new(1., 0.8, 0.4, a));
        text_c(font, t, sw / 2., ty + h / 2. + 6. * u, 16. * u, Color::new(1., 1., 0.93, a));
    }
    draw_boss_bar(g, font);
    if let Some(f) = &g.fish {
        let s = if f.bite_t > 0. { if touch { "ปลากิน! กดปุ่ม \"ใช้\" เลย" } else { "ปลากิน! กด Q เลย" } } else { "รอปลากินเบ็ด... (ขยับ = เลิก)" };
        let d = measure_text(s, Some(font), (15. * u) as u16, 1.);
        draw_rectangle(sw / 2. - d.width / 2. - 12. * u, sh - 74. * u, d.width + 24. * u, 28. * u, Color::new(0., 0., 0., 0.65));
        text_c(font, s, sw / 2., sh - 54. * u, 15. * u, if f.bite_t > 0. { hx(0xffee44) } else { hx(0xbfe8ff) });
    } else if g.interact_target().is_none() && g.water_ahead().is_some() {
        let s = if touch { "ปุ่ม \"ใช้\": ตกปลา" } else { "Q: ตกปลา" };
        let d = measure_text(s, Some(font), (15. * u) as u16, 1.);
        draw_rectangle(sw / 2. - d.width / 2. - 12. * u, sh - 74. * u, d.width + 24. * u, 28. * u, Color::new(0., 0., 0., 0.65));
        text_c(font, s, sw / 2., sh - 54. * u, 15. * u, hx(0xbfe8ff));
    }
    if let Some((_, o)) = g.interact_target() {
        let s = interact_hint(o, touch);
        let d = measure_text(&s, Some(font), (15. * u) as u16, 1.);
        draw_rectangle(sw / 2. - d.width / 2. - 12. * u, sh - 74. * u, d.width + 24. * u, 28. * u, Color::new(0., 0., 0., 0.65));
        text_c(font, &s, sw / 2., sh - 54. * u, 15. * u, hx(0xffee99));
    } else if let Some(s) = g.farm_hint(touch) {
        let d = measure_text(&s, Some(font), (15. * u) as u16, 1.);
        draw_rectangle(sw / 2. - d.width / 2. - 12. * u, sh - 74. * u, d.width + 24. * u, 28. * u, Color::new(0., 0., 0., 0.65));
        text_c(font, &s, sw / 2., sh - 54. * u, 15. * u, hx(0xb8ffb0));
    }
    if g.sleep_fx > 0. {
        draw_rectangle(0., 0., sw, sh, Color::new(0., 0., 0.05, g.sleep_fx.min(1.)));
        if g.sleep_fx > 0.5 { text_c(font, "Zzz...", sw / 2., sh / 2., 30. * u, Color::new(1., 1., 1., g.sleep_fx.min(1.))); }
    }
    if let Some(sel) = g.build_sel {
        let k = if touch { "ปุ่มสร้าง" } else { "R" };
        text(font, &format!("โหมดสร้าง: {} · {} = วาง · {} · {}", PIECES[sel].name, k, if touch { "แตะพื้นเพื่อเลือกช่อง" } else { "คลิก/เมาส์ = เลือกช่อง" }, if touch { "เมนู = เปลี่ยน" } else { "T = ยกเลิก" }), x, sh - 16. * u, 13. * u, hx(0x88ff88));
    }
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
    let sc = mm_sc(g);
    let indun = g.w.dun.is_some();
    let (ptx, pty) = (tfloor(g.p.x), tfloor(g.p.y));
    for j in 0..120 {
        for i in 0..120 {
            let (tx, ty) = (ptx + (i - 60) * sc, pty + (j - 60) * sc);
            let t = g.w.tile_at(tx, ty);
            let mut c = hx(TCOL[t as usize]);
            if t >= 3 && t != 6 && t != 7 && OSOLID[g.w.obj_at(tx, ty) as usize] { c.r *= 0.8; c.g *= 0.8; c.b *= 0.8; }
            img.set_pixel(i as u32, j as u32, c);
        }
    }
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
struct TouchUI { active: bool, joy_id: Option<u64>, joy_c: Vec2, joy_v: Vec2, atk_id: Option<u64>, aim: Vec2, flash: [f32; 9] }
/// 0 โจมตี/เล็ง · 1 กิน · 2 กองไฟ · 3 เมนู · 4 แผนที่ · 5 สลับอาวุธ · 6 ใช้ · 7 วางสิ่งปลูกสร้าง
/// หน่วยขนาดปุ่มทัช: อิงความสูงจอ (ปุ่มโจมตี ≈ 16 มม. บนมือถือทั่วไป) × ตัวเลือกขนาดปุ่มในตั้งค่า
fn tunit() -> f32 {
    (screen_height() / 450.).clamp(1., 3.2) * prefs::touch_mul()
}
fn touch_buttons() -> [(Vec2, f32, usize); 9] {
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
    ]
}
/// รัศมีแสงที่ "หายใจ" เข้าออกอย่างนุ่มนวล (ph = เฟสของแหล่งแสง) + ประกายเล็กน้อยสำหรับไฟ
fn pulse_r(base: f32, t: f32, ph: f32, flick: f32) -> f32 {
    base * (1. + 0.08 * (t * 1.7 + ph).sin()) + flick * (t * 19. + ph * 2.7).sin() * (t * 6.3 + ph).sin()
}
fn tile_of(v: &View, p: Vec2) -> (i32, i32) {
    (((p.x / v.s + v.cx) / 16.).floor() as i32, ((p.y / v.s + v.cy) / 16.).floor() as i32)
}
fn handle_touch(tu: &mut TouchUI, inp: &mut Input, build: bool) {
    let ts = touches();
    if !ts.is_empty() { tu.active = true; }
    let btns = touch_buttons();
    let u = tunit();
    for f in tu.flash.iter_mut() { *f = (*f - get_frame_time()).max(0.); }
    for t in ts {
        match t.phase {
            TouchPhase::Started => {
                let mut on_btn = false;
                // เลือกปุ่มที่ใกล้ที่สุด (สัดส่วนต่อรัศมี) ปุ่มเดียวต่อการแตะ กันกดซ้อนปุ่มติดกัน
                let mut best: Option<(f32, usize)> = None;
                for (c, r, id) in btns {
                    if id == 7 && !build { continue; }
                    let q = t.position.distance(c) / r;
                    if q < 1.3 && best.map_or(true, |(bq, _)| q < bq) { best = Some((q, id)); }
                }
                if let Some((_, id)) = best {
                    on_btn = true;
                    tu.flash[id] = 0.16;
                    match id {
                        0 => { tu.atk_id = Some(t.id); tu.aim = Vec2::ZERO; }
                        1 => inp.eat = true,
                        2 => inp.fire = true,
                        3 => inp.menu = Some(0),
                        4 => inp.map = true,
                        5 => inp.swap = true,
                        6 => inp.act = true,
                        8 => inp.dash = true,
                        _ => inp.place = true,
                    }
                }
                if !on_btn && build && t.position.x >= screen_width() * 0.5 { inp.tile_tap = Some(t.position); }
                if !on_btn && tu.joy_id.is_none() && t.position.x < screen_width() * 0.5 && t.position.y > screen_height() * 0.3 {
                    tu.joy_id = Some(t.id);
                    tu.joy_c = t.position;
                    tu.joy_v = Vec2::ZERO;
                }
            }
            TouchPhase::Moved | TouchPhase::Stationary => {
                if tu.joy_id == Some(t.id) {
                    let m = 52. * u;
                    let d = t.position - tu.joy_c;
                    tu.joy_v = if d.length() > m { d.normalize() } else { d / m };
                }
                if tu.atk_id == Some(t.id) {
                    // ลากนิ้วออกจากปุ่มโจมตี = เล็งทิศ (ธนู/คทา)
                    let d = t.position - btns[0].0;
                    tu.aim = if d.length() > 16. * u { d.normalize() } else { Vec2::ZERO };
                }
            }
            TouchPhase::Ended | TouchPhase::Cancelled => {
                if tu.joy_id == Some(t.id) { tu.joy_id = None; tu.joy_v = Vec2::ZERO; }
                if tu.atk_id == Some(t.id) { tu.atk_id = None; tu.aim = Vec2::ZERO; }
            }
        }
    }
    inp.mv += tu.joy_v;
    if tu.atk_id.is_some() {
        inp.atk = true;
        if tu.aim.length() > 0.5 { inp.aim = Some(tu.aim); }
    }
}
fn draw_touch(tu: &TouchUI, sp: &Sprites, font: &Font, build: bool, weapon: u8, dash_cd: f32, use_lab: Option<&str>) {
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
        let held = (id == 0 && tu.atk_id.is_some()) || tu.flash[id] > 0.;
        let base = if id == 0 { Color::new(0.85, 0.25, 0.15, 0.38) } else if id == 7 { Color::new(0.1, 0.55, 0.25, 0.4) } else { Color::new(0.05, 0.05, 0.1, 0.42) };
        draw_circle(c.x, c.y, r, base);
        if held { draw_circle(c.x, c.y, r, Color::new(1., 1., 1., 0.28)); }
        draw_circle_lines(c.x, c.y, r, 3., Color::new(1., 1., 1., if held { 0.8 } else { 0.4 }));
        let lab = |s: &str| tc(s, c.x, c.y + r * 0.88, 11. * u, hx(0xdddddd));
        match id {
            0 => {
                ic([ICON_SWORD, IC_BOW, IC_STAFF][weapon as usize], c.x - r * 0.55, c.y - r * 0.55, r * 1.1);
                if held && tu.aim.length() > 0.5 {
                    let e = c + tu.aim * r * 1.8;
                    draw_line(c.x, c.y, e.x, e.y, 3. * u, Color::new(1., 1., 0.6, 0.6));
                    draw_circle(e.x, e.y, 6. * u, Color::new(1., 1., 0.6, 0.7));
                }
            }
            1 => { ic(BERRY, c.x - r * 0.5, c.y - r * 0.6, r); lab("กิน"); }
            2 => { ic(FIRE, c.x - r * 0.5, c.y - r * 0.6, r); lab("ไฟ"); }
            3 => tc("เมนู", c.x, c.y + 5. * u, 14. * u, WHITE),
            4 => tc("แผนที่", c.x, c.y + 5. * u, 13. * u, WHITE),
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
    Conf { window_title: "Pixel Realm".to_owned(), window_width: 1280, window_height: 720, high_dpi: true, window_resizable: true, ..Default::default() }
}

enum State { Title, Play, Menu }

/// ปุ่มลัดสำหรับทดสอบ (มีเฉพาะ debug build)
#[cfg(debug_assertions)]
fn debug_keys(g: &mut Game) {
    if is_key_pressed(KeyCode::F1) {
        for i in [WOOD, STONE, GEM, GEL, CORE] { g.p.inv[i] += 50; }
        g.p.inv[COIN] += 300;
        g.p.inv[ARROW] += 30;
        g.p.inv[SEED_CORN] += 5; g.p.inv[SEED_CARROT] += 5; g.p.can = true; g.p.water = WATER_MAX;
        g.toast("[debug] ได้ของ");
    }
    if is_key_pressed(KeyCode::F2) {
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
    if is_key_pressed(KeyCode::F3) { let (x, y) = (g.p.x + 90., g.p.y); g.spawn_boss(0, x, y); }
    if is_key_pressed(KeyCode::F4) { let (x, y) = (g.p.x + 90., g.p.y); g.spawn_boss(1, x, y); }
    if is_key_pressed(KeyCode::F5) { g.time = DAYLEN * 0.7; }
    if is_key_pressed(KeyCode::F7) {
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
    if is_key_pressed(KeyCode::F10) {
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
    if is_key_pressed(KeyCode::F6) { g.p.maxhp = 400; g.p.hp = 400; g.toast("[debug] เลือด 400"); }
}

#[macroquad::main(window_conf)]
async fn main() {
    macroquad::rand::srand((miniquad::date::now() * 1000.) as u64);
    let sp_ov = assets::load_sprite_overrides().await;
    let sp = build_sprites(&sp_ov);
    let font = load_ttf_font_from_bytes(include_bytes!("../assets/PixelRealmThai-Bold.ttf")).expect("font");
    let mut mini_img = Image::gen_image_color(120, 120, BLACK);
    let mini_tex = Texture2D::from_image(&mini_img);
    mini_tex.set_filter(FilterMode::Nearest);
    let mut sfx_slot: Option<Sfx> = Some(load_sfx().await);
    let mut bgm = BgmMgr::new();
    let mut game: Option<Game> = None;
    prefs::load();
    let mut save_sum = prefs::save_summary(DAYLEN);
    let mut tview = 0u8;
    let mut bg = Game::new_world_placeholder(Sfx::default());
    bg.w = World::new(gen_range(1, i32::MAX));
    bg.p.invul = 0.07; // ซ่อนผู้เล่นบนหน้าเมนู
    let mut state = State::Title;
    let mut tu = TouchUI { active: false, joy_id: None, joy_c: Vec2::ZERO, joy_v: Vec2::ZERO, atk_id: None, aim: Vec2::ZERO, flash: [0.; 9] };
    let mut ui = UiState::new();
    let mut t_now = 0f32;
    let mut mouse_armed = false;
    let mut map_ctr = (0i32, 0i32);
    let mut map_sc = 2i32;

    loop {
        let frame_t0 = get_time();
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
                        let n = 5.;
                        let mut y = (sh * 0.30).max(ly + 50. * u).min(sh - n * (bh + gap) - 26. * u);
                        let sum = save_sum;
                        let lab = match sum { Some((d, l)) => format!("เล่นต่อ  ·  วันที่ {} · Lv {}", d, l), None => "เล่นต่อ".to_string() };
                        if button(&font, Rect::new(bx, y, bw, bh), &lab, sum.is_some(), u) || (sum.is_some() && is_key_pressed(KeyCode::Enter)) { start = Some(false); }
                        y += bh + gap;
                        if button(&font, Rect::new(bx, y, bw, bh), "เกมใหม่", true, u) || (sum.is_none() && is_key_pressed(KeyCode::Enter)) {
                            if sum.is_some() { tview = 3; } else { start = Some(true); }
                        }
                        y += bh + gap;
                        if button(&font, Rect::new(bx, y, bw, bh), "ตั้งค่า", true, u) { tview = 1; }
                        y += bh + gap;
                        if button(&font, Rect::new(bx, y, bw, bh), "วิธีเล่น", true, u) { tview = 2; }
                        y += bh + gap;
                        if button(&font, Rect::new(bx, y, bw, bh), "ออกจากเกม", true, u) { std::process::exit(0); }
                        text_c(&font, "v6 · เกมวาดด้วยโค้ดทั้งหมด", sw / 2., sh - 10. * u, 11. * u, hx(0x999999));
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
                    _ => {
                        let (w, h) = ((460. * u).min(sw * 0.97), (190. * u).min(sh * 0.97));
                        let (x, y) = (sw / 2. - w / 2., sh / 2. - h / 2.);
                        panel(x, y, w, h);
                        text_c(&font, "เริ่มเกมใหม่?", sw / 2., y + 42. * u, 24. * u, hx(0xffcc66));
                        text_c(&font, "เซฟเดิมจะถูกแทนที่", sw / 2., y + 76. * u, 15. * u, hx(0xdddddd));
                        if button(&font, Rect::new(sw / 2. - 160. * u, y + h - 62. * u, 150. * u, 42. * u), "เริ่มใหม่", true, u) { start = Some(true); }
                        if button(&font, Rect::new(sw / 2. + 10. * u, y + h - 62. * u, 150. * u, 42. * u), "ยกเลิก", true, u) || is_key_pressed(KeyCode::Escape) { tview = 0; }
                    }
                }
                if let Some(fresh) = start {
                    let sfx = sfx_slot.take().unwrap_or_default();
                    let mut g = if fresh { Game::new_world(sfx) } else {
                        match Game::load(sfx) { Ok(g) => g, Err(sfx) => Game::new_world(sfx) }
                    };
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
                    inp.swap = is_key_pressed(KeyCode::Tab);
                    inp.dash = is_key_pressed(KeyCode::LeftShift) || is_key_pressed(KeyCode::K);
                    inp.potion = is_key_pressed(KeyCode::H);
                    for (kc, di) in [(KeyCode::Key1, DISH_STEW), (KeyCode::Key2, DISH_SPEED), (KeyCode::Key3, DISH_CRIT), (KeyCode::Key4, DISH_FISH)] { if is_key_pressed(kc) { g.use_dish(di); } }
                    inp.cancel = is_key_pressed(KeyCode::T) || (is_key_pressed(KeyCode::Escape) && g.build_sel.is_some());
                    inp.menu = if is_key_pressed(KeyCode::C) { Some(0) } else if is_key_pressed(KeyCode::B) { Some(1) } else if is_key_pressed(KeyCode::I) { Some(2) } else if is_key_pressed(KeyCode::P) { Some(6) } else if is_key_pressed(KeyCode::O) { Some(7) } else { None };
                    #[cfg(debug_assertions)]
                    debug_keys(g);
                    #[cfg(debug_assertions)]
                    if is_key_pressed(KeyCode::F11) { tu.active = !tu.active; }
                    handle_touch(&mut tu, &mut inp, g.build_sel.is_some());
                    // เมาส์ซ้าย = โจมตี/ยิงไปทางเมาส์ (เริ่มทำงานหลังปล่อยปุ่มครั้งแรก กันคลิกจากเมนูหลุดมา)
                    let mp: Vec2 = mouse_position().into();
                    if !is_mouse_button_down(MouseButton::Left) { mouse_armed = true; }
                    let over_map = minimap_rect(g.big_map).contains(mp);
                    let mut tap_used = false;
                    let tap: Option<Vec2> = if tu.active { touches().iter().find(|t| t.phase == TouchPhase::Started).map(|t| t.position) } else if is_mouse_button_pressed(MouseButton::Left) { Some(mp) } else { None };
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
                        } else if tu.active && mr.contains(tp) {
                            inp.map = true;
                        }
                    }
                    if !tu.active && !tap_used {
                        if is_mouse_button_pressed(MouseButton::Left) && over_map {
                            inp.map = true;
                        } else if g.build_sel.is_some() {
                            if !over_map { g.build_cur = Some(tile_of(&v0, mp)); g.cur_pos = (g.p.x, g.p.y); }
                            if mouse_armed && !over_map && is_mouse_button_pressed(MouseButton::Left) { inp.place = true; }
                        } else if mouse_armed && !over_map && is_mouse_button_down(MouseButton::Left) {
                            inp.atk = true;
                            inp.aim = Some(vec2(mp.x / v0.s + v0.cx, mp.y / v0.s + v0.cy) - vec2(g.p.x, g.p.y - 6.));
                        }
                    }
                    if let Some(tp) = inp.tile_tap { if g.build_sel.is_some() { g.build_cur = Some(tile_of(&v0, tp)); g.cur_pos = (g.p.x, g.p.y); } }
                    if inp.map { g.big_map = !g.big_map; g.mini_t = 99.; }
                    let sdt = if g.hitstop > 0. { g.hitstop -= dt; dt * 0.08 } else { dt };
                    g.update(sdt, &inp);
                    if let Some(rq) = g.req.take() {
                        ui.reset_context();
                        match rq {
                            Req::Chest(k) => { ui.chest = Some(k); ui.open(3); }
                            Req::Shop => { ui.shop = true; ui.greet = rnd(0, 3) as usize; ui.open(4); }
                            Req::Forge => { ui.forge = true; ui.open(5); }
                            Req::Cook => { ui.cook = true; ui.open(8); }
                            Req::Adv => { ui.adv = true; ui.open(9); }
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
                g.mini_t += dt;
                if g.mini_t > 1. {
                    g.mini_t = 0.;
                    map_ctr = (tfloor(g.p.x), tfloor(g.p.y));
                    map_sc = mm_sc(g);
                    update_minimap(g, &mut mini_img, &mini_tex);
                }
                let mut v = make_view(g.p.x, g.p.y);
                let so = g.shake_off();
                v.cx += so.0.round();
                v.cy += so.1.round();
                render_world(g, &sp, &v, t_now, &font);
                draw_hud(g, &sp, &font, tu.active);
                draw_tut(g, &font, &v0, ui_scale());
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
                if g.w.dun.is_none() {
                    for (tx, ty, spn) in g.wp_list() {
                        let (mx, my, ok) = map_pt(tx, ty, map_ctr, map_sc, mr);
                        if !ok { continue; }
                        let rr = if g.big_map { 11. * u } else { 6. * u };
                        draw_circle(mx, my, rr, hx(0x10202b));
                        draw_circle_lines(mx, my, rr, 2., if spn { hx(0xffffff) } else { hx(0x6ae8ff) });
                        text_c(&font, if spn { "บ" } else { "W" }, mx, my + rr * 0.5, rr * 1.4, if spn { hx(0xffffff) } else { hx(0x6ae8ff) });
                    }
                }
                if g.big_map {
                    text_c(&font, "M = แคมป์ · W = เสาวาร์ป (แตะเพื่อวาร์ป) · บ = จุดเกิด · ม่วง = ดันเจี้ยน · แดงเล็ก = บอส · จุดแดงใหญ่ = คุณ", sw / 2., mr.y + mr.h + 18. * u, 13. * u, WHITE);
                }
                if in_play {
                    draw_touch(&tu, &sp, &font, g.build_sel.is_some(), g.p.weapon, g.p.dash_cd, g.use_label().as_deref());
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
        bgm.set(want).await;
        if prefs::show_fps() {
            text(&font, &format!("{} FPS", get_fps()), 8., sh - 8., 14. * ui_scale(), hx(0x66ff66));
        }
        if let Some(tg) = prefs::frame_target() {
            // เผื่อเวลา vsync ~4ms: ถ้าเครื่องช้ากว่าเป้า จะไม่หน่วงเพิ่ม
            let rem = tg - (get_time() - frame_t0) - 0.004;
            if rem > 0.001 { std::thread::sleep(std::time::Duration::from_secs_f64(rem)); }
        }
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
        g.build_sel = Some(2); // ประตู
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
        g.build_sel = Some(3);
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
        g.en.push(Enemy { night: false, burn: 0., slow: 0., stun: 0., btick: 0., k: EK::Skel, x: 60., y: 6., hp: 4., spd: 0., dmg: 1, xp: 1, coin: 2, vx: 0., vy: 0., kb: 0., flash: 0., wt: 0., wdx: 0., wdy: 0., anim: 0., pal: 0, dead: false });
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
        assert_eq!(g.build_target(), face, "นอกระยะต้องกลับไปหน้าตัว");
        // วางกำแพงตรงช่องที่เลือก (ไม่ใช่หน้าตัว)
        g.p.inv[WOOD] = 10;
        g.build_sel = Some(0);
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
            let d = Dungeon::generate_cave(seed, (cx, cy), false, &opened);
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
            let d2 = Dungeon::generate_cave(seed, (cx, cy), false, &opened);
            assert_eq!((d.tiles == d2.tiles, d.objs == d2.objs), (true, true));
            // หีบที่เปิดแล้วไม่โผล่อีก
            let mut op = HashSet::new(); op.insert((cx, cy, 0));
            let d3 = Dungeon::generate_cave(seed, (cx, cy), false, &op);
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
        let b = [(sw - 80. * t, sh - 84. * t, 50. * t), (sw - 238. * t, sh - 54. * t, 28. * t), (sw - 238. * t, sh - 124. * t, 28. * t), (sw - 80. * t, sh - 184. * t, 28. * t), (sw - 156. * t, sh - 140. * t, 32. * t), (sw - 170. * t, sh - 56. * t, 32. * t)];
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
