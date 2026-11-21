// v18 — อัปเกรดภาพ
// 1) แสง: shader คูณสีทั้งจอ (โทนสีตามเวลา/อากาศ + ความมืด) แล้ว "เจาะ" วงแสงจากไฟ/โคม/หน้าต่าง เป็นขั้นๆ แบบพิกเซล
//    + ชั้นบวกแสงเรืองรอบไฟ (additive) · ไม่ต้องใช้ render target · ถ้า shader ใช้ไม่ได้ ตกไปใช้ระบบเดิม
// 2) รอยต่อพื้น: สไปรต์ขอบหยักของพื้นที่ "สูงกว่า" ทับขอบพื้นข้างๆ + ฟองคลื่นริมฝั่ง 2 จังหวะ
// 3) ของที่ขยับ: ต้นไม้ไหวตามลม หญ้า ประกายน้ำ หิ่งห้อย ใบไม้/กลีบดอกปลิว
// 5) เงาเอียงตามดวงอาทิตย์ + ขอบเข้มรอบตัวละคร
// 6) จังหวะเดิน 4 จังหวะ (ก้าว-ผ่าน-ก้าว-ผ่าน) ตัวยกขึ้นตอนผ่าน
use super::*;
use std::cell::{Cell, RefCell};
use macroquad::window::miniquad::{BlendFactor, BlendState, BlendValue, Equation, PipelineParams, UniformDesc, UniformType};
fn hf(x: i32, y: i32, s: i32) -> f32 { hash(x, y, s) as f32 }

// ---------------------------------------------------------------- ค่าประจำเฟรม
#[derive(Clone, Copy, Default)]
pub struct Frame { pub sun_dx: f32, pub sun_a: f32, pub wind: f32, pub t: f32, pub outline: bool }
thread_local! {
    static FR: Cell<Frame> = Cell::new(Frame { sun_dx: 0., sun_a: 0.22, wind: 1., t: 0., outline: true });
    static MATS: RefCell<Option<Option<(Material, Material)>>> = RefCell::new(None);
}
pub fn fr() -> Frame { FR.with(|f| f.get()) }

/// เรียกต้นเฟรม: ทิศเงา ความแรงลม
pub fn set_frame(g: &Game, t_now: f32) {
    let h = g.hour();
    let day = (6.0..18.5).contains(&h);
    // เช้าแดดจากตะวันออก เงาทอดไปทางตะวันตก (ซ้าย) · เย็นกลับกัน · เที่ยงสั้น
    let (sun_dx, sun_a) = if g.w.dun.is_some() { (0., 0.25) } else if day { (((h - 12.) / 6.).clamp(-1.2, 1.2) * 0.9, 0.24 * (1. - g.rain * 0.5)) } else { (0.15, 0.12) };
    let wind = 1. + g.rain * 1.6 + if season(g) == 2 { 0.5 } else { 0. };
    FR.with(|f| f.set(Frame { sun_dx, sun_a, wind, t: t_now, outline: prefs::light_q() > 0 || true }));
}

