//! สไปรต์พิกเซลเพิ่มเติม (วาดด้วยโค้ดทั้งหมด ไม่มีไฟล์รูป)
//! ไอคอนไอเทม · สิ่งปลูกสร้าง · เต็นท์ · พ่อค้า · บอส · โปรเจกไทล์

use super::*;

pub struct Extras {
    pub king: Rect,
    pub golem: [Rect; 2], // [ปกติ, ยกแขน]
    pub merchant: [Rect; 2],
    /// เรียงตาม kind ของ Proj: 0 ธนู, 1 เวท, 2 เมือก, 3 ก้อนหิน, 4 เวทราชา, 5 เศษหิน
    pub proj: [Rect; 10],
    /// ผู้ติดตาม: [หมาป่า เฟรม0,1, อัศวิน เฟรม0,1]
    pub comp: [Rect; 4],
    /// ภาพเงาขาว (แฟลชตอนโดนตี): สไลม์ x4, โครงกระดูก, ค้างคาว x2
    pub flash_slime: [Rect; 4],
    pub flash_skel: Rect,
    pub flash_bat: [Rect; 2],
    /// v38 กำแพงเมืองแนวตั้ง (มองจากบน: สันกำแพงต่อกันเป็นเส้น)
    pub cwall_v: Rect,
}

fn blank(w: u16, h: u16) -> Image {
    Image::gen_image_color(w, h, BLANK)
}
/// ภาพเงาสีขาว (ใช้แฟลชตอนโดนตี)
pub fn white_of(im: &Image) -> Image {
    let mut o = im.clone();
    for y in 0..im.height as u32 {
        for x in 0..im.width as u32 {
            if im.get_pixel(x, y).a > 0. {
                o.set_pixel(x, y, Color::new(1., 1., 1., 1.));
            }
        }
    }
    o
}
fn px(im: &mut Image, x: i32, y: i32, c: u32) {
    if x >= 0 && y >= 0 && (x as u32) < im.width as u32 && (y as u32) < im.height as u32 {
        im.set_pixel(x as u32, y as u32, hx(c));
    }
}
fn rect(im: &mut Image, x: i32, y: i32, w: i32, h: i32, c: u32) {
    for j in 0..h {
        for i in 0..w {
            px(im, x + i, y + j, c);
        }
    }
}
/// ใส่เส้นขอบ 1px รอบส่วนที่ไม่โปร่งใส
fn outline(im: &mut Image, c: u32) {
    let (w, h) = (im.width as i32, im.height as i32);
    let mut solid = vec![false; (w * h) as usize];
    for y in 0..h {
        for x in 0..w {
            solid[(y * w + x) as usize] = im.get_pixel(x as u32, y as u32).a > 0.;
        }
    }
    let at = |x: i32, y: i32| x >= 0 && y >= 0 && x < w && y < h && solid[(y * w + x) as usize];
    for y in 0..h {
        for x in 0..w {
            if !at(x, y) && (at(x - 1, y) || at(x + 1, y) || at(x, y - 1) || at(x, y + 1)) {
                im.set_pixel(x as u32, y as u32, hx(c));
            }
        }
    }
}
/// ก้อนสี่เหลี่ยมมีไฮไลต์/เงา ใช้วาดโกเลม
fn block(im: &mut Image, x: i32, y: i32, w: i32, h: i32, base: u32, light: u32, dark: u32, line: u32) {
    rect(im, x, y, w, h, line);
    rect(im, x + 1, y + 1, w - 2, h - 2, base);
    rect(im, x + 1, y + 1, w - 2, 1, light);
    rect(im, x + 1, y + 1, 1, h - 2, light);
    rect(im, x + 1, y + h - 2, w - 2, 1, dark);
    rect(im, x + w - 2, y + 1, 1, h - 2, dark);
}
fn orb(d: u16, inner: u32, mid: u32, outer: u32) -> Image {
    let mut im = blank(d, d);
    let c = d as f32 / 2.;
    for y in 0..d as i32 {
        for x in 0..d as i32 {
            let r = ((x as f32 + 0.5 - c).hypot(y as f32 + 0.5 - c)) / c;
            if r <= 1.0 {
                px(&mut im, x, y, if r < 0.35 { inner } else if r < 0.7 { mid } else { outer });
            }
        }
    }
    im
}

