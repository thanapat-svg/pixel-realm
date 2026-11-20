// trees.rs — ต้นไม้หลากหลายตามไบโอม (v7d)
//  - ทุ่งหญ้า: โอ๊ก เบิร์ช แอปเปิล เมเปิล ต้นกล้า · ป่าทึบ: โอ๊ก สปรูซ เบิร์ช เมเปิล โอ๊กเก่าแก่ · ชายหาด: มะพร้าว · ทะเลทราย: กระบองเพชร ต้นตาย และมะพร้าวรอบโอเอซิส
//  - หินผา/หิมะ: สนทนต่อความหนาว (สปรูซ/สนหิมะ) และต้นไม้ตาย
//  - แอปเปิล/มะพร้าวเก็บผลได้โดยไม่ต้องโค่น (กด "ใช้") ผลงอกใหม่ใน 2 วัน · มะพร้าว = น้ำดื่มสะอาด
use super::*;

pub const T_BIRCH: u8 = 40;
pub const T_APPLE: u8 = 41;
pub const T_PALM: u8 = 42;
pub const T_SPRUCE: u8 = 43;
pub const T_DEAD: u8 = 44;
pub const T_MAPLE: u8 = 45;
pub const T_OLD: u8 = 46;
pub const T_SAPLING: u8 = 47;
/// ใช้เฉพาะตอนวาด (ต้นที่ผลถูกเก็บไปแล้ว) — ไม่ได้อยู่ในโลกจริง
pub const T_APPLE_BARE: u8 = 48;
pub const T_PALM_BARE: u8 = 49;
pub const FRUIT_DAYS: i32 = 2;

pub fn is_tree(o: u8) -> bool { o == 1 || o == 8 || (40..=47).contains(&o) || o == O_SWTREE || o == O_ACACIA }
pub fn is_fruit_tree(o: u8) -> bool { o == T_APPLE || o == T_PALM }
pub fn tree_name(o: u8) -> &'static str {
    match o { 1 => "ต้นโอ๊ก", 8 => "ต้นสนหิมะ", 40 => "ต้นเบิร์ช", 41 => "ต้นแอปเปิล", 42 => "ต้นมะพร้าว", 43 => "ต้นสปรูซ", 44 => "ต้นไม้ตาย", 45 => "ต้นเมเปิล", 46 => "ต้นโอ๊กเก่าแก่", 47 => "ต้นกล้า", 51 => "ต้นไม้หนองน้ำ", 53 => "ต้นอะคาเซีย", _ => "ต้นไม้" }
}
/// สีเศษไม้/ใบตอนตี
pub fn tree_col(o: u8) -> u32 {
    match o { 40 => 0x78c050, 41 => 0x3f9a3f, 42 => 0x4aae4a, 43 => 0x2e6b46, 44 => 0x8a7866, 45 => 0xd8601e, 46 => 0x2f7a34, 47 => 0x4aa84a, 51 => 0x4a6a32, 53 => 0x7a9a3a, _ => 0x3f9a3f }
}
/// ไม้ที่ได้เมื่อโค่น (ต่ำสุด, สูงสุด) — ต้นเก่าแก่ได้มาก ต้นกล้าได้น้อย
pub fn wood_range(o: u8) -> (i32, i32) {
    match o { 1 => (2, 3), 8 => (2, 2), 40 => (2, 3), 41 => (2, 3), 42 => (2, 2), 43 => (2, 4), 44 => (1, 1), 45 => (2, 3), 46 => (5, 7), 47 => (1, 1), 51 => (2, 3), 53 => (2, 3), _ => (1, 1) }
}

