//! สไปรต์พิกเซลเพิ่มเติม (วาดด้วยโค้ดทั้งหมด ไม่มีไฟล์รูป)
//! ไอคอนไอเทม · สิ่งปลูกสร้าง · เต็นท์ · พ่อค้า · บอส · โปรเจกไทล์

use super::*;

pub struct Extras {
    pub king: Rect,
    pub golem: [Rect; 2], // [ปกติ, ยกแขน]
    pub merchant: [Rect; 2],
    /// เรียงตาม kind ของ Proj: 0 ธนู, 1 เวท, 2 เมือก, 3 ก้อนหิน, 4 เวทราชา, 5 เศษหิน
    pub proj: [Rect; 6],
}

fn blank(w: u16, h: u16) -> Image {
    Image::gen_image_color(w, h, BLANK)
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
    obj[12] = at.add(&door(false));
    obj[13] = at.add(&door(true));
    obj[14] = at.add(&storage_box());
    let merch = [at.add(&merchant(false)), at.add(&merchant(true))];
    obj[15] = merch[0];
    obj[16] = at.add(&tent());

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
    // 12 ดาบ
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
    ];
    Extras { king: at.add(&king()), golem: [at.add(&golem(false)), at.add(&golem(true))], merchant: merch, proj }
}