// ---------------------------------------------------------------- เงา
/// วงรีแบบพิกเซล (เป็นแถวสี่เหลี่ยม)
pub fn pix_ellipse(v: &View, cx: f32, cy: f32, rw: f32, rh: f32, c: Color) {
    let rh = rh.max(1.);
    let n = rh.ceil() as i32;
    for j in -n..n {
        let yy = (j as f32 + 0.5) / rh;
        let hw = (rw * (1. - yy * yy).max(0.).sqrt()).round();
        if hw >= 1. { prect(v, (cx - hw).round(), (cy + j as f32).round(), hw * 2., 1., c); }
    }
}
/// เงาใต้ตัวละคร เลื่อนตามแดด
pub fn char_shadow(v: &View, sx: f32, sy: f32, w: f32) {
    let f = fr();
    pix_ellipse(v, sx + (f.sun_dx * 3.).round(), sy, w / 2. + f.sun_dx.abs() * 1.5, 1.5, Color::new(0., 0., 0.06, f.sun_a + 0.03));
}
/// เงาของสิ่งสูง (ต้นไม้ บ้าน หิน) ที่ฐาน ทอดไปตามแดด
pub fn obj_shadow(v: &View, px: f32, py: f32, r: Rect, o: u8) {
    if !casts_shadow(o) { return; }
    let f = fr();
    let h = r.h.max(16.);
    let cx = px + 8. + f.sun_dx * h * 0.35;
    let rw = (r.w * 0.42).max(5.) + f.sun_dx.abs() * h * 0.22;
    pix_ellipse(v, cx, py + 15., rw, if is_tree(o) { 3. } else { 2.5 }, Color::new(0., 0.02, 0.08, f.sun_a));
}
fn casts_shadow(o: u8) -> bool {
    is_tree(o) || matches!(o, 2 | 4 | 5 | 6 | 7 | 14 | 15 | 16 | 37 | O_STALL | O_STABLE | O_COACH | O_PILLAR | O_GIANT | O_HUT_FISH | O_HUT_SAND | O_HUT_LOG | O_WOODPILE | O_CRYSTAL)
}

// ---------------------------------------------------------------- ต้นไม้ไหว
/// วาดสไปรต์เป็นแถบแนวนอน ยอดเอนมากกว่าโคน (ลำต้นนิ่ง)
pub fn blit_sway(sp: &Sprites, v: &View, r: Rect, x: f32, y: f32, c: Color, seed: f32) {
    let f = fr();
    let ph = f.t * (1.1 + f.wind * 0.35) + seed;
    let a = (ph.sin() * 0.7 + (ph * 2.3).sin() * 0.3) * (0.8 + f.wind * 0.6);
    let band = 4.;
    let n = (r.h / band).ceil() as i32;
    for i in 0..n {
        let y0 = i as f32 * band;
        let hh = band.min(r.h - y0);
        // สัดส่วนความสูงจากโคน: ยอด = 1 โคน = 0 · ล่างสุด 6px ไม่ขยับ
        let up = ((r.h - y0 - 6.) / (r.h - 6.).max(1.)).clamp(0., 1.);
        let ox = (a * up * up * 1.6).round();
        let src = Rect::new(r.x, r.y + y0, r.w, hh);
        draw_texture_ex(&sp.tex, (x + ox) * v.s, (y + y0) * v.s, c, DrawTextureParams { source: Some(src), dest_size: Some(vec2(r.w * v.s, hh * v.s)), ..Default::default() });
    }
}

// ---------------------------------------------------------------- เดิน 4 จังหวะ
/// (เฟรมขา, ยกตัว) — ก้าวซ้าย · ผ่าน(ยก) · ก้าวขวา · ผ่าน(ยก)
pub fn walk_cycle(anim: f32, rate: f32) -> (usize, f32) {
    match (anim * rate) as usize % 4 { 0 => (1, 0.), 1 => (0, 1.), 2 => (2, 0.), _ => (0, 1.) }
}

// ---------------------------------------------------------------- ขอบตัวละคร
pub const OUTLINE_C: Color = Color { r: 0.07, g: 0.05, b: 0.1, a: 0.72 };
/// วาดเงาดำ 4 ทิศ 1 พิกเซลรอบสไปรต์ (ใช้ก่อนวาดตัวจริง)
pub fn blit_outline(sp: &Sprites, v: &View, r: Rect, x: f32, y: f32, w: f32, h: f32, flip: bool, a: f32) {
    if !fr().outline { return; }
    let c = Color::new(OUTLINE_C.r, OUTLINE_C.g, OUTLINE_C.b, OUTLINE_C.a * a);
    for (dx, dy) in [(-1., 0.), (1., 0.), (0., -1.), (0., 1.)] {
        draw_texture_ex(&sp.tex, (x + dx) * v.s, (y + dy) * v.s, c, DrawTextureParams { source: Some(r), dest_size: Some(vec2(w * v.s, h * v.s)), flip_x: flip, ..Default::default() });
    }
}