/// เลือกชนิดต้นไม้จากไบโอม (tile) และเลขสุ่ม r2 (0..1) — ตารางสะสม
pub fn pick_species(tile: u8, r2: f64) -> u8 {
    let table: &[(f64, u8)] = match tile {
        3 => &[(0.34, 1), (0.56, T_BIRCH), (0.64, T_APPLE), (0.78, T_SAPLING), (0.92, T_MAPLE), (1.0, T_OLD)],
        4 => &[(0.28, 1), (0.54, T_SPRUCE), (0.70, T_BIRCH), (0.82, T_MAPLE), (0.89, T_OLD), (0.97, T_SAPLING), (1.0, T_APPLE)],
        5 => &[(0.45, T_SPRUCE), (1.0, T_DEAD)],
        7 => &[(0.72, 8), (1.0, T_SPRUCE)],
        8 => &[(1.0, T_DEAD)],
        _ => &[(1.0, 1)],
    };
    for &(c, id) in table { if r2 < c { return id; } }
    table[table.len() - 1].1
}

impl World {
    /// ต้นไม้ธรรมชาติของไทล์นี้ (0 = ไม่มี) — เรียกจาก nat_obj เมื่อ r ตกในช่วงที่มีต้นไม้
    pub fn tree_at(&self, tx: i32, ty: i32, tile: u8) -> u8 {
        pick_species(tile, hash(tx, ty, self.seed.wrapping_add(55)))
    }
    /// มีน้ำในรัศมี 2 ช่อง (ใช้วางมะพร้าวรอบโอเอซิสกลางทะเลทราย)
    pub fn water_near2(&self, tx: i32, ty: i32) -> bool {
        [(1, 0), (-1, 0), (0, 1), (0, -1), (2, 0), (-2, 0), (0, 2), (0, -2)].iter().any(|&(dx, dy)| self.tile_at(tx + dx, ty + dy) < 2)
    }
}

impl Game {
    /// สไปรต์ที่ใช้วาด: แอปเปิล/มะพร้าวที่ผลถูกเก็บไปแล้วจะไม่มีผล
    pub fn display_obj(&self, o: u8, k: (i32, i32)) -> u8 {
        if is_fruit_tree(o) && !self.fruit_ready(k) { if o == T_APPLE { T_APPLE_BARE } else { T_PALM_BARE } } else { o }
    }
    pub fn fruit_ready(&self, k: (i32, i32)) -> bool {
        match self.fruit.get(&k) { None => true, Some(&d) => day_of(self) - d >= FRUIT_DAYS }
    }
    /// กด "ใช้" ที่ต้นแอปเปิล/มะพร้าว: เก็บผลโดยไม่โค่นต้น
    pub fn pick_fruit(&mut self, k: (i32, i32), o: u8) -> bool {
        if !is_fruit_tree(o) { return false; }
        if !self.fruit_ready(k) { self.toast("ผลยังไม่สุก — รอสัก 2 วัน"); return false; }
        let (x, y) = (k.0 as f32 * 16. + 8., k.1 as f32 * 16.);
        if o == T_APPLE {
            let n = rnd(2, 4);
            self.fruit.insert(k, day_of(self));
            self.gain(BERRY, n, x, y);
            self.toast(format!("เก็บแอปเปิลได้ {} ผล — ต้นจะออกผลใหม่ใน {} วัน", n, FRUIT_DAYS));
        } else {
            if self.survival_mode() > 0 && self.p.thirst >= 92. && self.p.hunger >= 92. { self.toast("ยังไม่หิวไม่กระหาย — เก็บไว้ก่อน"); return false; }
            self.fruit.insert(k, day_of(self));
            if self.survival_mode() > 0 { self.hydrate(28.); self.feed(6.); }
            self.add_float(x, y - 4., "มะพร้าว +น้ำ", hx(0x9fe6ff), None);
            self.toast("ดื่มน้ำมะพร้าว — น้ำสะอาด ไม่เค็ม");
            play(&self.sfx.pickup);
        }
        self.burst(x, y + 4., hx(tree_col(o)), 6, 40.);
        self.save();
        true
    }
    pub fn fruit_save_str(&self) -> String {
        let mut s = String::from("fruit");
        let mut v: Vec<(&(i32, i32), &i32)> = self.fruit.iter().collect();
        v.sort();
        for ((x, y), d) in v { s += &format!(" {},{},{}", x, y, d); }
        s += "\n";
        s
    }
    pub fn fruit_load_line(&mut self, rest: &[&str]) {
        for v in rest {
            let n: Vec<i32> = v.split(',').filter_map(|x| x.parse().ok()).collect();
            if n.len() == 3 { self.fruit.insert((n[0], n[1]), n[2]); }
        }
    }
}

