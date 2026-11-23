//! เมนูแบบแท็บ: คราฟต์ · สร้าง · ชุด/ของ · กล่องเก็บของ · ร้านค้า
//! เลื่อนได้ด้วยล้อเมาส์หรือลากนิ้ว (มี scissor ตัดส่วนที่เกินกรอบ) และกดปุ่มตอน "ปล่อย" เพื่อไม่ให้ชนกับการเลื่อน

use super::*;

pub struct UiState {
    pub tab: usize,
    pub scroll: f32,
    pub max_scroll: f32,
    pub chest: Option<(i32, i32)>,
    pub shop: bool,
    pub forge: bool,
    pub cook: bool,
    pub adv: bool,
    pub npc: Option<u32>,
    /// v29 กำลังเลือกคนให้ทำอาชีพนี้ (แท็บชุมชน)
    pub role_pick: Option<u8>,
    /// เมืองที่กำลังใช้บริการ (ดัชนี, 0 ตลาด 1 ปราสาท 2 คอกม้า 3 รถม้า)
    pub city: Option<(usize, u8)>,
    pub gift_open: bool,
    pub greet: usize,
    /// ช่องที่เลือกในตาราง (คราฟต์/สร้าง/กระเป๋า)
    pub sel_r: usize,
    pub sel_b: usize,
    pub sel_i: usize,
    /// v32 คู่มือหาของ (แท็บกระเป๋า): ของที่เลือกดู
    pub guide_i: usize,
    /// v36 หน้าช่างสถาปนิก (แท็บชุมชน)
    pub arch: bool,
    press: Option<Vec2>,
    last: Vec2,
    drag: f32,
    click: Option<Vec2>,
    clip: Rect,
    pub close: bool,
}
impl UiState {
    pub fn new() -> Self {
        UiState { tab: 0, scroll: 0., max_scroll: 0., chest: None, shop: false, forge: false, cook: false, adv: false, npc: None, role_pick: None, city: None, gift_open: false, greet: 0, sel_r: usize::MAX, sel_b: usize::MAX, sel_i: usize::MAX, guide_i: GEM, arch: false, press: None, last: Vec2::ZERO, drag: 0., click: None, clip: Rect::new(0., 0., 1., 1.), close: false }
    }
    pub fn open(&mut self, tab: usize) {
        self.tab = tab;
        self.scroll = 0.;
        self.close = false;
        self.press = None;
    }
    pub fn reset_context(&mut self) {
        self.chest = None;
        self.shop = false;
        self.forge = false;
        self.cook = false;
        self.adv = false;
        self.npc = None;
        self.role_pick = None;
        self.arch = false;
        self.gift_open = false;
        self.city = None;
        self.close = false;
    }
}

fn set_clip(r: Option<Rect>) {
    let mut gl = unsafe { get_internal_gl() };
    gl.flush();
    match r {
        Some(r) => {
            let d = screen_dpi_scale();
            gl.quad_gl.scissor(Some(((r.x * d) as i32, (r.y * d) as i32, (r.w * d) as i32, (r.h * d) as i32)));
        }
        None => gl.quad_gl.scissor(None),
    }
}

fn btn(ui: &UiState, font: &Font, r: Rect, label: &str, enabled: bool, u: f32) -> bool {
    let (mx, my) = mouse_position();
    let mp = vec2(mx, my);
    let hover = r.contains(mp) && ui.clip.contains(mp);
    let bg = if !enabled { hx(0x4a4a52) } else if hover { hx(0xffdd88) } else { hx(0xffcc66) };
    draw_rectangle(r.x, r.y, r.w, r.h, bg);
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., BLACK);
    let col = if enabled { hx(0x222222) } else { hx(0x999999) };
    let label = &thai(label);
    let fs = (14. * u) as u16;
    let d = measure_text(label, Some(font), fs, 1.);
    draw_text_ex(label, r.x + r.w / 2. - d.width / 2., r.y + r.h / 2. + 5. * u, TextParams { font: Some(font), font_size: fs, color: col, ..Default::default() });
    enabled && ui.click.map_or(false, |c| r.contains(c) && ui.clip.contains(c))
}
fn icon_c(sp: &Sprites, i: usize, x: f32, y: f32, sz: f32, c: Color) {
    draw_texture_ex(&sp.tex, x, y, c, DrawTextureParams { source: Some(sp.icons[i]), dest_size: Some(vec2(sz, sz)), ..Default::default() });
}
/// ไอคอนสิ่งปลูกสร้าง: เฟอร์นิเจอร์ใช้ภาพจริง · บ้านสำเร็จรูปวาดบ้านเล็กๆ · พื้นใช้ลายพื้น
fn piece_icon(sp: &Sprites, p: &Piece, x: f32, y: f32, s: f32, tint: Color) {
    let src = |r: Rect| draw_texture_ex(&sp.tex, x, y, tint, DrawTextureParams { source: Some(r), dest_size: Some(vec2(s, s)), ..Default::default() });
    if is_furniture(p.obj) || p.obj == O_WINDOW {
        src(sp.obj[p.obj as usize]);
    } else if p.obj == OBJ_FLOOR_W || p.obj == OBJ_FLOOR_S {
        src(sp.tiles[if p.obj == OBJ_FLOOR_W { T_FLOORW } else { T_FLOORS } as usize][0]);
    } else if is_prefab(p.obj) {
        let (bx, by, bs) = (x + s * 0.08, y + s * 0.38, s * 0.84);
        let wall = sp.obj[if p.obj == OBJ_HOUSE_L { 11 } else { 10 }];
        draw_texture_ex(&sp.tex, bx, by, tint, DrawTextureParams { source: Some(wall), dest_size: Some(vec2(bs, bs * 0.6)), ..Default::default() });
        let rc = if p.obj == OBJ_HOUSE_L { Color::new(0.35, 0.42, 0.53, tint.a) } else { Color::new(0.69, 0.29, 0.2, tint.a) };
        draw_triangle(vec2(bx - s * 0.06, by), vec2(bx + bs + s * 0.06, by), vec2(bx + bs / 2., by - s * 0.34), rc);
        draw_rectangle(bx + bs * 0.4, by + bs * 0.25, bs * 0.2, bs * 0.35, Color::new(0.35, 0.2, 0.08, tint.a));
    } else if p.obj == OBJ_MOVE {
        // ไอคอนย้ายของ: กล่อง + ลูกศร 4 ทิศ
        icon_c(sp, p.icon, x + s * 0.2, y + s * 0.2, s * 0.6, tint);
        let (cx, cy, r) = (x + s / 2., y + s / 2., s * 0.46);
        let c = Color::new(0.55, 0.85, 1., tint.a);
        for (dx, dy) in [(1., 0.), (-1., 0.), (0., 1.), (0., -1.)] {
            let (tx, ty) = (cx + dx * r, cy + dy * r);
            draw_triangle(vec2(tx, ty), vec2(tx - dx * s * 0.14 - dy * s * 0.1, ty - dy * s * 0.14 - dx * s * 0.1), vec2(tx - dx * s * 0.14 + dy * s * 0.1, ty - dy * s * 0.14 + dx * s * 0.1), c);
        }
    } else {
        icon_c(sp, p.icon, x, y, s, tint);
    }
}
fn visible(y: f32, h: f32, c: Rect) -> bool {
    y + h >= c.y && y <= c.y + c.h
}
fn sep(c: Rect, y: f32) {
    draw_line(c.x + 4., y, c.x + c.w - 4., y, 1., hx(0x333344));
}
fn header(font: &Font, c: Rect, y: f32, s: &str, u: f32) {
    draw_rectangle(c.x, y + 2. * u, c.w, 22. * u, hx(0x2a2d3f));
    text(font, s, c.x + 8. * u, y + 18. * u, 14. * u, hx(0xffcc66));
}
/// วาดต้นทุนเป็นไอคอน + จำนวน
fn costs(g: &Game, sp: &Sprites, font: &Font, cost: &[(usize, i32)], need: Option<usize>, x: f32, y: f32, u: f32) {
    let mut cx = x;
    for &(it, n) in cost {
        icon_c(sp, it, cx, y, 12. * u, WHITE);
        let s = format!("{}/{}", g.p.inv[it], n);
        let c = if g.p.inv[it] >= n { hx(0xaaffaa) } else { hx(0xff8888) };
        text(font, &s, cx + 14. * u, y + 11. * u, 11. * u, c);
        cx += 14. * u + measure_text(&s, Some(font), (11. * u) as u16, 1.).width + 10. * u;
    }
    if let Some(nd) = need {
        icon_c(sp, nd, cx, y, 12. * u, WHITE);
        let ok = g.p.inv[nd] >= 1;
        text(font, if ok { "มีพิมพ์เขียวแล้ว" } else { "ต้องมีพิมพ์เขียว" }, cx + 14. * u, y + 11. * u, 11. * u, if ok { hx(0xaaffaa) } else { hx(0xff8888) });
    }
}

fn tab_craft(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    recipe_list(g, sp, font, ui, c, u, &RECIPES)
}
fn tab_adv(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    recipe_list(g, sp, font, ui, c, u, &ADV_RECIPES)
}
/// ไอคอนผลลัพธ์ของสูตร
enum RIcon { At(usize, Color), Pick, Rod, Heart, Boat }
fn recipe_icon(r: &Recipe) -> RIcon {
    match r.eff {
        Eff::Sword(t) => RIcon::At(ICON_SWORD, hx(SWC[t.min(5)])),
        Eff::Pick => RIcon::Pick,
        Eff::Fire => RIcon::At(FIRE, WHITE),
        Eff::Heart => RIcon::Heart,
        Eff::Bow => RIcon::At(IC_BOW, WHITE),
        Eff::Staff(_) => RIcon::At(IC_STAFF, WHITE),
        Eff::Arrows(_) => RIcon::At(ARROW, WHITE),
        Eff::Elem(e) => RIcon::At(IC_STAFF, [hx(0xff8a4a), hx(0x8ae0ff), hx(0xffe060)][(e as usize).min(2)]),
        Eff::Rod => RIcon::Rod,
        Eff::Armor(sl, t) => RIcon::At(SLOT_ICON[sl], hx(ATC[(t as usize).min(4)])),
        Eff::Can => RIcon::At(IC_CAN, WHITE),
        Eff::Outfit(o) => RIcon::At(IC_CHEST, if o == 1 { hx(0xd8b85a) } else { hx(0x5a9a4a) }),
        Eff::Coat => RIcon::At(IC_CHEST, hx(0xa8784a)),
        Eff::Boat => RIcon::Boat,
        Eff::Guard(k) => RIcon::At(IC_HELM, match k { 0 => hx(0xc8a070), 1 => hx(0xa8b4c8), 2 => hx(0x8a92a8), _ => hx(0x7af0ff) }),
    }
}
fn draw_ricon(sp: &Sprites, ic: &RIcon, x: f32, y: f32, s: f32, dim: bool) {
    let a = if dim { 0.35 } else { 1. };
    match ic {
        RIcon::At(i, c) => icon_c(sp, *i, x, y, s, Color::new(c.r, c.g, c.b, a)),
        RIcon::Pick => mcui::pix_icon_pick(x, y, s / 12.),
        RIcon::Rod => mcui::pix_icon_rod(x, y, s / 12.),
        RIcon::Heart => mcui::pix_icon_heart(x, y, s / 12.),
        RIcon::Boat => mcui::pix_icon_boat(x, y, s / 12.),
    }
    if dim && !matches!(ic, RIcon::At(..)) { draw_rectangle(x, y, s, s, Color::new(0.1, 0.1, 0.12, 0.6)); }
}
/// วัตถุดิบเป็นช่องเล็ก: ไอคอน + มี/ต้องใช้ (เขียว = พอ แดง = ไม่พอ)
fn cost_chips(g: &Game, sp: &Sprites, font: &Font, cost: &[(usize, i32)], need: Option<usize>, x: f32, y: f32, u: f32) {
    let s = 26. * u;
    let mut cx = x;
    for &(it, n) in cost {
        mcui::slot(cx, y, s, false, false);
        icon_c(sp, it, cx + s * 0.15, y + s * 0.15, s * 0.7, WHITE);
        let ok = g.p.inv[it] >= n;
        let t = format!("{}/{}", g.p.inv[it].min(999), n);
        text(font, &t, cx + s + 3. * u, y + s * 0.68, 12. * u, if ok { hx(0x8cff8c) } else { hx(0xff7a7a) });
        cx += s + 8. * u + measure_text(&t, Some(font), (12. * u) as u16, 1.).width;
    }
    if let Some(nd) = need {
        mcui::slot(cx, y, s, false, false);
        icon_c(sp, nd, cx + s * 0.15, y + s * 0.15, s * 0.7, WHITE);
        let ok = g.p.inv[nd] >= 1;
        text(font, if ok { "มี" } else { "ต้องมี" }, cx + s + 3. * u, y + s * 0.68, 12. * u, if ok { hx(0x8cff8c) } else { hx(0xff7a7a) });
    }
}
/// เครื่องหมายถูก (วาดด้วยเส้น ไม่พึ่งฟอนต์)
fn tick(x: f32, y: f32, s: f32) {
    let w = (s * 0.12).max(2.);
    draw_line(x, y + s * 0.5, x + s * 0.35, y + s * 0.85, w, hx(0x1a1a1a));
    draw_line(x + s * 0.35, y + s * 0.85, x + s, y + s * 0.1, w, hx(0x1a1a1a));
    draw_line(x, y + s * 0.45, x + s * 0.35, y + s * 0.8, w * 0.6, hx(0x6cff6c));
    draw_line(x + s * 0.35, y + s * 0.8, x + s, y + s * 0.05, w * 0.6, hx(0x6cff6c));
}
/// ตารางช่องแบบ Minecraft: คืน index ที่แตะ
fn grid<F: Fn(usize, f32, f32, f32)>(ui: &UiState, c: Rect, y: &mut f32, n: usize, s: f32, gap: f32, sel: usize, draw: F) -> Option<usize> {
    let cols = (((c.w - 12.) / (s + gap)).floor() as usize).max(1);
    let x0 = c.x + (c.w - cols as f32 * (s + gap) + gap) / 2.;
    let mut hit = None;
    for k in 0..n {
        let (col, row) = (k % cols, k / cols);
        let (x, yy) = (x0 + col as f32 * (s + gap), *y + row as f32 * (s + gap));
        if visible(yy, s, c) {
            mcui::slot(x, yy, s, k == sel, false);
            draw(k, x, yy, s);
            if ui.click.map_or(false, |p| Rect::new(x, yy, s, s).contains(p) && ui.clip.contains(p)) { hit = Some(k); }
        }
    }
    let rows = (n + cols - 1) / cols;
    *y += rows as f32 * (s + gap);
    hit
}
/// กล่องรายละเอียดด้านบน (ช่องใหญ่ + ชื่อ + คำอธิบาย) — คืนความสูง
fn detail_box(font: &Font, c: Rect, y: f32, u: f32, name: &str, desc: &str, draw_big: &dyn Fn(f32, f32, f32)) -> f32 {
    let h = 108. * u;
    if visible(y, h, c) {
        draw_rectangle(c.x + 4. * u, y + 4. * u, c.w - 8. * u, h - 8. * u, hx(0x262938));
        let s = 58. * u;
        mcui::slot(c.x + 12. * u, y + 14. * u, s, false, false);
        draw_big(c.x + 12. * u + s * 0.12, y + 14. * u + s * 0.12, s * 0.76);
        text(font, name, c.x + 82. * u, y + 30. * u, 17. * u, hx(0xffcc66));
        let ws = wrap_th(font, desc, 12. * u, c.w - 200. * u);
        for (k, w) in ws.iter().take(2).enumerate() { text(font, w, c.x + 82. * u, y + 48. * u + k as f32 * 14. * u, 12. * u, hx(0xc8c8d0)); }
    }
    h
}

fn recipe_list(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32, list: &'static [Recipe]) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    // เปิดเผยทีละน้อย: แสดงสูตรพื้นฐาน + สูตรที่เกี่ยวกับของที่มีอยู่
    let basic = if list.len() >= 20 { 2 } else { 0 };
    let shown: Vec<usize> = (0..list.len()).filter(|&i| {
        let r = &list[i];
        i < basic || recipe_state(&g.p, r) == 1 || r.cost.iter().any(|&(it, _)| g.p.inv[it] > 0) || r.need.map_or(false, |n| g.p.inv[n] > 0) || matches!(r.eff, Eff::Coat if g.p.hides > 0)
    }).collect();
    let hidden = list.len() - shown.len();
    if shown.is_empty() { return 0.; }
    // ค่าเริ่มต้น: สูตรแรกที่คราฟต์ได้
    if !shown.contains(&ui.sel_r) || ui.sel_r >= list.len() {
        ui.sel_r = shown.iter().copied().find(|&i| recipe_state(&g.p, &list[i]) == 0 && can_pay(&g.p, list[i].cost)).unwrap_or(shown[0]);
    }
    let r = &list[ui.sel_r];
    let st = recipe_state(&g.p, r);
    let ic = recipe_icon(r);
    let dh = detail_box(font, c, y, u, r.name, r.desc, &|x, yy, s| draw_ricon(sp, &ic, x, yy, s, false));
    let mut act = false;
    if visible(y, dh, c) {
        cost_chips(g, sp, font, r.cost, r.need, c.x + 82. * u, y + 70. * u, u);
        let lab = match st { 1 => "มีแล้ว", 2 => "ล็อก", _ => if can_pay(&g.p, r.cost) { "คราฟต์!" } else { "ของไม่พอ" } };
        if btn(ui, font, Rect::new(c.x + c.w - 112. * u, y + 20. * u, 100. * u, 40. * u), lab, st == 0 && can_pay(&g.p, r.cost), u) { act = true; }
    }
    y += dh + 4. * u;
    if st != 1 { let lack = source::lack_of(g, r.cost, r.need); if !lack.is_empty() { y += source::draw_lack(g, sp, font, c, y, u, &lack); } }
    let sel_pos = shown.iter().position(|&i| i == ui.sel_r).unwrap_or(usize::MAX);
    let hit = grid(ui, c, &mut y, shown.len(), 50. * u, 6. * u, sel_pos, |k, x, yy, s| {
        let rr = &list[shown[k]];
        let stt = recipe_state(&g.p, rr);
        let ok = stt == 0 && can_pay(&g.p, rr.cost);
        if ok { draw_rectangle_lines(x + 2., yy + 2., s - 4., s - 4., 3., hx(0xffcc44)); }
        draw_ricon(sp, &recipe_icon(rr), x + s * 0.14, yy + s * 0.14, s * 0.72, stt == 2);
        if stt == 1 { tick(x + s * 0.55, yy + s * 0.58, s * 0.38); }
        if stt == 2 { mcui::pix_lock(x + s * 0.58, yy + s * 0.55, s * 0.042); }
    });
    if let Some(k) = hit { ui.sel_r = shown[k]; }
    if hidden > 0 {
        if visible(y, 30. * u, c) {
            text(font, &format!("ยังมีอีก {} สูตร — เก็บของใหม่ๆ แล้วจะโผล่มาเอง", hidden), c.x + 8. * u, y + 20. * u, 12. * u, hx(0x9a9aaa));
        }
        y += 30. * u;
    }
    if visible(y, 24. * u, c) {
        text(font, "กรอบทอง = คราฟต์ได้เลย · ติ๊กเขียว = มีแล้ว · แม่กุญแจ = ต้องมีของก่อนหน้า", c.x + 8. * u, y + 16. * u, 11. * u, hx(0x8a8a9a));
    }
    y += 26. * u;
    if act { g.craft_from(list, ui.sel_r); }
    y - top
}

