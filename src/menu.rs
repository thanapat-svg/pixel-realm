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
    pub gift_open: bool,
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
        UiState { tab: 0, scroll: 0., max_scroll: 0., chest: None, shop: false, forge: false, cook: false, adv: false, npc: None, gift_open: false, greet: 0, press: None, last: Vec2::ZERO, drag: 0., click: None, clip: Rect::new(0., 0., 1., 1.), close: false }
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
    recipe_list(g, sp, font, ui, c, u, &RECIPES)
}
fn tab_adv(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    recipe_list(g, sp, font, ui, c, u, &ADV_RECIPES)
}
fn recipe_list(g: &mut Game, sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32, list: &'static [Recipe]) -> f32 {
    let rh = 58. * u;
    let top = c.y - ui.scroll;
    let mut y = top;
    let mut act: Option<usize> = None;
    // เปิดเผยทีละน้อย: แสดงสูตรพื้นฐาน + สูตรที่เกี่ยวกับของที่มีอยู่ (ซ่อนที่เหลือไว้ ไม่ให้เมนูล้น)
    let basic = if list.len() >= 20 { 2 } else { 0 };
    let mut hidden = 0;
    for (i, r) in list.iter().enumerate() {
        let st0 = recipe_state(&g.p, r);
        let related = r.cost.iter().any(|&(it, _)| g.p.inv[it] > 0) || r.need.map_or(false, |n| g.p.inv[n] > 0);
        if !(i < basic || st0 == 1 || related) {
            hidden += 1;
            continue;
        }
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
    if hidden > 0 {
        if visible(y, 30. * u, c) {
            text(font, &format!("มีอีก {} สูตรที่ยังซ่อนอยู่ — เก็บวัตถุดิบใหม่ๆ แล้วจะปรากฏเอง", hidden), c.x + 8. * u, y + 20. * u, 11. * u, hx(0x888899));
        }
        y += 30. * u;
    }
    if let Some(i) = act {
        g.craft_from(list, i);
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
    header(font, c, y, "กระเป๋า", u);
    y += 28. * u;
    if visible(y, 24. * u, c) {
        let s = if g.p.can { format!("บัวรดน้ำ: น้ำ {}/{}", g.p.water, WATER_MAX) } else { "บัวรดน้ำ: ยังไม่มี (คราฟต์ได้ที่เมนู C)".to_string() };
        text(font, &s, c.x + 8. * u, y + 16. * u, 12. * u, hx(0x9ad0ff));
    }
    y += 24. * u;
    let mut seed_pick: Option<u8> = None;
    let mut crop_eat: Option<usize> = None;
    let mut raw_eat: Option<usize> = None;
    let mut drink = false;
    let mut dish: Option<usize> = None;
    for i in 0..NI {
        let rh2 = 30. * u;
        if visible(y, rh2, c) {
            icon_c(sp, i, c.x + 8. * u, y + 5. * u, 20. * u, WHITE);
            text(font, INAME[i], c.x + 36. * u, y + 20. * u, 13. * u, if g.p.inv[i] > 0 { WHITE } else { hx(0x777777) });
            text(font, &format!("x{}", g.p.inv[i]), c.x + c.w * 0.55, y + 20. * u, 13. * u, hx(0xffee99));
            if i == POTION && btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 2. * u, 92. * u, 26. * u), "ใช้ยา", g.p.inv[POTION] > 0, u) {
                drink = true;
            }
            if (i == SEED_CORN || i == SEED_CARROT) && btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 2. * u, 92. * u, 26. * u), if g.seed_sel as usize == i - SEED_CORN { "ปลูกอยู่" } else { "เลือกปลูก" }, g.seed_sel as usize != i - SEED_CORN, u) {
                seed_pick = Some((i - SEED_CORN) as u8);
            }
            if (i == CORN || i == CARROT) && btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 2. * u, 92. * u, 26. * u), "กิน", g.p.inv[i] > 0, u) {
                crop_eat = Some(i);
            }
            if matches!(i, BERRY | FISH | MEAT | SHROOM | HERB) && btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 2. * u, 92. * u, 26. * u), if i == HERB { "ใช้" } else if matches!(i, FISH | MEAT | SHROOM) { "กิน (ดิบ)" } else { "กิน" }, g.p.inv[i] > 0, u) {
                raw_eat = Some(i);
            }
            if (DISH_STEW..=DISH_FISH).contains(&i) && btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 2. * u, 92. * u, 26. * u), "กิน", g.p.inv[i] > 0, u) {
                dish = Some(i);
            }
        }
        y += rh2;
    }
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
        let kind = g.shop_camp.map_or(0, |k| g.w.camp_kind(k));
        text(font, camp_title(kind), c.x + 70. * u, y + 22. * u, 15. * u, hx(0xffcc66));
        text(font, if kind > 0 && ui.greet % 2 == 0 { camp_greet(kind) } else { GREETS[ui.greet % GREETS.len()] }, c.x + 70. * u, y + 42. * u, 12. * u, hx(0xdddddd));
        icon_c(sp, COIN, c.x + 70. * u, y + 50. * u, 14. * u, WHITE);
        text(font, &format!("เหรียญของคุณ: {}", g.p.inv[COIN]), c.x + 88. * u, y + 62. * u, 12. * u, hx(0xffd84a));
    }
    y += 76. * u;
    let rh = 58. * u;
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
    header(font, c, y, "สินค้า", u);
    y += 28. * u;
    let mut buy: Option<usize> = None;
    let offers = camp_offers(g.shop_camp.map_or(0, |k| g.w.camp_kind(k)));
    for (i, of) in offers.iter().enumerate() {
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
        g.buy_offer(offers[i]);
    }
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
    let rh = 62. * u;
    header(font, c, y, "หม้อทำอาหาร — วัตถุดิบดรอปจากมอนสเตอร์ (ค้างคาว=เนื้อ · สไลม์=เห็ด · โครงกระดูก=สมุนไพร)", u);
    y += 28. * u;
    let mut act: Option<usize> = None;
    for (i, d) in DISHES.iter().enumerate() {
        if visible(y, rh, c) {
            icon_c(sp, d.item, c.x + 8. * u, y + 10. * u, 28. * u, WHITE);
            text(font, d.name, c.x + 44. * u, y + 18. * u, 14. * u, hx(0xffcc66));
            text(font, d.desc, c.x + 44. * u, y + 33. * u, 11. * u, hx(0xaaaaaa));
            costs(g, sp, font, d.cost, None, c.x + 44. * u, y + 40. * u, u);
            if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 14. * u, 92. * u, 32. * u), "ทำ", can_pay(&g.p, d.cost), u) { act = Some(i); }
            sep(c, y + rh - 2.);
        }
        y += rh;
    }
    if visible(y, 60. * u, c) {
        text(font, "ใช้อาหารจากแท็บ \"ชุด/ของ\" หรือกด 1 สเต็ก · 2 ซุป · 3 ชา", c.x + 8. * u, y + 22. * u, 11. * u, hx(0x888899));
    }
    y += 40. * u;
    if let Some(i) = act { g.cook(i); }
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
    if let Some(i) = act { g.spend_stat(i); }
    y - top
}