// ---------------------------------------------------------------- รอยต่อพื้น
/// ลำดับชั้นพื้น: สูงกว่าทับขอบของต่ำกว่า · -1 = ไม่ทำขอบ (พื้นที่สร้างเอง ภูเขา ดันเจี้ยน ลาวา)
pub fn tile_pri(t: u8) -> i32 {
    match t { 0 => 0, 1 => 1, 2 => 3, 8 => 4, 15 => 5, 17 => 6, 16 => 7, 5 => 8, 3 => 9, 4 => 10, 7 => 11, _ => -1 }
}
pub struct Edges { pub e: Vec<Option<[Rect; 16]>>, pub foam: [[Rect; 16]; 2] }
impl Default for Edges { fn default() -> Self { Edges { e: vec![], foam: [[Rect::new(0., 0., 0., 0.); 16]; 2] } } }

/// โปรไฟล์ความลึกของขอบหยัก (ปลายทั้งสองข้าง = 3 เพื่อให้ต่อกับช่องข้างๆ ได้)
const PROFS: [[i32; 16]; 2] = [[3, 3, 4, 4, 5, 4, 3, 3, 4, 5, 5, 4, 3, 2, 3, 3], [3, 2, 2, 3, 4, 4, 5, 4, 3, 3, 2, 3, 4, 4, 3, 3]];
/// ช่อง (x,y) อยู่ในส่วนทับไหม — 0..4 ขอบ N E S W · 4..8 มุม NE SE SW NW
fn in_mask(k: usize, x: i32, y: i32, grow: i32) -> bool {
    let prof = &PROFS[k / 8];
    let k = k % 8;
    match k {
        0 => y < prof[x as usize] + grow,
        1 => 15 - x < prof[y as usize] + grow,
        2 => 15 - y < prof[15 - x as usize] + grow,
        3 => x < prof[15 - y as usize] + grow,
        _ => {
            let (cx, cy) = match k { 4 => (16., 0.), 5 => (16., 16.), 6 => (0., 16.), _ => (0., 0.) };
            let r = 4.6 + grow as f32;
            (x as f32 + 0.5 - cx).powi(2) + (y as f32 + 0.5 - cy).powi(2) < r * r
        }
    }
}
pub fn build_edges(at: &mut Atlas, tiles: &[Vec<Rect>]) -> Edges {
    let mut e = vec![None; tiles.len()];
    for (t, tr) in tiles.iter().enumerate() {
        if tile_pri(t as u8) < 0 || tr.is_empty() { continue; }
        let r = tr[0];
        let src: Vec<Color> = (0..256).map(|i| at.img.get_pixel(r.x as u32 + (i % 16) as u32, r.y as u32 + (i / 16) as u32)).collect();
        let mut out = [Rect::new(0., 0., 0., 0.); 16];
        for k in 0..16 {
            let mut im = Image::gen_image_color(16, 16, BLANK);
            for y in 0..16 {
                for x in 0..16 {
                    if !in_mask(k, x, y, 0) { continue; }
                    let mut c = src[(y * 16 + x) as usize];
                    // ขอบนอกสุดเข้มลงนิด ให้ดูมีความหนา
                    let rim = [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|&(dx, dy)| { let (a, b) = (x + dx, y + dy); (0..16).contains(&a) && (0..16).contains(&b) && !in_mask(k, a, b, 0) });
                    if rim { c = Color::new(c.r * 0.82, c.g * 0.82, c.b * 0.86, 1.); }
                    im.set_pixel(x as u32, y as u32, c);
                }
            }
            out[k] = at.add(&im);
        }
        e[t] = Some(out);
    }
    // ฟองคลื่น: แถบขาวนอกขอบฝั่ง 2 จังหวะ (ชิดฝั่ง / ห่างออกมา)
    let mut foam = [[Rect::new(0., 0., 0., 0.); 16]; 2];
    for f in 0..2 {
        for k in 0..16 {
            let mut im = Image::gen_image_color(16, 16, BLANK);
            for y in 0..16 {
                for x in 0..16 {
                    let a = if !in_mask(k, x, y, 0) && in_mask(k, x, y, 1 + f as i32) {
                        if f == 0 { 0.85 } else if in_mask(k, x, y, 1) { 0. } else { 0.55 }
                    } else { 0. };
                    if a > 0. && hf(x, y, (k * 7 + f) as i32) < 0.9 { im.set_pixel(x as u32, y as u32, Color::new(0.9, 0.96, 1., a)); }
                }
            }
            foam[f][k] = at.add(&im);
        }
    }
    Edges { e, foam }
}
const N8: [(i32, i32); 8] = [(0, -1), (1, 0), (0, 1), (-1, 0), (1, -1), (1, 1), (-1, 1), (-1, -1)];
/// วาดขอบที่ทับช่องนี้ · คืน bit ด้าน (N E S W) ที่เป็นฝั่ง (วาดฟองแล้ว)
pub fn draw_edges(w: &World, sp: &Sprites, v: &View, tx: i32, ty: i32, t: u8, px: f32, py: f32, ssn: u8, wf: usize) -> u8 {
    let p = tile_pri(t);
    if p < 0 || sp.edges.e.is_empty() { return 0; }
    let nb: [u8; 8] = std::array::from_fn(|i| w.tile_at(tx + N8[i].0, ty + N8[i].1));
    let mut done = 0u8;
    let mut kinds: Vec<u8> = nb.iter().copied().filter(|&n| tile_pri(n) > p).collect();
    if kinds.is_empty() { return 0; }
    kinds.sort_by_key(|&n| tile_pri(n));
    kinds.dedup();
    let water = t < 2;
    for u in kinds {
        let Some(rs) = sp.edges.e[u as usize] else { continue };
        let tint = ground_tint(ssn, u);
        let shore = water && u >= 2;
        // ขอบหยัก 2 แบบสลับตามช่อง ไม่ให้ลายซ้ำทุก 16 พิกเซล
        let var = if hf(tx, ty, 71) < 0.5 { 0 } else { 8 };
        let mut draw = |k: usize| {
            let k = if k < 4 { k + var } else { k };
            blit(sp, v, rs[k], px, py, false, tint);
            if shore { blit(sp, v, sp.edges.foam[wf][k], px, py, false, WHITE); }
        };
        for k in 0..4 { if nb[k] == u { draw(k); if shore { done |= 1 << k; } } }
        // มุม: เฉพาะเมื่อสองด้านข้างไม่ใช่ชนิดเดียวกัน
        for (k, (a, b)) in [(4, (0, 1)), (5, (1, 2)), (6, (2, 3)), (7, (3, 0))] { if nb[k] == u && nb[a] != u && nb[b] != u { draw(k); } }
    }
    done
}

