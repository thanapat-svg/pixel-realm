// mcui.rs — v10: หน้าตาแบบ Minecraft (ดูง่าย เด็กเล่นได้)
//  - แถบล่างกลางจอ: หัวใจ (เลือด) · น่องไก่ (อิ่ม) · หยดน้ำ (น้ำ) · สายฟ้า (แรง) · แถบ XP สีเขียวมีเลขเลเวล · ช่องของ 9 ช่อง
//  - ไอคอนพิกเซลวาดด้วยโค้ด · ช่องของแบบกรอบนูน
use super::*;

// ---------------- ไอคอนพิกเซล 9x9 ----------------
const HEART: [&str; 9] = [".XX...XX.", "XooX.XooX", "XoooXoooX", "XoooooooX", "XoooooooX", ".XoooooX.", "..XoooX..", "...XoX...", "....X...."];
const DRUM: [&str; 9] = ["......XX.", ".....XwwX", "....XwwX.", "..XXXXX..", ".XoooooX.", "XoooooX..", "XoooooX..", "XooooX...", ".XXXX...."];
const DROP: [&str; 9] = ["....X....", "...XoX...", "...XoX...", "..XoooX..", ".XooooXX.", ".XowoooX.", ".XowoooX.", "..XoooX..", "...XXX..."];
const BOLT: [&str; 9] = ["....XXXX.", "...XoooX.", "..XoooX..", ".XoooXXX.", "XooooooX.", "XXXooX...", "..XooX...", ".XooX....", ".XXX....."];
/// ไอคอนสำหรับสูตรที่ไม่มีภาพในชุดไอคอน
const PICK: [&str; 12] = ["..XXXXXXX...", ".XssssssX...", "XsX....XsX..", "X......XsX..", ".......XbX..", "......XbX...", ".....XbX....", "....XbX.....", "...XbX......", "..XbX.......", ".XbX........", "XXX........."];
const ROD: [&str; 12] = ["...........X", "..........Xb", ".........Xb.", "........Xb.l", ".......Xb..l", "......Xb...l", ".....Xb....l", "....Xb.....l", "...Xb......l", "..Xb.......h", ".Xb.........", "XX.........."];
const HEARTB: [&str; 12] = ["..XX....XX..", ".XooX..XooX.", "XoowoXXoooX.", "XowooooooX..", "XoooooooooX.", "XoooooooooX.", ".XoooooooX..", "..XoooooX...", "...XoooX....", "....XoX.....", ".....X......", "............"];
const LOCK: [&str; 9] = ["..XXXXX..", ".X.....X.", ".X.....X.", "XXXXXXXXX", "XoooooooX", "XoooXoooX", "XoooXoooX", "XoooooooX", "XXXXXXXXX"];