fn tab_build(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    if ui.sel_b >= BUILD_N { ui.sel_b = g.build_sel.filter(|&k| k < BUILD_N).unwrap_or(0); }
    // v31 ปุ่มย้ายของอยู่บนสุด หาง่าย (เดิมเป็นช่องสุดท้ายของตาราง)
    if visible(y, 46. * u, c) {
        let mi = PIECES.iter().position(|p| p.obj == OBJ_MOVE).unwrap();
        let on = g.build_sel == Some(mi);
        if btn(ui, font, Rect::new(c.x + 8. * u, y + 4. * u, c.w - 16. * u, 38. * u), if on { "กำลังใช้: ย้ายของ (แตะเพื่อเลิก)" } else { "ย้ายของที่สร้างแล้ว — กดที่ของแล้วลากไปวางได้เลย" }, true, u) {
            if on { g.move_cancel(); g.build_sel = None; } else { g.build_sel = Some(mi); g.build_cur = None; g.rs.carry = None; g.toast("ย้ายของ: กดที่ของค้างไว้แล้วลากไปวาง"); ui.close = true; }
        }
    }
    y += 50. * u;
    let pc = &PIECES[ui.sel_b];
    let dh = detail_box(font, c, y, u, pc.name, pc.desc, &|x, yy, s| piece_icon(sp, pc, x, yy, s, WHITE));
    let mut pick = false;
    let mut cancel = false;
    if visible(y, dh, c) {
        cost_chips(g, sp, font, pc.cost, None, c.x + 82. * u, y + 70. * u, u);
        let using = g.build_sel == Some(ui.sel_b);
        if btn(ui, font, Rect::new(c.x + c.w - 112. * u, y + 20. * u, 100. * u, 40. * u), if using { "ยกเลิก" } else if can_pay(&g.p, pc.cost) { "เลือกวาง" } else { "ของไม่พอ" }, using || can_pay(&g.p, pc.cost), u) {
            if using { cancel = true; } else { pick = true; }
        }
    }
    y += dh + 4. * u;
    { let lack = source::lack_of(g, pc.cost, None); if !lack.is_empty() { y += source::draw_lack(g, sp, font, c, y, u, &lack); } }
    let cur = ui.sel_b;
    let hit = grid(ui, c, &mut y, BUILD_N, 50. * u, 6. * u, cur, |k, x, yy, s| {
        let p = &PIECES[k];
        let ok = can_pay(&g.p, p.cost);
        piece_icon(sp, p, x + s * 0.14, yy + s * 0.14, s * 0.72, if ok { WHITE } else { Color::new(1., 1., 1., 0.4) });
        if g.build_sel == Some(k) { tick(x + s * 0.55, yy + s * 0.58, s * 0.38); }
    });
    if let Some(k) = hit { ui.sel_b = k; }
    if visible(y, 40. * u, c) {
        text(font, "เลือกแล้วแตะพื้นตรงที่จะวาง · ตีเพื่อรื้อถอน · อยากย้ายของที่สร้างแล้ว = ปุ่ม \"ย้ายของ\" ด้านบน (หรือปุ่ม \"ย้าย\" บนจอ / Z)", c.x + 8. * u, y + 22. * u, 12. * u, hx(0x9a9aaa));
    }
    y += 40. * u;
    if cancel { g.build_sel = None; }
    if pick {
        g.build_sel = Some(ui.sel_b);
        g.toast(format!("เลือก {} — กด R / ปุ่มสร้างเพื่อวาง", PIECES[ui.sel_b].name));
        ui.close = true;
    }
    y - top
}

fn tab_gear(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    let rh = 46. * u;
    header(font, c, y, "กระเป๋า", u);
    y += 28. * u;
    if g.p.can {
        if visible(y, 24. * u, c) { text(font, &format!("บัวรดน้ำ: น้ำ {}/{}", g.p.water, WATER_MAX), c.x + 8. * u, y + 16. * u, 12. * u, hx(0x9ad0ff)); }
        y += 24. * u;
    }
    let mut seed_pick: Option<u8> = None;
    let mut crop_eat: Option<usize> = None;
    let mut raw_eat: Option<usize> = None;
    let mut drink = false;
    let mut dish: Option<usize> = None;
    // กระเป๋าแบบตารางช่อง (แสดงเฉพาะของที่มี) + กล่องรายละเอียดของช่องที่เลือก
    let owned: Vec<usize> = (0..NI).filter(|&i| g.p.inv[i] > 0).collect();
    if owned.is_empty() {
        if visible(y, 30. * u, c) { text(font, "กระเป๋าว่าง — ตีต้นไม้/หิน เก็บผลไม้ เพื่อเริ่มต้น", c.x + 8. * u, y + 20. * u, 12. * u, hx(0x9a9aaa)); }
        y += 30. * u;
    } else {
        if !owned.contains(&ui.sel_i) { ui.sel_i = owned[0]; }
        let it = ui.sel_i;
        let desc = item_desc(it);
        let dh = detail_box(font, c, y, u, &format!("{} x{}", INAME[it], g.p.inv[it]), &desc, &|x, yy, s| icon_c(sp, it, x, yy, s, WHITE));
        if visible(y, dh, c) {
            let br = Rect::new(c.x + c.w - 112. * u, y + 20. * u, 100. * u, 40. * u);
            if it == POTION && btn(ui, font, br, "ดื่มยา", true, u) { drink = true; }
            if (it == SEED_CORN || it == SEED_CARROT) && btn(ui, font, br, if g.seed_sel as usize == it - SEED_CORN { "ปลูกอยู่" } else { "เลือกปลูก" }, g.seed_sel as usize != it - SEED_CORN, u) { seed_pick = Some((it - SEED_CORN) as u8); }
            if (it == CORN || it == CARROT) && btn(ui, font, br, "กิน", true, u) { crop_eat = Some(it); }
            if matches!(it, BERRY | FISH | MEAT | SHROOM | HERB) && btn(ui, font, br, if it == HERB { "ใช้" } else if matches!(it, FISH | MEAT | SHROOM) { "กิน (ดิบ)" } else { "กิน" }, true, u) { raw_eat = Some(it); }
            if (DISH_STEW..=DISH_FISH).contains(&it) && btn(ui, font, br, "กิน", true, u) { dish = Some(it); }
        }
        y += dh + 4. * u;
        let sel = owned.iter().position(|&i| i == ui.sel_i).unwrap_or(usize::MAX);
        let hit = grid(ui, c, &mut y, owned.len(), 50. * u, 6. * u, sel, |k, x, yy, s| {
            let i = owned[k];
            icon_c(sp, i, x + s * 0.14, yy + s * 0.14, s * 0.72, WHITE);
            mcui::slot_count(font, g.p.inv[i], x, yy, s);
        });
        if let Some(k) = hit { ui.sel_i = owned[k]; }
        y += 8. * u;
    }
    // v32 คู่มือ: ของแต่ละอย่างหาได้จากไหน (แตะเลือกชิ้นที่อยากรู้)
    header(font, c, y, "คู่มือ: หาของได้จากไหน", u);
    y += 28. * u;
    {
        let items: Vec<usize> = (0..NI).collect();
        if ui.guide_i >= NI { ui.guide_i = GEM; }
        let gi = ui.guide_i;
        y += source::draw_howto(g, sp, font, c, c.x + 6. * u, y, c.w - 12. * u, u, gi, None, 3) + 6. * u;
        if let Some(k) = grid(ui, c, &mut y, items.len(), 42. * u, 5. * u, gi, |k, x, yy, s| {
            icon_c(sp, items[k], x + s * 0.14, yy + s * 0.14, s * 0.72, if g.p.inv[items[k]] > 0 { WHITE } else { Color::new(1., 1., 1., 0.55) });
        }) { ui.guide_i = items[k]; }
        y += 8. * u;
    }
    header(font, c, y, "อาวุธ (กด Tab เพื่อสลับ)", u);
    y += 28. * u;
    let mut equip: Option<u8> = None;
    let mut elem_pick: Option<usize> = None;
    for w in 0..3u8 {
        if visible(y, rh, c) {
            let (ic, name, desc) = match w {
                0 => (ICON_SWORD, SWN[g.p.sword].to_string(), format!("ระยะประชิด · ดาเมจ {}", SWD[g.p.sword])),
                1 => (IC_BOW, "ธนู".to_string(), if g.p.bow { format!("ลูกธนูคงเหลือ {}", g.p.inv[ARROW]) } else { "ยังไม่มี — คราฟต์ด้วยไม้".to_string() }),
                _ => (IC_STAFF, STAFF_N[g.p.staff as usize].replace("-", "คทา"), if g.p.staff > 0 { "ยิงเวทไม่ใช้กระสุน".to_string() } else { "ยังไม่มี — คราฟต์ด้วยอัญมณี".to_string() }),
            };
            let own = g.p.has_weapon(w);
            icon_c(sp, ic, c.x + 8. * u, y + 8. * u, 28. * u, if own { WHITE } else { Color::new(1., 1., 1., 0.35) });
            text(font, &name, c.x + 46. * u, y + 20. * u, 15. * u, if own { hx(0xffcc66) } else { hx(0x777777) });
            text(font, &desc, c.x + 46. * u, y + 36. * u, 11. * u, hx(0xaaaaaa));
            let cur = g.p.weapon == w && !(w == 0 && g.p.bare);
            if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 7. * u, 92. * u, 32. * u), if cur { "ถืออยู่" } else if own { "ถือ" } else { "ไม่มี" }, own && !cur, u) {
                equip = Some(w);
            }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    if let Some(w) = equip {
        g.p.weapon = w;
        g.p.bare = false;
    }
    // มือเปล่า (ไม่ถืออะไร)
    if g.p.sword > 0 || g.p.bow || g.p.staff > 0 {
        if visible(y, rh, c) {
            draw_fist(c.x + 22. * u, y + 22. * u, 11. * u);
            text(font, "มือเปล่า", c.x + 46. * u, y + 20. * u, 15. * u, hx(0xffcc66));
            text(font, "ต่อยดาเมจ 1 · เก็บดาบ/อาวุธไว้", c.x + 46. * u, y + 36. * u, 11. * u, hx(0xaaaaaa));
            let cur = g.p.weapon == 0 && g.p.bare;
            if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 7. * u, 92. * u, 32. * u), if cur { "ถืออยู่" } else { "ถือ" }, !cur, u) { g.p.weapon = 0; g.p.bare = true; }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    if let Some(e) = elem_pick {
        g.p.elem = e as u8;
    }
    if g.p.staff > 0 {
        if visible(y, rh, c) {
            text(font, "ธาตุคทา (แตะ Tab ตอนถือคทาเพื่อสลับ)", c.x + 8. * u, y + 28. * u, 12. * u, hx(0xaaaaaa));
            for e in 0..3usize {
                let r = Rect::new(c.x + c.w - (3 - e) as f32 * 100. * u, y + 7. * u, 94. * u, 32. * u);
                let own = g.p.elems & (1 << e) != 0;
                let cur = g.p.elem as usize == e;
                if btn(ui, font, r, if cur { "ใช้อยู่" } else if own { ELEM_N[e] } else { "ล็อก" }, own && !cur, u) { elem_pick = Some(e); }
            }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    header(font, c, y, "ชุดเกราะ", u);
    y += 28. * u;
    // พรีวิวตัวละคร (ใส่จริงตามเกราะ/ชุดที่สวม) + ปุ่มสลับชุดที่คราฟต์
    let ph = 100. * u;
    if visible(y, ph, c) {
        draw_rectangle(c.x + 6. * u, y + 4. * u, 110. * u, ph - 8. * u, hx(0x2a2d3f));
        let sc = 5. * u;
        let f = ((get_time() * 2.) as usize) % 3;
        draw_skin(sp, &g.p, c.x + 61. * u - 6. * sc, y + 8. * u, sc, 0, if f == 0 { 0 } else { f }, false, WHITE);
        let parts = skin_parts(&g.p);
        let nm = |v: usize| match v { 0 => "พื้นฐาน".to_string(), 1..=4 => ATN[v].to_string(), 5 => "ชาวไร่".to_string(), _ => "นักล่า".to_string() };
        text(font, &format!("หัว: {}", nm(parts[0])), c.x + 128. * u, y + 24. * u, 13. * u, hx(0xdddddd));
        text(font, &format!("ตัว: {}", nm(parts[1])), c.x + 128. * u, y + 44. * u, 13. * u, hx(0xdddddd));
        text(font, &format!("ขา: {}", nm(parts[2])), c.x + 128. * u, y + 64. * u, 13. * u, hx(0xdddddd));
        text(font, &format!("ชุด: {}", OUTFIT_N[g.p.outfit as usize]), c.x + 128. * u, y + 86. * u, 12. * u, hx(0xffcc66));
        let owned = g.p.outfits != 0;
        if btn(ui, font, Rect::new(c.x + c.w - 124. * u, y + 30. * u, 112. * u, 32. * u), "เปลี่ยนชุด", owned, u) {
            // วนระหว่าง ไม่ใส่ → ชุดที่มี
            let mut o = g.p.outfit;
            for _ in 0..3 { o = (o + 1) % 3; if o == 0 || g.p.outfits & (1 << (o - 1)) != 0 { break; } }
            g.p.outfit = o;
            g.save();
        }
        if !owned { text(font, "คราฟต์ชุดได้ในเมนูคราฟต์", c.x + c.w - 190. * u, y + 80. * u, 11. * u, hx(0x888899)); }
    }
    y += ph;
    for s in 0..3 {
        if visible(y, rh, c) {
            let t = g.p.armor[s] as usize;
            draw_rectangle(c.x + 6. * u, y + 6. * u, 34. * u, 34. * u, hx(0x2a2d3f));
            icon_c(sp, SLOT_ICON[s], c.x + 8. * u, y + 8. * u, 30. * u, if t > 0 { hx(ATC[t]) } else { Color::new(1., 1., 1., 0.25) });
            text(font, SLOT_N[s], c.x + 48. * u, y + 20. * u, 15. * u, hx(0xffcc66));
            text(font, &format!("ระดับ: {}", ATN[t]), c.x + 48. * u, y + 36. * u, 11. * u, hx(ATC[t.max(1)]));
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    if visible(y, rh, c) {
        text(font, &format!("ลดดาเมจรวม {:.0}% · ความเร็วเดิน +{:.0}%", g.p.reduction() * 100., (g.p.speed_mul() - 1.) * 100.), c.x + 8. * u, y + 26. * u, 13. * u, hx(0x88ff88));
    }
    y += 34. * u;
    if let Some(s) = seed_pick { g.seed_sel = s; }
    if let Some(i) = crop_eat { g.eat_crop(i); }
    if let Some(i) = raw_eat { g.eat_generic(i); }
    if drink {
        g.drink();
    }
    if let Some(i) = dish {
        g.use_dish(i);
    }
    y - top
}

/// คำอธิบายสั้นของไอเทม (สำหรับเด็ก: บอกว่าใช้ทำอะไร)
fn item_desc(i: usize) -> String {
    match i {
        WOOD => "ใช้คราฟต์และสร้างของแทบทุกอย่าง",
        STONE => "ใช้ทำเครื่องมือ เกราะ และกำแพง",
        BERRY => "กินแก้หิว · ใช้ปลูกต้นไม้ (เมนูสร้าง)",
        GEM => "ของมีค่า ใช้ทำดาบ/คทา/เกราะชั้นดี",
        COIN => "ใช้ซื้อของจากพ่อค้า",
        FIRE => "วางเป็นกองไฟ ให้แสงและความอบอุ่น",
        ARROW => "กระสุนของธนู",
        POTION => "ดื่มแล้วเลือดเพิ่ม",
        MEAT => "กินดิบอาจท้องเสีย — เอาไปย่างที่กองไฟก่อน",
        FISH => "กินดิบอาจท้องเสีย — ทำปลาย่างก่อน",
        SHROOM | HERB => "ใช้ทำอาหาร/ยา",
        DISH_STEW | DISH_SPEED | DISH_CRIT | DISH_FISH => "อาหารปรุงสุก อิ่มนานและได้บัฟ",
        SEED_CORN | SEED_CARROT => "เลือกแล้วใช้จอบปลูกในแปลง",
        CORN | CARROT => "ผลผลิต กินได้หรือใช้ทำอาหาร",
        SHARD => "วัสดุหายาก ใช้ที่โต๊ะคราฟต์ขั้นสูง",
        _ => "วัสดุพิเศษสำหรับคราฟต์",
    }.to_string()
}

fn tab_chest(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let Some(k) = ui.chest else { return 0. };
    let top = c.y - ui.scroll;
    let mut y = top;
    let mut mv: Option<(usize, i32, bool)> = None;
    let content = g.chests.get(&k).copied().unwrap_or([0; NI]);
    // แบบ Minecraft: แตะช่องเพื่อย้ายทั้งกอง (กล่อง ↔ กระเป๋า)
    header(font, c, y, "กล่องเก็บของ — แตะช่องเพื่อเอาออกทั้งกอง", u);
    y += 28. * u;
    let inside: Vec<usize> = (0..NI).filter(|&i| content[i] > 0).collect();
    if inside.is_empty() {
        if visible(y, 30. * u, c) { text(font, "(กล่องว่าง)", c.x + 10. * u, y + 20. * u, 13. * u, hx(0x888899)); }
        y += 30. * u;
    } else if let Some(h) = grid(ui, c, &mut y, inside.len(), 48. * u, 6. * u, usize::MAX, |n, x, yy, s| {
        icon_c(sp, inside[n], x + s * 0.14, yy + s * 0.14, s * 0.72, WHITE);
        mcui::slot_count(font, content[inside[n]], x, yy, s);
    }) { mv = Some((inside[h], content[inside[h]], false)); }
    y += 10. * u;
    header(font, c, y, "กระเป๋า — แตะช่องเพื่อใส่ลงกล่องทั้งกอง", u);
    y += 28. * u;
    let bag: Vec<usize> = (0..NI).filter(|&i| g.p.inv[i] > 0).collect();
    if let Some(h) = grid(ui, c, &mut y, bag.len(), 48. * u, 6. * u, usize::MAX, |n, x, yy, s| {
        icon_c(sp, bag[n], x + s * 0.14, yy + s * 0.14, s * 0.72, WHITE);
        mcui::slot_count(font, g.p.inv[bag[n]], x, yy, s);
    }) { mv = Some((bag[h], g.p.inv[bag[h]], true)); }
    y += 10. * u;
    if let Some((i, n, into)) = mv {
        g.chest_move(k, i, n, into);
        play(&g.sfx.eat);
    }
    y - top
}

fn tab_shop(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    if visible(y, 70. * u, c) {
        let m = sp.ex.merchant[((get_time() * 2.) as i32 % 2) as usize];
        draw_texture_ex(&sp.tex, c.x + 10. * u, y + 6. * u, WHITE, DrawTextureParams { source: Some(m), dest_size: Some(vec2(m.w * 2.6 * u, m.h * 2.6 * u)), ..Default::default() });
        let kind = g.shop_camp.map_or(0, |k| g.w.camp_kind(k));
        text(font, camp_title(kind), c.x + 70. * u, y + 22. * u, 15. * u, hx(0xffcc66));
        text(font, if kind > 0 && ui.greet % 2 == 0 { camp_greet(kind) } else { GREETS[ui.greet % GREETS.len()] }, c.x + 70. * u, y + 42. * u, 12. * u, hx(0xdddddd));
        icon_c(sp, COIN, c.x + 70. * u, y + 50. * u, 14. * u, WHITE);
        text(font, &format!("เหรียญของคุณ: {}", g.p.inv[COIN]), c.x + 88. * u, y + 62. * u, 12. * u, hx(0xffd84a));
    }
    y += 76. * u;
    let rh = 58. * u;
    // ---- สินค้า (ตารางช่อง + กล่องรายละเอียด)
    let offers = camp_offers(g.shop_camp.map_or(0, |k| g.w.camp_kind(k)));
    if ui.sel_i >= 1000 || ui.sel_i < 900 || ui.sel_i - 900 >= offers.len() { ui.sel_i = 900; }
    let oi = ui.sel_i - 900;
    let of = offers[oi];
    let owned = of.once && g.p.inv[of.item] > 0;
    let mut buy = false;
    let dh = detail_box(font, c, y, u, of.name, of.desc, &|x, yy, s| icon_c(sp, of.item, x, yy, s, WHITE));
    if visible(y, dh, c) {
        cost_chips(g, sp, font, &[(COIN, of.price)], None, c.x + 82. * u, y + 70. * u, u);
        if btn(ui, font, Rect::new(c.x + c.w - 112. * u, y + 20. * u, 100. * u, 40. * u), if owned { "มีแล้ว" } else if g.p.inv[COIN] >= of.price { "ซื้อ" } else { "เงินไม่พอ" }, !owned && g.p.inv[COIN] >= of.price, u) { buy = true; }
    }
    y += dh + 4. * u;
    if !owned && g.p.inv[COIN] < of.price { y += source::draw_lack(g, sp, font, c, y, u, &[(COIN, of.price - g.p.inv[COIN])]); }
    if let Some(h) = grid(ui, c, &mut y, offers.len(), 48. * u, 6. * u, oi, |n, x, yy, s| {
        let o = offers[n];
        icon_c(sp, o.item, x + s * 0.14, yy + s * 0.14, s * 0.72, if g.p.inv[COIN] >= o.price { WHITE } else { Color::new(1., 1., 1., 0.45) });
        mcui::slot_count(font, o.qty, x, yy, s);
    }) { ui.sel_i = 900 + h; }
    if buy { g.buy_offer(of); }
    y += 8. * u;
    // ---- เควสต์ย่อย
    header(font, c, y, "เควสต์ของแคมป์นี้", u);
    y += 28. * u;
    let mut qact = 0u8;
    if visible(y, 70. * u, c) {
        match g.quest.clone() {
            Some(q) => {
                text(font, &quest_title(&q), c.x + 8. * u, y + 18. * u, 14. * u, hx(0xffcc66));
                text(font, &format!("ความคืบหน้า {}/{} · รางวัล: {}", q.prog, q.goal, quest_reward_text(&q)), c.x + 8. * u, y + 36. * u, 11. * u, hx(0xaaaaaa));
                let done = q.prog >= q.goal;
                if btn(ui, font, Rect::new(c.x + c.w - 210. * u, y + 40. * u, 98. * u, 26. * u), "ส่งเควสต์", done && g.shop_camp == Some(q.camp), u) { qact = 2; }
                if btn(ui, font, Rect::new(c.x + c.w - 106. * u, y + 40. * u, 98. * u, 26. * u), "ยกเลิก", true, u) { qact = 3; }
                if done && g.shop_camp != Some(q.camp) {
                    text(font, "ต้องส่งที่แคมป์ที่รับเควสต์", c.x + 8. * u, y + 56. * u, 11. * u, hx(0xff9988));
                }
            }
            None => match g.shop_camp {
                Some(k) => {
                    let q = g.offered_quest(k);
                    text(font, &quest_title(&q), c.x + 8. * u, y + 18. * u, 14. * u, hx(0xffcc66));
                    text(font, &format!("รางวัล: {}", quest_reward_text(&q)), c.x + 8. * u, y + 36. * u, 11. * u, hx(0xaaffaa));
                    if btn(ui, font, Rect::new(c.x + c.w - 106. * u, y + 40. * u, 98. * u, 26. * u), "รับเควสต์", true, u) { qact = 1; }
                }
                None => text(font, "ไม่มีเควสต์", c.x + 8. * u, y + 24. * u, 13. * u, hx(0x888899)),
            },
        }
    }
    y += 72. * u;
    {
        let qk = match g.quest.as_ref() { Some(q) => Some(q.kind), None => g.shop_camp.map(|k| g.offered_quest(k).kind) };
        if let Some(qk) = qk { y += source::draw_quest_howto(g, sp, font, c, c.x + 6. * u, y, c.w - 12. * u, u, qk) + 6. * u; }
    }
    match qact {
        1 => { if let Some(k) = g.shop_camp { g.accept_quest(k); } }
        2 => g.claim_quest(),
        3 => g.abandon_quest(),
        _ => {}
    }
    // ---- จ้างผู้ติดตาม
    header(font, c, y, "จ้างผู้ติดตาม (สู้ + เก็บของ)", u);
    y += 28. * u;
    let mut hire: Option<u8> = None;
    for k in 0..2u8 {
        if visible(y, rh, c) {
            let cur = g.comp.as_ref().map_or(false, |c| c.kind == k);
            let r = sp.ex.comp[k as usize * 2];
            draw_texture_ex(&sp.tex, c.x + 8. * u, y + 8. * u, WHITE, DrawTextureParams { source: Some(r), dest_size: Some(vec2(r.w * 2.2 * u, r.h * 2.2 * u)), ..Default::default() });
            text(font, COMP_N[k as usize], c.x + 52. * u, y + 20. * u, 14. * u, hx(0xffcc66));
            text(font, COMP_D[k as usize], c.x + 52. * u, y + 36. * u, 11. * u, hx(0xaaaaaa));
            icon_c(sp, COIN, c.x + 52. * u, y + 40. * u, 11. * u, WHITE);
            text(font, &format!("{}", COMP_PRICE[k as usize]), c.x + 66. * u, y + 50. * u, 11. * u, if g.p.inv[COIN] >= COMP_PRICE[k as usize] { hx(0xaaffaa) } else { hx(0xff8888) });
            if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 10. * u, 92. * u, 32. * u), if cur { "จ้างอยู่" } else { "จ้าง" }, !cur && g.p.inv[COIN] >= COMP_PRICE[k as usize], u) {
                hire = Some(k);
            }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    if let Some(k) = hire { g.hire(k); }
    if visible(y, 50. * u, c) {
        icon_c(sp, FISH, c.x + 8. * u, y + 8. * u, 26. * u, WHITE);
        text(font, &format!("รับซื้อปลา ตัวละ {} เหรียญ", FISH_PRICE), c.x + 44. * u, y + 20. * u, 14. * u, hx(0xffcc66));
        text(font, &format!("ปลาในกระเป๋า {} ตัว", g.p.inv[FISH]), c.x + 44. * u, y + 36. * u, 11. * u, hx(0xaaaaaa));
        if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 8. * u, 92. * u, 32. * u), "ขายทั้งหมด", g.p.inv[FISH] > 0, u) { g.sell_fish(); }
        sep(c, y + 48. * u);
    }
    y += 52. * u;
    if visible(y, 50. * u, c) {
        icon_c(sp, CORN, c.x + 8. * u, y + 8. * u, 26. * u, WHITE);
        text(font, &format!("รับซื้อผลผลิต ข้าวโพด {} · แครอท {} เหรียญ", CROP_PRICE[0], CROP_PRICE[1]), c.x + 44. * u, y + 20. * u, 14. * u, hx(0xffcc66));
        text(font, &format!("ข้าวโพด {} · แครอท {}", g.p.inv[CORN], g.p.inv[CARROT]), c.x + 44. * u, y + 36. * u, 11. * u, hx(0xaaaaaa));
        if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 8. * u, 92. * u, 32. * u), "ขายทั้งหมด", g.p.inv[CORN] + g.p.inv[CARROT] > 0, u) { g.sell_crops(); }
        sep(c, y + 48. * u);
    }
    y += 52. * u;
    y - top
}