// ---------------------------------------------------------------- ประกายน้ำ หญ้า
pub fn tile_detail(v: &View, tx: i32, ty: i32, t: u8, px: f32, py: f32, ssn: u8, seed: i32) {
    let f = fr();
    if t < 2 {
        // ประกายแดดบนน้ำ: จุดสว่างโผล่ทีละนิด
        let h = hf(tx, ty, seed.wrapping_add(41));
        if h < 0.35 {
            let ph = (f.t * 0.7 + h * 13.).fract();
            if ph < 0.25 {
                let a = (1. - (ph / 0.25 - 0.5).abs() * 2.) * 0.85;
                let (x, y) = ((hf(tx, ty, 77) * 13.).floor() + 1., (hf(ty, tx, 78) * 13.).floor() + 1.);
                prect(v, px + x, py + y, 2., 1., Color::new(1., 1., 1., a));
                prect(v, px + x + 0.5, py + y - 1., 1., 3., Color::new(1., 1., 1., a * 0.5));
            }
        }
    } else if (t == 3 || t == 16) && ssn != 3 {
        // ใบหญ้าไหวตามลม 0–2 กอต่อช่อง
        let n = (hf(tx, ty, seed.wrapping_add(43)) * 2.6) as i32;
        let gc = if t == 16 { Color::new(0.72, 0.62, 0.3, 1.) } else if ssn == 2 { Color::new(0.62, 0.55, 0.22, 1.) } else { Color::new(0.26, 0.55, 0.2, 1.) };
        let hc = if t == 16 { Color::new(0.85, 0.76, 0.42, 1.) } else if ssn == 2 { Color::new(0.75, 0.66, 0.3, 1.) } else { Color::new(0.45, 0.75, 0.32, 1.) };
        for i in 0..n {
            let (x, y) = ((hf(tx * 3 + i, ty, 51) * 12.).floor() + 2., (hf(tx, ty * 3 + i, 52) * 10.).floor() + 5.);
            let sw = ((f.t * (1.3 + f.wind * 0.4) + tx as f32 * 0.6 + ty as f32 * 0.3).sin() * f.wind * 0.8).round();
            prect(v, px + x, py + y, 1., 2., gc);
            prect(v, px + x - 1. + sw.min(0.), py + y - 2., 1., 2., hc);
            prect(v, px + x + 1. + sw.max(0.), py + y - 2., 1., 2., hc);
            prect(v, px + x + sw, py + y - 3., 1., 2., hc);
        }
    }
}

