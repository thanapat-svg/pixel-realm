//! เมนูแบบแท็บ: คราฟต์ · สร้าง · ชุด/ของ · กล่องเก็บของ · ร้านค้า
//! เลื่อนได้ด้วยล้อเมาส์หรือลากนิ้ว (มี scissor ตัดส่วนที่เกินกรอบ) และกดปุ่มตอน "ปล่อย" เพื่อไม่ให้ชนกับการเลื่อน

use super::*;

pub struct UiState {
    pub tab: usize,
    pub scroll: f32,
    pub max_scroll: f32,
    pub chest: Option<(i32, i32)>,
    pub shop: bool,
    pub greet: usize,
    press: Option<Vec2>,
    last: Vec2,
    drag: f32,
    click: Option<Vec2>,
    clip: Rect,
    pub close: bool,
}
impl UiState {
    pub fn new() -> Self {
        UiState { tab: 0, scroll: 0., max_scroll: 0., chest: None, shop: false, greet: 0, press: None, last: Vec2::ZERO, drag: 0., click: None, clip: Rect::new(0., 0., 1., 1.), close: false }
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
    let rh = 58. * u;
    let top = c.y - ui.scroll;
    let mut y = top;
    let mut act: Option<usize> = None;
    for (i, r) in RECIPES.iter().enumerate() {
        if visible(y, rh, c) {
            let st = recipe_state(&g.p, r);
            text(font, r.name, c.x + 8. * u, y + 18. * u, 15. * u, if st == 1 { hx(0x88aa88) } else { hx(0xffcc66) });
            text(font, r.desc, c.x + 8. * u, y + 34. * u, 11. * u, hx(0xaaaaaa));
            costs(g, sp, font, r.cost, r.need, c.x + 8. * u, y + 39. * u, u);
            let br = Rect::new(c.x + c.w - 104. * u, y + 13. * u, 92. * u, 32. * u);
            if btn(ui, font, br, ["คราฟต์", "มีแล้ว", "ล็อกอยู่"][st as usize], st == 0 && can_pay(&g.p, r.cost), u) {
                act = Some(i);
            }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    if let Some(i) = act {
        g.craft(i);
    }
    y - top
}

fn tab_build(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let rh = 58. * u;
    let top = c.y - ui.scroll;
    let mut y = top;
    let mut pick: Option<usize> = None;
    for (i, pc) in PIECES.iter().enumerate() {
        if visible(y, rh, c) {
            draw_rectangle(c.x + 6. * u, y + 8. * u, 34. * u, 34. * u, hx(0x2a2d3f));
            icon_c(sp, pc.icon, c.x + 8. * u, y + 10. * u, 30. * u, WHITE);
            text(font, pc.name, c.x + 48. * u, y + 18. * u, 15. * u, hx(0xffcc66));
            text(font, pc.desc, c.x + 48. * u, y + 34. * u, 11. * u, hx(0xaaaaaa));
            costs(g, sp, font, pc.cost, None, c.x + 48. * u, y + 39. * u, u);
            let sel = g.build_sel == Some(i);
            if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 13. * u, 92. * u, 32. * u), if sel { "กำลังใช้" } else { "เลือก" }, !sel, u) {
                pick = Some(i);
            }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    if let Some(s) = g.build_sel {
        if visible(y, rh, c) {
            text(font, &format!("โหมดสร้าง: {}", PIECES[s].name), c.x + 8. * u, y + 24. * u, 15. * u, hx(0x88ff88));
            text(font, "R = วาง · T / Esc = ยกเลิก", c.x + 8. * u, y + 42. * u, 11. * u, hx(0xaaaaaa));
            if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 13. * u, 92. * u, 32. * u), "ยกเลิก", true, u) {
                g.build_sel = None;
            }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    if visible(y, 40. * u, c) {
        text(font, "เลือกแล้วหันหน้าไปทางที่ต้องการ กด R / ปุ่มสร้าง เพื่อวาง · ตีเพื่อรื้อถอน", c.x + 8. * u, y + 22. * u, 11. * u, hx(0x888899));
    }
    y += 40. * u;
    if let Some(i) = pick {
        g.build_sel = Some(i);
        g.toast(format!("เลือก {} — กด R / ปุ่มสร้างเพื่อวาง", PIECES[i].name));
        ui.close = true;
    }
    y - top
}

fn tab_gear(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    let rh = 46. * u;
    header(font, c, y, "อาวุธ (กด Tab เพื่อสลับ)", u);
    y += 28. * u;
    let mut equip: Option<u8> = None;
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
            let cur = g.p.weapon == w;
            if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 7. * u, 92. * u, 32. * u), if cur { "ถืออยู่" } else if own { "ถือ" } else { "ไม่มี" }, own && !cur, u) {
                equip = Some(w);
            }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    if let Some(w) = equip {
        g.p.weapon = w;
    }
    header(font, c, y, "ชุดเกราะ", u);
    y += 28. * u;
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
    header(font, c, y, "กระเป๋า", u);
    y += 28. * u;
    let mut drink = false;
    for i in 0..NI {
        let rh2 = 30. * u;
        if visible(y, rh2, c) {
            icon_c(sp, i, c.x + 8. * u, y + 5. * u, 20. * u, WHITE);
            text(font, INAME[i], c.x + 36. * u, y + 20. * u, 13. * u, if g.p.inv[i] > 0 { WHITE } else { hx(0x777777) });
            text(font, &format!("x{}", g.p.inv[i]), c.x + c.w * 0.55, y + 20. * u, 13. * u, hx(0xffee99));
            if i == POTION && btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 2. * u, 92. * u, 26. * u), "ใช้ยา", g.p.inv[POTION] > 0, u) {
                drink = true;
            }
        }
        y += rh2;
    }
    if drink {
        g.drink();
    }
    y - top
}