fn pix(rows: &[&str], x: f32, y: f32, s: f32, pal: &[(char, Color)]) {
    for (j, r) in rows.iter().enumerate() {
        for (i, ch) in r.chars().enumerate() {
            if let Some(&(_, c)) = pal.iter().find(|p| p.0 == ch) {
                draw_rectangle((x + i as f32 * s).floor(), (y + j as f32 * s).floor(), s.ceil(), s.ceil(), c);
            }
        }
    }
}
/// v: 0 ว่าง · 1 ครึ่ง · 2 เต็ม
fn need_icon(rows: &[&str], x: f32, y: f32, s: f32, v: u8, col: Color, blink: bool) {
    let dark = Color::new(0.12, 0.12, 0.14, 0.9);
    let empty = Color::new(0.25, 0.25, 0.28, 0.9);
    let edge = if blink { WHITE } else { dark };
    pix(rows, x, y, s, &[('X', edge), ('o', empty), ('w', empty)]);
    if v == 0 { return; }
    let w = if v == 1 { 5 } else { 9 };
    for (j, r) in rows.iter().enumerate() {
        for (i, ch) in r.chars().enumerate().take(w) {
            if ch == 'o' || ch == 'w' {
                let c = if ch == 'w' { Color::new(1., 1., 1., 0.85) } else { col };
                draw_rectangle((x + i as f32 * s).floor(), (y + j as f32 * s).floor(), s.ceil(), s.ceil(), c);
            }
        }
    }
}
/// แถวไอคอน 10 อัน (ค่า 0..1) — ขวาไปซ้ายได้แบบ Minecraft (ความอิ่มเรียงจากขวา)
fn need_row(rows: &[&str], x: f32, y: f32, s: f32, frac: f32, col: Color, rtl: bool, low: bool) {
    let units = (frac.clamp(0., 1.) * 20.).round() as i32;
    let t = get_time() as f32;
    for k in 0..10 {
        let v = (units - k * 2).clamp(0, 2) as u8;
        let ix = if rtl { 9 - k } else { k } as f32;
        let jig = if low && ((t * 12.) as i32 + k) % 5 == 0 { -s } else { 0. };
        need_icon(rows, x + ix * 9.5 * s, y + jig, s, v, col, low && (t * 4.).sin() > 0.6);
    }
}
pub fn pix_icon_heart(x: f32, y: f32, s: f32) { pix(&HEARTB, x, y, s, &[('X', hx(0x3a0a0a)), ('o', hx(0xe23a3a)), ('w', WHITE)]); }
pub fn pix_icon_pick(x: f32, y: f32, s: f32) { pix(&PICK, x, y, s, &[('X', hx(0x1e1e22)), ('s', hx(0xa8a8b0)), ('b', hx(0x8a5a2c))]); }
pub fn pix_icon_rod(x: f32, y: f32, s: f32) { pix(&ROD, x, y, s, &[('X', hx(0x1e1e22)), ('b', hx(0x9a6a3a)), ('l', hx(0xdddddd)), ('h', hx(0xaaaaaa))]); }
pub fn pix_lock(x: f32, y: f32, s: f32) { pix(&LOCK, x, y, s, &[('X', hx(0x2a2a2a)), ('o', hx(0xd8b040))]); }

// ---------------- ช่องของ ----------------
/// ช่องแบบกรอบนูน (เข้ม/สว่าง) — sel = กรอบขาวหนา, glow = กรอบทอง
pub fn slot(x: f32, y: f32, s: f32, sel: bool, glow: bool) {
    let b = (s / 16.).max(1.5);
    draw_rectangle(x, y, s, s, Color::new(0.2, 0.2, 0.22, 0.85));
    draw_rectangle(x, y, s, b, hx(0x111114));
    draw_rectangle(x, y, b, s, hx(0x111114));
    draw_rectangle(x, y + s - b, s, b, hx(0x6e6e74));
    draw_rectangle(x + s - b, y, b, s, hx(0x6e6e74));
    if glow { draw_rectangle_lines(x + b, y + b, s - 2. * b, s - 2. * b, 2. * b, hx(0xffcc44)); }
    if sel { draw_rectangle_lines(x - b, y - b, s + 2. * b, s + 2. * b, 2.5 * b, WHITE); }
}
/// ตัวเลขจำนวนมุมขวาล่าง (ตัวอักษรขาวมีเงา)
pub fn slot_count(font: &Font, n: i32, x: f32, y: f32, s: f32) {
    if n <= 1 { return; }
    let t = if n >= 1000 { format!("{}k", n / 1000) } else { n.to_string() };
    let fs = s * 0.36;
    let d = measure_text(&t, Some(font), fs as u16, 1.);
    let (tx, ty) = (x + s - d.width - s * 0.08, y + s - s * 0.1);
    draw_text_ex(&t, tx + 1.5, ty + 1.5, TextParams { font: Some(font), font_size: fs as u16, color: hx(0x2a2a2a), ..Default::default() });
    draw_text_ex(&t, tx, ty, TextParams { font: Some(font), font_size: fs as u16, color: WHITE, ..Default::default() });
}

/// ของที่แสดงในแถบล่าง (สูงสุด 9 ช่อง เรียงตามความสำคัญ เฉพาะที่มี)
pub fn hotbar_items(g: &Game) -> Vec<usize> {
    let order = [WOOD, STONE, BERRY, MEAT, FISH, GEM, COIN, FIRE, ARROW, POTION, DISH_STEW, DISH_FISH, DISH_SPEED, DISH_CRIT, CORN, CARROT, HERB, SHROOM, SHARD];
    order.iter().copied().filter(|&i| g.p.inv[i] > 0 && !(i == ARROW && !g.p.bow)).take(9).collect()
}