fn tab_forge(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    let rh = 66. * u;
    header(font, c, y, "ทั่งตีบวก — ใช้แร่/เศษดาว/ชิ้นส่วนบอส (ออร่าอาวุธเปลี่ยนสีตามระดับ)", u);
    y += 28. * u;
    let mut act: Option<usize> = None;
    for t in 0..6usize {
        if !visible(y, rh, c) { y += rh; continue; }
        let own = g.p.forge_owned(t);
        let pl = g.p.forge_plus(t);
        let ic = [ICON_SWORD, IC_BOW, IC_STAFF, IC_HELM, IC_CHEST, IC_BOOT][t];
        icon_c(sp, ic, c.x + 8. * u, y + 8. * u, 28. * u, if own { WHITE } else { Color::new(1., 1., 1., 0.3) });
        let nm = g.p.forge_name(t);
        text(font, &format!("{} +{}", nm, pl), c.x + 44. * u, y + 18. * u, 14. * u, if own { hx(AURA[pl as usize]) } else { hx(0x777777) });
        let eff = if t < 3 { format!("ดาเมจ x{:.1}", 1. + 0.2 * pl as f32) } else { format!("ลดดาเมจ +{:.1}%", 1.5 * pl as f32) };
        text(font, &eff, c.x + 44. * u, y + 33. * u, 11. * u, hx(0xaaaaaa));
        if own && pl < PLUS_MAX {
            costs(g, sp, font, forge_cost(pl + 1), None, c.x + 44. * u, y + 40. * u, u);
            if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 16. * u, 92. * u, 32. * u), &format!("ตีเป็น +{}", pl + 1), can_pay(&g.p, forge_cost(pl + 1)), u) { act = Some(t); }
        } else {
            text(font, if !own { "ยังไม่มีไอเทมนี้" } else { "สูงสุดแล้ว" }, c.x + 44. * u, y + 52. * u, 11. * u, hx(0x888899));
        }
        sep(c, y + rh - 2.);
        y += rh;
    }
    if let Some(t) = act { g.forge(t); }
    y - top
}

fn tab_cook(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    if ui.sel_r >= 2000 || ui.sel_r < 1000 || ui.sel_r - 1000 >= DISHES.len() { ui.sel_r = 1000; }
    let di = ui.sel_r - 1000;
    let d = &DISHES[di];
    let mut act = false;
    let dh = detail_box(font, c, y, u, d.name, d.desc, &|x, yy, s| icon_c(sp, d.item, x, yy, s, WHITE));
    if visible(y, dh, c) {
        cost_chips(g, sp, font, d.cost, None, c.x + 82. * u, y + 70. * u, u);
        if btn(ui, font, Rect::new(c.x + c.w - 112. * u, y + 20. * u, 100. * u, 40. * u), if can_pay(&g.p, d.cost) { "ทำอาหาร!" } else { "ของไม่พอ" }, can_pay(&g.p, d.cost), u) { act = true; }
    }
    y += dh + 4. * u;
    if let Some(h) = grid(ui, c, &mut y, DISHES.len(), 50. * u, 6. * u, di, |n, x, yy, s| {
        let dd = &DISHES[n];
        if can_pay(&g.p, dd.cost) { draw_rectangle_lines(x + 2., yy + 2., s - 4., s - 4., 3., hx(0xffcc44)); }
        icon_c(sp, dd.item, x + s * 0.14, yy + s * 0.14, s * 0.72, WHITE);
        mcui::slot_count(font, g.p.inv[dd.item], x, yy, s);
    }) { ui.sel_r = 1000 + h; }
    if visible(y, 40. * u, c) {
        text(font, "วัตถุดิบ: เนื้อ/ปลา (ล่าสัตว์ ตกปลา) · เห็ด (สไลม์ หนองน้ำ) · สมุนไพร (โครงกระดูก ต้นไม้ยักษ์) · กินได้จากแถบล่างหรือกระเป๋า", c.x + 8. * u, y + 20. * u, 11. * u, hx(0x888899));
    }
    y += 40. * u;
    if act { g.cook(di); }
    y - top
}

fn tab_stats(g: &mut Game, _sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    let rh = 56. * u;
    text(font, &format!("เลเวล {} · แต้มคงเหลือ {} (ได้ {} แต้มต่อเลเวล)", g.p.lvl, g.p.sp, SP_PER_LV), c.x + 8. * u, y + 20. * u, 14. * u, if g.p.sp > 0 { hx(0x88ff88) } else { hx(0xdddddd) });
    y += 30. * u;
    let mut act: Option<usize> = None;
    for i in 0..4usize {
        if visible(y, rh, c) {
            text(font, &format!("{} ({}/{})", STN[i], g.p.st[i], ST_MAX), c.x + 8. * u, y + 20. * u, 15. * u, hx(0xffcc66));
            text(font, STD[i], c.x + 8. * u, y + 36. * u, 11. * u, hx(0xaaaaaa));
            for k in 0..ST_MAX {
                let on = k < g.p.st[i];
                draw_rectangle(c.x + 8. * u + k as f32 * 11. * u, y + 41. * u, 9. * u, 7. * u, if on { hx(0xffcc66) } else { hx(0x333344) });
            }
            if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 10. * u, 92. * u, 32. * u), "+1", g.p.sp > 0 && g.p.st[i] < ST_MAX, u) { act = Some(i); }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    if visible(y, 60. * u, c) {
        text(font, &format!("คริติคอล {:.0}% · ดาเมจ ดาบ x{:.2} · เลือด {}", g.p.crit_rate() * 100., g.p.atk_mul(0), g.p.maxhp), c.x + 8. * u, y + 22. * u, 12. * u, hx(0x88ff88));
    }
    y += 40. * u;
    // ---- สมุดบันทึกการค้นพบ: ครบหมวดได้รางวัล
    header(font, c, y, "สมุดบันทึกการค้นพบ (ครบหมวดได้รางวัล)", u);
    y += 28. * u;
    for cat in 0..CODEX.len() {
        let (a, b, list) = g.codex_progress(cat);
        let done = g.codex_done & (1 << cat) != 0;
        let names: Vec<String> = list.iter().map(|(n, f)| if *f { n.clone() } else { "???".to_string() }).collect();
        let body = format!("{} {}/{} · รางวัล: {}{} — {}", CODEX[cat].0, a, b, CODEX[cat].1, if done { " (ได้แล้ว)" } else { "" }, names.join(" · "));
        let ws = wrap_th(font, &body, 12. * u, c.w - 24. * u);
        let hh = ws.len() as f32 * 15. * u + 8. * u;
        if visible(y, hh, c) { for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, y + 14. * u + k as f32 * 15. * u, 12. * u, if done { hx(0x88ff88) } else { hx(0xdddddd) }); } }
        y += hh;
    }
    // ---- เป้าหมาย (ความสำเร็จ)
    header(font, c, y, &format!("เป้าหมาย {}/{}", g.adv_done.count_ones(), ADV.len()), u);
    y += 28. * u;
    for (k, (n, h, cn, _)) in ADV.iter().enumerate() {
        let done = g.adv_done & (1 << k) != 0;
        if visible(y, 20. * u, c) {
            if done { tick(c.x + 8. * u, y + 4. * u, 12. * u); } else { draw_rectangle_lines(c.x + 8. * u, y + 4. * u, 12. * u, 12. * u, 2., hx(0x777788)); }
            text(font, &format!("{} — {} (+{} เหรียญ)", n, h, cn), c.x + 28. * u, y + 15. * u, 12. * u, if done { hx(0x88ff88) } else { hx(0xdddddd) });
        }
        y += 20. * u;
    }
    y += 8. * u;
    if let Some(i) = act { g.spend_stat(i); }
    y - top
}

fn tab_settings(g: &mut Game, _sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    let rh = 50. * u;
    let mut flip = 0;
    let rows: [(&str, &str, bool); 4] = [
        ("เสียงเอฟเฟกต์", if crate::assets::HQ_LOADED.load(std::sync::atomic::Ordering::Relaxed) >= 60 { "ไฟล์เสียงสมจริงโหลดครบแล้ว" } else { "ใช้เสียงสำรองในโค้ด (โหลดไฟล์เสียงไม่เจอ)" }, g.sfx_on),
        ("เพลงประกอบ", "เปลี่ยนตามฉาก: สำรวจ · แคมป์ · บอส · ดันเจี้ยน/กลางคืน", g.bgm_on),
        ("เล็งอัตโนมัติ", "แตะโจมตี = ยิง/ฟันมอนสเตอร์ใกล้สุด (ลากเล็งเองได้เหมือนเดิม)", g.auto_aim),
        ("เครื่องมืออัตโนมัติ (Smart Tool)", "กดโจมตีใกล้ต้นไม้/หิน = ถือดาบแล้วหันไปตีให้ · ใกล้มอนสเตอร์ = ถืออาวุธสู้ · เมืองห้ามอาวุธ = ใช้มือเปล่า", g.smart),
    ];
    for (i, (n, d, on)) in rows.iter().enumerate() {
        if visible(y, rh, c) {
            text(font, n, c.x + 8. * u, y + 20. * u, 15. * u, hx(0xffcc66));
            text(font, d, c.x + 8. * u, y + 38. * u, 11. * u, hx(0xaaaaaa));
            if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 9. * u, 92. * u, 32. * u), if *on { "เปิด" } else { "ปิด" }, true, u) { flip = i + 1; }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    // ---- กราฟิก/ประสิทธิภาพ (ค่าส่วนกลาง ใช้ร่วมกับหน้าเมนูหลัก)
    let grows: [(&str, &str, String); 9] = [
        ("ความยาก", "ง่าย = โดนตีเบา หิวช้า · ยาก = โหดขึ้น · สันติ = ไม่มีมอนสเตอร์/สัตว์ดุนอกดันเจี้ยน ไม่หิว ไม่กระหาย (เหมาะกับเด็ก)", diff_name().to_string()),
        ("เฟรมเรต", "30 ประหยัดแบต · 60 ลื่น · ไม่จำกัด = ตามรีเฟรชเรตจอ", prefs::fps_name().to_string()),
        ("เอฟเฟกต์ฝน/หิมะ", "ปิดแล้วเครื่องแรงน้อยจะลื่นขึ้น (ผลช้าลง/มืดยังอยู่)", if prefs::weather_fx() { "เปิด" } else { "ปิด" }.to_string()),
        ("แสงไดนามิก", "สูง = ละเอียด · ต่ำ = เร็วขึ้น · ปิด = มืดเรียบ", prefs::light_name().to_string()),
        ("ขนาดปุ่มทัช", "ปรับให้ใหญ่/เล็กตามมือ", prefs::touch_name().to_string()),
        ("กล้องสั่น", "ตอนโดนตี/ระเบิด", if prefs::shake() { "เปิด" } else { "ปิด" }.to_string()),
        ("แสดง FPS", "มุมซ้ายล่าง", if prefs::show_fps() { "เปิด" } else { "ปิด" }.to_string()),
        ("เอาตัวรอด", "หิว/กระหาย/เหนื่อย · ปกติ = ไม่ตายเพราะหิว · ยาก = หมดสติได้ · ปิด = ไม่มี", prefs::survival_name().to_string()),
        ("ที่ปรึกษา", "เด้งคำแนะนำตามสถานการณ์ (ไม่มีน้ำ หิว ทัพศัตรูมา ฯลฯ) · ดูรายการเต็มที่หน้าภาพรวมเมือง", if prefs::advisor() { "เปิด" } else { "ปิด" }.to_string()),
    ];
    let mut gflip = usize::MAX;
    for (i, (n, d, val)) in grows.iter().enumerate() {
        if visible(y, rh, c) {
            text(font, n, c.x + 8. * u, y + 20. * u, 15. * u, hx(0xffcc66));
            text(font, d, c.x + 8. * u, y + 38. * u, 11. * u, hx(0xaaaaaa));
            if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 9. * u, 92. * u, 32. * u), val, true, u) { gflip = i; }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    match gflip {
        0 => prefs::cycle_diff(),
        1 => prefs::cycle_fps(),
        2 => prefs::toggle(&prefs::WEATHER_FX),
        3 => prefs::cycle_light(),
        4 => prefs::cycle_touch(),
        5 => prefs::toggle(&prefs::SHAKE),
        6 => prefs::toggle(&prefs::SHOW_FPS),
        7 => prefs::cycle_survival(),
        8 => { prefs::toggle(&prefs::ADVISOR); if !prefs::advisor() { g.rs.adv_show = None; } }
        _ => {}
    }
    if visible(y, rh, c) {
        let on = g.tut < TUT_N;
        text(font, if on { "บทเรียนเริ่มเกม: กำลังทำ" } else { "บทเรียนเริ่มเกม: จบแล้ว" }, c.x + 8. * u, y + 28. * u, 14. * u, hx(0xdddddd));
        if btn(ui, font, Rect::new(c.x + c.w - 124. * u, y + 9. * u, 112. * u, 32. * u), if on { "ข้าม" } else { "ทำใหม่" }, true, u) {
            if on { g.tut = TUT_DONE; } else { g.tut = 0; g.tut_ev0 = g.ev; }
            g.save();
        }
    }
    y += rh;
    if visible(y, rh, c) {
        text(font, "บันทึกอัตโนมัติ แล้วกลับหน้าเมนูหลัก", c.x + 8. * u, y + 28. * u, 13. * u, hx(0xdddddd));
        if btn(ui, font, Rect::new(c.x + c.w - 124. * u, y + 9. * u, 112. * u, 32. * u), "เมนูหลัก", true, u) { g.to_title = true; }
    }
    y += rh;
    if visible(y, rh, c) {
        let n = g.comp.as_ref().map(|c| COMP_N[c.kind as usize]).unwrap_or("ไม่มี");
        text(font, &format!("ผู้ติดตาม: {}", n), c.x + 8. * u, y + 28. * u, 14. * u, hx(0xdddddd));
        if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 9. * u, 92. * u, 32. * u), "ปล่อย", g.comp.is_some(), u) { g.dismiss_comp(); }
    }
    y += rh;
    if visible(y, rh, c) {
        text(font, "คีย์บอร์ด: Shift/K = พุ่ง · Z = ย้ายของ · P = สเตตัส · O = ตั้งค่า", c.x + 8. * u, y + 24. * u, 11. * u, hx(0x888899));
    }
    y += 30. * u;
    match flip {
        1 => { g.sfx_on = !g.sfx_on; SFX_ON.store(g.sfx_on, std::sync::atomic::Ordering::Relaxed); g.save(); }
        2 => { g.bgm_on = !g.bgm_on; g.save(); }
        3 => { g.auto_aim = !g.auto_aim; g.save(); }
        4 => { g.smart = !g.smart; g.smart_prev = None; g.smart_set = None; g.save(); }
        _ => {}
    }
    y - top
}