fn tab_settings(g: &mut Game, _sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    let rh = 50. * u;
    let mut flip = 0;
    let rows: [(&str, &str, bool); 3] = [
        ("เสียงเอฟเฟกต์", "ดาบ ธนู โดนตี เก็บของ ฯลฯ", g.sfx_on),
        ("เพลงประกอบ", "เปลี่ยนตามฉาก: สำรวจ · แคมป์ · บอส · ดันเจี้ยน/กลางคืน", g.bgm_on),
        ("เล็งอัตโนมัติ", "แตะโจมตี = ยิง/ฟันมอนสเตอร์ใกล้สุด (ลากเล็งเองได้เหมือนเดิม)", g.auto_aim),
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
    let grows: [(&str, &str, String); 7] = [
        ("เฟรมเรต", "30 ประหยัดแบต · 60 ลื่น · ไม่จำกัด = ตามรีเฟรชเรตจอ", prefs::fps_name().to_string()),
        ("เอฟเฟกต์ฝน/หิมะ", "ปิดแล้วเครื่องแรงน้อยจะลื่นขึ้น (ผลช้าลง/มืดยังอยู่)", if prefs::weather_fx() { "เปิด" } else { "ปิด" }.to_string()),
        ("แสงไดนามิก", "สูง = ละเอียด · ต่ำ = เร็วขึ้น · ปิด = มืดเรียบ", prefs::light_name().to_string()),
        ("ขนาดปุ่มทัช", "ปรับให้ใหญ่/เล็กตามมือ", prefs::touch_name().to_string()),
        ("กล้องสั่น", "ตอนโดนตี/ระเบิด", if prefs::shake() { "เปิด" } else { "ปิด" }.to_string()),
        ("แสดง FPS", "มุมซ้ายล่าง", if prefs::show_fps() { "เปิด" } else { "ปิด" }.to_string()),
        ("เอาตัวรอด", "หิว/กระหาย/เหนื่อย · ปกติ = ไม่ตายเพราะหิว · ยาก = หมดสติได้ · ปิด = ไม่มี", prefs::survival_name().to_string()),
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
        0 => prefs::cycle_fps(),
        1 => prefs::toggle(&prefs::WEATHER_FX),
        2 => prefs::cycle_light(),
        3 => prefs::cycle_touch(),
        4 => prefs::toggle(&prefs::SHAKE),
        5 => prefs::toggle(&prefs::SHOW_FPS),
        6 => prefs::cycle_survival(),
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
        text(font, "คีย์บอร์ด: Shift/K = พุ่ง · P = สเตตัส · O = ตั้งค่า", c.x + 8. * u, y + 24. * u, 11. * u, hx(0x888899));
    }
    y += 30. * u;
    match flip {
        1 => { g.sfx_on = !g.sfx_on; SFX_ON.store(g.sfx_on, std::sync::atomic::Ordering::Relaxed); g.save(); }
        2 => { g.bgm_on = !g.bgm_on; g.save(); }
        3 => { g.auto_aim = !g.auto_aim; g.save(); }
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
    let mut tabs: Vec<(usize, &str)> = vec![(0, "คราฟต์"), (1, "สร้าง"), (2, "ชุด/ของ")];
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
    if !g.npcs.is_empty() || ui.npc.is_some() {
        tabs.push((10, "ชุมชน"));
    }
    tabs.push((7, "ตั้งค่า"));
    if !tabs.iter().any(|t| t.0 == ui.tab) {
        ui.tab = 0;
    }
    ui.clip = Rect::new(0., 0., sw, sh);
    let tw = (w - 16. * u) / tabs.len() as f32;
    let tfs = if tabs.len() > 5 { 12. } else { 15. };
    for (i, &(id, name)) in tabs.iter().enumerate() {
        let r = Rect::new(x + 8. * u + i as f32 * tw, y + 10. * u, tw - 4. * u, tabs_h);
        let on = ui.tab == id;
        draw_rectangle(r.x, r.y, r.w, r.h, if on { hx(0xffcc66) } else { hx(0x2e3144) });
        draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., if on { BLACK } else { hx(0x555a70) });
        let label = &thai(name);
        let fs = (tfs * u) as u16;
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
        5 => tab_forge(g, sp, font, ui, c, u),
        8 => tab_cook(g, sp, font, ui, c, u),
        9 => tab_adv(g, sp, font, ui, c, u),
        6 => tab_stats(g, sp, font, ui, c, u),
        7 => tab_settings(g, sp, font, ui, c, u),
        10 => tab_town(g, sp, font, ui, c, u),
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

// ======================= แท็บ "ชุมชน": ภาพรวมเมือง + บทสนทนา NPC =======================
fn bar(x: f32, y: f32, w: f32, h: f32, v: f32, col: Color) {
    draw_rectangle(x, y, w, h, hx(0x2a2d3f));
    draw_rectangle(x, y, w * v.clamp(0., 1.), h, col);
}
fn mood_col(m: i32) -> Color { if m >= 60 { hx(0x66dd66) } else if m >= 30 { hx(0xeedd44) } else { hx(0xee5544) } }

/// ตัดบรรทัดตามความกว้างจริง (ภาษาไทยไม่มีเว้นวรรคระหว่างคำ จึงตัดทีละตัวอักษร แต่ไม่ตัดก่อนสระ/วรรณยุกต์)
fn wrap_th(font: &Font, s: &str, fs: f32, maxw: f32) -> Vec<String> {
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
    let tier = g.tier();
    header(font, c, y, &format!("ระดับ: {} · ชาวบ้าน {} คน", TIER_N[tier as usize], g.pop()), u);
    y += 28. * u;
    let rh = 24. * u;
    let lines = [
        format!("เตียงว่าง {} / ทั้งหมด {} · อาหารในกล่องเก็บของ {} ชิ้น", g.free_beds().len(), g.beds().len(), g.food_stock()),
        match tier {
            0 => "ช่วยผู้หลงทางในโลก (เครื่องหมาย ! สีเหลือง) แล้วชวนมาอยู่ที่ฐาน — ต้องมีเตียงว่าง".to_string(),
            1 => "ค่าย (1-3 คน): ชาวไร่รดน้ำ/เก็บเกี่ยวให้ทุกเช้า · ครบ 4 คนเป็นหมู่บ้าน".to_string(),
            2 => format!("หมู่บ้าน: มีพ่อค้าเฉพาะทาง+ทหารยาม · อาณาจักรต้อง >10 คน + ปราสาท({}) + กำแพงเมือง {}/12", if g.has_castle() { "มีแล้ว" } else { "ยังไม่มี" }, g.walls()),
            _ => "อาณาจักร: เก็บภาษีได้ทุกเช้า".to_string(),
        },
        "ชาวบ้านอยู่ต่อเมื่อมีเตียงและอาหารในกล่องเก็บของใกล้เตียง (รัศมี 16 ช่อง) — ถ้าหิว/ไม่มีเตียงความสุขจะลด แล้วย้ายออก".to_string(),
    ];
    for l in &lines {
        let ws = wrap_th(font, l, 11. * u, c.w - 24. * u);
        let hh = (ws.len() as f32 * 14. * u + 10. * u).max(rh);
        if visible(y, hh, c) {
            for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, y + 16. * u + k as f32 * 14. * u, 11. * u, hx(0xbbbbcc)); }
        }
        y += hh;
    }
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
        for (d, t) in g.news.iter().rev().take(8) {
            let ws = wrap_th(font, &format!("วันที่ {}: {}", d + 1, t), 11. * u, c.w - 24. * u);
            let hh = ws.len() as f32 * 14. * u + 6. * u;
            if visible(y, hh, c) { for (k, w) in ws.iter().enumerate() { text(font, w, c.x + 8. * u, y + 13. * u + k as f32 * 14. * u, 11. * u, hx(0xc9d1e6)); } }
            y += hh;
        }
    }
    header(font, c, y, "ผู้คน", u);
    y += 28. * u;
    let rows: Vec<(u32, String, u8, i32, u8, bool, bool, u8)> = g.npcs.iter().map(|n| (n.id, NPC_NAMES[n.name as usize].to_string(), n.state, n.mood, n.hearts, n.married, n.child, n.role)).collect();
    let mut open: Option<u32> = None;
    let rh2 = 46. * u;
    for (id, nm, st, mood, hearts, married, child, role) in &rows {
        if visible(y, rh2, c) {
            let status = match *st { ST_FOLLOW => "ติดตามคุณ".to_string(), _ => if *child { "ลูก".to_string() } else { ROLE_N[*role as usize].to_string() } };
            text(font, &format!("{}{} · {}", nm, if *married { " (คู่สมรส)" } else { "" }, status), c.x + 8. * u, y + 18. * u, 14. * u, hx(0xffcc66));
            text(font, &format!("สนิท {}/10", hearts), c.x + 8. * u, y + 36. * u, 11. * u, hx(0xff9ab0));
            bar(c.x + 100. * u, y + 28. * u, 90. * u, 8. * u, *mood as f32 / 100., mood_col(*mood));
            text(font, "สุข", c.x + 196. * u, y + 36. * u, 11. * u, hx(0xaaaaaa));
            if btn(ui, font, Rect::new(c.x + c.w - 104. * u, y + 7. * u, 92. * u, 32. * u), "คุย/จัดการ", !*child, u) { open = Some(*id); }
            sep(c, y + rh2 - 2.);
        }
        y += rh2;
    }
    if rows.is_empty() && visible(y, 30. * u, c) {
        text(font, "ยังไม่มีใคร — ออกสำรวจแล้วช่วยผู้หลงทาง!", c.x + 8. * u, y + 20. * u, 13. * u, hx(0x888899));
        y += 30. * u;
    }
    if tax { g.collect_tax(); }
    if let Some(id) = open { ui.npc = Some(id); ui.scroll = 0.; }
    y - top
}