// ---------------------------------------------------------------- อนุภาคบรรยากาศ
/// หิ่งห้อย (กลางคืน ฤดูร้อน/ใบไม้ผลิ บนหญ้า/ป่า/หนอง) · ใบไม้ร่วง · กลีบดอกไม้ผลิ — ตำแหน่งคำนวณจากเวลา ไม่เก็บสถานะ
pub fn draw_ambient(g: &Game, v: &View, lights: &mut Vec<(f32, f32, f32, u8)>) {
    if g.w.dun.is_some() || g.indoors() || !prefs::weather_fx() { return; }
    let f = fr();
    let ssn = season(g);
    // หิ่งห้อย
    if g.darkv > 0.35 && ssn <= 1 && g.rain < 0.3 {
        let cell = 48.;
        let (cx0, cy0) = ((v.cx / cell).floor() as i32, (v.cy / cell).floor() as i32);
        for cy in cy0..=cy0 + (v.vh / cell) as i32 + 1 {
            for cx in cx0..=cx0 + (v.vw / cell) as i32 + 1 {
                let h = hf(cx, cy, 991);
                if h > 0.45 { continue; }
                let (bx, by) = (cx as f32 * cell + hf(cx, cy, 992) * cell, cy as f32 * cell + hf(cx, cy, 993) * cell);
                let t = g.w.tile_at(tfloor(bx), tfloor(by));
                if !matches!(t, 3 | 4 | 15) { continue; }
                let ph = f.t * 0.5 + h * 40.;
                let (x, y) = (bx + (ph * 0.9).sin() * 14. + (ph * 2.1).cos() * 4., by + (ph * 0.7).cos() * 10. - (ph * 1.7).sin().abs() * 4.);
                let blink = ((ph * 1.9).sin() * 0.5 + 0.5).powf(2.);
                if blink < 0.08 { continue; }
                let (sx, sy) = ((x - v.cx).round(), (y - v.cy).round());
                prect(v, sx, sy, 1., 1., Color::new(0.85, 1., 0.45, blink));
                prect(v, sx - 1., sy, 3., 1., Color::new(0.85, 1., 0.45, blink * 0.25));
                if lights.len() < 60 && blink > 0.5 { lights.push((sx, sy, 4. + blink * 3., 5)); }
            }
        }
    }
    // ใบไม้ร่วง (ใบไม้ร่วง) / กลีบดอก (ใบไม้ผลิ) ลอยลงเฉียงตามลม
    if (ssn == 2 || ssn == 0) && g.darkv < 0.6 {
        let n = if ssn == 2 { 22 } else { 12 };
        for i in 0..n {
            let sd = i as f32 * 17.31;
            let life = 9. + (i % 5) as f32;
            let ph = ((f.t + sd) / life).fract();
            let x0 = (hf(i, (f.t + sd) as i32 / life as i32, 5) as f32) * (v.vw + 80.) - 40.;
            let x = x0 + ph * 60. * f.wind + (ph * 12. + sd).sin() * 8.;
            let y = ph * (v.vh + 30.) - 15.;
            let flip = (ph * 20. + sd).sin();
            let c = if ssn == 2 { [Color::new(0.85, 0.42, 0.12, 0.9), Color::new(0.9, 0.62, 0.15, 0.9), Color::new(0.65, 0.28, 0.1, 0.9)][i as usize % 3] } else { Color::new(1., 0.78, 0.86, 0.85) };
            prect(v, x.round(), y.round(), if flip.abs() > 0.4 { 2. } else { 1. }, 1., c);
            if ssn == 2 { prect(v, x.round() + if flip > 0. { 1. } else { 0. }, y.round() + 1., 1., 1., Color::new(c.r * 0.8, c.g * 0.8, c.b, c.a)); }
        }
    }
}