fn tab_chest(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let Some(k) = ui.chest else { return 0. };
    let top = c.y - ui.scroll;
    let mut y = top;
    let rh = 40. * u;
    let mut mv: Option<(usize, i32, bool)> = None;
    let content = g.chests.get(&k).copied().unwrap_or([0; NI]);
    header(font, c, y, "ในกล่อง (กดเพื่อเอาออก)", u);
    y += 28. * u;
    let mut any = false;
    for i in 0..NI {
        if content[i] <= 0 {
            continue;
        }
        any = true;
        if visible(y, rh, c) {
            icon_c(sp, i, c.x + 8. * u, y + 8. * u, 24. * u, WHITE);
            text(font, INAME[i], c.x + 40. * u, y + 20. * u, 14. * u, WHITE);
            text(font, &format!("x{}", content[i]), c.x + 40. * u, y + 34. * u, 11. * u, hx(0xffee99));
            if btn(ui, font, Rect::new(c.x + c.w - 210. * u, y + 6. * u, 98. * u, 28. * u), "เอาออก 1", true, u) {
                mv = Some((i, 1, false));
            }
            if btn(ui, font, Rect::new(c.x + c.w - 106. * u, y + 6. * u, 98. * u, 28. * u), "ทั้งหมด", true, u) {
                mv = Some((i, content[i], false));
            }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    if !any {
        if visible(y, rh, c) {
            text(font, "(กล่องว่างเปล่า)", c.x + 10. * u, y + 24. * u, 13. * u, hx(0x888899));
        }
        y += rh;
    }
    header(font, c, y, "ในกระเป๋า (กดเพื่อใส่ลงกล่อง)", u);
    y += 28. * u;
    for i in 0..NI {
        if g.p.inv[i] <= 0 {
            continue;
        }
        if visible(y, rh, c) {
            icon_c(sp, i, c.x + 8. * u, y + 8. * u, 24. * u, WHITE);
            text(font, INAME[i], c.x + 40. * u, y + 20. * u, 14. * u, WHITE);
            text(font, &format!("x{}", g.p.inv[i]), c.x + 40. * u, y + 34. * u, 11. * u, hx(0xffee99));
            if btn(ui, font, Rect::new(c.x + c.w - 210. * u, y + 6. * u, 98. * u, 28. * u), "ใส่ 1", true, u) {
                mv = Some((i, 1, true));
            }
            if btn(ui, font, Rect::new(c.x + c.w - 106. * u, y + 6. * u, 98. * u, 28. * u), "ทั้งหมด", true, u) {
                mv = Some((i, g.p.inv[i], true));
            }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
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
        text(font, "พ่อค้าเร่", c.x + 70. * u, y + 22. * u, 15. * u, hx(0xffcc66));
        text(font, GREETS[ui.greet % GREETS.len()], c.x + 70. * u, y + 42. * u, 12. * u, hx(0xdddddd));
        icon_c(sp, COIN, c.x + 70. * u, y + 50. * u, 14. * u, WHITE);
        text(font, &format!("เหรียญของคุณ: {}", g.p.inv[COIN]), c.x + 88. * u, y + 62. * u, 12. * u, hx(0xffd84a));
    }
    y += 76. * u;
    let rh = 58. * u;
    let mut buy: Option<usize> = None;
    for (i, of) in OFFERS.iter().enumerate() {
        if visible(y, rh, c) {
            let owned = of.once && g.p.inv[of.item] > 0;
            icon_c(sp, of.item, c.x + 8. * u, y + 10. * u, 26. * u, WHITE);
            text(font, of.name, c.x + 44. * u, y + 20. * u, 14. * u, hx(0xffcc66));
            text(font, of.desc, c.x + 44. * u, y + 36. * u, 11. * u, hx(0xaaaaaa));
            icon_c(sp, COIN, c.x + 44. * u, y + 40. * u, 11. * u, WHITE);
            text(font, &format!("{}", of.price), c.x + 58. * u, y + 50. * u, 11. * u, if g.p.inv[COIN] >= of.price { hx(0xaaffaa) } else { hx(0xff8888) });
            if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 10. * u, 92. * u, 32. * u), if owned { "มีแล้ว" } else { "ซื้อ" }, !owned && g.p.inv[COIN] >= of.price, u) {
                buy = Some(i);
            }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    if let Some(i) = buy {
        g.buy(i);
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
    let mut tabs: Vec<(usize, &str)> = vec![(0, "คราฟต์"), (1, "สร้าง"), (2, "ชุด/ของ")];
    if ui.chest.is_some() {
        tabs.push((3, "กล่อง"));
    }
    if ui.shop {
        tabs.push((4, "ร้านค้า"));
    }
    if !tabs.iter().any(|t| t.0 == ui.tab) {
        ui.tab = 0;
    }
    ui.clip = Rect::new(0., 0., sw, sh);
    let tw = (w - 16. * u) / tabs.len() as f32;
    for (i, &(id, name)) in tabs.iter().enumerate() {
        let r = Rect::new(x + 8. * u + i as f32 * tw, y + 10. * u, tw - 4. * u, tabs_h);
        let on = ui.tab == id;
        draw_rectangle(r.x, r.y, r.w, r.h, if on { hx(0xffcc66) } else { hx(0x2e3144) });
        draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., if on { BLACK } else { hx(0x555a70) });
        let label = &thai(name);
        let fs = (15. * u) as u16;
        let d = measure_text(label, Some(font), fs, 1.);
        draw_text_ex(label, r.x + r.w / 2. - d.width / 2., r.y + r.h / 2. + 5. * u, TextParams { font: Some(font), font_size: fs, color: if on { hx(0x222222) } else { hx(0xdddddd) }, ..Default::default() });
        if ui.click.map_or(false, |p| r.contains(p)) && !on {
            ui.tab = id;
            ui.scroll = 0.;
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