/// ขนาด/ตำแหน่งแถบล่าง: (x ซ้าย, y บนของช่องของ, ขนาดช่อง)
pub fn hotbar_geom(u: f32) -> (f32, f32, f32) {
    let (sw, sh) = (screen_width(), screen_height());
    let s = 30. * u;
    let w = s * 9.;
    ((sw - w) / 2., sh - s - 8. * u, s)
}
/// ความสูงทั้งหมดของ HUD ล่าง (ใช้วางข้อความแนะนำเหนือมัน)
pub fn bottom_hud_h(g: &Game, u: f32) -> f32 {
    let (_, _, s) = hotbar_geom(u);
    s + 8. * u + 10. * u + if g.survival_mode() > 0 { 2. * 12. * u } else { 12. * u } + 6. * u
}

pub fn draw_mc_hud(g: &Game, sp: &Sprites, font: &Font) {
    let u = ui_scale();
    let p = &g.p;
    let (hx0, hy, s) = hotbar_geom(u);
    let w = s * 9.;
    // ช่องของ
    let items = hotbar_items(g);
    for k in 0..9 {
        let x = hx0 + k as f32 * s;
        slot(x, hy, s, false, false);
        if let Some(&it) = items.get(k) {
            icon(sp, it, x + s * 0.15, hy + s * 0.15, s * 0.7);
            slot_count(font, p.inv[it], x, hy, s);
        }
    }
    // อาวุธที่ถืออยู่ (ช่องแยกทางซ้าย แบบมือรอง)
    let wx = hx0 - s - 6. * u;
    slot(wx, hy, s, true, false);
    let (ic, col) = match p.weapon { 1 => (IC_BOW, WHITE), 2 => (IC_STAFF, WHITE), _ => (ICON_SWORD, hx(SWC[p.sword])) };
    if p.weapon != 0 || p.sword > 0 {
        draw_texture_ex(&sp.tex, wx + s * 0.15, hy + s * 0.15, col, DrawTextureParams { source: Some(sp.icons[ic]), dest_size: Some(vec2(s * 0.7, s * 0.7)), ..Default::default() });
    }
    if p.weapon == 1 { slot_count(font, p.inv[ARROW].max(2), wx, hy, s); }
    // แถบ XP + เลขเลเวล
    let xy = hy - 8. * u;
    let need = (p.lvl * 10).max(1);
    draw_rectangle(hx0, xy, w, 5. * u, hx(0x1a1a1a));
    draw_rectangle(hx0, xy, w * (p.xp as f32 / need as f32).clamp(0., 1.), 5. * u, hx(0x7ee83a));
    draw_rectangle_lines(hx0, xy, w, 5. * u, 1., hx(0x000000));
    let lv = p.lvl.to_string();
    let fs = 13. * u;
    let d = measure_text(&lv, Some(font), fs as u16, 1.);
    for (ox, oy) in [(-1., 0.), (1., 0.), (0., -1.), (0., 1.)] {
        draw_text_ex(&lv, hx0 + w / 2. - d.width / 2. + ox * u, xy - 1. * u + oy * u, TextParams { font: Some(font), font_size: fs as u16, color: BLACK, ..Default::default() });
    }
    draw_text_ex(&lv, hx0 + w / 2. - d.width / 2., xy - 1. * u, TextParams { font: Some(font), font_size: fs as u16, color: hx(0x9cff5a), ..Default::default() });
    // หัวใจ (ซ้าย) · อิ่ม (ขวา)
    let is = 1.3 * u;
    let row1 = xy - 3. * u - 9. * is;
    let rw = 9.5 * is * 10.;
    need_row(&HEART, hx0, row1, is, p.hp as f32 / p.maxhp.max(1) as f32, hx(0xe23a3a), false, p.hp * 4 <= p.maxhp);
    if g.survival_mode() > 0 {
        need_row(&DRUM, hx0 + w - rw, row1, is, p.hunger / 100., hx(0xc8803a), true, p.hunger < 20.);
        let row2 = row1 - 10.5 * is;
        need_row(&BOLT, hx0, row2, is, p.rest / 100., hx(0xf0d040), false, p.rest < 20.);
        need_row(&DROP, hx0 + w - rw, row2, is, p.thirst / 100., hx(0x3a9be8), true, p.thirst < 20.);
    }
}