/// วาดเมนู คืนค่า true ถ้าต้องปิด
pub fn draw_menu(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState) -> bool {
    let u = ui_scale();
    let (sw, sh) = (screen_width(), screen_height());
    draw_rectangle(0., 0., sw, sh, Color::new(0., 0., 0., 0.72));
    let (w, h) = ((540. * u).min(sw * 0.97), (430. * u).min(sh * 0.97));
    let (x, y) = (sw / 2. - w / 2., sh / 2. - h / 2.);
    let tabs_h = 34. * u;
    let c = Rect::new(x + 8. * u, y + 10. * u + tabs_h + 4. * u, w - 16. * u, h - 10. * u - tabs_h - 4. * u - 50. * u);

    // ---- อินพุต: ลากเลื่อน / ล้อเมาส์ / คลิกตอนปล่อย
    let m: Vec2 = mouse_position().into();
    ui.click = None;
    if is_mouse_button_pressed(MouseButton::Left) {
        ui.press = Some(m);
        ui.drag = 0.;
        ui.last = m;
    }
    if is_mouse_button_down(MouseButton::Left) {
        if let Some(p0) = ui.press {
            let dy = m.y - ui.last.y;
            ui.last = m;
            ui.drag += dy.abs();
            if ui.drag > 6. * u && c.contains(p0) {
                ui.scroll -= dy;
            }
        }
    }
    if is_mouse_button_released(MouseButton::Left) && ui.press.take().is_some() && ui.drag <= 8. * u {
        ui.click = Some(m);
    }
    let (_, wy) = mouse_wheel();
    if wy != 0. {
        ui.scroll -= wy.signum() * 52. * u;
    }
    ui.scroll = ui.scroll.clamp(0., ui.max_scroll);

    draw_rectangle(x, y, w, h, hx(0x1d1f2b));
    draw_rectangle_lines(x, y, w, h, 3., hx(0xffcc66));

    // ---- แท็บ
    let mut tabs: Vec<(usize, &str)> = vec![(0, "คราฟต์"), (1, "สร้าง"), (2, "กระเป๋า")];
    if ui.chest.is_some() {
        tabs.push((3, "กล่อง"));
    }
    if ui.shop {
        tabs.push((4, "ร้านค้า"));
    }
    if ui.forge {
        tabs.push((5, "ตีบวก"));
    }
    if ui.cook {
        tabs.push((8, "ทำอาหาร"));
    }
    if ui.adv {
        tabs.push((9, "โต๊ะขั้นสูง"));
    }
    if g.p.lvl >= 2 || g.p.sp > 0 {
        tabs.push((6, "สเตตัส"));
    }
    if g.rs.inited { tabs.push((13, "เส้นทาง")); }
    if !g.npcs.is_empty() || ui.npc.is_some() {
        tabs.push((10, "ชุมชน"));
    }
    if g.p.lvl >= 3 || g.animals.iter().any(|a| a.tame) || g.comp.is_some() || !g.rs.crew.m.is_empty() { tabs.push((14, "บริวาร")); }
    if let Some((ci, _)) = ui.city { let _ = ci; tabs.push((12, "เมือง")); }
    if g.rs.inited { tabs.push((11, "อาณาจักร")); }
    tabs.push((7, "ตั้งค่า"));
    if !tabs.iter().any(|t| t.0 == ui.tab) {
        ui.tab = 0;
    }
    ui.clip = Rect::new(0., 0., sw, sh);
    let tw = (w - 16. * u) / tabs.len() as f32;
    let tfs = if tabs.len() > 7 { 10.5 } else if tabs.len() > 5 { 12. } else { 15. };
    for (i, &(id, name)) in tabs.iter().enumerate() {
        let r = Rect::new(x + 8. * u + i as f32 * tw, y + 10. * u, tw - 4. * u, tabs_h);
        let on = ui.tab == id;
        draw_rectangle(r.x, r.y, r.w, r.h, if on { hx(0xffcc66) } else { hx(0x2e3144) });
        draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., if on { BLACK } else { hx(0x555a70) });
        let label = &thai(name);
        let fs = (tfs * u) as u16;
        let d = measure_text(label, Some(font), fs, 1.);
        draw_text_ex(label, r.x + r.w / 2. - d.width / 2., r.y + r.h / 2. + 5. * u, TextParams { font: Some(font), font_size: fs, color: if on { hx(0x222222) } else { hx(0xdddddd) }, ..Default::default() });
        // v29 กดแท็บ = ออกจากหน้าคน/หน้าเลือกคนเสมอ (เดิมกลับมาแท็บชุมชนแล้วค้างอยู่ที่หน้าคนเดิม)
        if ui.click.map_or(false, |p| r.contains(p)) && (!on || ui.npc.is_some() || ui.role_pick.is_some()) {
            ui.tab = id;
            ui.scroll = 0.;
            ui.npc = None;
            ui.role_pick = None;
            ui.gift_open = false;
        }
    }

    // ---- เนื้อหา (ตัดด้วย scissor)
    draw_rectangle(c.x, c.y, c.w, c.h, hx(0x171925));
    set_clip(Some(c));
    ui.clip = c;
    let content_h = match ui.tab {
        0 => tab_craft(g, sp, font, ui, c, u),
        1 => tab_build(g, sp, font, ui, c, u),
        2 => tab_gear(g, sp, font, ui, c, u),
        3 => tab_chest(g, sp, font, ui, c, u),
        5 => tab_forge(g, sp, font, ui, c, u),
        8 => tab_cook(g, sp, font, ui, c, u),
        9 => tab_adv(g, sp, font, ui, c, u),
        6 => tab_stats(g, sp, font, ui, c, u),
        7 => tab_settings(g, sp, font, ui, c, u),
        10 => tab_town(g, sp, font, ui, c, u),
        11 => tab_realm(g, sp, font, ui, c, u),
        12 => tab_city(g, sp, font, ui, c, u),
        14 => tab_crew(g, sp, font, ui, c, u),
        13 => { let top = c.y - ui.scroll; tab_saga(g, font, c, top, u) - top }
        _ => tab_shop(g, sp, font, ui, c, u),
    };
    set_clip(None);
    ui.max_scroll = (content_h - c.h).max(0.);
    if ui.max_scroll > 0. {
        let bh = (c.h * c.h / content_h).max(20. * u);
        let by = c.y + (c.h - bh) * (ui.scroll / ui.max_scroll);
        draw_rectangle(c.x + c.w - 5. * u, by, 4. * u, bh, Color::new(1., 0.8, 0.4, 0.6));
    }
    draw_rectangle_lines(c.x, c.y, c.w, c.h, 1., hx(0x444a60));

    // ---- ปุ่มปิด
    ui.clip = Rect::new(0., 0., sw, sh);
    text(font, "เลื่อน: ล้อเมาส์ / ลากนิ้ว · Esc = ปิด", x + 10. * u, y + h - 20. * u, 11. * u, hx(0x888899));
    let br = Rect::new(x + w - 130. * u, y + h - 44. * u, 120. * u, 34. * u);
    btn(ui, font, br, "ปิด", true, u) || ui.close
}

/// v33 แท็บ "บริวาร": สัตว์เลี้ยง · ผู้ติดตามจ้าง · โกเลม/หุ่น · บอสที่ยอมมาเป็นพวก — เปลี่ยนหน้าที่/ดูแล/ปล่อยแตะเดียว
pub struct CCard {
    /// 0 พวก (โกเลม/หุ่น/บอส) · 1 สัตว์เลี้ยง · 2 ผู้ติดตามจ้าง
    pub src: u8,
    pub id: u32,
    pub title: String,
    pub role: u8,
    pub hp: (f32, f32),
    /// ความอิ่ม/อารมณ์ (None = ไม่ใช้)
    pub sat: Option<(String, f32)>,
    /// ความภักดี 0..100 (None = โกเลม/หุ่น ซื่อสัตย์ 100%)
    pub loy: Option<f32>,
    pub traits: String,
    pub info: String,
    pub wher: String,
    pub warn: Option<String>,
    pub down: bool,
    pub mind: bool,
    pub golem: bool,
    pub can_feed: bool,
    pub feed_lbl: String,
    pub can_pat: bool,
}
fn rel_text(g: &Game, x: f32, y: f32) -> String {
    let (dx, dy) = (((x - g.p.x) / 16.).round() as i32, ((y - g.p.y) / 16.).round() as i32);
    if dx.abs() + dy.abs() <= 2 { "อยู่ข้างตัวคุณ".to_string() } else { source::dir_text(dx, dy) }
}
pub fn crew_cards(g: &Game) -> Vec<CCard> {
    use crew::*;
    let mut v: Vec<CCard> = vec![];
    for (i, a) in g.animals.iter().filter(|a| a.tame).enumerate() {
        let mx = AN_STAT[a.k as usize].0;
        let tr = format!("{} · {}", if a.bold > 0.6 { "กล้าหาญ" } else if a.bold < 0.3 { "ขี้อาย" } else { "ร่าเริง" }, if a.k == A_WOLF { "ซื่อสัตย์ ชอบเนื้อ" } else { "ขี้อ้อน ชอบแครอท/ผลไม้" });
        let warn = if a.sat <= 0. || a.loy < 25. { Some("ใกล้หนีกลับป่า! ให้อาหาร/ลูบด่วน".to_string()) } else if a.sat < PET_WEAK { Some("หิวมาก — อ่อนแอ เดินช้า และไม่ยอมช่วยสู้ จนกว่าจะได้กิน".to_string()) } else { None };
        let food = Game::pet_food(a.k);
        let fl = food.iter().find(|&&f| g.p.inv[f] > 0).copied().unwrap_or(food[0]);
        v.push(CCard { src: 1, id: i as u32, title: format!("{} · สัตว์เลี้ยง", AN_NAME[a.k as usize]), role: a.role, hp: (a.hp.min(mx), mx), sat: Some(("อิ่ม".into(), a.sat)), loy: Some(a.loy), traits: tr,
            info: if a.k == A_WOLF { "กัดมอนสเตอร์ใกล้ตัว (เฝ้าจุด: กัดเฉพาะที่เข้าใกล้จุดประจำ)".into() } else { "น่ารักเดินตามไปทุกที่".into() },
            wher: rel_text(g, a.x, a.y), warn, down: false, mind: true, golem: false, can_feed: a.sat < 90., feed_lbl: format!("ให้{}", INAME[fl]), can_pat: a.pat <= 0. });
    }
    if let Some(cm) = g.comp.as_ref() {
        v.push(CCard { src: 2, id: 0, title: format!("{} · ผู้ติดตามจ้าง", COMP_N[cm.kind as usize]), role: 0, hp: (0., 0.), sat: None, loy: None, traits: "รับจ้างไว้ — ไม่ต้องกินอาหาร".into(), info: COMP_D[cm.kind as usize].to_string(), wher: rel_text(g, cm.x, cm.y), warn: None, down: false, mind: false, golem: false, can_feed: false, feed_lbl: String::new(), can_pat: false });
    }
    let day = day_of(g);
    for m in &g.rs.crew.m {
        let (info, warn);
        if m.mind() {
            let a = m.arch.min(2) as usize;
            info = if m.mode != m.wished() && m.mode != M_ROAM { format!("{} · ตอนนี้ไม่ได้ทำสิ่งที่อยาก ใจจะค่อยๆ ห่าง", AR_D[a]) } else { AR_D[a].to_string() };
            warn = if m.loyalty < 40 {
                Some(format!("ใจห่าง ({}) — {}{}", cause_text(m.cause), Game::tip_text(m.cause), if m.sad > 0 { format!(" · เหลือ {} วัน ก่อนจะ{}", (2 - m.sad).max(0), if m.arch == AR_HONOR { "ลาไป" } else { "ทรยศ" }) } else { String::new() }))
            } else { None };
        } else {
            info = m.skill_text().to_string();
            warn = if m.down { Some("ล้มอยู่ — ซ่อมตัวเองแล้วลุกขึ้นตอนเช้า (ไม่ต้องกินอาหาร)".to_string()) } else { None };
        }
        let fed = m.fed_day == day;
        let _ = fed;
        v.push(CCard { src: 0, id: m.id, title: m.name(), role: m.mode, hp: (m.hp, m.maxhp()), sat: if m.mind() { Some(("อารมณ์".into(), m.mood as f32)) } else { None }, loy: if m.mind() { Some(m.loyalty as f32) } else { None },
            traits: if m.mind() { format!("นิสัย{} · {}", AR_N[m.arch.min(2) as usize], if m.wished() == M_GUARD { "อยากเฝ้าบ้าน" } else { "อยากผจญภัยกับคุณ" }) } else { "ซื่อสัตย์ 100% · ไม่กินอาหาร · ไม่ทรยศ".into() }, info, wher: rel_text(g, m.x, m.y), warn, down: m.down, mind: m.mind(), golem: !m.mind(),
            can_feed: m.mind() && m.fed_day != day, feed_lbl: format!("ให้{}", INAME[m.fav()]), can_pat: false });
    }
    v
}
fn tab_crew(g: &mut Game, _sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    use crew::*;
    let top = c.y - ui.scroll;
    let mut y = top;
    let cards = crew_cards(g);
    let npets = cards.iter().filter(|k| k.src == 1).count();
    header(font, c, y, &format!("บริวาร / สัตว์เลี้ยง (สัตว์ {}/{} · พวก {}/{})", npets, PET_MAX, g.rs.crew.m.len(), CREW_MAX), u);
    y += 28. * u;
    let now = get_time();
    let armed = |g: &Game, kind: u8, id: u32| g.rs.crew.confirm.map_or(false, |(k, i, t)| k == kind && i == id && now - t < 4.);
    // (ชนิด, id, คำสั่ง) คำสั่ง: 0 ตามตัว · 1 เฝ้าจุด · 2 พัก/เดินเล่น · 3 ให้อาหาร/ของ · 4 ลูบ · 5 ปล่อย/รื้อ
    let mut act: Option<(u8, u32, u8)> = None;
    let gold = hx(0xffcc66);
    for k in &cards {
        let hh = 194. * u;
        if visible(y, hh, c) {
            let (lw, bw) = (c.w - 16. * u, (c.w - 16. * u - 14. * u) / 2.);
            text(font, &fit_line(font, &k.title, 14. * u, c.w - 140. * u), c.x + 8. * u, y + 18. * u, 14. * u, if k.down { hx(0x999999) } else { gold });
            let (rl, rc) = if k.src == 2 { ("ตามตัว", hx(0x9ae89a)) } else { match k.role { M_GUARD => ("เฝ้าจุดนี้", hx(0x8ac0ff)), M_ROAM => ("พัก/เดินเล่น", hx(0xb0e890)), _ => ("ตามตัว", hx(0x9ae89a)) } };
            text(font, rl, c.x + c.w - 8. * u - measure_text(rl, Some(font), (11. * u) as u16, 1.).width, y + 18. * u, 11. * u, rc);
            // แถบสถานะ: HP | อิ่ม/อารมณ์ แถวบน · ภักดี | ที่อยู่ แถวล่าง
            let (r1, r2) = (y + 30. * u, y + 47. * u);
            let lab = 38. * u;
            if k.hp.1 > 0. {
                text(font, "HP", c.x + 8. * u, r1 + 8. * u, 10.5 * u, hx(0xff8888));
                bar(c.x + 8. * u + lab * 0.6, r1, bw - lab * 0.6 - 4. * u, 8. * u, k.hp.0 / k.hp.1, if k.hp.0 / k.hp.1 > 0.5 { hx(0x6ad06a) } else if k.hp.0 / k.hp.1 > 0.25 { hx(0xe8c84a) } else { hx(0xd05a5a) });
                text(font, &format!("{}/{}", k.hp.0.round() as i32, k.hp.1 as i32), c.x + 8. * u + lab * 0.6 + 4. * u, r1 + 8. * u, 9. * u, WHITE);
            }
            let rx = c.x + 8. * u + bw + 14. * u;
            if let Some((nm, v)) = &k.sat {
                text(font, nm, rx, r1 + 8. * u, 10.5 * u, hx(0x9ad8ff));
                bar(rx + lab * 1.1, r1, bw - lab * 1.1, 8. * u, v / 100., if k.src == 1 { if *v >= 50. { hx(0x6ad06a) } else if *v >= PET_WEAK { hx(0xe8c84a) } else { hx(0xd05a5a) } } else { mood_col(*v as i32) });
            }
            match k.loy {
                Some(l) => {
                    text(font, "ภักดี", c.x + 8. * u, r2 + 8. * u, 10.5 * u, hx(0xff9ab0));
                    bar(c.x + 8. * u + lab * 0.9, r2, bw - lab * 0.9 - 4. * u, 8. * u, l / 100., if l >= 50. { hx(0xff7aa0) } else if l >= 25. { hx(0xe8c84a) } else { hx(0xd05a5a) });
                }
                None => if k.golem { text(font, "ภักดี 100%", c.x + 8. * u, r2 + 8. * u, 10.5 * u, hx(0xffb0c0)); },
            }
            text(font, &fit_line(font, &format!("ที่: {}", k.wher), 10.5 * u, bw), rx, r2 + 8. * u, 10.5 * u, hx(0xaabbcc));
            text(font, &fit_line(font, &k.traits, 10.5 * u, lw), c.x + 8. * u, y + 76. * u, 10.5 * u, hx(0xc8b8d8));
            let mut ly = y + 90. * u;
            let (txt, col) = match &k.warn { Some(w) => (w.clone(), hx(0xff8877)), None => (k.info.clone(), hx(0x9aa4b4)) };
            for l in wrap_th(font, &txt, 10.5 * u, lw).iter().take(3) { text(font, l, c.x + 8. * u, ly, 10.5 * u, col); ly += 12.5 * u; }
            // ปุ่ม: แถว 1 = หน้าที่ (อันที่ใช้อยู่เป็นสีเทา) · แถว 2 = ดูแล/ปล่อย
            let (gap, bh) = (5. * u, 30. * u);
            let w3 = (lw - gap * 2.) / 3.;
            let (b1, b2) = (y + hh - 2. * bh - gap - 6. * u, y + hh - bh - 6. * u);
            if k.src != 2 {
                let roles = [("ตามฉัน", 0u8), ("เฝ้าจุดนี้", 1), ("พัก/เดินเล่น", 2)];
                for (j, (lbl, r)) in roles.iter().enumerate() {
                    if btn(ui, font, Rect::new(c.x + 8. * u + j as f32 * (w3 + gap), b1, w3, bh), lbl, k.role != *r, u) { act = Some((k.src, k.id, *r)); }
                }
            } else {
                text(font, "ผู้ติดตามจ้างเดินตามอย่างเดียว", c.x + 8. * u, b1 + bh * 0.65, 11. * u, hx(0x889099));
            }
            let rel_lbl = if armed(g, k.src, k.id) { "แน่ใจ?" } else if k.src == 1 { "ปล่อย" } else if k.src == 2 { "ไล่ออก" } else if k.mind { "ปล่อย" } else { "รื้อ" };
            let rel_r = Rect::new(c.x + 8. * u + 2. * (w3 + gap), b2, w3, bh);
            if btn(ui, font, rel_r, rel_lbl, true, u) { act = Some((k.src, k.id, 5)); }
            if k.src == 1 {
                if btn(ui, font, Rect::new(c.x + 8. * u, b2, w3, bh), &k.feed_lbl, k.can_feed, u) { act = Some((1, k.id, 3)); }
                if btn(ui, font, Rect::new(c.x + 8. * u + w3 + gap, b2, w3, bh), "ลูบ", k.can_pat, u) { act = Some((1, k.id, 4)); }
            } else if k.src == 0 && k.mind {
                if btn(ui, font, Rect::new(c.x + 8. * u, b2, w3 * 2. + gap, bh), &k.feed_lbl, k.can_feed, u) { act = Some((0, k.id, 3)); }
            } else if k.golem {
                text(font, "ล้มแล้วซ่อมตัวเองตอนเช้า", c.x + 8. * u, b2 + bh * 0.65, 10.5 * u, hx(0x889099));
            }
            sep(c, y + hh - 2.);
        }
        y += hh;
    }
    if cards.is_empty() {
        let ws = wrap_th(font, "ยังไม่มีบริวาร — เลี้ยงหมาป่า/กระต่าย (ป้อนเนื้อ/แครอทแล้วกด ใช้) · จ้างผู้ติดตามที่เมนูตั้งค่า · คราฟต์โกเลมหิน/เหล็ก/คริสตัลที่โต๊ะขั้นสูง (ต้องใช้แกนโกเลม) · หรือสู้ราชาสไลม์/โกเลมจนยอมแพ้แล้วชวนเป็นพวก (ถ้าเขาเต็มใจ)", 11. * u, c.w - 24. * u);
        let hh = ws.len() as f32 * 14. * u + 12. * u;
        if visible(y, hh, c) { for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, y + 16. * u + k as f32 * 14. * u, 11. * u, hx(0x9999aa)); } }
        y += hh;
        // แกนโกเลม/แร่เหล็กหาได้จากไหน (v32)
        y += source::draw_lack(g, _sp, font, c, y, u, &[(CORE, 1), (IRON, 1)]);
    }
    if let Some((src, id, a)) = act {
        if a == 5 && !armed(g, src, id) {
            g.rs.crew.confirm = Some((src, id, now));
        } else {
            g.rs.crew.confirm = None;
            match (src, a) {
                (0, 0) => g.crew_set_mode(id, M_FOLLOW),
                (0, 1) => g.crew_set_mode(id, M_GUARD),
                (0, 2) => g.crew_set_mode(id, M_ROAM),
                (0, 3) => g.crew_gift_member(id),
                (0, _) => g.crew_release(id),
                (1, 0) => g.pet_set_role(id as usize, PET_FOLLOW),
                (1, 1) => g.pet_set_role(id as usize, PET_GUARD),
                (1, 2) => g.pet_set_role(id as usize, PET_ROAM),
                (1, 3) => g.pet_feed(id as usize),
                (1, 4) => g.pet_pat(id as usize),
                (1, _) => g.pet_release(id as usize),
                _ => g.dismiss_comp(),
            }
        }
    }
    y - top
}
fn fit_line(font: &Font, t: &str, size: f32, maxw: f32) -> String { source::fit_text(font, t, size, maxw) }