// ---------------- สิ่งปลูกสร้าง ----------------
fn wall_wood() -> Image {
    let mut im = blank(16, 16);
    rect(&mut im, 0, 0, 16, 16, 0x4a2e14);
    rect(&mut im, 1, 1, 14, 14, 0x9a6a38);
    for x in [4, 8, 12] {
        rect(&mut im, x, 1, 1, 14, 0x7a4e26);
    }
    rect(&mut im, 1, 1, 14, 1, 0xc08a50);
    rect(&mut im, 1, 14, 14, 1, 0x6a4220);
    for x in [2, 6, 10, 14] {
        px(&mut im, x, 3, 0x3a2410);
        px(&mut im, x, 12, 0x3a2410);
    }
    for i in 0..10 {
        px(&mut im, 1 + (hash(i, 1, 7) * 13.) as i32, 2 + (hash(1, i, 9) * 11.) as i32, 0xb07a44);
    }
    im
}
fn wall_stone() -> Image {
    let mut im = blank(16, 16);
    rect(&mut im, 0, 0, 16, 16, 0x3a3a44);
    rect(&mut im, 1, 1, 14, 14, 0x9a9aa6);
    for y in [5, 10] {
        rect(&mut im, 1, y, 14, 1, 0x62626e);
    }
    rect(&mut im, 8, 1, 1, 4, 0x62626e);
    rect(&mut im, 4, 6, 1, 4, 0x62626e);
    rect(&mut im, 12, 6, 1, 4, 0x62626e);
    rect(&mut im, 8, 11, 1, 4, 0x62626e);
    rect(&mut im, 1, 1, 14, 1, 0xc4c4d0);
    rect(&mut im, 1, 6, 3, 1, 0xb4b4c0);
    rect(&mut im, 9, 11, 5, 1, 0xb4b4c0);
    rect(&mut im, 1, 14, 14, 1, 0x70707c);
    im
}
fn door(open: bool) -> Image {
    let mut im = blank(16, 16);
    if !open {
        rect(&mut im, 0, 0, 16, 16, 0x3a2410);
        rect(&mut im, 2, 2, 12, 14, 0xa06a30);
        for x in [5, 8, 11] {
            rect(&mut im, x, 2, 1, 14, 0x7a4e26);
        }
        rect(&mut im, 2, 2, 12, 1, 0xc08a50);
        rect(&mut im, 2, 5, 12, 1, 0x5a3a1c);
        rect(&mut im, 2, 12, 12, 1, 0x5a3a1c);
        rect(&mut im, 11, 8, 2, 2, 0xffd84a);
    } else {
        rect(&mut im, 0, 0, 16, 2, 0x5a3a1c);
        rect(&mut im, 0, 0, 2, 16, 0x5a3a1c);
        rect(&mut im, 14, 0, 2, 16, 0x5a3a1c);
        rect(&mut im, 2, 2, 3, 13, 0xa06a30);
        rect(&mut im, 2, 2, 1, 13, 0xc08a50);
        rect(&mut im, 4, 8, 1, 2, 0xffd84a);
        outline(&mut im, 0x3a2410);
    }
    im
}
fn storage_box() -> Image {
    let mut im = blank(16, 16);
    rect(&mut im, 1, 5, 14, 10, 0x8a5a2a);
    rect(&mut im, 1, 3, 14, 4, 0xa87a3a);
    rect(&mut im, 1, 3, 14, 1, 0xd0a060);
    rect(&mut im, 1, 7, 14, 1, 0x5a3a1c);
    for x in [3, 11] {
        rect(&mut im, x, 3, 2, 12, 0x8a96b0);
        rect(&mut im, x, 3, 1, 12, 0xb4bed4);
    }
    rect(&mut im, 7, 7, 2, 3, 0xffd84a);
    px(&mut im, 7, 7, 0xfff2a0);
    rect(&mut im, 1, 14, 14, 1, 0x5a3a1c);
    outline(&mut im, 0x2a160a);
    im
}
fn tent() -> Image {
    let mut im = blank(16, 16);
    rect(&mut im, 8, 0, 1, 3, 0x5a3a1e);
    px(&mut im, 9, 0, 0xe04040);
    px(&mut im, 9, 1, 0xe04040);
    for y in 2..15 {
        let half = 1 + (y - 2) * 7 / 12;
        for x in (8 - half)..(8 + half) {
            let stripe = ((x + 16) / 3) % 2 == 0;
            px(&mut im, x, y, if stripe { 0xd04a3a } else { 0xf2e8d0 });
        }
    }
    for y in 8..15 {
        let hw = 1 + (y - 8) * 2 / 6;
        rect(&mut im, 8 - hw, y, hw * 2, 1, 0x3a1a10);
    }
    outline(&mut im, 0x4a1a1a);
    im
}
fn merchant(wave: bool) -> Image {
    let mut im = blank(16, 22);
    // เป้สะพายหลัง
    rect(&mut im, 0, 11, 3, 8, 0x7a4a24);
    rect(&mut im, 13, 11, 3, 8, 0x7a4a24);
    rect(&mut im, 0, 11, 3, 1, 0x9a6a38);
    rect(&mut im, 13, 11, 3, 1, 0x9a6a38);
    // ลำตัว
    rect(&mut im, 3, 13, 10, 8, 0x2a8a8a);
    rect(&mut im, 3, 13, 10, 1, 0x3aaaaa);
    rect(&mut im, 7, 14, 2, 7, 0x1e6a6a);
    rect(&mut im, 3, 17, 10, 1, 0x5a3a1e);
    px(&mut im, 8, 17, 0xffd84a);
    // แขน
    rect(&mut im, 1, 14, 2, 5, 0x2a8a8a);
    rect(&mut im, 1, 19, 2, 1, 0xf2c79a);
    if wave {
        rect(&mut im, 13, 8, 2, 6, 0x2a8a8a);
        rect(&mut im, 13, 7, 2, 1, 0xf2c79a);
    } else {
        rect(&mut im, 13, 14, 2, 5, 0x2a8a8a);
        rect(&mut im, 13, 19, 2, 1, 0xf2c79a);
    }
    // ผ้าพันคอ + หัว
    rect(&mut im, 4, 12, 8, 2, 0xd04040);
    rect(&mut im, 5, 7, 6, 5, 0xf2c79a);
    px(&mut im, 6, 9, 0x222222);
    px(&mut im, 9, 9, 0x222222);
    rect(&mut im, 5, 11, 6, 1, 0xdcdcdc);
    // หมวก
    rect(&mut im, 2, 5, 12, 2, 0x8a5a2a);
    rect(&mut im, 4, 1, 8, 5, 0xa06a30);
    rect(&mut im, 4, 4, 8, 1, 0xe0b040);
    rect(&mut im, 4, 1, 8, 1, 0xc08a50);
    // รองเท้า
    rect(&mut im, 4, 20, 3, 2, 0x3a2414);
    rect(&mut im, 9, 20, 3, 2, 0x3a2414);
    outline(&mut im, 0x1d1410);
    im
}

// ---------------- บอส ----------------
fn king() -> Image {
    let (w, h) = (32u16, 26u16);
    let mut im = blank(w, h);
    let (cx, cy, rx, ry) = (16.0f32, 15.5f32, 14.5f32, 10.5f32);
    let inside = |x: i32, y: i32| -> bool {
        let dx = (x as f32 + 0.5 - cx) / rx;
        let dy = (y as f32 + 0.5 - cy) / ry;
        dx * dx + dy * dy <= 1.0 && y < 25
    };
    for y in 0..h as i32 {
        for x in 0..w as i32 {
            if !inside(x, y) { continue; }
            let edge = !inside(x - 1, y) || !inside(x + 1, y) || !inside(x, y - 1) || !inside(x, y + 1);
            let dx = (x as f32 + 0.5 - cx) / rx;
            let dy = (y as f32 + 0.5 - cy) / ry;
            let t = dx * 0.5 + dy * 0.85;
            let c = if edge { 0x1d6b2b } else if t > 0.45 { 0x2e9a3c } else if t < -0.5 { 0x8af08a } else { 0x4cc24c };
            px(&mut im, x, y, c);
        }
    }
    rect(&mut im, 6, 9, 4, 2, 0xc8ffc8);
    px(&mut im, 5, 11, 0xc8ffc8);
    for &(bx, by) in &[(23, 19), (8, 20), (25, 15), (20, 21)] {
        px(&mut im, bx, by, 0xa8f8a8);
        px(&mut im, bx + 1, by, 0x2e9a3c);
    }
    // ตา
    for ex in [9, 18] {
        rect(&mut im, ex, 12, 5, 6, 0xffffff);
        rect(&mut im, ex, 12, 5, 1, 0xc8c8c8);
        rect(&mut im, ex + 2, 14, 2, 3, 0x102010);
    }
    // คิ้วโกรธ
    for i in 0..4 {
        px(&mut im, 9 + i, 10 + i / 2, 0x103010);
        px(&mut im, 22 - i, 10 + i / 2, 0x103010);
    }
    // ปากมีเขี้ยว
    rect(&mut im, 11, 20, 10, 1, 0x103010);
    px(&mut im, 12, 21, 0xffffff);
    px(&mut im, 19, 21, 0xffffff);
    // มงกุฎ
    rect(&mut im, 9, 3, 14, 3, 0xffd84a);
    rect(&mut im, 9, 5, 14, 1, 0xc89a20);
    for sx in [9, 15, 21] {
        rect(&mut im, sx, 1, 3, 2, 0xffd84a);
        px(&mut im, sx + 1, 0, 0xffd84a);
    }
    px(&mut im, 16, 4, 0xe83a4a);
    px(&mut im, 11, 4, 0x4ac8ff);
    px(&mut im, 21, 4, 0x4ac8ff);
    rect(&mut im, 9, 3, 14, 1, 0xfff2a0);
    im
}
fn golem(raised: bool) -> Image {
    let (base, light, dark, line) = (0x8a8a96, 0xb4b4c2, 0x5e5e6c, 0x25252e);
    let mut im = blank(32, 34);
    block(&mut im, 8, 24, 6, 10, base, light, dark, line);
    block(&mut im, 18, 24, 6, 10, base, light, dark, line);
    block(&mut im, 7, 10, 18, 16, base, light, dark, line);
    if raised {
        block(&mut im, 1, 0, 7, 14, base, light, dark, line);
        block(&mut im, 24, 0, 7, 14, base, light, dark, line);
    } else {
        block(&mut im, 1, 11, 6, 15, base, light, dark, line);
        block(&mut im, 25, 11, 6, 15, base, light, dark, line);
        block(&mut im, 0, 25, 8, 8, base, light, dark, line);
        block(&mut im, 24, 25, 8, 8, base, light, dark, line);
    }
    block(&mut im, 11, 1, 10, 10, base, light, dark, line);
    // ตาเรืองแสง
    rect(&mut im, 13, 5, 2, 2, 0x4ef0ff);
    rect(&mut im, 17, 5, 2, 2, 0x4ef0ff);
    px(&mut im, 13, 4, 0x1a7a98);
    px(&mut im, 18, 4, 0x1a7a98);
    // แกนพลังงาน
    rect(&mut im, 14, 16, 4, 4, 0x4ef0ff);
    rect(&mut im, 15, 17, 2, 2, 0xffffff);
    rect(&mut im, 13, 15, 6, 1, 0x1a7a98);
    rect(&mut im, 13, 20, 6, 1, 0x1a7a98);
    // มอสและรอยร้าว
    rect(&mut im, 8, 11, 5, 2, 0x4a8a4a);
    rect(&mut im, 20, 11, 4, 1, 0x4a8a4a);
    rect(&mut im, 12, 2, 3, 1, 0x4a8a4a);
    for &(cx, cy) in &[(10, 15), (11, 16), (11, 17), (12, 18), (21, 14), (22, 15), (22, 16), (9, 28), (10, 29)] {
        px(&mut im, cx, cy, 0x3a3a46);
    }
    im
}