// ---------------------------------------------------------------- แสง + โทนสี
const VERT: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying vec2 pos;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    pos = position.xy;
}"#;
const FRAG_MUL: &str = r#"#version 100
precision mediump float;
varying vec2 pos;
uniform vec4 lights[40];
uniform vec4 amb;
uniform vec4 info;
vec3 lc(float k) {
    if (k < 0.5) return vec3(0.78, 0.86, 1.0);
    if (k < 1.5) return vec3(1.0, 0.70, 0.40);
    if (k < 2.5) return vec3(1.0, 0.86, 0.56);
    if (k < 3.5) return vec3(0.80, 0.66, 1.0);
    if (k < 4.5) return vec3(1.0, 0.48, 0.22);
    return vec3(0.80, 1.0, 0.50);
}
void main() {
    vec2 p = floor(pos / info.x) * info.x + info.x * 0.5;
    vec3 m = amb.rgb;
    for (int i = 0; i < 40; i++) {
        if (float(i) >= info.y) break;
        vec4 L = lights[i];
        float d = distance(p, L.xy);
        if (d < L.z) {
            float f = 1.0 - d / L.z;
            f = floor(f * info.w + 0.5) / info.w;
            f = min(f * 1.5, 1.0);
            m = max(m, mix(amb.rgb, lc(L.w), f));
        }
    }
    gl_FragColor = vec4(m, 1.0);
}"#;
const FRAG_ADD: &str = r#"#version 100
precision mediump float;
varying vec2 pos;
uniform vec4 lights[40];
uniform vec4 amb;
uniform vec4 info;
void main() {
    vec2 p = floor(pos / info.x) * info.x + info.x * 0.5;
    vec3 g = vec3(0.0);
    for (int i = 0; i < 40; i++) {
        if (float(i) >= info.y) break;
        vec4 L = lights[i];
        if (L.w < 0.5 || L.w > 2.5) continue;
        float d = distance(p, L.xy);
        float r = L.z * 0.55;
        if (d < r) {
            float f = 1.0 - d / r;
            f = floor(f * 4.0 + 0.5) / 4.0;
            vec3 c = L.w < 1.5 ? vec3(1.0, 0.45, 0.12) : vec3(1.0, 0.7, 0.3);
            g += c * f * f * 0.16;
        }
    }
    gl_FragColor = vec4(g * amb.a, 1.0);
}"#;

