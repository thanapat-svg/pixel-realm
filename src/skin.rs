//! สกินตัวละครแบบแยกชิ้น: หัว (7 แถว) · ตัว (5 แถว) · ขา (3 แถว) — เปลี่ยนตามเกราะ 4 ระดับและชุดที่คราฟต์
//! variant: 0 พื้นฐาน · 1..4 เกราะระดับ หิน/อัญมณี/ราชา/ดาวตก · 5 ชุดชาวไร่ · 6 ชุดนักล่า
use super::*;

pub const NV: usize = 7;
pub const OUTFIT_N: [&str; 3] = ["ไม่ใส่", "ชุดชาวไร่", "ชุดนักล่า"];

/// สไปรต์ 1 ชิ้นแบ่งเป็นชั้น: base = สีตายตัว (ลาย/ตา/เกราะ) · skin/hair/cloth/hi = ภาพขาวที่ย้อมสีตอนวาด (สีผิว/ผม/เสื้อ/เสื้อสว่าง)
#[derive(Clone, Copy, Default)]
pub struct Part { pub base: Option<Rect>, pub skin: Option<Rect>, pub hair: Option<Rect>, pub cloth: Option<Rect>, pub hi: Option<Rect> }

pub struct Skin {
    head: Vec<Part>,  // [(dir*NV + var)*STYLE_N + ทรงผม]
    torso: Vec<Part>, // [dir*NV + var]
    legs: Vec<Part>,  // [(dir*3 + frame)*NV + var]
}
/// ชุดของอาชีพชาวบ้าน → variant (หัว, ตัว, ขา): ชาวบ้าน · ชาวไร่ · ทหารยาม · พ่อค้า
pub const ROLE_PARTS: [[usize; 3]; 4] = [[0, 0, 0], [5, 5, 5], [1, 1, 1], [0, 4, 0]];

fn shade(c: u32, d: i32) -> u32 {
    let ch = |s: u32| (((c >> s) & 255) as i32 + d).clamp(0, 255) as u32;
    (ch(16) << 16) | (ch(8) << 8) | ch(0)
}

/// แปลงแถวพิกเซลของชิ้นหนึ่งให้เป็นรูปแบบ var (part 0 หัว · 1 ตัว · 2 ขา)
fn variant(rows: &[&str], part: usize, var: usize, dir: usize) -> Vec<String> {
    let mut v: Vec<String> = rows.iter().map(|s| s.to_string()).collect();
    let sub = |r: &mut String, from: char, to: char| *r = r.replace(from, &to.to_string());
    let set = |r: &mut String, x: usize, ch: char| {
        let mut cs: Vec<char> = r.chars().collect();
        if x < cs.len() { cs[x] = ch; }
        *r = cs.into_iter().collect();
    };
    match (part, var) {
        (_, 0) => {}
        // ---------- หมวก/หมวกเกราะ
        (0, 1..=4) => {
            for r in v.iter_mut() { sub(r, 'h', 'A'); }
            if v.len() > 1 { let n = v[1].len(); for x in 3..n.saturating_sub(3) { set(&mut v[1], x, 'a'); } }
            if var >= 3 { v[0] = if dir == 2 { "...aWAAa...".into() } else { "...aWAAWa...".into() }; }
            if var == 4 { set(&mut v[0], 5, 'W'); }
        }
        (0, 5) => {
            v[0] = "....HHHH....".into();
            v[1] = "...HHyyHH...".into();
            v[2] = "..HHHHHHHH..".into();
            v[3] = if dir == 2 { ".HHHHHHHHHHH".into() } else { ".HHHHHHHHHH.".into() };
        }
        (0, 6) => {
            for r in v.iter_mut().take(3) { sub(r, 'h', 'n'); }
            set(&mut v[0], 8, 'f'); set(&mut v[1], 9, 'f'); set(&mut v[2], 10, 'f');
        }
        // ---------- ตัว
        (1, 1..=4) => {
            for r in v.iter_mut() { sub(r, 'r', 'A'); sub(r, 'R', 'a'); sub(r, 'b', 'd'); }
            v[0] = ".aAAAAAAAAa.".into();
            if var == 4 { for r in v.iter_mut() { sub(r, 'a', 'W'); } v[0] = ".WAAAAAAAAW.".into(); }
        }
        (1, 5) => {
            for r in v.iter_mut() { sub(r, 'r', 'g'); sub(r, 'R', 'G'); }
            if dir != 2 { set(&mut v[1], 3, 'u'); set(&mut v[1], 8, 'u'); set(&mut v[2], 3, 'u'); set(&mut v[2], 8, 'u'); }
        }
        (1, 6) => {
            for r in v.iter_mut() { sub(r, 'r', 'n'); sub(r, 'R', 'N'); sub(r, 'b', 'l'); }
        }
        // ---------- ขา
        (2, 1..=4) => { for r in v.iter_mut() { sub(r, 'k', 'A'); } if var == 4 { for r in v.iter_mut() { sub(r, 'p', 'd'); } } }
        (2, 5) => { for r in v.iter_mut() { sub(r, 'p', 'q'); sub(r, 'k', 'Q'); } }
        (2, 6) => { for r in v.iter_mut() { sub(r, 'p', 'l'); sub(r, 'k', 'Q'); } }
        _ => {}
    }
    v
}