// ---------------- ลงทะเบียนทั้งหมดลง atlas ----------------
pub fn build_extra(at: &mut Atlas, obj: &mut Vec<Rect>, icons: &mut Vec<Rect>) -> Extras {
    obj[10] = at.add(&wall_wood());
    obj[11] = at.add(&wall_stone());

    // ---- เฟอร์นิเจอร์ (v13.1)
    let fp: &[(char, u32)] = &[('k', 0x3a2410), ('T', 0x9a6a38), ('t', 0x7a4e26), ('d', 0x5a3418), ('y', 0xffd84a), ('Y', 0xfff2a0), ('w', 0xffffff), ('r', 0xb8343a), ('R', 0x8a2028), ('g', 0x5ab04a), ('G', 0x3a8a3a), ('b', 0x3a6ad8), ('o', 0xb0603a), ('O', 0x8a4428)];
    obj[O_TABLE as usize] = at.add(&spr(&["................", "................", "................", "..kkkkkkkkkkkk..", ".kTTTTTTTTTTTTk.", ".kTttTTTTTTTTTk.", ".kTTTTTTTTttTTk.", ".kTTTTTTTTTTTTk.", ".kTTTtTTTTTTTTk.", ".kddddddddddddk.", ".kkkkkkkkkkkkkk.", "..kd........dk..", "..kd........dk..", "..kk........kk..", "................", "................"], fp));
    obj[O_CHAIR as usize] = at.add(&spr(&["................", "................", ".....kkkkkk.....", ".....kTTTTk.....", ".....kTttTk.....", ".....kTTTTk.....", ".....kkkkkk.....", "....kTTTTTTk....", "....kTTTTTTk....", "....kddddddk....", "....kkkkkkkk....", "....kd....dk....", "....kd....dk....", "....kk....kk....", "................", "................"], fp));
    obj[O_RUG as usize] = at.add(&spr(&["................", ".RRRRRRRRRRRRRR.", ".RyyyyyyyyyyyyR.", ".yrrrrrrrrrrrry.", ".yrRRRRRRRRRRry.", ".yrRyyyyyyyyRry.", ".yrRyrrrrrryRry.", ".yrRyrRRRRryRry.", ".yrRyrRRRRryRry.", ".yrRyrrrrrryRry.", ".yrRyyyyyyyyRry.", ".yrRRRRRRRRRRry.", ".yrrrrrrrrrrrry.", ".RyyyyyyyyyyyyR.", ".RRRRRRRRRRRRRR.", "................"], fp));
    obj[O_SHELF as usize] = at.add(&spr(&["..kkkkkkkkkkkk..", "..kTTTTTTTTTTk..", "..kTrbgyrbgyTk..", "..kTrbgyrbgyTk..", "..kTrbgyrbgyTk..", "..kTTTTTTTTTTk..", "..kTgyrbbgryTk..", "..kTgyrbbgryTk..", "..kTgyrbbgryTk..", "..kTTTTTTTTTTk..", "..kTbrygbryyTk..", "..kTbrygbryyTk..", "..kTbrygbryyTk..", "..kTTTTTTTTTTk..", "..kkkkkkkkkkkk..", "................"], fp));
    obj[O_PLANT as usize] = at.add(&spr(&["................", "......gG........", "....gGGgG.Gg....", "...GgGGGGgGGg...", "..gGGgGGGGGgGG..", "...GGGGgGGGGg...", "..gGgGGGGGgGGg..", "...ggGGGGGGgg...", ".....gGGGGg.....", "....kkkkkkkk....", "....kooooook....", "....kOooooOk....", ".....kooook.....", ".....kOOOOk.....", "......kkkk......", "................"], fp));
    obj[O_LAMP as usize] = at.add(&spr(&["................", ".....yyyyyy.....", "....yYYYYYYy....", "...yYYwwwwYYy...", "...yYYYYYYYYy...", "....yyyyyyyy....", ".......kk.......", ".......kk.......", ".......kk.......", ".......kk.......", ".......kk.......", ".......kk.......", ".......kk.......", ".....kkkkkk.....", "....kddddddk....", "................"], fp));
    obj[O_WARDROBE as usize] = at.add(&spr(&["..kkkkkkkkkkkk..", "..kTTTTTTTTTTk..", "..kTttTTkTttTk..", "..kTTTTTkTTTTk..", "..kTTTTTkTTTTk..", "..kTTTTTkTTTTk..", "..kTTTyTkTyTTk..", "..kTTTTTkTTTTk..", "..kTTTTTkTTTTk..", "..kTttTTkTttTk..", "..kTTTTTkTTTTk..", "..kTTTTTkTTTTk..", "..kddddddddddk..", "..kkkkkkkkkkkk..", "..kk........kk..", "................"], fp));
    // ---- วัตถุในเมือง (v17)
    let cp: &[(char, u32)] = &[('k', 0x3a2410), ('T', 0x9a6a38), ('t', 0x7a4e26), ('r', 0xd84a3a), ('w', 0xf0e6d0), ('y', 0xe8c040), ('g', 0x7ab04a), ('o', 0xe08a30), ('s', 0x8a8a92), ('d', 0x5a3418)];
    obj[O_STALL as usize] = at.add(&spr(&["rrwwrrwwrrwwrrww", "rrwwrrwwrrwwrrww", "rrwwrrwwrrwwrrww", ".k............k.", ".k............k.", ".k..y.g.o.y...k.", ".k.yyygggooo..k.", "kkkkkkkkkkkkkkkk", "kTTTTTTTTTTTTTTk", "kTtTTTTTTTTTtTTk", "kTTTTTTTTTTTTTTk", "kTTTTTTTTTTTTTTk", "kkkkkkkkkkkkkkkk", ".kd..........dk.", ".kd..........dk.", ".kk..........kk."], cp));
    obj[O_STABLE as usize] = at.add(&spr(&["................", "..kkkkkkkkkkkk..", ".kTTTTTTTTTTTTk.", "kTTTTTTTTTTTTTTk", "kkkkkkkkkkkkkkkk", ".k............k.", ".k..yy....yy..k.", ".k.yyyy..yyyy.k.", ".kyyyyyyyyyyyyk.", "kkkkkkkkkkkkkkkk", ".kT.kT.kT.kT.kT.", "kkkkkkkkkkkkkkkk", ".kT.kT.kT.kT.kT.", ".kT.kT.kT.kT.kT.", ".kk.kk.kk.kk.kk.", "................"], cp));
    obj[O_COACH as usize] = at.add(&spr(&["......kkkk......", ".....kTTTTk.....", "....kTTTTTTk....", "....kkkkkkkk....", ".......kk.......", "..kkkkkkkkkkkk..", "..kwwwwwwwwwwk..", "..kwyyywwyyywk..", "..kwwwwwwwwwwk..", "..kkkkkkkkkkkk..", ".......kk.......", "....ksssk.......", "...ks.k.sk......", "...kskkksk......", "...ks.k.sk......", "....ksssk......."], cp));
    // โอ่งน้ำดินเผา (ส่วนตัว)
    let jp: &[(char, u32)] = &[('k', 0x4a2a14), ('c', 0xb0643a), ('C', 0xd88a52), ('d', 0x8a4a28), ('w', 0x4a8ad8), ('W', 0x9ac8f8)];
    obj[O_WJAR as usize] = at.add(&spr(&["................", "................", "................", "....kkkkkkkk....", "...kwWwwwwwwk...", "...kkkkkkkkkk...", "....kcCccccdk...", "...kcCcccccdk...", "..kcCccccccddk..", "..kcCccccccddk..", "..kcCcckkcccdk..", "..kcccccccccdk..", "...kccccccddk...", "....kddddddk....", ".....kkkkkk.....", "................"], jp));
    // บ่อน้ำกลางเมือง (หินใหญ่ หลังคา รอกตักน้ำ) กว้าง 32
    let bp: &[(char, u32)] = &[('k', 0x2a2a30), ('s', 0x8a8e98), ('S', 0xb8bcc6), ('d', 0x5a5e68), ('r', 0xa84a32), ('R', 0xd8724a), ('t', 0x6a4022), ('w', 0x3a78c8), ('W', 0x8ac0f0), ('y', 0xd8b040)];
    obj[O_BIGWELL as usize] = at.add(&spr(&[
        "........kkkkkkkkkkkkkkkk........",
        "......kkRRRRRRRRRRRRRRRRkk......",
        "....kkRRrrRRrrRRrrRRrrRRRRkk....",
        "...kRRrrRRrrRRrrRRrrRRrrRRRRk...",
        "..kkkkkkkkkkkkkkkkkkkkkkkkkkkk..",
        ".....kt.......kyk.......tk......",
        ".....kt.......kyk.......tk......",
        ".....kt........y........tk......",
        ".....kt........y........tk......",
        ".....kt.......kkk.......tk......",
        "...kkkkkkkkkkkkkkkkkkkkkkkkkk...",
        "..kSSsSSsSSsSSsSSsSSsSSsSSsSSk..",
        "..ksWwwwwWwwwwwwWwwwwwwWwwwwsk..",
        "..kswwWwwwwwWwwwwwwWwwwwwwWwsk..",
        "..kSSsSSsSSsSSsSSsSSsSSsSSsSSk..",
        "..ksdsdsdsdsdsdsdsdsdsdsdsdsdk..",
        "..kSsSsSsSsSsSsSsSsSsSsSsSsSsk..",
        "..ksdsdsdsdsdsdsdsdsdsdsdsdsdk..",
        "..kddddddddddddddddddddddddddk..",
        "...kkkkkkkkkkkkkkkkkkkkkkkkkk...",
    ], bp));
    obj[O_BANNER as usize] = at.add(&spr(&["................", "................", "................", "................", "................", "................", "................", "................", "................", "................", "................", "................", "................", "................", "................", "................"], cp));
    obj[O_LOG as usize] = at.add(&spr(&["................", "................", "................", "................", "................", "..kkkkkkkkkkkk..", ".kTTTtTTTTTtTTTk", "kooOkTTTTTTTTtTk", "koOokTtTTTTTTTTk", "kOookTTTTtTTTTTk", ".kkkkddddddddddk", "..kkkkkkkkkkkkk.", "................", "................", "................", "................"], &[('k', 0x3a2410), ('T', 0x8a5a2c), ('t', 0x6a4220), ('d', 0x5a3418), ('o', 0xd8b080), ('O', 0xa87844)]));
    obj[O_TENTWALL as usize] = at.add(&{
        let mut im = Image::gen_image_color(16, 16, hx(0xd8c8a0));
        for x in 0..16 { if x % 4 == 0 { rect(&mut im, x, 0, 1, 16, 0xc0ac80); } }
        rect(&mut im, 0, 0, 16, 2, 0xa08860);
        rect(&mut im, 0, 14, 16, 2, 0x8a7450);
        rect(&mut im, 7, 2, 2, 12, 0xb09a70);
        im
    });
    obj[O_WINDOW as usize] = at.add(&{
        let mut im = wall_wood();
        rect(&mut im, 3, 3, 10, 9, 0x4a2e14);
        rect(&mut im, 4, 4, 8, 7, 0x9ad8ff);
        rect(&mut im, 5, 5, 2, 2, 0xe0f6ff);
        rect(&mut im, 7, 4, 2, 7, 0x6a4220);
        rect(&mut im, 4, 7, 8, 1, 0x6a4220);
        rect(&mut im, 2, 12, 12, 2, 0x7a4e26);
        im
    });
    obj[12] = at.add(&door(false));
    obj[13] = at.add(&door(true));
    obj[14] = at.add(&storage_box());
    let merch = [at.add(&merchant(false)), at.add(&merchant(true))];
    obj[15] = merch[0];
    obj[16] = at.add(&tent());

    obj[17] = at.add(&spr(&["................", "................", "................", "................", "..kkkkkkkkkkkk..", ".kLLLLLLLLLLLLk.", "kLllLLLLLLLLLDDk", "kkLLLLLLLLLLDDkk", "..kkLLLLLLDDkk..", "....kLLLLDDk....", "....kLLLLDDk....", "....kLLLLDDk....", "...kkLLLLDDkk...", "..kLLLLLLLLDDk..", "..kkkkkkkkkkkk..", "................"], &[('k', 0x2a2a32), ('L', 0x8a909c), ('l', 0xc8ced8), ('D', 0x5a6070)]));
    obj[18] = at.add(&spr(&["................", "................", ".......y........", "......yoy.......", ".....yoroy......", ".....yorRy......", "......orro......", ".......rr.......", "......tTTt......", ".......TT.......", ".......TT.......", ".......Tt.......", ".......TT.......", ".......TT.......", "......tTTt......", "................"], &[('y', 0xffe680), ('o', 0xffa630), ('r', 0xff5a1a), ('R', 0xfff2a0), ('T', 0x8a5a32), ('t', 0x5a3418)]));
    obj[19] = at.add(&spr(&["................", "................", "....kkkkkkkk....", "..kkssssssssskk.", ".kssbbbbbbbbbssk", ".ksbbbbbbbbbbbsk", "ksbbbppbbppbbbsk", "ksbbbbbbbbbbbbsk", "ksbbppbbbbppbbsk", "ksbbbbbbbbbbbbsk", "kssbbbbbbbbbbssk", ".kssssbbbbssssk.", "..kkssssssssskk.", "....kkkkkkkk....", "................", "................"], &[('k', 0x1a1a22), ('s', 0x7a7a88), ('b', 0x0a0a12), ('p', 0x7a4aff)]));
    obj[20] = at.add(&spr(&[".......yy.......", "......yyyy......", "....wwwwwwww....", "...wkkkkkkkkw...", "...wwwwwwwwww...", "..wkkkkkkkkkkw..", "..wwwwwwwwwwww..", ".wkkkkkkkkkkkkw.", ".wwwwwwwwwwwwww.", "wkkkkkkkkkkkkkkw", "wwwwwwwwwwwwwwww", "................", "................", "................", "................", "................"], &[('w', 0xa0a0b0), ('k', 0x40404c), ('y', 0xffd84a)]));
    let sp_pal: &[(char, u32)] = &[('k', 0x2a2a32), ('d', 0x55555f), ('S', 0xd8dce8), ('T', 0xffffff)];
    obj[21] = at.add(&spr(&["................", "................", "................", "................", "................", "..kkkkkkkkkkkk..", "..kddddddddddk..", "..kdkdddddkddk..", "..kddddddddddk..", "..kdkdddddkddk..", "..kddddddddddk..", "..kkkkkkkkkkkk..", "................", "................", "................", "................"], sp_pal));
    obj[22] = at.add(&spr(&["................", "................", "....T.....T.....", "...SSS...SSS....", "...SSS...SSS....", "..kkkkkkkkkkkk..", "..kddddddddddk..", "..kdSdddddSddk..", "..kddddddddddk..", "..kdSdddddSddk..", "..kddddddddddk..", "..kkkkkkkkkkkk..", "................", "................", "................", "................"], sp_pal));
    obj[23] = at.add(&spr(&["................", "................", "....kkkkkkkk....", "...kssssssssk...", "..kssLLLLLLssk..", "..ksLrLLLLrLsk..", "..ksLLLLLLLLsk..", "..kssLooooLssk..", "..kssLLLLLLssk..", "...kssssssssk...", "...kDDDDDDDDk...", "..kDDDDDDDDDDk..", "..kDDDDDDDDDDk..", "..kkkkkkkkkkkk..", "................", "................"], &[('k', 0x1e1e26), ('s', 0x7a7a88), ('L', 0xa4a4b4), ('D', 0x55556a), ('r', 0xff3030), ('o', 0xff9a20)]));
    obj[25] = at.add(&spr(&["................", "......cc........", ".....cCCc.......", ".....cCwc.......", "......cc........", "..kkkkkkkkkk....", "..ksssssssSk....", "...ksSsssSk.....", "...ksssssSk.....", "...ksSsssSk.....", "...ksssssSk.....", "..kksssssSkk....", ".kssssssssSsk...", ".kkkkkkkkkkkk...", "................", "................"], &[('k', 0x2a2a3a), ('s', 0x8a8ea0), ('S', 0xc0c4d4), ('c', 0x3ad0f0), ('C', 0xa0f0ff), ('w', 0xffffff)]));
    obj[26] = at.add(&spr(&["................", "................", "................", "....y..y..y.....", "...y..y..y......", "..kkkkkkkkkk....", ".kKKKKKKKKKKk...", "kKKrrrrrrrrKKk..", "kKKKKKKKKKKKKk..", "kKKKKKKKKKKKKk..", ".kKKKKKKKKKKk...", "..kkkkkkkkkk....", "..t.t....t.t....", ".tFtFt..tFtFt...", "................", "................"], &[('k', 0x22222a), ('K', 0x5a5a68), ('r', 0xc86a2a), ('y', 0xe8e8e8), ('t', 0x5a3418), ('F', 0xff7a1a)]));
    obj[24] = at.add(&spr(&["................", "................", "................", "................", "...oooooooooo...", "..oyyyyyyyyyyo..", "..oyYYYYYYYYyo..", "..oyyyyyyyyyyo..", "..oooooGGooooo..", "..oyyyyGGyyyyo..", "..oyyyyyyyyyyo..", "..oyyyyyyyyyyo..", "..oyyyyyyyyyyo..", "..oooooooooooo..", "................", "................"], &[('o', 0x2a0f4a), ('y', 0x6a3aa8), ('Y', 0xa070e8), ('G', 0xffd84a)]));
    let cr: &[(char, u32)] = &[('g', 0x4aa84a), ('G', 0x8aea6a), ('s', 0x6a9a3a), ('y', 0xf0c840), ('Y', 0xfff0a0), ('o', 0xe8802a), ('O', 0xffa850)];
    let z16 = "................";
    // 27..29 ข้าวโพด (งอก/กำลังโต/สุก)
    obj[27] = at.add(&spr(&[z16, z16, z16, z16, z16, z16, z16, z16, z16, z16, "......g..g......", ".....gG..Gg.....", "......g..g......", ".......ss.......", ".......ss.......", z16], cr));
    obj[28] = at.add(&spr(&[z16, z16, z16, z16, ".......G........", "......gGg.......", ".....gGsGg......", "....gGGsGGg.....", ".....gGsGg......", "......gsg.......", ".....gGsGg......", "....gGGsGGg.....", ".....gGsGg......", ".......ss.......", ".......ss.......", z16], cr));
    obj[29] = at.add(&spr(&[".......G........", "......gGg.......", ".....gGsGg......", "...gGgysyGGg....", "..gGGgyYsyGg....", "...gGgyYsyg.....", ".....gyyyg......", "..gG.gGsGg.Gg...", ".gGg.gGsGg.gGg..", "..gGgGgsgGgGg...", ".....gGsGg......", "....gGGsGGg.....", ".....gGsGg......", ".......ss.......", ".......ss.......", z16], cr));
    // 30..32 แครอท
    obj[30] = at.add(&spr(&[z16, z16, z16, z16, z16, z16, z16, z16, z16, ".......g........", "......gGg.......", ".....g.g.g......", ".......s........", ".......s........", z16, z16], cr));
    obj[31] = at.add(&spr(&[z16, z16, z16, z16, z16, z16, ".......G........", ".....G.gG.G.....", "....gGgGgGgG....", ".....gGGgGGg....", "......gGGg......", ".......gg.......", ".......s........", ".......s........", z16, z16], cr));
    obj[32] = at.add(&spr(&[z16, z16, z16, z16, ".......G........", ".....G.gG.G.....", "....gGgGgGgG....", ".....gGGgGGg....", "......gGGg......", ".....OOooOO.....", ".....oOOOOo.....", "......oOOo......", ".......oo.......", z16, z16, z16], cr));
    // 33 เตียง
    obj[33] = at.add(&spr(&[z16, z16, "..kkkkkkkkkkkk..", ".kwwwkrrrrrrrrk.", ".kwWwkrRrrrrrrk.", ".kwwwkrrrrrrrrk.", ".kwwwkrrrRrrrrk.", ".kwwwkrrrrrrrrk.", ".kkkkkkkkkkkkkk.", ".tt..........tt.", ".tt..........tt.", z16, z16, z16, z16, z16], &[('k', 0x4a2e14), ('w', 0xf0f0f8), ('W', 0xffffff), ('r', 0xc83a4a), ('R', 0xff8a96), ('t', 0x5a3418)]));
    // 34 โต๊ะคราฟต์ขั้นสูง
    obj[34] = at.add(&spr(&[z16, "......cCc.......", ".....cCCCc......", "......ccc.......", "..kkkkkkkkkkkk..", ".kLLLLLLLLLLLLk.", ".kLbbLLLLLLbbLk.", ".kkkkkkkkkkkkkk.", "..kDk......kDk..", "..kDk..ee..kDk..", "..kDk..ee..kDk..", "..kDk......kDk..", "..kkk......kkk..", z16, z16, z16], &[('k', 0x22222a), ('L', 0x8a6aff), ('b', 0xd0b0ff), ('D', 0x5a4a8a), ('c', 0x4ef0ff), ('C', 0xffffff), ('e', 0x1a1a24)]));
    // 36 ปากถ้ำ
    obj[37] = at.add(&spr(&[z16, z16, z16, z16, z16, "k..k..kk..k..k..", "kSkkSkkSSkkSkkSk", "kSSSSSSSSSSSSSSk", "kSsSSSSSSSSSsSSk", "kSSSkSSSSkSSSSSk", "kSSSSSSSSSSSSSSk", "kSsSSSkkkkSSSsSk", "kSSSSkbbbbkSSSSk", "kSSSSkbbbbkSSSSk", "kSsSSkbbbbkSSsSk", "kkkkkkbbbbkkkkkk"], &[('k', 0x2a2630), ('S', 0xa8a4b0), ('s', 0x7e7a88), ('b', 0x4a2e14)]));
    obj[38] = at.add(&spr(&[z16, z16, z16, z16, z16, z16, z16, "k.kk.kk.kk.kk.k.", "kSkkSkkSkkSkkSkk", "kSSSSSSSSSSSSSSk", "kSsSSSSSSSSSSsSk", "kSSSSsSSSSSsSSSk", "kSSSSSSSSSSSSSSk", "kSsSSSSSSsSSSSsk", "kSSSSsSSSSSSSSSk", "kkkkkkkkkkkkkkkk"], &[('k', 0x2a2630), ('S', 0xa8a4b0), ('s', 0x7e7a88)]));
    obj[39] = at.add(&spr(&[z16, z16, z16, z16, "...dddddddddd...", "..dDDDDDDDDDDd..", "...d........d...", "...d........d...", "...d..ee....d...", "..kSSSSSSSSSSk..", ".kSSwwwwwwwwSSk.", ".kSswwWWwwwwsSk.", ".kSSwwwwwwwwSSk.", "..kSSSSSSSSSSk..", "...kkkkkkkkkk...", z16], &[('k', 0x2a2630), ('S', 0xa8a4b0), ('s', 0x7e7a88), ('w', 0x3b8fe0), ('W', 0x9fd4ff), ('d', 0x5a3a1e), ('D', 0x8a5a32), ('e', 0xc8b070)]));
    obj[36] = at.add(&spr(&[z16, "....kkkkkkkk....", "..kkSSSSSSSSkk..", ".kSSSsssssssSSk.", ".kSsskkkkkksssk.", "kSssk000000kssSk", "kSsk00000000ksSk", "kSsk00000000ksSk", "kSsk00000000ksSk", "kSsk00000000ksSk", "kSsk00000000ksSk", "kkkk00000000kkkk", z16, z16, z16, z16], &[('k', 0x2a2622), ('S', 0x8c8680), ('s', 0x6a645e), ('0', 0x07070b)]));
    let cp: &[(char, u32)] = &[('g', 0x8e949e), ('G', 0xc8ccd6), ('d', 0x4e5460), ('e', 0x151515), ('n', 0x151515)];
    let kp: &[(char, u32)] = &[('r', 0xd04040), ('h', 0xc8ccd8), ('k', 0x222222), ('b', 0x3a6fd8), ('B', 0xbcd4ff), ('s', 0xe8e8f0), ('d', 0x5a3a1e)];
    let comp = [at.add(&spr(&[".........d.d", ".gg.....dgdg", "ggggggggggge", ".gGGgggggggn", ".ggGGGGgggg.", "..ggggggggg.", "..g.g..g.g..", "..d.d..d.d.."], cp)), at.add(&spr(&[".........d.d", ".gg.....dgdg", "ggggggggggge", ".gGGgggggggn", ".ggGGGGgggg.", "..ggggggggg.", "..gg..g..gg.", "..dd..d..dd."], cp)), at.add(&spr(&["...rr...", "..hhhh..", ".hhhhhh.", ".hkkkkh.", ".hhhhhh.", "..bbbb.s", ".bBbbbbs", ".bBbbbbs", ".bbbbbb.", "..b..b..", "..d..d..", "..d..d.."], kp)), at.add(&spr(&["...rr...", "..hhhh..", ".hhhhhh.", ".hkkkkh.", ".hhhhhh.", "..bbbb.s", ".bBbbbbs", ".bBbbbbs", ".bbbbbb.", "...bb...", "...dd...", "...dd..."], kp))];

    // ไอคอน 8x8: ต่อจากไอคอนเดิม (0..5) ตามดัชนีไอเทมและค่าคงที่ IC_*
    let mut ic = |rows: &[&str], pal: &[(char, u32)]| {
        let r = at.add(&spr(rows, pal));
        icons.push(r);
    };
    // 6 ลูกธนู
    ic(&["........", "r.....s.", "rrttttss", "r.....s.", "........", "........", "........", "........"], &[('r', 0xd04040), ('t', 0x8a5a32), ('s', 0xd8d8e0)]);
    // 7 เจลราชาสไลม์
    ic(&["........", "...gg...", "..gGgg..", ".gGgggg.", "gggggggg", "gGggggdg", ".ggddgg.", "........"], &[('g', 0x6cd06c), ('G', 0xd8ffd8), ('d', 0x2e8a3c)]);
    // 8 แกนโกเลม
    ic(&["...cc...", "..cCCc..", ".cCwCCc.", "cCCCCCCd", ".cCCCCd.", "..cCCd..", "...cd...", "........"], &[('c', 0xd8602a), ('C', 0xffa060), ('w', 0xffffff), ('d', 0x8a3a18)]);
    // 9 ยาฟื้นฟู
    ic(&["...cc...", "...gg...", "..gggg..", ".grrrrg.", ".grRrrg.", ".grrrrg.", "..gggg..", "........"], &[('c', 0x8a5a32), ('g', 0xb8e0f0), ('r', 0xe83a4a), ('R', 0xff9aa4)]);
    // 10 พิมพ์เขียวเกราะ
    ic(&["kkkkkkkk", "kbbbbbbk", "kbwwwwbk", "kbwbbwbk", "kbwwwwbk", "kbbwwbbk", "kbbbbbbk", "kkkkkkkk"], &[('k', 0x14246a), ('b', 0x2a4aa8), ('w', 0xcfe0ff)]);
    // 11 พิมพ์เขียวอาวุธ
    ic(&["kkkkkkkk", "kbbbbbwk", "kbbbbwbk", "kbbbwbbk", "kbwwwbbk", "kbwbbbbk", "kbbbbbbk", "kkkkkkkk"], &[('k', 0x14246a), ('b', 0x2a4aa8), ('w', 0xcfe0ff)]);
    // 12 เศษดาว (ดรอปจากมอนสเตอร์กลางคืน / หีบดันเจี้ยน)
    ic(&["...y....", "..yYy...", "..yYy...", ".yYYYy..", "yYYwYYy.", ".yYYYy..", "..yYy...", "...y...."], &[('y', 0x8a5ad8), ('Y', 0xc8a0ff), ('w', 0xffffff)]);
    // 13 เนื้อสัตว์ / 14 เห็ด / 15 สมุนไพร / 16 สเต็ก / 17 ซุปเห็ด / 18 ชา
    ic(&["........", "..rrr...", ".rRRrr..", "rRRRRrw.", "rRRRRrww", ".rrrrr.w", "........", "........"], &[('r', 0xb83a3a), ('R', 0xe86a6a), ('w', 0xf0e8d8)]);
    ic(&["........", "..rrrr..", ".rwrrwr.", "rrrrrrrr", "..ssss..", "..ssss..", "..ssss..", "........"], &[('r', 0xc8602a), ('w', 0xfff0d8), ('s', 0xe8dcc0)]);
    ic(&["...g....", "..ggg.g.", ".gGgggg.", "..gGgg..", "g.gggG..", "ggg.g...", ".gg.g...", "....g..."], &[('g', 0x3a9a3a), ('G', 0x8aea6a)]);
    ic(&["........", "........", ".bbbbbb.", "bBBBBBBb", "bBrrRBBb", "bBRrrBBb", ".bbbbbb.", "........"], &[('b', 0x8a5a32), ('B', 0xd8a060), ('r', 0xa83a2a), ('R', 0xe87a5a)]);
    ic(&["........", "..y..y..", ".kkkkkk.", "kkwwwwkk", "kwBBBBwk", ".kBBBBk.", "..kkkk..", "........"], &[('k', 0x3a3a44), ('w', 0xd8d8e0), ('B', 0xc8602a), ('y', 0xffffff)]);
    ic(&["........", "..y..y..", "..tttt..", ".tGGGGt.", ".tGGGGt.", ".tGGGGt.", "..tttt..", "........"], &[('t', 0xe8e8e8), ('G', 0x6ad06a), ('y', 0xffffff)]);
    // 19 ปลาย่าง / 20 ปลา
    ic(&["........", "........", ".bbbbbb.", "bBBBBBBb", "bBwBBBBb", ".bbbbbbb", "..t..t..", "........"], &[('b', 0x8a5a32), ('B', 0xd8a060), ('w', 0xfff0d8), ('t', 0x5a3418)]);
    ic(&["........", "..b.....", ".bbbb.bb", "bwbbbbbb", ".bbbb.bb", "..b.....", "........", "........"], &[('b', 0x4a90d0), ('w', 0xffffff)]);
    // 21 เมล็ดข้าวโพด / 22 เมล็ดแครอท / 23 ข้าวโพด / 24 แครอท
    ic(&["........", "..y..y..", ".yYy.yY.", "..y..y..", "yYy.yYy.", ".y...y..", "........", "........"], &[('y', 0xd8a830), ('Y', 0xf8e070)]);
    ic(&["........", "..o..o..", ".oOo.oO.", "..o..o..", "oOo.oOo.", ".o...o..", "........", "........"], &[('o', 0xc8601c), ('O', 0xff9a40)]);
    ic(&["..g..g..", ".gg.gg..", "..gyyg..", ".gyYyyg.", ".gyyYyg.", ".gyYyyg.", "..gyyg..", "...gg..."], &[('g', 0x4aa84a), ('y', 0xf0c840), ('Y', 0xfff0a0)]);
    ic(&["...gg...", "..gGGg..", "...oo...", "..oOOo..", "..oOOo..", "...oo...", "...oo...", "....o..."], &[('g', 0x4aa84a), ('G', 0x8aea6a), ('o', 0xe8802a), ('O', 0xffa850)]);
    // 25 แร่เหล็ก (ไอเทมใหม่ v33)
    ic(&["........", "..kkkk..", ".kSSsSk.", "kSsSSbSk", "kSSbsSSk", ".kSsSbk.", "..kkkk..", "........"], &[('k', 0x2a2630), ('S', 0xb4bcc8), ('s', 0x8a92a0), ('b', 0x5a6a88)]);
    // 13 ดาบ
    ic(&[".......w", "......wW", ".....wW.", "....wW..", ".t.wW...", "..tw....", ".t.t....", "t......."], &[('w', 0xd8d8e0), ('W', 0xffffff), ('t', 0x8a5a32)]);
    // 13 ธนู
    ic(&["sttt....", "s...tt..", "s.....t.", "s.....t.", "s.....t.", "s...tt..", "sttt....", "........"], &[('s', 0xe8e8e8), ('t', 0x9a6a38)]);
    // 14 คทาเวทมนตร์
    ic(&["...cc...", "..cCCc..", "..cCCc..", "...cc...", "...t....", "...t....", "...t....", "...t...."], &[('c', 0x4ef0ff), ('C', 0xe0ffff), ('t', 0x8a5a32)]);
    // 15 หมวก / 16 เกราะอก / 17 รองเท้า (สีเทาอ่อน เพื่อย้อมสีตามระดับ)
    let mp: &[(char, u32)] = &[('m', 0xc8ccd8), ('M', 0xf0f2f8), ('d', 0x9aa0b4), ('k', 0x50566a)];
    ic(&["........", "..mmmm..", ".mMMMMd.", "mMMMMMMd", "mMkkkkMd", "mMk..kMd", "mmm..mmd", "........"], mp);
    ic(&["........", "mm....mm", "mmmmmmmm", "mMMMMMMd", ".mMMMMd.", ".mMkkMd.", ".mMMMMd.", "..mmmm.."], mp);
    ic(&["........", "..bb....", "..bb....", "..bb....", "..bbbb..", ".bBBBBb.", ".bbbbbbb", "........"], &[('b', 0xc8ccd8), ('B', 0xf0f2f8)]);
    // 18 กำแพงไม้ / 19 กำแพงหิน / 20 ประตู / 21 กล่องเก็บของ
    ic(&["kkkkkkkk", "kTTkTTTk", "kTTkTTTk", "kTTkTTTk", "kTTkTTTk", "kTTkTTTk", "kTTkTTTk", "kkkkkkkk"], &[('k', 0x4a2e14), ('T', 0x9a6a38)]);
    ic(&["kkkkkkkk", "kLLkLLLk", "kkkkkkkk", "kLLLkLLk", "kkkkkkkk", "kLLkLLLk", "kkkkkkkk", "kLLLkLLk"], &[('k', 0x3a3a44), ('L', 0xb0b0bc)]);
    ic(&["kkkkkkkk", "kTTTTTTk", "kTTTTTTk", "kTTTTTTk", "kTTTTyTk", "kTTTTTTk", "kTTTTTTk", "kkkkkkkk"], &[('k', 0x3a2410), ('T', 0xa06a30), ('y', 0xffd84a)]);
    ic(&["kkkkkkkk", "kTTTTTTk", "kbbbbbbk", "kTTyyTTk", "kTTyyTTk", "kbbbbbbk", "kTTTTTTk", "kkkkkkkk"], &[('k', 0x3a2010), ('T', 0xa87a3a), ('b', 0x8a96b0), ('y', 0xffd84a)]);

    // 23 โต๊ะตีเหล็ก / 24 คบเพลิง
    ic(&["........", ".kkkkkk.", "kLLLLLLk", ".kkLLkk.", "...kk...", "..kLLk..", ".kkkkkk.", "........"], &[('k', 0x2a2a32), ('L', 0x9aa0ac)]);
    ic(&["...y....", "..yoy...", "..yry...", "...r....", "...T....", "...T....", "...T....", "..tTt..."], &[('y', 0xffe680), ('o', 0xffa630), ('r', 0xff5a1a), ('T', 0x8a5a32), ('t', 0x5a3418)]);

    // 31 หม้อทำอาหาร
    ic(&["........", "..yyyy..", ".kkkkkk.", "kKKKKKKk", "kKrrrrKk", "kKKKKKKk", ".kkkkkk.", "..t..t.."], &[('k', 0x2a2a32), ('K', 0x5a5a68), ('r', 0xff9a30), ('y', 0xffffff), ('t', 0x8a5a32)]);

    // 33.. ไถดิน / เตียง / โต๊ะขั้นสูง / บัวรดน้ำ
    ic(&["........", ".kkkkkk.", "kttttttk", ".kkkkkk.", "...t....", "...t....", "..tt....", "........"], &[('k', 0x8a909c), ('t', 0x8a5a32)]);
    ic(&["........", ".wwkrrrr", ".wwkrRrr", ".wwkrrrr", ".kkkkkkk", ".t....t.", "........", "........"], &[('w', 0xf0f0f8), ('k', 0x3a2410), ('r', 0xc83a4a), ('R', 0xff8a96), ('t', 0x5a3418)]);
    ic(&["........", "..cCc...", ".kkkkkk.", "kLLLLLLk", "kLLbbLLk", ".kkkkkk.", ".k....k.", "........"], &[('k', 0x22222a), ('L', 0x8a6aff), ('b', 0xd0b0ff), ('c', 0x4ef0ff), ('C', 0xffffff)]);
    ic(&["........", "......bb", ".kkkk.bb", "kBBBBkbb", "kBBBBBk.", "kBBBBBk.", ".kkkkk..", "........"], &[('k', 0x2a4a7a), ('B', 0x7ab8f0), ('b', 0xc8e4ff)]);

    // โปรเจกไทล์
    let arrow = spr(&["r......s.", "rttttttss", "r......s."], &[('r', 0xe05050), ('t', 0x9a6a38), ('s', 0xe8e8f0)]);
    let mut shard = orb(4, 0xb4b4c2, 0x8a8a96, 0x4a4a56);
    px(&mut shard, 1, 1, 0xd8d8e4);
    let mut boulder = orb(9, 0xb4b4c2, 0x8a8a96, 0x4a4a56);
    px(&mut boulder, 3, 3, 0xd8d8e4);
    px(&mut boulder, 5, 5, 0x3a3a46);
    px(&mut boulder, 6, 3, 0x3a3a46);
    let proj = [
        at.add(&arrow),
        at.add(&orb(7, 0xffffff, 0x7ae8ff, 0x2a90d0)),
        at.add(&orb(6, 0xd8ffd8, 0x6cd06c, 0x2e8a3c)),
        at.add(&boulder),
        at.add(&orb(7, 0xffffff, 0x9aff7a, 0x2aa040)),
        at.add(&shard),
        at.add(&orb(7, 0xfff2a0, 0xff9a30, 0xd0401a)),
        at.add(&orb(7, 0xfff2a0, 0xff8a20, 0xc83010)),
        at.add(&orb(7, 0xffffff, 0xa0e8ff, 0x4a90e0)),
        at.add(&orb(7, 0xffffff, 0xfff060, 0xd0a000)),
    ];
    let z = Rect::new(0., 0., 1., 1.);
    let cwall_v = at.add(&spr(&["..kSkkSSSSkkSk..", "..kSSSSSSSSSSk..", "..kSsSSSSSSsSk..", "..kSSSSsSSSSSk..", "..kkSSSSSSSSkk..", "..kSSSSSSSSSSk..", "..kSSsSSSSSSSk..", "..kSSSSSSSsSSk..", "..kSkkSSSSkkSk..", "..kSSSSSSSSSSk..", "..kSsSSSSSSsSk..", "..kSSSSSsSSSSk..", "..kkSSSSSSSSkk..", "..kSSSSSSSSSSk..", "..kSSSsSSSSSSk..", "..kSSSSSSSSsSk.."], &[('k', 0x2a2630), ('S', 0xa8a4b0), ('s', 0x7e7a88)]));
    Extras { king: at.add(&king()), golem: [at.add(&golem(false)), at.add(&golem(true))], merchant: merch, proj, comp, flash_slime: [z; 4], flash_skel: z, flash_bat: [z; 2], cwall_v }
}