fn tab_npc_dialog(g: &mut Game, _sp: &Sprites, font: &Font, ui: &mut UiState, c: Rect, u: f32, id: u32) -> f32 {
    let top = c.y - ui.scroll;
    let mut y = top;
    let Some(n) = g.npcs.iter().find(|n| n.id == id).cloned() else { ui.npc = None; return 0.; };
    let nm = NPC_NAMES[n.name as usize];
    header(font, c, y, &format!("{} — {}{}", nm, if n.state == ST_FOLLOW { "ผู้ติดตาม" } else { ROLE_N[n.role as usize] }, if n.married { " · คู่สมรส" } else { "" }), u);
    y += 30. * u;
    let say = g.npc_say(&n);
    if visible(y, 30. * u, c) { text(font, &format!("\"{}\"", say), c.x + 8. * u, y + 18. * u, 13. * u, hx(0xdddddd)); }
    y += 30. * u;
    let op = opinion(&n);
    if visible(y, 40. * u, c) {
        text(font, &format!("สนิท {}/10", n.hearts), c.x + 8. * u, y + 14. * u, 12. * u, hx(0xff9ab0));
        bar(c.x + 8. * u, y + 20. * u, 160. * u, 9. * u, n.hearts as f32 / 10., hx(0xff6688));
        text(font, &format!("ความสุข {}", n.mood), c.x + 190. * u, y + 14. * u, 12. * u, hx(0xaaaaaa));
        bar(c.x + 190. * u, y + 20. * u, 160. * u, 9. * u, n.mood as f32 / 100., mood_col(n.mood));
    }
    y += 40. * u;
    // นิสัย/ของโปรด: ยิ่งสนิทยิ่งรู้จักมากขึ้น (เหมือนรู้จักคนจริงๆ)
    {
        let known = (n.hearts as usize / 2 + 1).min(5);
        let trs: Vec<&str> = n.soc.traits_ranked().into_iter().take(known).map(|(k, hi)| Soc::trait_name(k, hi)).collect();
        let body = g.npc_body(&n);
        let mut info = vec![format!("รูปร่าง: {} · {}", body.describe(), body.look_words())];
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
    // ---- คำขอ/ปัญหาที่เกี่ยวกับคนนี้
    let mut qdo: Option<(usize, QA)> = None;
    let rq = g.reqs_for(id);
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