// ======================= แท็บ "ชุมชน": ภาพรวมเมือง + บทสนทนา NPC =======================
fn bar(x: f32, y: f32, w: f32, h: f32, v: f32, col: Color) {
    draw_rectangle(x, y, w, h, hx(0x2a2d3f));
    draw_rectangle(x, y, w * v.clamp(0., 1.), h, col);
}
fn mood_col(m: i32) -> Color { if m >= 60 { hx(0x66dd66) } else if m >= 30 { hx(0xeedd44) } else { hx(0xee5544) } }

/// ตัดบรรทัดตามความกว้างจริง (ภาษาไทยไม่มีเว้นวรรคระหว่างคำ จึงตัดทีละตัวอักษร แต่ไม่ตัดก่อนสระ/วรรณยุกต์)
pub fn wrap_th(font: &Font, s: &str, fs: f32, maxw: f32) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    let mut cur = String::new();
    for ch in s.chars() {
        let mark = ('\u{0E31}'..='\u{0E3A}').contains(&ch) || ('\u{0E47}'..='\u{0E4E}').contains(&ch) || ch == '\u{0E33}';
        let mut t = cur.clone();
        t.push(ch);
        if !mark && !cur.is_empty() && measure_text(&thai(&t), Some(font), fs as u16, 1.).width > maxw {
            out.push(std::mem::take(&mut cur));
        }
        cur.push(ch);
    }
    if !cur.is_empty() { out.push(cur); }
    out
}

fn tab_town(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    if let Some(id) = ui.npc {
        return tab_npc_dialog(g, sp, font, ui, c, u, id);
    }
    if let Some(r) = ui.role_pick {
        return tab_role_pick(g, font, ui, c, u, r);
    }
    if ui.arch { return tab_arch(g, font, ui, c, u); }
    let tier = g.tier();
    // ===== v21 ภาพรวมเมือง =====
    let st = g.town_stats();
    let title = if g.rs.founded { g.my_name() } else { TIER_N[tier as usize].to_string() };
    header(font, c, y, &format!("ภาพรวม: {} · ชาวบ้าน {} คน", title, st.pop), u);
    y += 28. * u;
    // การ์ดสถิติ (สีบอกสถานะ: เขียว ดี · เหลือง ระวัง · แดง มีปัญหา)
    let good = hx(0x66dd66);
    let warn = hx(0xeedd44);
    let bad = hx(0xee5544);
    let soldiers = g.npcs.iter().filter(|n| n.state == ST_HOME && matches!(n.role, 2 | R_SOLDIER | R_GENERAL)).count();
    let housed = st.pop - st.outdoor.len() - st.no_bed.len();
    let cards: Vec<(&str, String, String, Color)> = vec![
        ("ประชากร", format!("{}", st.pop), format!("เตียงว่าง {}", st.free_beds), if st.no_bed.is_empty() { if st.free_beds > 0 { good } else { warn } } else { bad }),
        ("อาหาร", format!("{}", st.food), if st.pop > 0 { format!("พอราว {:.0} วัน", st.food_days.min(99.)) } else { "-".into() }, if !st.hungry.is_empty() || (st.pop > 0 && st.food_days < 1.) { bad } else if st.food_days < 3. { warn } else { good }),
        ("น้ำดื่ม", format!("{}/{}", st.water_ok, st.pop), "คนมีน้ำใช้".into(), if st.no_water.is_empty() { good } else { bad }),
        ("ความสุข", format!("{}%", st.mood_avg), format!("ไม่พอใจ {} คน", st.unhappy.len()), mood_col(st.mood_avg)),
        ("บ้าน", format!("{}/{}", housed, st.pop), "มีหลังคา".into(), if st.outdoor.is_empty() && st.no_bed.is_empty() { good } else { warn }),
        ("ไร่", format!("{} แปลง", st.plots), format!("สุกรอเก็บ {}", st.ripe), if st.plots > 0 { good } else { hx(0x9999aa) }),
        ("ป้องกัน", format!("{:.0}", g.defense()), format!("ทหาร/ยาม {}", soldiers), if soldiers > 0 { good } else { hx(0x9999aa) }),
        ("ภาษี", format!("{}", g.tax_pool), format!("ชื่อเสียง {}", g.fame()), hx(0xffee99)),
    ];
    let cols = if c.w > 400. * u { 4 } else { 2 };
    let gap = 6. * u;
    let cw = (c.w - gap * (cols as f32 + 1.)) / cols as f32;
    let ch = 56. * u;
    for (k, (lab, val, sub, col)) in cards.iter().enumerate() {
        let (cx, cy) = (c.x + gap + (k % cols) as f32 * (cw + gap), y + (k / cols) as f32 * (ch + gap));
        if visible(cy, ch, c) {
            draw_rectangle(cx, cy, cw, ch, Color::new(0.1, 0.11, 0.17, 0.95));
            draw_rectangle(cx, cy, 4. * u, ch, *col);
            text(font, lab, cx + 10. * u, cy + 15. * u, 11. * u, hx(0xaaaabb));
            text(font, val, cx + 10. * u, cy + 36. * u, 18. * u, *col);
            text(font, sub, cx + 10. * u, cy + 50. * u, 10. * u, hx(0x9999aa));
        }
    }
    y += ((cards.len() + cols - 1) / cols) as f32 * (ch + gap) + 4. * u;
    // ควรทำตอนนี้ (ที่ปรึกษา)
    let adv = g.advice();
    header(font, c, y, if adv.is_empty() { "ควรทำตอนนี้: ไม่มีปัญหา เมืองไปได้ดี!" } else { "ควรทำตอนนี้" }, u);
    y += 28. * u;
    let mut act: Option<AdvAct> = None;
    for a in adv.iter().take(8) {
        let tw = c.w - if a.act != AdvAct::None { 130. * u } else { 24. * u } - 44. * u;
        let ws = wrap_th(font, &a.text, 11. * u, tw);
        let hh = (ws.len() as f32 * 14. * u + 14. * u).max(42. * u);
        if visible(y, hh, c) {
            let pc = hx(PRIO_C[a.prio as usize]);
            draw_rectangle(c.x + 6. * u, y + 4. * u, 4. * u, hh - 8. * u, pc);
            text(font, PRIO_N[a.prio as usize], c.x + 14. * u, y + 17. * u, 10. * u, pc);
            for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 56. * u, y + 17. * u + k as f32 * 14. * u, 11. * u, hx(0xeeeeee)); }
            if a.act != AdvAct::None && btn(ui, font, Rect::new(c.x + c.w - 116. * u, y + (hh - 32. * u) / 2., 104. * u, 32. * u), a.btn, true, u) { act = Some(a.act); }
            sep(c, y + hh - 2.);
        }
        y += hh;
    }
    if let Some(a) = act {
        match a {
            AdvAct::Tab(t) => { ui.open(t as usize); return y; }
            AdvAct::Npc(id) => { ui.npc = Some(id); ui.scroll = 0.; return y; }
            AdvAct::Role(r) => { ui.role_pick = Some(r); ui.scroll = 0.; return y; }
            AdvAct::Build(_) => { g.adv_do(a); ui.close = true; }
            _ => g.adv_do(a),
        }
    }
    // ===== v36 ผังเมือง & ช่างสถาปนิก =====
    if g.pop() >= 2 || g.rs.ar.active.is_some() {
        header(font, c, y, "ผังเมือง & ช่างสถาปนิก", u);
        y += 28. * u;
        let line = match (&g.rs.ar.active, &g.rs.ar.prop) {
            (Some(p), _) => { let (d, n) = p.progress(); format!("ช่างกำลังวางผังเมืองตามแบบ \"{}\" — ทำแล้ว {}/{}{}", arch::VARIANT_NAMES[p.variant as usize & 1], d, n, if p.shortfall().is_empty() { "" } else { " · รอวัสดุ" }) }
            (None, Some(p)) => format!("สถาปนิกเสนอแบบ \"{}\" แล้ว (ย้ายบ้าน {} หลัง) — รอคุณตัดสินใจ", arch::VARIANT_NAMES[p.variant as usize & 1], p.count_houses()),
            _ => "จ้างช่างมาจัดบ้าน ถนน ลานกลางเมือง บ่อน้ำ ให้เป็นระเบียบ (ค่าจ้างจ่ายเมื่อรับแบบ) · จะสร้าง/ย้ายเองต่อไปก็ได้เหมือนเดิม".to_string(),
        };
        para(font, c, &mut y, &line, hx(0xc9d1e6), u);
        if visible(y, 32. * u, c) && btn(ui, font, Rect::new(c.x + 8. * u, y, 200. * u, 28. * u), "เปิดหน้าสถาปนิก", true, u) { ui.arch = true; ui.scroll = 0.; return y - top; }
        y += 34. * u;
    }
    // ===== v29 ผู้คน (ขึ้นมาไว้ใกล้ด้านบน: ใช้บ่อยสุด) · คุยได้ทันทีจากรายชื่อ ไม่ต้องเข้าหน้าคน =====
    header(font, c, y, &format!("ผู้คน ({} คน)", g.npcs.len()), u);
    y += 28. * u;
    let served = g.water_served();
    let rows: Vec<(u32, String, u8, i32, u8, bool, bool, u8, String, bool, String)> = g.npcs.iter().map(|n| {
        let iss = g.npc_issues(n, &served);
        let doing = if g.rs.sleeping.contains(&n.id) { "นอน".to_string() }
            else if let Some(j) = g.rs.work.get(&n.id) { if j.stage == 1 { "ขนของเข้ากล่อง".to_string() } else if j.working { J_NAMES[j.kind as usize].to_string() } else { format!("ไป{}", J_NAMES[j.kind as usize]) } }
            else if n.child { "เล่น".to_string() } else if work_hr(g.hour()) { "ว่าง".to_string() } else { "พักผ่อน".to_string() };
        (n.id, NPC_NAMES[n.name as usize].to_string(), n.state, n.mood, n.hearts, n.married, n.child, n.role, iss.join(" · "), n.talked, doing)
    }).collect();
    let mut open: Option<u32> = None;
    let mut talk: Option<u32> = None;
    let rh2 = 54. * u;
    for (id, nm, st, mood, hearts, married, child, role, iss, talked, doing) in &rows {
        let hh = if iss.is_empty() { rh2 } else { rh2 + 12. * u };
        if visible(y, hh, c) {
            let status = match *st { ST_FOLLOW => "ติดตามคุณ".to_string(), _ => if *child { "ลูก".to_string() } else { ROLE_N[*role as usize].to_string() } };
            text(font, &format!("{}{} · {}", nm, if *married { " (คู่สมรส)" } else { "" }, status), c.x + 8. * u, y + 18. * u, 14. * u, hx(0xffcc66));
            text(font, &format!("สนิท {}/10 · กำลัง{}", hearts, doing), c.x + 8. * u, y + 36. * u, 11. * u, hx(0xff9ab0));
            bar(c.x + 8. * u, y + 42. * u, 120. * u, 5. * u, *mood as f32 / 100., mood_col(*mood));
            if !iss.is_empty() { text(font, &format!("ปัญหา: {}", iss), c.x + 8. * u, y + 60. * u, 10. * u, hx(0xff8877)); }
            let bw = 84. * u;
            if btn(ui, font, Rect::new(c.x + c.w - bw * 2. - 16. * u, y + 10. * u, bw, 32. * u), if *talked { "คุยแล้ว" } else { "คุย" }, !*talked, u) { talk = Some(*id); }
            if btn(ui, font, Rect::new(c.x + c.w - bw - 8. * u, y + 10. * u, bw, 32. * u), "จัดการ", true, u) { open = Some(*id); }
            sep(c, y + hh - 2.);
        }
        y += hh;
    }
    if rows.is_empty() && visible(y, 30. * u, c) {
        text(font, "ยังไม่มีใคร — ออกสำรวจแล้วช่วยผู้หลงทาง!", c.x + 8. * u, y + 20. * u, 13. * u, hx(0x888899));
        y += 30. * u;
    }
    if let Some(id) = talk { g.npc_talk(id); }
    if let Some(id) = open { ui.npc = Some(id); ui.scroll = 0.; return y - top; }
    // งานตอนนี้: อาชีพละกี่คน กำลังทำอะไร
    if !st.roles.is_empty() {
        header(font, c, y, "อาชีพ & งานตอนนี้", u);
        y += 28. * u;
        for &(r, nr) in &st.roles {
            let doing: Vec<String> = g.npcs.iter().filter(|n| n.state == ST_HOME && !n.child && n.role == r).map(|n| {
                let what = if g.rs.sleeping.contains(&n.id) { "นอน".to_string() }
                    else if let Some(j) = g.rs.work.get(&n.id) { if j.stage == 1 { "ขนของเข้ากล่อง".to_string() } else if j.working { J_NAMES[j.kind as usize].to_string() } else { format!("ไป{}", J_NAMES[j.kind as usize]) } }
                    else if work_hr(g.hour()) { "ว่าง".to_string() } else { "พักผ่อน".to_string() };
                format!("{} ({})", NPC_NAMES[n.name as usize], what)
            }).collect();
            let line = format!("{} {} คน: {}", ROLE_N[r as usize], nr, doing.join(" · "));
            let ws = wrap_th(font, &line, 11. * u, c.w - 24. * u);
            let hh = ws.len() as f32 * 14. * u + 8. * u;
            if visible(y, hh, c) { for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, y + 14. * u + k as f32 * 14. * u, 11. * u, if k == 0 { hx(0xffcc66) } else { hx(0xcccccc) }); } }
            y += hh;
        }
    }
    // โครงการของชาวบ้าน
    if !g.rs.projects.is_empty() || !g.sites.is_empty() {
        header(font, c, y, "โครงการ & ก่อสร้าง", u);
        y += 28. * u;
        let mut lines: Vec<(String, Color)> = vec![];
        for p in &g.rs.projects {
            let nm = g.idx(p.by).map(|i| NPC_NAMES[g.npcs[i].name as usize]).unwrap_or("");
            let status = if p.started { "กำลังสร้าง".to_string() } else if !p.ok { "รอคุณอนุญาตที่ดิน".to_string() } else if let Some(h) = g.rs.help.get(&p.by) { h.splitn(2, "ยังขาด ").nth(1).map(|x| format!("ขาด {}", x.split(" —").next().unwrap_or(x))).unwrap_or_else(|| "รอวัสดุ".into()) } else { "รอวัสดุ".into() };
            lines.push((format!("{}: {} — {}", nm, project_what(p.kind), status), if p.started { good } else { warn }));
        }
        for s2 in &g.sites { let total = prefab_time(s2.kind); lines.push((format!("ไซต์ {} — {:.0}%", prefab_name(s2.kind), (s2.prog / total * 100.).min(100.)), hx(0x99ccff))); }
        for (l, col) in lines {
            let ws = wrap_th(font, &l, 11. * u, c.w - 24. * u);
            let hh = ws.len() as f32 * 14. * u + 6. * u;
            if visible(y, hh, c) { for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, y + 14. * u + k as f32 * 14. * u, 11. * u, col); } }
            y += hh;
        }
    }
    // ระดับเมือง/เป้าหมายถัดไป
    let goal = match tier {
        0 => "ช่วยผู้หลงทางในโลก (เครื่องหมาย ! สีเหลือง) แล้วชวนมาอยู่ที่ฐาน — ต้องมีเตียงว่าง".to_string(),
        1 => "ค่าย (1-3 คน) · ครบ 4 คนเป็นหมู่บ้าน".to_string(),
        2 => format!("หมู่บ้าน · เป็นเมืองต้อง >10 คน + ปราสาท({}) + กำแพงเมือง {}/12", if g.has_castle() { "มีแล้ว" } else { "ยังไม่มี" }, g.walls()),
        _ => if g.rs.founded { "อาณาจักรของคุณ — ดูการทูต/กฎหมายที่แท็บอาณาจักร".to_string() } else { "เมือง: เก็บภาษีได้ทุกเช้า · สถาปนาอาณาจักรได้ที่แท็บอาณาจักร".to_string() },
    };
    let ws = wrap_th(font, &format!("เป้าหมายถัดไป: {}", goal), 11. * u, c.w - 24. * u);
    let hh = ws.len() as f32 * 14. * u + 10. * u;
    if visible(y, hh, c) { for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, y + 16. * u + k as f32 * 14. * u, 11. * u, hx(0xbbbbcc)); } }
    y += hh;
    let mut tax = false;
    if tier == 3 {
        if visible(y, 40. * u, c) {
            text(font, &format!("ภาษีสะสม: {} เหรียญ", g.tax_pool), c.x + 8. * u, y + 24. * u, 14. * u, hx(0xffee99));
            if btn(ui, font, Rect::new(c.x + c.w - 124. * u, y + 6. * u, 112. * u, 32. * u), "เก็บภาษี", g.tax_pool > 0, u) { tax = true; }
        }
        y += 44. * u;
    }
    if !g.nreqs.is_empty() {
        header(font, c, y, "คำขอจากชาวบ้าน (คุยกับคนที่มีกรอบคำพูดสีเหลือง)", u);
        y += 28. * u;
        let day = day_of(g);
        let txts: Vec<String> = g.nreqs.iter().map(|r| format!("{} · เหลือ {} วัน", g.req_text(r), (REQ_DAYS - (day - r.day)).max(0) + 1)).collect();
        for t in txts {
            let ws = wrap_th(font, &t, 11. * u, c.w - 24. * u);
            let hh = ws.len() as f32 * 14. * u + 6. * u;
            if visible(y, hh, c) { for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, y + 13. * u + k as f32 * 14. * u, 11. * u, hx(0xfff0b0)); } }
            y += hh;
        }
    }
    if !g.news.is_empty() {
        header(font, c, y, "ข่าวหมู่บ้าน", u);
        y += 28. * u;
        // v29 ข่าวเรื่องเดิมซ้ำหลายวัน รวมเป็นบรรทัดเดียว ("วันที่ 14–16: …")
        let mut merged: Vec<(i32, i32, String)> = vec![];
        for (d, t) in g.news.iter().rev() {
            if let Some(last) = merged.iter_mut().find(|m| &m.2 == t) { last.0 = last.0.min(*d); continue; }
            merged.push((*d, *d, t.clone()));
            if merged.len() >= 6 { break; }
        }
        for (d0, d1, t) in merged.iter() {
            let when = if d0 == d1 { format!("วันที่ {}", d1 + 1) } else { format!("วันที่ {}–{}", d0 + 1, d1 + 1) };
            let ws = wrap_th(font, &format!("{}: {}", when, t), 11. * u, c.w - 24. * u);
            let hh = ws.len() as f32 * 14. * u + 6. * u;
            if visible(y, hh, c) { for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, y + 13. * u + k as f32 * 14. * u, 11. * u, hx(0xc9d1e6)); } }
            y += hh;
        }
    }
    if tax { g.collect_tax(); }
    y - top
}