pub const BIRCH_ROWS: &[&str] = &[
        "................",
        "......gggg......",
        "....ggGGGGgg....",
        "...gGGGgGGGGg...",
        "..gGGgGGGGGGGg..",
        "..gGGGGGGgGGGg..",
        "..GGGgGGGGGGGG..",
        "..GGGGGGGGgGGd..",
        "...GGGgGGGGGdd..",
        "...dGGGGGGGdd...",
        "....ddGGGGdd....",
        ".....dddddd.....",
        "......WkWW......",
        "......WWWW......",
        "......WWkW......",
        "......WWWW......",
        "......kWWW......",
        "......WWWW......",
        "......WkWW......",
        "......WWWW......",
        ".....WWWWWW.....",
        "....tt....tt....",
    ];
pub const BIRCH_PAL: &[(char, u32)] = &[('g', 0xa8e070), ('G', 0x78c050), ('d', 0x4f9038), ('W', 0xece8de), ('k', 0x3a3a3a), ('t', 0x6a5a48)];

pub const APPLE_ROWS: &[&str] = &[
        "......gggg......",
        "....gGGGGGGg....",
        "...GGGrGGGGGG...",
        "..GGGGGGGrGGGG..",
        ".GGrGGGGGGGGGGG.",
        ".GGGGGGGGGGrGGG.",
        "GGGGGrGGGGGGGGGG",
        "GGGGGGGGGGGGrGGG",
        "GGrGGGGGGGGGGGGG",
        "GGGGGGGrGGGGGGGd",
        ".GGGGGGGGGGGGGd.",
        ".dGGGrGGGGGGGdd.",
        "..ddGGGGGGGGdd..",
        "...dddGGGGddd...",
        ".....ddTTdd.....",
        "......TTTT......",
        "......TTtT......",
        "......TTTT......",
        "......TtTT......",
        "......TTTT......",
        ".....TTTTTT.....",
        "....tt....tt....",
    ];
pub const APPLE_PAL: &[(char, u32)] = &[('g', 0x7cd06a), ('G', 0x3f9a3f), ('d', 0x2a6a2a), ('T', 0x7a4a24), ('t', 0x5a3418), ('r', 0xe03030)];

pub const APPLE_BARE_ROWS: &[&str] = &[
        "......gggg......",
        "....gGGGGGGg....",
        "...GGGGGGGGGG...",
        "..GGGGGGGGGGGG..",
        ".GGGGGGGGGGGGGG.",
        ".GGGGGGGGGGGGGG.",
        "GGGGGGGGGGGGGGGG",
        "GGGGGGGGGGGGGGGG",
        "GGGGGGGGGGGGGGGG",
        "GGGGGGGGGGGGGGGd",
        ".GGGGGGGGGGGGGd.",
        ".dGGGGGGGGGGGdd.",
        "..ddGGGGGGGGdd..",
        "...dddGGGGddd...",
        ".....ddTTdd.....",
        "......TTTT......",
        "......TTtT......",
        "......TTTT......",
        "......TtTT......",
        "......TTTT......",
        ".....TTTTTT.....",
        "....tt....tt....",
    ];
pub const APPLE_BARE_PAL: &[(char, u32)] = &[('g', 0x7cd06a), ('G', 0x3f9a3f), ('d', 0x2a6a2a), ('T', 0x7a4a24), ('t', 0x5a3418), ('r', 0xe03030)];