fn mats() -> Option<(Material, Material)> {
    MATS.with(|m| {
        let mut m = m.borrow_mut();
        if m.is_none() {
            let mk = |frag: &str, blend: BlendState| {
                load_material(ShaderSource::Glsl { vertex: VERT, fragment: frag }, MaterialParams {
                    pipeline_params: PipelineParams { color_blend: Some(blend), ..Default::default() },
                    uniforms: vec![UniformDesc::new("lights", UniformType::Float4).array(40), UniformDesc::new("amb", UniformType::Float4), UniformDesc::new("info", UniformType::Float4)],
                    ..Default::default()
                })
            };
            let mul = mk(FRAG_MUL, BlendState::new(Equation::Add, BlendFactor::Value(BlendValue::DestinationColor), BlendFactor::Zero));
            let add = mk(FRAG_ADD, BlendState::new(Equation::Add, BlendFactor::One, BlendFactor::One));
            *m = Some(match (mul, add) {
                (Ok(a), Ok(b)) => Some((a, b)),
                (a, b) => { eprintln!("[vfx] shader ใช้ไม่ได้: {:?} {:?}", a.err(), b.err()); None }
            });
        }
        m.as_ref().unwrap().clone()
    })
}

/// โทนสีตามเวลา/อากาศ/ฤดู (ตัวคูณ) และระดับความมืด 0..1
pub fn grade(g: &Game) -> ([f32; 3], f32) {
    if g.w.dun.is_some() {
        let k = 1. - g.darkv;
        return ([0.16 + k * 0.84, 0.17 + k * 0.83, 0.26 + k * 0.74], g.darkv);
    }
    let h = g.hour();
    let ramp = |a: f32, b: f32, x: f32| ((x - a) / (b - a)).clamp(0., 1.);
    // กลางวันขาว · เช้าทอง (5–8) · เย็นส้มชมพู (16.5–19)
    let morning = (1. - ramp(6., 8.5, h)) * ramp(4.5, 6., h);
    let evening = ramp(16., 18., h) * (1. - ramp(18.6, 19.6, h));
    let mut c = [1.0f32, 1.0, 1.0];
    let mix = |c: &mut [f32; 3], t: [f32; 3], k: f32| { for i in 0..3 { c[i] += (t[i] - c[i]) * k; } };
    match season(g) { 3 => mix(&mut c, [0.9, 0.95, 1.0], 0.5), 2 => mix(&mut c, [1.0, 0.95, 0.86], 0.4), _ => {} }
    mix(&mut c, [0.78, 0.82, 0.88], g.rain * 0.8);
    // กลางคืนน้ำเงินเข้ม
    let night = (g.darkv / 0.82).clamp(0., 1.);
    mix(&mut c, [0.13, 0.16, 0.33], night * 0.97);
    // แสงเช้าสีทอง/แสงเย็นสีส้ม คูณทับ (ยังเห็นโทนอุ่นแม้ฟ้าเริ่มมืด)
    let warm = |c: &mut [f32; 3], t: [f32; 3], k: f32| { for i in 0..3 { c[i] *= 1. + (t[i] - 1.) * k; } };
    let lift = 1. + (morning * 0.25 + evening * 0.3) * night;
    for x in c.iter_mut() { *x = (*x * lift).min(1.); }
    warm(&mut c, [1.0, 0.86, 0.66], morning * 0.85 * (1. - g.rain * 0.6));
    warm(&mut c, [1.0, 0.72, 0.52], evening * 0.9 * (1. - g.rain * 0.6));
    (c, g.darkv)
}