fn pal_for(base_pal: &[(char, u32)], ac: u32) -> Vec<(char, u32)> {
    let mut pal: Vec<(char, u32)> = base_pal.to_vec();
    pal.extend_from_slice(&[
        ('A', ac), ('a', shade(ac, 60)), ('d', shade(ac, -70)), ('W', 0xffffff),
        ('H', 0xe0c060), ('y', 0xb08a30), ('g', 0x4a9a3a), ('G', 0x78c060), ('u', 0x8a5a2a), ('q', 0x6a4a2a), ('Q', 0x3a2818),
        ('n', 0x2f6b4a), ('N', 0x4f9a6a), ('f', 0xd04040), ('l', 0x7a5a38),
    ]);
    pal
}

/// ทรงผมของหัว (7 แถว) ตามมุมมอง dir: 0 หน้า · 1 หลัง · 2 ข้าง — ใช้เฉพาะตอนไม่สวมหมวกเกราะ
pub fn hair_style(head: &[&str], dir: usize, style: usize) -> Vec<String> {
    let mut v: Vec<String> = head.iter().map(|s| s.to_string()).collect();
    match style {
        1 => { v[0] = "............".into(); v[1] = "...hhhhhh...".into(); } // เกรียน
        2 => match dir { // ยาว
            0 => { v[4] = ".hhessssehh.".into(); v[5] = ".hhsssssshh.".into(); v[6] = ".hhssmmsshh.".into(); }
            1 => { v[5] = "..hhhhhhhh..".into(); v[6] = "..hhhhhhhh..".into(); }
            _ => { v[6] = "..hhsssss...".into(); }
        },
        3 => match dir { // หางม้า
            1 => { v[6] = "....hhhh....".into(); }
            2 => { v[4] = "hhhhhssses..".into(); v[5] = ".hhhssssss..".into(); }
            _ => {}
        },
        4 => { for r in v.iter_mut() { *r = r.replace('h', "s"); } } // ล้าน
        _ => {}
    }
    v
}

/// แยกภาพชิ้นเดียวออกเป็นชั้นสี (ชั้นที่ว่างเปล่าคืน None ไม่เปลืองที่ในแอตลาส)
pub fn split_layers(at: &mut Atlas, rows: &[String], pal: &[(char, u32)]) -> Part {
    let w = rows.iter().map(|r| r.chars().count()).max().unwrap_or(1);
    let h = rows.len();
    let mut ims: Vec<Image> = (0..5).map(|_| Image::gen_image_color(w as u16, h as u16, BLANK)).collect();
    let mut used = [false; 5];
    for (y, r) in rows.iter().enumerate() {
        for (x, ch) in r.chars().enumerate() {
            let (layer, col) = match ch {
                's' => (1, WHITE), 'h' => (2, WHITE), 'r' => (3, WHITE), 'R' => (4, WHITE),
                _ => match pal.iter().find(|p| p.0 == ch) { Some(&(_, c)) => (0, hx(c)), None => continue },
            };
            ims[layer].set_pixel(x as u32, y as u32, col);
            used[layer] = true;
        }
    }
    let mut add = |i: usize| if used[i] { Some(at.add(&ims[i])) } else { None };
    Part { base: add(0), skin: add(1), hair: add(2), cloth: add(3), hi: add(4) }
}

pub fn build(at: &mut Atlas, pd: &[&str; 12], pu: &[&str; 12], pr: &[&str; 12], lf: &[[&str; 3]; 3], ls: &[[&str; 3]; 3], base_pal: &[(char, u32)]) -> Skin {
    let mut sk = Skin { head: vec![Part::default(); 3 * NV * STYLE_N], torso: vec![Part::default(); 3 * NV], legs: vec![Part::default(); 3 * 3 * NV] };
    let bodies = [pd, pu, pr];
    for var in 0..NV {
        // พาเลตต์ของ variant: ระดับเกราะ = ATC[var] (1..4)
        let pal = pal_for(base_pal, ATC[var.clamp(1, 4)]);
        for (dir, body) in bodies.iter().enumerate() {
            // หัว: ทรงผมเปลี่ยนได้เฉพาะตอนไม่สวมหมวกเกราะ (variant 1..4)
            let styled = !(1..=4).contains(&var);
            let mut first = Part::default();
            for st in 0..STYLE_N {
                let idx = (dir * NV + var) * STYLE_N + st;
                if st == 0 || styled {
                    let rows = hair_style(&body[0..7], dir, st);
                    let rr: Vec<&str> = rows.iter().map(|s| s.as_str()).collect();
                    let hv = variant(&rr, 0, var, dir);
                    let p = split_layers(at, &hv, &pal);
                    if st == 0 { first = p; }
                    sk.head[idx] = p;
                } else {
                    sk.head[idx] = first;
                }
            }
            let tv = variant(&body[7..12], 1, var, dir);
            sk.torso[dir * NV + var] = split_layers(at, &tv, &pal);
            for f in 0..3 {
                let legs = if dir == 2 { &ls[f] } else { &lf[f] };
                let lv = variant(&legs[..], 2, var, dir);
                sk.legs[(dir * 3 + f) * NV + var] = split_layers(at, &lv, &pal);
            }
        }
    }
    sk
}