pub const PALM_ROWS: &[&str] = &[
        "................",
        "..G..GGGGG..G...",
        ".GGGGGGgGGGGGG..",
        "GGgGGGGGGGgGGGG.",
        "GG.GGGGGGGGG.GG.",
        "G...GGGgGGG...G.",
        "...GGccGGccGG...",
        ".....GccccG.....",
        "......ccT.......",
        ".......TT.......",
        ".......Tt.......",
        "......TT........",
        "......Tt........",
        "......TT........",
        ".......TT.......",
        ".......tT.......",
        ".......TT.......",
        "......TTT.......",
        "......Tt........",
        "......TT........",
        ".....TTTT.......",
        "....tt..tt......",
    ];
pub const PALM_PAL: &[(char, u32)] = &[('G', 0x4aae4a), ('g', 0x8ae070), ('T', 0xb28a55), ('t', 0x7a5a34), ('c', 0x6a4a2a)];

pub const PALM_BARE_ROWS: &[&str] = &[
        "................",
        "..G..GGGGG..G...",
        ".GGGGGGgGGGGGG..",
        "GGgGGGGGGGgGGGG.",
        "GG.GGGGGGGGG.GG.",
        "G...GGGgGGG...G.",
        "...GGGGGGGGGG...",
        ".....GGGGGG.....",
        "......GGT.......",
        ".......TT.......",
        ".......Tt.......",
        "......TT........",
        "......Tt........",
        "......TT........",
        ".......TT.......",
        ".......tT.......",
        ".......TT.......",
        "......TTT.......",
        "......Tt........",
        "......TT........",
        ".....TTTT.......",
        "....tt..tt......",
    ];
pub const PALM_BARE_PAL: &[(char, u32)] = &[('G', 0x4aae4a), ('g', 0x8ae070), ('T', 0xb28a55), ('t', 0x7a5a34), ('c', 0x6a4a2a)];

pub const SPRUCE_ROWS: &[&str] = &[
        "................",
        ".......dd.......",
        "......dDDd......",
        ".....dDlDDd.....",
        "......DDDD......",
        ".....dDDlDd.....",
        "....dDDDDDDd....",
        ".....DDlDDD.....",
        "....dDDDDDDd....",
        "...dDDlDDDDDd...",
        "....DDDDDlDD....",
        "...dDDDDDDDDd...",
        "..dDDlDDDDDDDd..",
        "...DDDDDDDlDD...",
        "..dDDDDDDDDDDd..",
        ".dDDDlDDDDDDDDd.",
        "..dddDDDDDDddd..",
        ".....dddTTdd....",
        "......TTTT......",
        "......TtTT......",
        ".....TTTTTT.....",
        "....tt....tt....",
    ];
pub const SPRUCE_PAL: &[(char, u32)] = &[('D', 0x2e6b46), ('d', 0x1f4a32), ('l', 0x4a8a5e), ('T', 0x6a4426), ('t', 0x4a2c16)];

pub const DEAD_ROWS: &[&str] = &[
        "................",
        "..b.........b...",
        "..bB......bBb...",
        "...bB....Bb.....",
        "....bB..Bb..b...",
        ".b...bBBb..bB...",
        ".bB...bB..bB....",
        "..bB..BB.Bb.....",
        "...bBBBBBb......",
        "....bBBBb.......",
        ".....bBB........",
        ".....BBb........",
        "......BB........",
        "......Bb........",
        "......BB........",
        "......bB........",
        "......BB........",
        ".....bBBb.......",
        ".....BBBB.......",
        "....tb..bt......",
    ];
pub const DEAD_PAL: &[(char, u32)] = &[('b', 0x5a4a3c), ('B', 0x8a7866), ('t', 0x3a2e24)];