/// วาดชั้นแสง+โทนสีทับโลก · false = ใช้ shader ไม่ได้ (ให้ใช้ระบบเดิม)
pub fn lighting(g: &Game, v: &View, lights: &mut Vec<(f32, f32, f32, u8)>) -> bool {
    let Some((mul, add)) = mats() else { return false };
    let (mut c, dark) = grade(g);
    // ปิดแสงไดนามิก: ไม่มีวงแสง จึงให้มืดน้อยลงพอมองเห็น
    if prefs::light_q() == 0 { for x in c.iter_mut() { *x = x.max(0.4); } }
    if dark < 0.02 && c.iter().all(|&x| x > 0.995) { return true; }
    let lq = prefs::light_q();
    let (cw, ch) = (v.vw / 2., v.vh / 2.);
    // ใกล้กลางจอก่อน ตัดเหลือ 40
    lights.retain(|l| l.0 > -l.2 && l.0 < v.vw + l.2 && l.1 > -l.2 && l.1 < v.vh + l.2);
    lights.sort_by(|a, b| ((a.0 - cw).abs() + (a.1 - ch).abs() - a.2).partial_cmp(&((b.0 - cw).abs() + (b.1 - ch).abs() - b.2)).unwrap());
    let n = if lq == 0 { 0 } else { lights.len().min(40) };
    let mut arr = [Vec4::ZERO; 40];
    for (i, l) in lights.iter().take(n).enumerate() { arr[i] = vec4(l.0 * v.s, l.1 * v.s, l.2 * v.s, l.3 as f32); }
    let cell = if lq >= 2 { 2. } else { 4. } * v.s;
    for m in [&mul, &add] {
        m.set_uniform_array("lights", &arr[..]);
        m.set_uniform("amb", vec4(c[0], c[1], c[2], dark));
        m.set_uniform("info", vec4(cell, n as f32, 0., 7.));
    }
    gl_use_material(&mul);
    draw_rectangle(0., 0., screen_width(), screen_height(), WHITE);
    if dark > 0.15 && n > 0 {
        gl_use_material(&add);
        draw_rectangle(0., 0., screen_width(), screen_height(), WHITE);
    }
    gl_use_default_material();
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn edge_masks_connect_and_priorities_are_consistent() {
        // ขอบหยักต่อกันที่มุมช่อง (ปลาย = 3 ทั้งสองด้าน)
        for p in PROFS { assert_eq!((p[0], p[15]), (3, 3)); }
        for k in [0, 1, 2, 3, 8, 9, 10, 11] { for i in 0..16 { let (x, y) = match k % 8 { 0 => (i, 0), 1 => (15, i), 2 => (i, 15), _ => (0, i) }; assert!(in_mask(k, x, y, 0), "ขอบติดเส้นแบ่งช่อง"); } }
        // ฟองอยู่นอกแผ่นดินเสมอ
        for k in 0..16 { for y in 0..16 { for x in 0..16 { if in_mask(k, x, y, 0) { assert!(in_mask(k, x, y, 1)); } } } }
        // น้ำต่ำสุด หิมะสูงสุด · พื้นสร้างเอง/ดันเจี้ยนไม่ทำขอบ
        assert!(tile_pri(0) < tile_pri(1) && tile_pri(1) < tile_pri(2) && tile_pri(2) < tile_pri(3) && tile_pri(3) < tile_pri(4));
        for t in [6u8, 9, 10, 11, 12, 13, 14, 18, 19, 20] { assert_eq!(tile_pri(t), -1); }
        let mut seen = std::collections::HashSet::new();
        for t in 0..21u8 { let p = tile_pri(t); if p >= 0 { assert!(seen.insert(p), "ลำดับซ้ำ {}", t); } }
    }
    #[test]
    fn walk_cycle_has_four_beats_with_bob_on_passing() {
        let beats: Vec<(usize, f32)> = (0..4).map(|i| walk_cycle(i as f32 / 10. + 0.01, 10.)).collect();
        assert_eq!(beats, vec![(1, 0.), (0, 1.), (2, 0.), (0, 1.)]);
    }
    #[test]
    fn grading_day_neutral_dusk_warm_night_blue() {
        let mut g = Game::new_world_seed(4242, Sfx::default());
        let at = |g: &mut Game, h: f32| { g.time = ((h - 6.).rem_euclid(24.)) / 24. * DAYLEN; g.rain = 0.; let hr = g.hour(); g.darkv = (((hr - 12.).abs() - 5.) / 2.).clamp(0., 1.) * 0.82; grade(g).0 };
        let noon = at(&mut g, 12.);
        assert!(noon.iter().all(|&x| x > 0.9), "เที่ยงเกือบขาว {:?}", noon);
        let dusk = at(&mut g, 17.5);
        assert!(dusk[0] > dusk[2] + 0.15, "เย็นอมส้ม {:?}", dusk);
        let night = at(&mut g, 23.);
        assert!(night[2] > night[0] && night[0] < 0.3, "กลางคืนน้ำเงินมืด {:?}", night);
    }
}