/// v36 หน้าช่างสถาปนิก: เลือกหมวด → จ้าง/เสนอแบบ → รับ/ไม่รับ → ใส่วัสดุที่ขาด
fn tab_arch(g: &mut Game, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    header(font, c, y, "ช่างสถาปนิก — วางผังเมือง", u);
    y += 28. * u;
    if visible(y, 30. * u, c) && btn(ui, font, Rect::new(c.x + 8. * u, y, 110. * u, 26. * u), "ย้อนกลับ", true, u) { ui.arch = false; ui.scroll = 0.; return 0.; }
    y += 32. * u;
    let item = |it: usize| INAME[it].to_string();
    let list = |v: &[(usize, i32)]| v.iter().map(|&(i, q)| format!("{} {}", INAME[i], q)).collect::<Vec<_>>().join(" · ");
    let _ = item;
    if let Some(p) = g.rs.ar.active.clone() {
        // ---- ช่างกำลังทำงาน
        let (d, n) = p.progress();
        para(font, c, &mut y, &format!("แบบ \"{}\" — ทำแล้ว {}/{} อย่าง (ย้ายบ้าน {} หลัง) · ช่างลงมือทีละอย่างตลอดเวลา แม้เราไม่อยู่ในเมือง", arch::VARIANT_NAMES[p.variant as usize & 1], d, n, p.count_houses()), hx(0xc9d1e6), u);
        bar(c.x + 8. * u, y, c.w - 24. * u, 8. * u, d as f32 / n.max(1) as f32, hx(0x66dd88));
        y += 16. * u;
        let sf = p.shortfall();
        if sf.is_empty() { para(font, c, &mut y, "วัสดุครบแล้ว", hx(0x88ee99), u); }
        else {
            para(font, c, &mut y, &format!("ช่างยังขาดวัสดุ: {} — ไปหามาแล้วกดใส่ ช่างทำส่วนอื่นไปก่อนได้ ส่วนที่ต้องใช้ของจะรอ", list(&sf)), hx(0xeedd77), u);
            let have: Vec<(usize, i32)> = sf.iter().map(|&(i, q)| (i, q.min(g.p.inv[i]))).filter(|x| x.1 > 0).collect();
            if !have.is_empty() { para(font, c, &mut y, &format!("ในกระเป๋าใส่ได้ตอนนี้: {}", list(&have)), hx(0x88ccff), u); }
        }
        if visible(y, 32. * u, c) {
            match btn_row(ui, font, c, y, &[("ใส่วัสดุจากกระเป๋า", !sf.is_empty()), (if g.rs.ar.show { "ซ่อนแผนที่" } else { "ดูแผนที่ในเกม" }, true), ("ยกเลิกแผน", true)], u) {
                Some(0) => { g.arch_deposit(); }
                Some(1) => { g.rs.ar.show = !g.rs.ar.show; if g.rs.ar.show { ui.close = true; } }
                Some(2) => g.arch_cancel(),
                _ => {}
            }
        }
        y += 34. * u;
        if let Some(note) = p.notes.first() { para(font, c, &mut y, note, hx(0xaa9988), u); }
        return y - top;
    }
    // ---- เลือกหมวด
    let opts = g.arch_opts();
    para(font, c, &mut y, "เลือกว่าจะให้ช่างจัดอะไรบ้าง แล้วขอให้เสนอแบบ — ดูแผนที่สีบนพื้นเกมก่อน ถ้าไม่ชอบก็ไม่เสียเงิน", hx(0xc9d1e6), u);
    for (i, bit) in [arch::OPT_NPC, arch::OPT_MINE, arch::OPT_WALLS, arch::OPT_PLAZA, arch::OPT_ROAD].iter().enumerate() {
        if visible(y, 30. * u, c) {
            let on = opts & bit != 0;
            if btn(ui, font, Rect::new(c.x + 8. * u, y, 70. * u, 26. * u), if on { "เปิด" } else { "ปิด" }, true, u) { g.arch_toggle(*bit); g.rs.ar.prop = None; }
            text(font, arch::OPT_NAMES[i], c.x + 88. * u, y + 18. * u, 12. * u, if on { hx(0xeeeeee) } else { hx(0x888899) });
        }
        y += 30. * u;
    }
    if let Some(p) = g.rs.ar.prop.clone() {
        // ---- ข้อเสนอ
        header(font, c, y, &format!("ข้อเสนอ: {}", arch::VARIANT_NAMES[p.variant as usize & 1]), u);
        y += 28. * u;
        let (nat, wall) = p.count_clear();
        let places = p.steps.iter().filter(|s| matches!(s, arch::Step::Place { .. })).count();
        let moves1 = p.steps.iter().filter(|s| matches!(s, arch::Step::MoveOne { .. })).count();
        let roads = p.steps.iter().filter(|s| matches!(s, arch::Step::Pave { .. })).count();
        para(font, c, &mut y, &format!("ย้ายและหมุนบ้าน {} หลัง ให้เรียงสองฝั่งถนน ประตูหันเข้าถนน · เคลียร์ต้นไม้/หิน {} จุด · รื้อกำแพงสะเปะสะปะ {} ชิ้น · ย้ายของส่วนกลาง {} ชิ้น · วางของใหม่ {} ชิ้น · ปูถนน {} สาย", p.count_houses(), nat, wall, moves1, places, roads), hx(0xdde4f5), u);
        for n in &p.notes { para(font, c, &mut y, n, hx(0xee9988), u); }
        let fee = g.arch_fee(&p);
        let need = p.need();
        let need_l: Vec<(usize, i32)> = (0..NI).filter(|&i| need[i] > 0).map(|i| (i, need[i])).collect();
        para(font, c, &mut y, &format!("ค่าจ้าง {} เหรียญ (มี {}) · วัสดุที่ใช้: {}{}", fee, g.p.inv[COIN] + g.tax_pool, if need_l.is_empty() { "ไม่ต้องใช้".to_string() } else { list(&need_l) }, if p.shortfall().is_empty() { "".to_string() } else { " — รับแล้วค่อยไปหามาเพิ่มได้".to_string() }), hx(0xffee99), u);
        para(font, c, &mut y, "บนพื้นเกม: เหลือง = บ้านที่จะย้าย · เขียว = ที่ใหม่ · แดง = เคลียร์/รื้อ · ฟ้า = ลานกลาง/บ่อ/กองไฟ · น้ำตาล = ถนน", hx(0x9aa2b8), u);
        if visible(y, 32. * u, c) {
            match btn_row(ui, font, c, y, &[("รับแบบนี้", g.p.inv[COIN] + g.tax_pool >= fee), ("ไม่เอา", true), ("ดูแผนที่ในเกม", true)], u) {
                Some(0) => { if g.arch_accept() { return 0.; } }
                Some(1) => g.arch_reject(),
                Some(2) => { g.rs.ar.show = true; ui.close = true; }
                _ => {}
            }
        }
        y += 34. * u;
        para(font, c, &mut y, "อยากได้แบบอื่น: กดเสนอแบบด้านล่างอีกครั้ง", hx(0x888899), u);
    }
    if visible(y, 32. * u, c) {
        match btn_row(ui, font, c, y, &[("เสนอแบบ ถนนตะวันออก-ตะวันตก", true), ("เสนอแบบ ถนนเหนือ-ใต้", true)], u) {
            Some(i) => { g.arch_hire(i as u8); }
            None => {}
        }
    }
    y += 34. * u;
    y - top
}