pub const MAPLE_ROWS: &[&str] = &[
        "................",
        ".....oooyo......",
        "...ooOOoooOo....",
        "..oOOoyOOOOOo...",
        ".oOOOOOOoOOOOo..",
        ".oOyOOOOOOOyOo..",
        "oOOOOOoOOOOOOOo.",
        "OOOOOOOOOyOOOOO.",
        "OOyOOOOOOOOOOOOd",
        "dOOOOOOoOOOOOOdd",
        ".dOOOOOOOOOOOdd.",
        "..ddOOOOyOOOdd..",
        "...ddOOOOOOdd...",
        "....dddOOddd....",
        ".....ddTTdd.....",
        "......TTTT......",
        "......TTtT......",
        "......TTTT......",
        "......TtTT......",
        "......TTTT......",
        ".....TTTTTT.....",
        "....tt....tt....",
    ];
pub const MAPLE_PAL: &[(char, u32)] = &[('o', 0xf0a040), ('O', 0xd8601e), ('y', 0xf8d060), ('d', 0x9a3a14), ('T', 0x6a5646), ('t', 0x4a3a2e)];

pub const OLD_ROWS: &[&str] = &[
        "....gggggggg....",
        "..ggGGGGGGGGgg..",
        ".gGGGgGGGGGGGGg.",
        "gGGGGGGGGgGGGGGg",
        "GGgGGGGGGGGGGGGG",
        "GGGGGGGGGGGgGGGG",
        "GGGGgGGGGGGGGGGG",
        "GGGGGGGGGGGGGgGG",
        "dGGGGGGgGGGGGGGd",
        "dGGGGGGGGGGGGGGd",
        ".dGGGGGGGGGGGGd.",
        ".ddGGGGgGGGGGdd.",
        "..dddGGGGGGddd..",
        "...ddddTTTdddd..",
        "....TTTTTTTTT...",
        ".....TTkTTTT....",
        ".....TTkkTTT....",
        ".....TTTkTTT....",
        ".....TTTTTTT....",
        "....TTTTTTTTT...",
        "...TTTTTTTTTTT..",
        "..ttt.t..t.ttt..",
    ];
pub const OLD_PAL: &[(char, u32)] = &[('g', 0x68b85a), ('G', 0x2f7a34), ('d', 0x1f5a26), ('T', 0x6a4426), ('t', 0x4a2c16), ('k', 0x24160c)];

pub const SAPLING_ROWS: &[&str] = &[
        "................",
        "................",
        "................",
        "................",
        "................",
        "......gg........",
        ".....gGGg.......",
        "....gGGGGg......",
        ".....gGgg.......",
        "......gT........",
        "......T.........",
        "......TT........",
        "......T.........",
        ".....tTt........",
    ];
pub const SAPLING_PAL: &[(char, u32)] = &[('g', 0x8ad870), ('G', 0x4aa84a), ('T', 0x7a4a24), ('t', 0x5a3418)];

/// สร้างสไปรต์ต้นไม้ใหม่ลงแอตลาส (obj[40..50])
pub fn build_trees(at: &mut Atlas, obj: &mut Vec<Rect>) {
    let mut put = |id: u8, rows: &[&str], pal: &[(char, u32)]| { obj[id as usize] = at.add(&spr(rows, pal)); };
    put(T_BIRCH, BIRCH_ROWS, BIRCH_PAL);
    put(T_APPLE, APPLE_ROWS, APPLE_PAL);
    put(T_PALM, PALM_ROWS, PALM_PAL);
    put(T_SPRUCE, SPRUCE_ROWS, SPRUCE_PAL);
    put(T_DEAD, DEAD_ROWS, DEAD_PAL);
    put(T_MAPLE, MAPLE_ROWS, MAPLE_PAL);
    put(T_OLD, OLD_ROWS, OLD_PAL);
    put(T_SAPLING, SAPLING_ROWS, SAPLING_PAL);
    put(T_APPLE_BARE, APPLE_BARE_ROWS, APPLE_BARE_PAL);
    put(T_PALM_BARE, PALM_BARE_ROWS, PALM_BARE_PAL);
}