/// variant ที่ใส่อยู่: เกราะชิ้นนั้นมาก่อน ถ้าไม่มีใช้ชุดที่ใส่ (ชาวไร่=5 · นักล่า=6)
pub fn skin_parts(p: &Player) -> [usize; 3] {
    let o = match p.outfit { 1 => 5, 2 => 6, _ => 0 };
    let pick = |slot: usize| if p.armor[slot] > 0 { p.armor[slot] as usize } else { o };
    [pick(0), pick(1), pick(2)]
}
pub fn full_meteor(p: &Player) -> bool { p.armor.iter().all(|&a| a == 4) }

fn tint(c: Color, t: Color) -> Color { Color::new(c.r * t.r, c.g * t.g, c.b * t.b, c.a) }

/// วาดตัวละครแบบแยกชิ้น+ย้อมสีตามรูปร่าง (x,y = มุมซ้ายบนของกรอบ 12x15 พิกเซลสไปรต์, s = พิกเซลจอต่อพิกเซลสไปรต์)
/// ส่วนสูง/น้ำหนักปรับสัดส่วนโดยยึดเท้าไว้ที่เดิม
pub fn draw_figure(sp: &Sprites, body: &Body, vars: [usize; 3], x: f32, y: f32, s: f32, dir: usize, frame: usize, flip: bool, c: Color) {
    draw_figure_ex(sp, body, vars, x, y, s, dir, frame, flip, c, 1.);
}
/// leg < 1 = ท่านั่ง (ขาพับ ลำตัวลดลงมา เท้ายังอยู่ที่เดิม = ที่นั่ง)
pub fn draw_figure_ex(sp: &Sprites, body: &Body, vars: [usize; 3], x: f32, y: f32, s: f32, dir: usize, frame: usize, flip: bool, c: Color, leg: f32) {
    let k = &sp.skin;
    // แต่ละส่วนกว้าง/สูงต่างกัน ยึดเท้าไว้ที่เดิมและจัดกึ่งกลาง
    let (hw, hh, tw, lw, rest) = body.part_scale();
    let feet = y + 15. * s;
    let cx = x + 6. * s;
    let (sk, hr, cl, hi) = (body.skin_col(), body.hair_col(), body.cloth_col(), body.cloth_hi());
    let parts = [
        (k.head[(dir * NV + vars[0]) * STYLE_N + body.style as usize], feet - ((5. + 3. * leg) * rest + 7. * hh) * s, hw, hh),
        (k.torso[dir * NV + vars[1]], feet - (5. + 3. * leg) * rest * s, tw, rest),
        (k.legs[(dir * 3 + frame) * NV + vars[2]], feet - 3. * leg * rest * s, lw, rest * leg),
    ];
    // ขอบเข้ม 1 พิกเซลรอบตัว (ให้แยกจากพื้นชัด)
    if c.a > 0.95 && crate::vfx::fr().outline {
        let oc = crate::vfx::OUTLINE_C;
        for (dx, dy) in [(-s, 0.), (s, 0.), (0., -s), (0., s)] {
            for &(p, py, pw, ph) in &parts {
                for r in [p.base, p.skin, p.hair, p.cloth, p.hi].into_iter().flatten() {
                    let w = r.w * s * pw;
                    draw_texture_ex(&sp.tex, cx - w / 2. + dx, py + dy, oc, DrawTextureParams { source: Some(r), dest_size: Some(vec2(w, r.h * s * ph)), flip_x: flip, ..Default::default() });
                }
            }
        }
    }
    for (p, py, pw, ph) in parts {
        for (r, col) in [(p.base, c), (p.skin, tint(c, sk)), (p.hair, tint(c, hr)), (p.cloth, tint(c, cl)), (p.hi, tint(c, hi))] {
            if let Some(r) = r {
                let w = r.w * s * pw;
                draw_texture_ex(&sp.tex, cx - w / 2., py, col, DrawTextureParams { source: Some(r), dest_size: Some(vec2(w, r.h * s * ph)), flip_x: flip, ..Default::default() });
            }
        }
    }
}

/// ผู้เล่น: ใส่เกราะ/ชุดตามที่สวม
pub fn draw_skin(sp: &Sprites, p: &Player, x: f32, y: f32, s: f32, dir: usize, frame: usize, flip: bool, c: Color) {
    draw_figure(sp, &p.body, skin_parts(p), x, y, s, dir, frame, flip, c);
}
pub fn draw_skin_sit(sp: &Sprites, p: &Player, x: f32, y: f32, s: f32, dir: usize, flip: bool, c: Color) {
    draw_figure_ex(sp, &p.body, skin_parts(p), x, y, s, dir, 0, flip, c, 0.4);
}