/// v29 หน้าเลือกคนให้ทำอาชีพ (จากปุ่ม "เลือกคน" ของที่ปรึกษา)
fn tab_role_pick(g: &mut Game, font: &Font, ui: &mut UiState, c: Rect, u: f32, r: u8) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    if btn(ui, font, Rect::new(c.x + 6. * u, y + 4. * u, 90. * u, 30. * u), "< กลับ", true, u) { ui.role_pick = None; ui.scroll = 0.; return 0.; }
    text(font, &format!("เลือกคนเป็น{}", ROLE_N[r as usize]), c.x + 108. * u, y + 25. * u, 16. * u, hx(0xffcc66));
    y += 40. * u;
    let ws = wrap_th(font, &format!("งาน: {}", ROLE_DESC[(r as usize).min(13)]), 11. * u, c.w - 24. * u);
    for (k, w) in ws.iter().enumerate() { if visible(y, 14. * u, c) { text(font, w, c.x + 8. * u, y + 13. * u + k as f32 * 14. * u, 11. * u, hx(0x9fe0ff)); } }
    y += ws.len() as f32 * 14. * u + 8. * u;
    let cands = g.role_candidates(r);
    let mut pick: Option<u32> = None;
    let rh = 50. * u;
    for (id, ok) in &cands {
        let Some(i) = g.idx(*id) else { continue };
        let n = &g.npcs[i];
        if visible(y, rh, c) {
            let cur = if n.role == r { " (ทำอยู่แล้ว)".to_string() } else { format!(" · ตอนนี้: {}", ROLE_N[n.role as usize]) };
            text(font, &format!("{}{}", NPC_NAMES[n.name as usize], cur), c.x + 8. * u, y + 20. * u, 14. * u, if ok.is_ok() { hx(0xffcc66) } else { hx(0x888899) });
            let sub = match ok { Ok(()) => n.soc.traits_ranked().into_iter().take(2).map(|(k, hi)| Soc::trait_name(k, hi)).collect::<Vec<_>>().join(" · "), Err(e) => format!("ทำไม่ได้: {}", e) };
            text(font, &sub, c.x + 8. * u, y + 38. * u, 11. * u, if ok.is_ok() { hx(0xaaaaaa) } else { hx(0xff8877) });
            if btn(ui, font, Rect::new(c.x + c.w - 112. * u, y + 9. * u, 100. * u, 32. * u), "เลือก", ok.is_ok() && n.role != r, u) { pick = Some(*id); }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    if cands.is_empty() && visible(y, 30. * u, c) { text(font, "ยังไม่มีผู้ใหญ่ในเมือง", c.x + 8. * u, y + 20. * u, 13. * u, hx(0x888899)); y += 30. * u; }
    if let Some(id) = pick {
        match g.set_role(id, r) { Ok(m) => { g.toast(m); ui.role_pick = None; ui.scroll = 0.; } Err(e) => g.toast(e) }
    }
    y - top
}

fn tab_npc_dialog(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32, id: u32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    let Some(n) = g.npcs.iter().find(|n| n.id == id).cloned() else { ui.npc = None; return 0.; };
    let nm = NPC_NAMES[n.name as usize];
    // v29 ปุ่มกลับอยู่บนสุด (เดิมอยู่ล่างสุดของหน้ายาว เลยเหมือนติดค้างอยู่กับคนนี้)
    if btn(ui, font, Rect::new(c.x + 6. * u, y + 4. * u, 90. * u, 30. * u), "< กลับ", true, u) { ui.npc = None; ui.scroll = 0.; ui.gift_open = false; return 0.; }
    text(font, &format!("{} — {}{}", nm, if n.state == ST_FOLLOW { "ผู้ติดตาม" } else if n.child { "ลูก" } else { ROLE_N[n.role as usize] }, if n.married { " · คู่สมรส" } else { "" }), c.x + 108. * u, y + 25. * u, 16. * u, hx(0xffcc66));
    y += 40. * u;
    let say = g.npc_say(&n);
    if visible(y, 30. * u, c) { text(font, &format!("\"{}\"", say), c.x + 8. * u, y + 18. * u, 13. * u, hx(0xdddddd)); }
    y += 30. * u;
    // v31 อยากได้อะไร: มอบ/ไม่ให้ ได้ทันที (ของอื่นๆ ยังให้ได้ที่ "ให้ของขวัญ" ด้านล่าง)
    let ws = g.wants(id);
    if !ws.is_empty() {
        header(font, c, y, "อยากได้", u);
        y += 28. * u;
        let mut hit: Option<(WantK, bool)> = None;
        for w in &ws {
            let lines = wrap_th(font, &w.text, 12. * u, c.w - 24. * u - 2. * 104. * u);
            let hh = (lines.len() as f32 * 15. * u + 10. * u).max(40. * u);
            if visible(y, hh, c) {
                for (k, l) in lines.iter().enumerate() { text(font, l, c.x + 8. * u, y + 18. * u + k as f32 * 15. * u, 12. * u, hx(0xffe9a0)); }
                if btn(ui, font, Rect::new(c.x + c.w - 212. * u, y + 4. * u, 100. * u, 32. * u), &w.give, w.can, u) { hit = Some((w.k.clone(), true)); }
                if btn(ui, font, Rect::new(c.x + c.w - 106. * u, y + 4. * u, 98. * u, 32. * u), "ไม่ให้", true, u) { hit = Some((w.k.clone(), false)); }
            }
            y += hh;
            sep(c, y - 2.);
            // v32 ขาดของ → บอกว่าหาได้จากไหน (รูป + ทิศ/ระยะ)
            let lack = g.want_lack(&w.k);
            if !lack.is_empty() { y += source::draw_lack(g, sp, font, c, y, u, &lack); }
        }
        if let Some((k, give)) = hit { if give { g.want_give(id, &k); } else { g.want_no(id, &k); } }
        y += 6. * u;
    }
    // v30 คำขอที่ดิน: ตอบได้จากตรงนี้ด้วย
    if let Some(a) = g.ask_of(id).cloned() {
        let s = format!("ขออนุญาต: {} — {}", g.ask_what(&a), g.ask_where(&a));
        let ws = wrap_th(font, &s, 12. * u, c.w - 24. * u);
        for (k, w) in ws.iter().enumerate() { if visible(y, 15. * u, c) { text(font, w, c.x + 8. * u, y + 14. * u + k as f32 * 15. * u, 12. * u, hx(0xffd84a)); } }
        y += ws.len() as f32 * 15. * u + 6. * u;
        if visible(y, 30. * u, c) {
            match btn_row(ui, font, c, y, &[("ให้เลย", true), ("เลือกที่ให้เอง", true), ("ไม่ให้", true)], u) {
                Some(0) => g.permit_yes(id),
                Some(1) => { g.permit_pick_start(id); ui.close = true; }
                Some(2) => g.permit_no(id),
                _ => {}
            }
        }
        y += 36. * u;
    }
    let op = opinion(&n);
    if visible(y, 40. * u, c) {
        text(font, &format!("สนิท {}/10", n.hearts), c.x + 8. * u, y + 14. * u, 12. * u, hx(0xff9ab0));
        bar(c.x + 8. * u, y + 20. * u, 160. * u, 9. * u, n.hearts as f32 / 10., hx(0xff6688));
        text(font, &format!("ความสุข {}", n.mood), c.x + 190. * u, y + 14. * u, 12. * u, hx(0xaaaaaa));
        bar(c.x + 190. * u, y + 20. * u, 160. * u, 9. * u, n.mood as f32 / 100., mood_col(n.mood));
    }
    y += 40. * u;
    // v29 อาชีพ: กดเลือกจากปุ่มได้เลย (เดิมต้องกด "เปลี่ยนอาชีพ" วนทีละอาชีพ) · ทำไม่ได้ = ปุ่มจาง กดแล้วบอกเหตุผล
    if n.state == ST_HOME && !n.child {
        let manual = g.rs.manual.contains_key(&n.id);
        let desc = format!("อาชีพ: {}{} — {}", ROLE_N[n.role as usize], if manual { " (เลือกเอง)" } else { " (เกมจัดให้)" }, ROLE_DESC[(n.role as usize).min(13)]);
        let ws = wrap_th(font, &desc, 11. * u, c.w - 24. * u);
        for (k, w) in ws.iter().enumerate() { if visible(y, 14. * u, c) { text(font, w, c.x + 8. * u, y + 13. * u + k as f32 * 14. * u, 11. * u, hx(0x9fe0ff)); } }
        y += ws.len() as f32 * 14. * u + 6. * u;
        let cols = if c.w > 520. * u { 5 } else { 3 };
        let gap = 6. * u;
        let bw = (c.w - 16. * u - gap * (cols as f32 - 1.)) / cols as f32;
        let bh = 30. * u;
        let mut chosen: Option<u8> = None;
        let mut list: Vec<u8> = PICK_ROLES.to_vec();
        if n.role == 3 { list.insert(0, 3); }
        for (k, &r) in list.iter().enumerate() {
            let (bx, by) = (c.x + 8. * u + (k % cols) as f32 * (bw + gap), y + (k / cols) as f32 * (bh + gap));
            if !visible(by, bh, c) { continue; }
            let ok = role_ok(&n, r, &g.rs).is_ok();
            let on = n.role == r;
            let rr = Rect::new(bx, by, bw, bh);
            draw_rectangle(rr.x, rr.y, rr.w, rr.h, if on { hx(0xffcc66) } else if ok { hx(0x2e3a52) } else { hx(0x22242e) });
            draw_rectangle_lines(rr.x, rr.y, rr.w, rr.h, 2., if on { BLACK } else { hx(0x4a5068) });
            text_c(font, ROLE_N[r as usize], rr.x + rr.w / 2., rr.y + rr.h * 0.66, 11.5 * u, if on { hx(0x222222) } else if ok { WHITE } else { hx(0x777788) });
            if ui.click.map_or(false, |p| rr.contains(p) && ui.clip.contains(p)) && !on { chosen = Some(r); }
        }
        y += ((list.len() + cols - 1) / cols) as f32 * (bh + gap) + 4. * u;
        if let Some(r) = chosen {
            match g.set_role(n.id, r) { Ok(m) => g.toast(m), Err(e) => g.toast(format!("{}: {}", ROLE_N[r as usize], e)) }
        }
        if manual && visible(y, 34. * u, c) {
            if btn(ui, font, Rect::new(c.x + 8. * u, y + 2. * u, 150. * u, 28. * u), "ให้เกมเลือกให้", true, u) { g.rs.manual.remove(&n.id); g.assign_roles_pub(); }
        }
        if manual { y += 34. * u; }
    }
    // ---- v35 เขาดูแลตัวเองได้แค่ไหน (หาเอง/ทำเอง) และเรื่องที่ต้องพึ่งเรา
    let own = g.self_status(id);
    if !own.is_empty() {
        header(font, c, y, "พึ่งตัวเอง", u);
        y += 28. * u;
        for (k, l) in own.iter().enumerate() {
            let ws = wrap_th(font, l, 11. * u, c.w - 24. * u);
            let hh = ws.len() as f32 * 14. * u + 4. * u;
            if visible(y, hh, c) { for (j, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, y + 14. * u + j as f32 * 14. * u, 11. * u, if k == 0 && l.starts_with("ตอนนี้") { hx(0x99ddaa) } else if l.contains("ต้องพึ่งคุณ") || l.contains("ขอแรงช่วย") { hx(0xffcc66) } else { hx(0xcccccc) }); } }
            y += hh;
        }
        let fine = "ความปลอดภัย · ตัดสินข้อพิพาท · อนุญาตที่ดิน คือสิ่งที่เขาพึ่งคุณ";
        if visible(y, 18. * u, c) { text(font, fine, c.x + 8. * u, y + 13. * u, 10.5 * u, hx(0x8899aa)); }
        y += 22. * u;
    }
    // ---- คำขอ/ปัญหาที่เกี่ยวกับคนนี้
    let mut qdo: Option<(usize, QA)> = None;
    // ขอของ/ส่งของ ย้ายไปอยู่ในหัวข้อ "อยากได้" ด้านบนแล้ว
    let rq: Vec<usize> = g.reqs_for(id).into_iter().filter(|&ri| !matches!(g.nreqs[ri].k, QK::Fetch { .. } if g.nreqs[ri].from == id) && !matches!(g.nreqs[ri].k, QK::Deliver { to, .. } if to == id)).collect();
    if !rq.is_empty() {
        header(font, c, y, "เรื่องที่ขอให้ช่วย", u);
        y += 28. * u;
        for ri in rq {
            let t = g.req_text(&g.nreqs[ri]);
            let ws = wrap_th(font, &t, 12. * u, c.w - 24. * u);
            let hh = ws.len() as f32 * 15. * u + 6. * u;
            if visible(y, hh, c) { for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, y + 14. * u + k as f32 * 15. * u, 12. * u, hx(0xfff0b0)); } }
            y += hh;
            let acts = g.req_actions(ri, id);
            let bw = ((c.w - 16. * u) / acts.len().max(1) as f32).min(150. * u);
            if visible(y, 40. * u, c) {
                for (k, (a, lab, en)) in acts.iter().enumerate() {
                    if btn(ui, font, Rect::new(c.x + 8. * u + k as f32 * bw, y + 4. * u, bw - 6. * u, 32. * u), lab, *en, u) { qdo = Some((ri, *a)); }
                }
            }
            y += 42. * u;
            sep(c, y - 2.);
        }
    }
    if let Some((ri, a)) = qdo {
        if let Some(m) = g.req_do(ri, a) { g.toast(m); }
        if g.idx(id).is_none() { ui.npc = None; return y - top; }
    }
    #[derive(Clone, Copy, PartialEq)]
    enum A { Talk, GiftToggle, Give(usize), Apology, Invite, Marry, Shop, Dismiss, Back }
    let mut act: Option<A> = None;
    let mut buy: Option<usize> = None;
    let rh = 44. * u;
    let mut row = |ui: &UiState, y: &mut f32, label: &str, desc: &str, btn_t: &str, en: bool, a: A, act: &mut Option<A>| {
        if visible(*y, rh, c) {
            text(font, label, c.x + 8. * u, *y + 18. * u, 14. * u, hx(0xffcc66));
            text(font, desc, c.x + 8. * u, *y + 35. * u, 11. * u, hx(0xaaaaaa));
            if btn(ui, font, Rect::new(c.x + c.w - 124. * u, *y + 6. * u, 112. * u, 32. * u), btn_t, en, u) { *act = Some(a); }
            sep(c, *y + rh - 2.);
        }
        *y += rh;
    };
    row(ui, &mut y, "คุย", "วันละครั้ง · คนขี้อายเปิดใจช้ากว่า · คนที่กำลังโกรธจะไม่ยอมคุย", if n.talked { "คุยแล้ว" } else { "คุย" }, !n.talked, A::Talk, &mut act);
    row(ui, &mut y, "ให้ของขวัญ", "เลือกของที่ตรงใจ — ของที่ไม่ชอบทำให้โกรธ · ให้ซ้ำๆ จะเบื่อ · วันละครั้ง", if n.gifted { "ให้แล้ว" } else if ui.gift_open { "ซ่อน" } else { "เลือกของ" }, !n.gifted, A::GiftToggle, &mut act);
    if ui.gift_open && !n.gifted {
        for &it in GIFTS.iter() {
            if g.p.inv[it] < 1 { continue; }
            if visible(y, 36. * u, c) {
                let mark = if it == n.soc.love as usize && n.hearts >= 4 { " · ของโปรดของเขา" } else if it == n.soc.hate as usize && n.hearts >= 6 { " · ของที่เขาไม่ชอบ!" } else { "" };
                text(font, &format!("{} x{}{}", INAME[it], g.p.inv[it], mark), c.x + 24. * u, y + 22. * u, 12. * u, hx(0xdddddd));
                if btn(ui, font, Rect::new(c.x + c.w - 124. * u, y + 3. * u, 112. * u, 30. * u), "มอบ", true, u) { act = Some(A::Give(it)); }
            }
            y += 36. * u;
        }
        if GIFTS.iter().all(|&it| g.p.inv[it] < 1) {
            if visible(y, 28. * u, c) { text(font, "ไม่มีของที่ให้ได้ในกระเป๋า (อัญมณี ผลผลิต ปลา เนื้อ อาหารปรุง)", c.x + 24. * u, y + 18. * u, 11. * u, hx(0x999999)); }
            y += 28. * u;
        }
    }
    if n.state == ST_HOME || n.state == ST_FOLLOW {
        let sorry = n.soc.anger > 0 || n.soc.grudge > 0;
        row(ui, &mut y, "ขอโทษ", if sorry { "ได้ผลหรือไม่ขึ้นกับนิสัยและความสนิท · วันละครั้ง" } else { "ตอนนี้ไม่มีเรื่องค้างคาใจ" }, "ขอโทษ", sorry && n.soc.apol_day != day_of(g), A::Apology, &mut act);
    }
    if n.state == ST_FOLLOW {
        let fb = g.free_beds().len();
        row(ui, &mut y, "ชวนมาอยู่ที่ฐาน", &format!("เตียงว่าง {} (สร้างเตียงในเมนูสร้าง)", fb), "ชวน", fb > 0, A::Invite, &mut act);
    } else if !n.child {
        let can = n.hearts >= 8 && n.mood >= 50 && n.soc.grudge < 15 && n.soc.anger == 0 && !g.npcs.iter().any(|x| x.married);
        row(ui, &mut y, "ขอแต่งงาน", &format!("สนิท 8/10 ขึ้นไป · {} เหรียญ + {} อัญมณี · แต่งได้คนเดียว (มีลูกหลังแต่ง 3 วัน)", MARRY_COIN, MARRY_GEM), if n.married { "แต่งแล้ว" } else { "ขอแต่งงาน" }, can && !n.married, A::Marry, &mut act);
        if n.role == 3 {
            row(ui, &mut y, "ร้านค้า", "พ่อค้าเฉพาะทางของหมู่บ้าน (ด้านล่าง)", "เปิดร้าน", false, A::Shop, &mut act);
            header(font, c, y, "สินค้า", u);
            y += 28. * u;
            for (i, o) in VILLAGE_OFFERS.iter().enumerate() {
                if visible(y, rh, c) {
                    text(font, o.name, c.x + 8. * u, y + 18. * u, 14. * u, hx(0xffffff));
                    let pr = g.npc_price(id, o.price);
                    text(font, &match pr { Some(p) => format!("{} · {} เหรียญ{}", o.desc, p, if p < o.price { " (ลดให้คนสนิท)" } else if p > o.price { " (แพงกว่าปกติเพราะยังไม่สนิท)" } else { "" }), None => "เขาไม่ขายให้คุณตอนนี้".to_string() }, c.x + 8. * u, y + 35. * u, 11. * u, hx(0xaaaaaa));
                    if btn(ui, font, Rect::new(c.x + c.w - 124. * u, y + 6. * u, 112. * u, 32. * u), "ซื้อ", pr.map_or(false, |p| g.p.inv[COIN] >= p), u) { buy = Some(i); }
                    sep(c, y + rh - 2.);
                }
                y += rh;
            }
        }
    }
    header(font, c, y, "รู้จักเขามากขึ้น", u);
    y += 28. * u;
    // นิสัย/ของโปรด: ยิ่งสนิทยิ่งรู้จักมากขึ้น (เหมือนรู้จักคนจริงๆ)
    {
        let known = (n.hearts as usize / 2 + 1).min(5);
        let trs: Vec<&str> = n.soc.traits_ranked().into_iter().take(known).map(|(k, hi)| Soc::trait_name(k, hi)).collect();
        let body = g.npc_body(&n);
        let mut info = vec![format!("รูปร่าง: {} · {}", body.describe(), body.look_words())];
        for l in g.psy_info(n.id) { info.push(l); }
        if n.state == ST_HOME {
            let bed_ok = g.w.placed.get(&n.bed) == Some(&33);
            info.push(if !bed_ok { "บ้าน: ไม่มีเตียง (ความสุขลดทุกวัน)".to_string() } else if let Some(r) = g.w.room_from(n.bed) { format!("บ้าน: มีบ้านมีหลังคา {} (ความสุข +{}/วัน{})", stars(r.comfort), if r.comfort >= 3 { 6 } else { 4 }, if r.comfort >= 3 { "" } else { " · บ้าน 3 ดาวขึ้นไป = +6" }) } else { "บ้าน: นอนกลางแจ้ง — ล้อมเตียงด้วยกำแพง+ประตู (ความสุข -3/วัน)".to_string() });
        }
        info.push(format!("ความรู้สึกต่อคุณ: {}", if n.married { "คู่สมรส" } else { opinion_label(op) }));
        info.push(format!("นิสัย: {}{}", trs.join(" · "), if known < 5 { " · (ยังรู้จักไม่หมด)" } else { "" }));
        info.push(format!("ของที่ชอบ: {}", if n.hearts >= 4 { INAME[n.soc.love as usize] } else { "ยังไม่รู้ (สนิท 4 ขึ้นไปถึงจะรู้)" }));
        info.push(format!("ของที่ไม่ชอบ: {}", if n.hearts >= 6 { INAME[n.soc.hate as usize] } else { "ยังไม่รู้ (สนิท 6 ขึ้นไปถึงจะรู้)" }));
        if n.soc.anger >= 20 { info.push("กำลังโกรธคุณอยู่ — ให้เวลาหรือขอโทษ".to_string()); }
        if n.soc.grudge >= 30 { info.push("ยังแค้นเรื่องเก่าที่คุณทำไว้ — ต้องใช้เวลา/ขอโทษหลายครั้ง".to_string()); }
        if n.state == ST_HOME {
            let mut foes: Vec<String> = vec![];
            for o in g.npcs.iter().filter(|o| o.id != n.id && o.state == ST_HOME && !o.child) {
                let r = g.rel_between(n.id, o.id);
                if r <= -30 { foes.push(NPC_NAMES[o.name as usize].to_string()); }
            }
            if !foes.is_empty() { info.push(format!("เข้ากับ {} ไม่ได้", foes.join(", "))); }
        }
        for l in info {
            let ws = wrap_th(font, &l, 11. * u, c.w - 24. * u);
            let hh = ws.len() as f32 * 14. * u + 4. * u;
            if visible(y, hh, c) { for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, y + 12. * u + k as f32 * 14. * u, 11. * u, hx(0xb9c3d9)); } }
            y += hh;
        }
        y += 6. * u;
    }
    row(ui, &mut y, "ให้ย้ายออก", "ถ้าไม่ต้องการแล้ว (ย้ายออกถาวร)", "ย้ายออก", true, A::Dismiss, &mut act);
    row(ui, &mut y, "กลับ", "ภาพรวมชุมชน", "กลับ", true, A::Back, &mut act);
    if let Some(i) = buy { g.npc_buy(id, i); }
    match act {
        Some(A::Talk) => g.npc_talk(id),
        Some(A::GiftToggle) => ui.gift_open = !ui.gift_open,
        Some(A::Give(it)) => { g.npc_gift_item(id, it); ui.gift_open = false; }
        Some(A::Apology) => g.npc_apologize(id),
        Some(A::Invite) => g.npc_invite(id),
        Some(A::Marry) => g.npc_marry(id),
        Some(A::Dismiss) => { g.npc_dismiss(id); ui.npc = None; }
        Some(A::Back) => { ui.npc = None; ui.scroll = 0.; ui.gift_open = false; }
        _ => {}
    }
    y - top
}


// ======================= แท็บ "อาณาจักร" / "เมือง" (v17) =======================
fn para(font: &Font, c: Rect, y: &mut f32, s: &str, col: Color, u: f32) {
    let ws = wrap_th(font, s, 11. * u, c.w - 20. * u);
    let hh = ws.len() as f32 * 14. * u + 6. * u;
    if visible(*y, hh, c) { for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, *y + 13. * u + k as f32 * 14. * u, 11. * u, col); } }
    *y += hh;
}
fn stance_word(s: u8) -> &'static str { ["เป็นกลาง", "คู่ค้า", "พันธมิตร", "สงคราม!", "เราเป็นเมืองขึ้น"][s.min(4) as usize] }
/// ปุ่มเรียงแถว คืนดัชนีที่กด
fn btn_row(ui: &UiState, font: &Font, c: Rect, y: f32, labels: &[(&str, bool)], u: f32) -> Option<usize> {
    let n = labels.len().max(1) as f32;
    let w = (c.w - 16. * u) / n;
    let mut hit = None;
    for (i, (l, en)) in labels.iter().enumerate() {
        if btn(ui, font, Rect::new(c.x + 8. * u + i as f32 * w, y, w - 4. * u, 26. * u), l, *en, u) { hit = Some(i); }
    }
    hit
}
fn kingdom_actions(g: &mut Game, ui: &UiState, font: &Font, c: Rect, y: &mut f32, k: usize, u: f32) {
    let st = g.rs.ks[k].stance;
    if !g.rs.ks[k].alive { return; }
    if !g.can_diplo(k) {
        let cap = g.w.realm.cities[g.w.realm.ks[k].cap].name;
        para(font, c, y, &format!("เจรจาได้ที่ปราสาทเมือง{} (กลางเมือง) หรือตั้งชาวบ้านช่างพูดเป็น \"ทูต\" เพื่อเจรจาจากที่นี่", cap), hx(0x8890a8), u);
        return;
    }
    if visible(*y, 30. * u, c) {
        let labels = [
            ("ของขวัญ 50", true),
            (if st == 1 { "เลิกค้าขาย" } else { "สัญญาการค้า" }, st != 3 && st != 4),
            (if st == 2 { "ถอนพันธมิตร" } else { "ขอเป็นพันธมิตร" }, st != 3 && st != 4),
            (if st == 3 { "ขอสงบศึก" } else if st == 4 { "ประกาศเอกราช" } else { "ประกาศสงคราม" }, true),
        ];
        match btn_row(ui, font, c, *y, &labels, u) {
            Some(0) => g.diplo_gift(k),
            Some(1) => g.diplo_trade(k),
            Some(2) => g.diplo_ally(k),
            Some(3) => if st == 3 { g.diplo_peace(k) } else if st == 4 { g.independence(k) } else { g.diplo_war(k) },
            _ => {}
        }
    }
    *y += 32. * u;
}
fn kingdom_card(g: &mut Game, ui: &UiState, font: &Font, c: Rect, y: &mut f32, k: usize, u: f32) {
    let kd = g.w.realm.ks[k].clone();
    let st = g.rs.ks[k].clone();
    let col = hx(kd.color);
    if visible(*y, 24. * u, c) {
        draw_rectangle(c.x, *y + 2. * u, 6. * u, 20. * u, col);
        text(font, &format!("{} — {}{}", kd.name, kd.title, kd.ruler), c.x + 12. * u, *y + 18. * u, 14. * u, col);
    }
    *y += 26. * u;
    if !st.alive { para(font, c, y, "ล่มสลายแล้ว", hx(0x888888), u); return; }
    let cap = &g.w.realm.cities[kd.cap];
    let owned: Vec<&str> = (0..g.w.realm.cities.len()).filter(|&ci| g.city_owner(ci) == k as u8).map(|ci| g.w.realm.cities[ci].name).collect();
    let wars: Vec<String> = (0..KN).filter(|&j| st.war[j]).map(|j| g.kname(j)).collect();
    let allies: Vec<String> = (0..KN).filter(|&j| st.ally[j]).map(|j| g.kname(j)).collect();
    let laws: Vec<&str> = kd.laws.iter().map(|&l| LAW_N[l as usize]).collect();
    para(font, c, y, &format!("นิสัย: {} · เมืองหลวง {} ({} ช่อง ทาง{}) · เมือง: {}", TRAIT_N[kd.trait_ as usize], cap.name, g.dist_to(cap.pos), g.dir_word(cap.pos), owned.join(", ")), hx(0xc9d1e6), u);
    para(font, c, y, &format!("กำลังทหาร {} · ความมั่งคั่ง {} · กฎ: {}", st.strength as i32, st.wealth as i32, laws.join(" / ")), hx(0xb0b8cc), u);
    para(font, c, y, &format!("กับเรา: {} · ชื่อเสียงของเรา {}{}{}", stance_word(st.stance), st.rep, if wars.is_empty() { String::new() } else { format!(" · รบกับ {}", wars.join(", ")) }, if allies.is_empty() { String::new() } else { format!(" · พันธมิตร {}", allies.join(", ")) }), if st.stance == 3 { hx(0xff8080) } else { hx(0xffe0a0) }, u);
    if let Some(a) = &st.army { para(font, c, y, &format!("ทัพ {} นาย กำลังเดินทัพ ห่างเมืองเรา ~{} ช่อง", a.n, g.town_center().map_or(0, |t| ((a.x - t.0 as f32).hypot(a.y - t.1 as f32)) as i32)), hx(0xff6060), u); }
    // v24 ยกทัพไปตีเมืองของอาณาจักรที่รบกับเรา
    if st.stance == 3 && !peaceful() {
        if let Some(ci) = g.nearest_city_of(k) {
            let n = g.army_candidates().len();
            let marching = g.rs.war.march;
            let lab = if marching { "ถอยทัพกลับเมือง".to_string() } else { format!("ยกทัพไปตีเมือง{} ({} นาย)", g.w.realm.cities[ci].name, n) };
            if visible(*y, 30. * u, c) && btn_row(ui, font, c, *y, &[(lab.as_str(), true)], u).is_some() {
                if marching { g.army_home("ทัพกลับเข้าเมืองแล้ว"); } else { g.army_call(Some(ci)); }
            }
            *y += 32. * u;
            para(font, c, y, "ทหาร/ยาม/แม่ทัพในเมืองจะเดินตามคุณ · เข้าเมืองศัตรู = ศึกยึดเมือง · ล้มทหารตั้งรับหมด (เมืองหลวงต้องล้มแม่ทัพด้วย) = ยึดได้ · ทหารบาดเจ็บจะกลับไปพักที่บ้าน 1 วัน", hx(0x8890a8), u);
        }
    }
    kingdom_actions(g, ui, font, c, y, k, u);
    sep(c, *y);
    *y += 6. * u;
}

