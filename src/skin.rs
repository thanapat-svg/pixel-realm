//! สกินตัวละครแบบแยกชิ้น: หัว (7 แถว) · ตัว (5 แถว) · ขา (3 แถว) — เปลี่ยนตามเกราะ 4 ระดับและชุดที่คราฟต์
//! variant: 0 พื้นฐาน · 1..4 เกราะระดับ หิน/อัญมณี/ราชา/ดาวตก · 5 ชุดชาวไร่ · 6 ชุดนักล่า
use super::*;

pub const NV: usize = 7;
pub const OUTFIT_N: [&str; 3] = ["ไม่ใส่", "ชุดชาวไร่", "ชุดนักล่า"];

pub struct Skin {
    head: Vec<Rect>,  // [dir*NV + var]
    torso: Vec<Rect>, // [dir*NV + var]
    legs: Vec<Rect>,  // [(dir*3 + frame)*NV + var]
}

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

pub fn build(at: &mut Atlas, pd: &[&str; 12], pu: &[&str; 12], pr: &[&str; 12], lf: &[[&str; 3]; 3], ls: &[[&str; 3]; 3], base_pal: &[(char, u32)]) -> Skin {
    let mut sk = Skin { head: vec![], torso: vec![], legs: vec![] };
    let bodies = [pd, pu, pr];
    for var in 0..NV {
        // พาเลตต์ของ variant: ระดับเกราะ = ATC[var] (1..4)
        let t = var.clamp(1, 4);
        let ac = ATC[t];
        let mut pal: Vec<(char, u32)> = base_pal.to_vec();
        pal.extend_from_slice(&[
            ('A', ac), ('a', shade(ac, 60)), ('d', shade(ac, -70)), ('W', 0xffffff),
            ('H', 0xe0c060), ('y', 0xb08a30), ('g', 0x4a9a3a), ('G', 0x78c060), ('u', 0x8a5a2a), ('q', 0x6a4a2a), ('Q', 0x3a2818),
            ('n', 0x2f6b4a), ('N', 0x4f9a6a), ('f', 0xd04040), ('l', 0x7a5a38),
        ]);
        let _ = &mut pal;
        // หัว: แถว 0..7 · ตัว: แถว 7..12
        for (dir, body) in bodies.iter().enumerate() {
            let hv = variant(&body[0..7], 0, var, dir);
            let hr: Vec<&str> = hv.iter().map(|s| s.as_str()).collect();
            if sk.head.len() < dir * NV + var + 1 { sk.head.resize(dir * NV + var + 1, Rect::new(0., 0., 1., 1.)); }
            sk.head[dir * NV + var] = at.add(&spr(&hr, &pal));
            let tv = variant(&body[7..12], 1, var, dir);
            let tr: Vec<&str> = tv.iter().map(|s| s.as_str()).collect();
            if sk.torso.len() < dir * NV + var + 1 { sk.torso.resize(dir * NV + var + 1, Rect::new(0., 0., 1., 1.)); }
            sk.torso[dir * NV + var] = at.add(&spr(&tr, &pal));
            for f in 0..3 {
                let legs = if dir == 2 { &ls[f] } else { &lf[f] };
                let lv = variant(&legs[..], 2, var, dir);
                let lr: Vec<&str> = lv.iter().map(|s| s.as_str()).collect();
                let idx = (dir * 3 + f) * NV + var;
                if sk.legs.len() < idx + 1 { sk.legs.resize(idx + 1, Rect::new(0., 0., 1., 1.)); }
                sk.legs[idx] = at.add(&spr(&lr, &pal));
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

/// วาดตัวละครแบบแยกชิ้นที่ตำแหน่งหน้าจอ (x,y = มุมซ้ายบน, s = พิกเซลจอต่อพิกเซลสไปรต์)
pub fn draw_skin(sp: &Sprites, p: &Player, x: f32, y: f32, s: f32, dir: usize, frame: usize, flip: bool, c: Color) {
    let [vh, vt, vl] = skin_parts(p);
    let k = &sp.skin;
    let parts = [(k.head[dir * NV + vh], 0.), (k.torso[dir * NV + vt], 7.), (k.legs[(dir * 3 + frame) * NV + vl], 12.)];
    for (r, off) in parts {
        draw_texture_ex(&sp.tex, x, y + off * s, c, DrawTextureParams { source: Some(r), dest_size: Some(vec2(r.w * s, r.h * s)), flip_x: flip, ..Default::default() });
    }
}