fn tab_realm(g: &mut Game, _sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    // ---- เมืองเรา
    header(font, c, y, &format!("{} · ชื่อเสียง {} · ป้องกัน {}", g.my_name(), g.fame(), g.defense() as i32), u);
    y += 28. * u;
    let over = (0..KN).find(|&k| g.rs.ks.get(k).map_or(false, |s| s.stance == 4));
    let claim = g.town_center().and_then(|t| g.claim_of(t));
    para(font, c, &mut y, &format!("ประชากร {} · คลังภาษี {} เหรียญ · ชนะศึก {} ครั้ง · {}{}", g.pop(), g.tax_pool, g.rs.siege_wins,
        match over { Some(k) => format!("เป็นเมืองขึ้นของ{} (ส่งบรรณาการทุก 5 วัน)", g.kname(k)), None => "อิสระ".to_string() },
        match claim { Some(k) => format!(" · อยู่ในเขตแดน{}", g.kname(k)), None => String::new() }), hx(0xc9d1e6), u);
    if g.tax_pool > 0 && visible(y, 30. * u, c) && btn(ui, font, Rect::new(c.x + 8. * u, y, 160. * u, 26. * u), "เก็บภาษีเข้ากระเป๋า", true, u) { g.collect_tax(); }
    if g.tax_pool > 0 { y += 32. * u; }
    if g.pop() >= 4 || g.rs.founded {
        header(font, c, y, "กฎหมายเมืองเรา", u);
        y += 28. * u;
        let l = g.rs.laws;
        let tax = if l & ML_LOWTAX != 0 { "ภาษีต่ำ" } else if l & ML_HIGHTAX != 0 { "ภาษีสูง" } else { "ภาษีปกติ" };
        let rows: [(String, &str); 5] = [
            (tax.to_string(), "ต่ำ: รายได้ -40% ชาวบ้าน +2 สุข/วัน ผู้อพยพมาง่าย · สูง: รายได้ +50% สุข -3/วัน คนมาน้อย"),
            (format!("เกณฑ์ทหาร: {}", if l & ML_MILITIA != 0 { "เปิด" } else { "ปิด" }), "ป้องกัน +1.2 ต่อประชากร แต่สุข -1/วัน"),
            (format!("เปิดพรมแดน: {}", if l & ML_OPEN != 0 { "เปิด" } else { "ปิด" }), "ผู้อพยพ ×1.6 (สายลับก็มาง่ายขึ้น)"),
            (format!("เคอร์ฟิว: {}", if l & ML_CURFEW != 0 { "เปิด" } else { "ปิด" }), "สุข -1/วัน"),
            (format!("ห้ามอาวุธในเมือง: {}", if l & ML_NOWEAPON != 0 { "เปิด" } else { "ปิด" }), "ชาวบ้านรู้สึกปลอดภัย สุข +1/วัน"),
        ];
        for (i, (lab, desc)) in rows.iter().enumerate() {
            if visible(y, 30. * u, c) {
                text(font, desc, c.x + 150. * u, y + 17. * u, 10. * u, hx(0x9aa2b8));
                if btn(ui, font, Rect::new(c.x + 8. * u, y + 2. * u, 136. * u, 24. * u), lab, true, u) {
                    g.rs.laws = match i {
                        0 => { let t = if l & ML_LOWTAX != 0 { 0 } else if l & ML_HIGHTAX != 0 { ML_LOWTAX } else { ML_HIGHTAX }; (l & !(ML_LOWTAX | ML_HIGHTAX)) | t }
                        1 => l ^ ML_MILITIA, 2 => l ^ ML_OPEN, 3 => l ^ ML_CURFEW, _ => l ^ ML_NOWEAPON,
                    };
                }
            }
            y += 30. * u;
        }
        // ---- v36 กฎบัตรเมือง
        header(font, c, y, "กฎบัตรเมือง (สังคม)", u);
        y += 28. * u;
        let (ok, mid, no) = g.law_mood_split(None);
        para(font, c, &mut y, &format!("ความเห็นของชาวบ้านต่อกฎทั้งหมด: พอใจ {} · เฉย {} · ไม่พอใจ {} (แต่ละคนคิดต่างกันตามนิสัย: ใจดี โลภ ใจร้อน กล้า ช่างพูด)", ok, mid, no), hx(0xc9d1e6), u);
        for l in 0..v36::LAW_N {
            let v = g.ch_lv(l) as usize;
            let (a, b, d) = g.law_mood_split(Some(l));
            let ws = wrap_th(font, v36::LAW_DESC[l][v], 10. * u, c.w - 24. * u);
            let hh = 34. * u + ws.len() as f32 * 13. * u + 16. * u;
            if visible(y, hh, c) {
                text(font, v36::LAW_NAME[l], c.x + 8. * u, y + 20. * u, 13. * u, hx(0xffcc66));
                if btn(ui, font, Rect::new(c.x + c.w - 156. * u, y + 2. * u, 148. * u, 26. * u), v36::LAW_LV[l][v], true, u) { g.ch_cycle(l); }
                for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, y + 42. * u + k as f32 * 13. * u, 10. * u, hx(0x9aa2b8)); }
                if v > 0 { text(font, &format!("ความเห็น: พอใจ {} · เฉย {} · ไม่พอใจ {}", a, b, d), c.x + 8. * u, y + hh - 6. * u, 10. * u, if d > a { hx(0xee9988) } else { hx(0x88ee99) }); }
                sep(c, y + hh);
            }
            y += hh + 4. * u;
        }
    }
    if !g.rs.founded {
        header(font, c, y, "สถาปนาอาณาจักรของเราเอง", u);
        y += 28. * u;
        let reqs = g.found_reqs();
        for (t, ok) in &reqs { para(font, c, &mut y, &format!("{} {}", if *ok { "[ผ่าน]" } else { "[ยัง]" }, t), if *ok { hx(0x88ee88) } else { hx(0xee9988) }, u); }
        let all = reqs.iter().all(|r| r.1);
        if visible(y, 32. * u, c) && btn(ui, font, Rect::new(c.x + 8. * u, y, 200. * u, 28. * u), "สถาปนาอาณาจักร", all, u) { g.found_kingdom(); }
        y += 34. * u;
    }
    // ---- เรื่องด่วน
    let mut urgent = false;
    for k in 0..g.rs.ks.len() {
        let st = g.rs.ks[k].clone();
        if let Some((dl, amt)) = st.ult {
            if !urgent { header(font, c, y, "เรื่องด่วน", u); y += 28. * u; urgent = true; }
            para(font, c, &mut y, &format!("คำขาดจาก{}: จ่าย {} เหรียญภายในวันที่ {} ไม่งั้นยกทัพมา (กำลังเขา ~{} · ป้องกันเรา {})", g.kname(k), amt, dl + 1, (st.strength * 0.25) as i32, g.defense() as i32), hx(0xff9090), u);
            if visible(y, 30. * u, c) { match btn_row(ui, font, c, y, &[("จ่าย", true), ("ปฏิเสธ (สงคราม)", true)], u) { Some(0) => g.pay_ultimatum(k, true), Some(1) => g.pay_ultimatum(k, false), _ => {} } }
            y += 32. * u;
        }
        if st.vas_ask {
            if !urgent { header(font, c, y, "เรื่องด่วน", u); y += 28. * u; urgent = true; }
            para(font, c, &mut y, &format!("{}ให้เรายอมเป็นเมืองขึ้น (ส่งบรรณาการทุก 5 วัน แลกกับการคุ้มครอง) — ปฏิเสธอาจโดนบุก", g.kname(k)), hx(0xffc080), u);
            if visible(y, 30. * u, c) { match btn_row(ui, font, c, y, &[("ยอมเป็นเมืองขึ้น", true), ("ปฏิเสธ", true)], u) { Some(0) => g.answer_vassal(k, true), Some(1) => g.answer_vassal(k, false), _ => {} } }
            y += 32. * u;
        }
        if let Some((dl, e)) = st.help {
            if !urgent { header(font, c, y, "เรื่องด่วน", u); y += 28. * u; urgent = true; }
            para(font, c, &mut y, &format!("พันธมิตร{}ขอความช่วยเหลือในสงครามกับ{} ภายในวันที่ {} — ไม่ช่วย = ความสัมพันธ์ตก/พันธมิตรแตกหัก", g.kname(k), g.kname(e), dl + 1), hx(0xffd080), u);
            if visible(y, 30. * u, c) && btn_row(ui, font, c, y, &[("ส่งทุนสงคราม 60 เหรียญ", true)], u).is_some() { g.send_help(k); }
            y += 32. * u;
        }
    }
    let pets = g.rs.petitions.clone();
    for (i, p) in pets.iter().enumerate() {
        if !urgent { header(font, c, y, "เรื่องด่วน", u); y += 28. * u; urgent = true; }
        para(font, c, &mut y, &format!("{} {} จากเมือง{} ขอย้ายมา · ความสามารถ: {} · ขอเงินเดือน {} เหรียญ/วัน + บ้าน 3 ดาวขึ้นไป", RANK_N[p.rank as usize], NPC_NAMES[p.name as usize], g.w.realm.cities[p.city].name, PERK_N[p.perk as usize], p.salary), hx(0xe0d0ff), u);
        if visible(y, 30. * u, c) { match btn_row(ui, font, c, y, &[("รับเข้าเมือง", true), ("ปฏิเสธ", true)], u) { Some(0) => { g.accept_petition(i, true); break; } Some(1) => { g.accept_petition(i, false); break; } _ => {} } }
        y += 32. * u;
    }
    // ---- อาณาจักรทั้งหมด
    header(font, c, y, "อาณาจักรในโลก", u);
    y += 28. * u;
    for k in 0..g.rs.ks.len() { kingdom_card(g, ui, font, c, &mut y, k, u); }
    // ---- ข่าว
    if !g.news.is_empty() {
        header(font, c, y, "ข่าวสาร", u);
        y += 28. * u;
        let news: Vec<(i32, String)> = g.news.iter().rev().take(14).cloned().collect();
        for (d, t) in news { para(font, c, &mut y, &format!("วันที่ {}: {}", d + 1, t), hx(0xc9d1e6), u); }
    }
    y - top + 10. * u
}

fn tab_city(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let Some((ci, kind)) = ui.city else { return 0. };
    let top = c.y - ui.scroll;
    let mut y = top;
    let city = g.w.realm.cities[ci].clone();
    let o = g.city_owner(ci);
    header(font, c, y, &format!("{}{} — {}", if city.cap { "เมืองหลวง" } else { "เมือง" }, city.name, g.owner_name(o)), u);
    y += 28. * u;
    match kind {
        0 => {
            para(font, c, &mut y, &format!("ของถูกที่นี่: {}, {} · เมืองนี้ต้องการ: {}, {}{}", INAME[city.cheap[0]], INAME[city.cheap[1]], INAME[city.want[0]], INAME[city.want[1]], if g.law_on(ci, L_TAX) { " · มีภาษีการค้า +15%" } else { "" }), hx(0xc9d1e6), u);
            para(font, c, &mut y, &format!("เหรียญ {} · ขายเยอะราคาจะตก ซื้อเยอะราคาจะขึ้น (ค่อยๆ กลับปกติทุกเช้า)", g.p.inv[COIN]), hx(0xffe080), u);
            for gi in 0..GOODS.len() {
                let item = GOODS[gi].0;
                let (bp, spp) = (g.price(ci, gi, true), g.price(ci, gi, false));
                if visible(y, 30. * u, c) {
                    icon_c(sp, item, c.x + 8. * u, y + 3. * u, 22. * u, WHITE);
                    text(font, &format!("{} (มี {})", INAME[item], g.p.inv[item]), c.x + 36. * u, y + 13. * u, 11. * u, WHITE);
                    text(font, &format!("ซื้อ {} · ขาย {}", bp, spp), c.x + 36. * u, y + 26. * u, 10. * u, if city.cheap.contains(&item) { hx(0x88ee88) } else if city.want.contains(&item) { hx(0xffc060) } else { hx(0xaab0c0) });
                    let bw = 64. * u;
                    let bx = c.x + c.w - 3. * bw - 12. * u;
                    if btn(ui, font, Rect::new(bx, y + 2. * u, bw - 4. * u, 26. * u), "ซื้อ", g.p.inv[COIN] >= bp, u) { g.trade(ci, gi, true, 1); }
                    if btn(ui, font, Rect::new(bx + bw, y + 2. * u, bw - 4. * u, 26. * u), "ขาย", g.p.inv[item] > 0, u) { g.trade(ci, gi, false, 1); }
                    if btn(ui, font, Rect::new(bx + 2. * bw, y + 2. * u, bw - 4. * u, 26. * u), "ขาย 10", g.p.inv[item] >= 10, u) { g.trade(ci, gi, false, 10); }
                }
                y += 32. * u;
            }
        }
        1 => {
            if o == PLAYER_OWN { para(font, c, &mut y, "ศาลาว่าการของเมืองเรา — เมืองนี้ส่งรายได้ 15 เหรียญ/วันเข้าคลัง", hx(0xc9d1e6), u); }
            else {
                let k = o as usize;
                let kd = g.w.realm.ks[k].clone();
                let rep = g.rs.ks[k].rep;
                let greet = if g.rs.ks[k].stance == 3 { "เจ้ากล้าดีอย่างไรถึงมาที่นี่ในยามสงคราม!" } else if rep >= 50 { "ยินดีต้อนรับ สหายของเรา" } else if rep >= 10 { "ว่ามาเถิด นักเดินทาง" } else if rep <= -30 { "เราไม่ไว้ใจเจ้า... พูดมาเร็วๆ" } else { "มีธุระอันใด" };
                para(font, c, &mut y, &format!("{}{}: \"{}\"", kd.title, kd.ruler, greet), hx(0xffe0a0), u);
                kingdom_card(g, ui, font, c, &mut y, k, u);
            }
        }
        2 => {
            para(font, c, &mut y, "คอกม้า: ม้าพาวิ่งเร็วขึ้น 70% บนพื้นนอกอาคาร (ลงอัตโนมัติเมื่อเข้าบ้าน/ดันเจี้ยน/ขึ้นเรือ) · ขึ้น/ลง: ปุ่ม \"ม้า\" หรือ H", hx(0xc9d1e6), u);
            if visible(y, 32. * u, c) {
                if g.rs.horse { text(font, "คุณมีม้าแล้ว", c.x + 8. * u, y + 18. * u, 13. * u, hx(0x88ee88)); }
                else if btn(ui, font, Rect::new(c.x + 8. * u, y, 200. * u, 28. * u), "ซื้อม้า 120 เหรียญ", g.p.inv[COIN] >= 120, u) { g.buy_horse(); }
            }
            y += 34. * u;
        }
        _ => {
            para(font, c, &mut y, "รถม้าโดยสาร: ไปได้เฉพาะเมืองที่เคยไปแล้ว · ค่าโดยสารตามระยะทาง · เวลาในเกมผ่านไประหว่างเดินทาง", hx(0xc9d1e6), u);
            let mut dest: Vec<usize> = g.rs.visited.iter().copied().filter(|&d| d != ci).collect();
            dest.sort();
            if dest.is_empty() { para(font, c, &mut y, "ยังไม่เคยไปเมืองอื่น — เดินทางไปเองก่อนสักครั้ง", hx(0xee9988), u); }
            let mut go = None;
            for d in dest {
                let (cost, hours) = g.coach_cost(ci, d);
                let dc = &g.w.realm.cities[d];
                if visible(y, 30. * u, c) {
                    text(font, &format!("{} ({}) · {} เหรียญ · {:.0} ชม.", dc.name, g.owner_name(g.city_owner(d)), cost, hours.max(1.)), c.x + 8. * u, y + 18. * u, 12. * u, WHITE);
                    if btn(ui, font, Rect::new(c.x + c.w - 112. * u, y + 2. * u, 104. * u, 26. * u), "เดินทาง", g.p.inv[COIN] >= cost, u) { go = Some(d); }
                }
                y += 32. * u;
            }
            if let Some(d) = go { g.coach_go(ci, d); ui.close = true; }
        }
    }
    y - top + 10. * u
}

#[cfg(test)]
mod tests_v29 {
    use super::*;
    fn villager(g: &mut Game, id: u32, brave: u8) {
        let b = (tfloor(g.p.x) + id as i32 * 2, tfloor(g.p.y) + 3);
        g.w.placed.insert(b, 33);
        let mut soc = Soc::generate(id, id as u8);
        soc.tr[T_BRAVE] = brave;
        g.npcs.push(Npc { id, name: id as u8, look: 0, x: 0., y: 0., state: ST_HOME, hearts: 3, mood: 60, married: false, child: false, role: 0, bed: b, anim: 0., wt: 0., wd: (0., 0.), atk_cd: 0., talked: false, gifted: false, moving: false, left: false, soc });
    }
    #[test]
    fn pick_person_button_opens_role_picker_and_sets_role() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        villager(&mut g, 1, 80);
        villager(&mut g, 2, 10);
        villager(&mut g, 3, 50);
        // ปุ่ม "เลือกคน" ของที่ปรึกษา = เปิดหน้าเลือกคน (เดิมสั่งเปิดแท็บเดิมซ้ำ เลยไม่มีอะไรเกิดขึ้น)
        let a = g.advice().into_iter().find(|a| a.id == "farmer").expect("แนะนำให้มีชาวไร่");
        assert_eq!(a.act, AdvAct::Role(1));
        g.adv_do(a.act);
        assert!(matches!(g.req, Some(Req::RolePick(1))));
        // รายชื่อ: ทุกคนเป็นชาวไร่ได้ · ทหาร: คนขี้ขลาดทำไม่ได้ (บอกเหตุผล) และเรียงคนที่ทำได้ก่อน
        assert!(g.role_candidates(1).iter().all(|(_, ok)| ok.is_ok()));
        let sol = g.role_candidates(R_SOLDIER);
        assert!(sol.last().unwrap().1.is_err() && sol.last().unwrap().0 == 2);
        assert!(g.set_role(2, R_SOLDIER).is_err());
        assert!(g.set_role(1, 1).is_ok());
        assert_eq!(g.npcs[0].role, 1);
        assert!(g.rs.manual.contains_key(&1), "จำว่าเลือกเอง");
        // แม่ทัพมีได้คนเดียว
        assert!(g.set_role(1, R_GENERAL).is_ok());
        g.npcs[2].soc.tr[T_BRAVE] = 90;
        assert!(g.set_role(3, R_GENERAL).is_err());
        assert!(g.advice().iter().all(|a| a.id != "farmer") || g.npcs.iter().all(|n| n.role != 1));
    }
    #[test]
    fn menu_context_never_sticks_to_a_person() {
        let mut ui = UiState::new();
        ui.npc = Some(5);
        ui.role_pick = Some(1);
        ui.gift_open = true;
        ui.reset_context();
        assert!(ui.npc.is_none() && ui.role_pick.is_none() && !ui.gift_open, "เปิดเมนูใหม่ = เริ่มที่หน้ารวม");
    }
}
