//! v32 "หาได้จากไหน" — คู่มือที่มาของของทุกอย่าง (มีรูป + ทิศ/ระยะของจุดที่ใกล้ที่สุด)
//! ใช้ตอน: ชาวบ้านอยากได้ของที่เราไม่มี · วัสดุสร้าง/คราฟต์ไม่พอ · เควสต์พ่อค้า · เปิดดูของในกระเป๋า
//! ข้อมูลที่มาอ้างจากกติกาในเกมจริง (ปริมาณ/ราคาดึงจากตารางสูตร ร้านค้า ไม่เขียนตัวเลขซ้ำ)
use super::*;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

/// รูปที่ใช้โชว์ที่มา (วาดจาก sprite ในเกม)
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Pic { Obj(u8), Icon(usize), Tile(u8), Merchant, Slime, Skel, Bat, Golem, Animal(u8), Rod, Pick }

/// วิธีหาจุดที่ใกล้ที่สุดในโลก
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Find {
    /// ไม่มีจุดตายตัว (ซื้อ/คราฟต์/ทำอาหาร ฯลฯ)
    No,
    /// วัตถุเหล่านี้
    Obj(&'static [u8]),
    /// ต้นไม้ที่โค่นได้
    Tree,
    /// พื้นชนิดนี้
    Tile(u8),
    /// น้ำ (ตกปลาได้)
    Water,
    /// ศัตรูที่เดินอยู่ตอนนี้: 0 อะไรก็ได้ 1 สไลม์ 2 โครงกระดูก 3 ค้างคาว 4 มอนสเตอร์ราตรี
    Mob(u8),
    /// สัตว์ป่าที่ล่าเอาเนื้อได้
    Animal,
}

#[derive(Clone, Debug)]
pub struct Src {
    pub pic: Pic,
    /// ชื่อสั้นๆ สำหรับแถบเล็ก (เช่น "ต้นไม้")
    pub short: String,
    /// อธิบายวิธีได้มา
    pub how: String,
    pub find: Find,
}
fn s(pic: Pic, short: &str, how: &str, find: Find) -> Src { Src { pic, short: short.into(), how: how.into(), find } }

const SCAN_R: i32 = 24;
const TREES: Find = Find::Tree;
const MERCH: &[u8] = &[15];
const DUNG: &[u8] = &[19, 36];

fn cost_text(cost: &[(usize, i32)]) -> String {
    cost.iter().map(|&(it, n)| format!("{} {}", INAME[it], n)).collect::<Vec<_>>().join(" + ")
}
fn buy_src(it: usize) -> Vec<Src> {
    let mut v = vec![];
    if let Some(o) = OFFERS.iter().find(|o| o.item == it) {
        v.push(s(Pic::Merchant, "พ่อค้า", &format!("ซื้อที่พ่อค้าแคมป์ {} เหรียญ{}", o.price, if o.qty > 1 { format!(" (ได้ {})", o.qty) } else { String::new() }), Find::Obj(MERCH)));
    } else if let Some(o) = VILLAGE_OFFERS.iter().find(|o| o.item == it) {
        v.push(s(Pic::Merchant, "พ่อค้าชาวบ้าน", &format!("พ่อค้าเฉพาะทางในหมู่บ้านขาย {} เหรียญ", o.price), Find::No));
    }
    if let Some((k, o)) = KIND_OFFERS.iter().enumerate().find_map(|(k, l)| l.iter().find(|o| o.item == it).map(|o| (k, o))) {
        let who = ["แคมป์ชาวประมง", "คาราวานทะเลทราย", "แคมป์นายพราน"][k];
        v.push(s(Pic::Merchant, who, &format!("{}ขาย {} เหรียญ", who, o.price), Find::No));
    }
    v
}

/// ที่มาของของชิ้นหนึ่ง (เรียงจากง่ายไปยาก) — g ใช้บอกว่ายังขาดเครื่องมือไหม
pub fn item_howto(g: &Game, it: usize) -> Vec<Src> {
    let mut v: Vec<Src> = vec![];
    let pick_note = if g.p.pick { "" } else { " (ยังไม่มี: ทำที่แท็บคราฟต์)" };
    match it {
        WOOD => v.push(s(Pic::Obj(1), "ต้นไม้", "ตีต้นไม้ให้ล้ม ได้ไม้ทีละ 2-3 ท่อน", TREES)),
        STONE => {
            v.push(s(Pic::Obj(4), "ก้อนหิน", "ทุบก้อนหินสีเทาตามพื้น", Find::Obj(&[4])));
            v.push(s(Pic::Pick, "ภูเขา", &format!("ขุดภูเขาด้วยอีเต้อ{}", pick_note), Find::Tile(6)));
        }
        BERRY => {
            v.push(s(Pic::Obj(2), "พุ่มไม้", "เก็บผลไม้จากพุ่มไม้ (หน้าหนาวได้น้อย)", Find::Obj(&[2])));
            v.push(s(Pic::Obj(41), "ต้นแอปเปิล", "เก็บผลจากต้นแอปเปิล/มะพร้าว (กดปุ่ม ใช้)", Find::Obj(&[41, 42])));
        }
        GEM => {
            v.push(s(Pic::Obj(5), "แร่", "ทุบแร่สีฟ้าในเขตหินผา/ดินแดนหิมะ", Find::Obj(&[5])));
            v.push(s(Pic::Obj(19), "ดันเจี้ยน", "ในดันเจี้ยน/ถ้ำมีแร่และคริสตัลเยอะ", Find::Obj(DUNG)));
            v.extend(buy_src(it));
        }
        COIN => {
            v.push(s(Pic::Merchant, "พ่อค้า", "รับเควสต์ที่แคมป์พ่อค้า ทำเสร็จได้เหรียญ", Find::Obj(MERCH)));
            v.push(s(Pic::Obj(O_STALL), "ตลาดเมือง", "ขายผลผลิต/ของที่ตลาดในเมือง", Find::Obj(&[O_STALL])));
            v.push(s(Pic::Slime, "มอนสเตอร์", "ปราบมอนสเตอร์ บางตัวทิ้งเหรียญ", Find::Mob(0)));
        }
        FIRE => {
            if let Some(r) = RECIPES.iter().find(|r| matches!(r.eff, Eff::Fire)) { v.push(s(Pic::Icon(IC_ANVIL), "คราฟต์", &format!("คราฟต์เอง: {}", cost_text(r.cost)), Find::No)); }
        }
        ARROW => {
            if let Some(r) = RECIPES.iter().find(|r| matches!(r.eff, Eff::Arrows(_))) { v.push(s(Pic::Icon(IC_ANVIL), "คราฟต์", &format!("คราฟต์เอง: {} ได้ 10 ดอก", cost_text(r.cost)), Find::No)); }
            v.extend(buy_src(it));
            v.push(s(Pic::Skel, "โครงกระดูก", "โครงกระดูกทิ้งลูกธนูให้ (ถ้ามีธนู)", Find::Mob(2)));
        }
        GEL => v.push(s(Pic::Slime, "ป่าทึบ", "ปราบราชาสไลม์ — บอสที่โผล่ลึกๆ ในป่าทึบ เตรียมยากับดาบดีๆ", Find::Tile(4))),
        CORE => {
            v.push(s(Pic::Golem, "เขตหินผา", "ปราบโกเลมหินยักษ์ — บอสที่โผล่ในเขตหินผา/ภูเขา (ปราบทิ้งได้แกน 2-4 ชิ้น · ตัวที่ดุร้ายคลั่งทิ้งแกนเพิ่ม)", Find::Tile(5)));
            v.push(s(Pic::Obj(19), "ดันเจี้ยน", "มินิบอสโกเลมในดันเจี้ยนก็ทิ้งแกนให้", Find::Obj(DUNG)));
        }
        IRON => {
            v.push(s(Pic::Obj(5), "แร่", &format!("ทุบแร่สีฟ้าในเขตหินผา/ดินแดนหิมะ ได้แร่เหล็กบ่อยๆ{}", pick_note), Find::Obj(&[5])));
            v.push(s(Pic::Obj(19), "ดันเจี้ยน", "ในดันเจี้ยนมีแร่เยอะ", Find::Obj(DUNG)));
        }
        SHARD => {
            v.push(s(Pic::Skel, "มอนสเตอร์ราตรี", "ปราบมอนสเตอร์ตอนกลางคืน ทิ้งเศษดาวบ่อยๆ", Find::Mob(4)));
            v.push(s(Pic::Obj(19), "ดันเจี้ยน", "เปิดหีบ/ทุบแร่ในดันเจี้ยน", Find::Obj(DUNG)));
            v.push(s(Pic::Obj(O_ACHEST), "หีบโบราณ", "หีบในซากปรักหักพังเก่า", Find::Obj(&[O_ACHEST])));
        }
        BP_ARM | BP_WPN => {
            v.extend(buy_src(it));
            v.push(s(Pic::Obj(19), "ดันเจี้ยน", "หีบสมบัติในดันเจี้ยนบางครั้งมีให้", Find::Obj(DUNG)));
            v.push(s(Pic::Merchant, "เควสต์แคมป์", "เควสต์พ่อค้าบางครั้งให้เป็นรางวัล", Find::Obj(MERCH)));
        }
        MEAT => {
            v.push(s(Pic::Animal(A_DEER), "สัตว์ป่า", "ล่ากวาง/หมูป่า/กระต่ายในทุ่งหญ้า (เนื้อดิบกินแล้วท้องเสียได้ ปรุงก่อนดีกว่า)", Find::Animal));
            v.push(s(Pic::Bat, "ค้างคาว", "ปราบค้างคาวตอนกลางคืน", Find::Mob(3)));
            v.extend(buy_src(it));
        }
        FISH => {
            v.push(s(Pic::Rod, "ริมน้ำ", &format!("ตกปลาริมน้ำด้วยคันเบ็ด{}", if g.p.rod { "" } else { " (ยังไม่มี: ทำที่แท็บคราฟต์)" }), Find::Water));
            v.extend(buy_src(it));
        }
        SHROOM => {
            v.push(s(Pic::Obj(O_SHROOM), "เห็ดแดง", "เก็บเห็ดแดงในหนองน้ำ", Find::Obj(&[O_SHROOM])));
            v.push(s(Pic::Slime, "สไลม์", "ปราบสไลม์ ทิ้งเห็ดบ้าง", Find::Mob(1)));
        }
        HERB => {
            v.push(s(Pic::Obj(O_GIANT), "ต้นไม้ยักษ์", "เก็บจากต้นไม้ยักษ์ (วันละครั้ง)", Find::Obj(&[O_GIANT])));
            v.push(s(Pic::Skel, "โครงกระดูก", "ปราบโครงกระดูกตอนกลางคืน ทิ้งสมุนไพรบ่อย", Find::Mob(2)));
            v.extend(buy_src(it));
        }
        SEED_CORN | SEED_CARROT => {
            v.extend(buy_src(it));
            v.push(s(Pic::Slime, "สไลม์", "สไลม์ทิ้งเมล็ดให้บางครั้ง", Find::Mob(1)));
        }
        CORN | CARROT => v.push(s(Pic::Icon(it - 2), "ปลูกเอง", "ปลูกเมล็ดบนดินที่ไถแล้ว รดน้ำ แล้วรอโต (ซื้อเมล็ดที่พ่อค้าได้)", Find::No)),
        _ => {}
    }
    if let Some(d) = DISHES.iter().find(|d| d.item == it) {
        v.insert(0, s(Pic::Obj(26), "หม้อทำอาหาร", &format!("ปรุงที่หม้อทำอาหาร: {}", cost_text(d.cost)), Find::Obj(&[26])));
        if it == POTION { v.extend(buy_src(it)); }
    }
    v
}

/// เควสต์พ่อค้า: ไปหาของที่ต้องใช้ได้จากไหน (ตามชนิดเควสต์)
pub fn quest_howto(kind: u8) -> Vec<Src> {
    match kind {
        0 => vec![s(Pic::Slime, "สไลม์", "สไลม์ออกมาเดินใกล้ตัวคุณทั้งวัน", Find::Mob(1)), s(Pic::Skel, "โครงกระดูก/ค้างคาว", "กลางคืนมีโครงกระดูกกับค้างคาวเพิ่ม", Find::Mob(0))],
        1 => vec![s(Pic::Skel, "โครงกระดูก", "โครงกระดูกออกมาตอนกลางคืน (หลัง 18:00)", Find::Mob(2))],
        2 => vec![s(Pic::Slime, "สไลม์", "สไลม์ออกมาเดินทั้งวัน ไม่ต้องรอกลางคืน", Find::Mob(1))],
        3 => vec![s(Pic::Bat, "ค้างคาว", "ค้างคาวบินมาตอนกลางคืน (หลัง 18:00)", Find::Mob(3))],
        4 => vec![s(Pic::Skel, "มอนสเตอร์ราตรี", "ออกล่าตอนกลางคืน — เตรียมยาและกองไฟไว้", Find::Mob(4))],
        5 => vec![s(Pic::Obj(5), "แร่", "ทุบแร่สีฟ้าในเขตหินผา/ดินแดนหิมะ", Find::Obj(&[5])), s(Pic::Obj(19), "ดันเจี้ยน", "ดันเจี้ยนมีแร่เยอะ", Find::Obj(DUNG))],
        6 => vec![s(Pic::Obj(4), "ก้อนหิน", "ทุบก้อนหินสีเทาตามพื้น", Find::Obj(&[4])), s(Pic::Pick, "ภูเขา", "ขุดภูเขาด้วยอีเต้อ", Find::Tile(6))],
        _ => vec![s(Pic::Obj(1), "ต้นไม้", "ตีต้นไม้ให้ล้ม", TREES)],
    }
}

// ======================= หาจุดที่ใกล้ที่สุด =======================
thread_local! {
    /// ผลสแกนล่าสุดของแต่ละชนิด: (ช่องผู้เล่นตอนสแกน, ผล: ช่องจริงของเป้าหมาย)
    static CACHE: RefCell<HashMap<Find, ((i32, i32), Option<(i32, i32)>)>> = RefCell::new(HashMap::new());
    /// สแกนได้กี่ครั้งในเฟรมนี้ (กันกระตุกเมื่อมีหลายแถวพร้อมกัน)
    static BUDGET: Cell<i32> = Cell::new(1);
}
/// เรียกต้นเฟรม: ให้สแกนโลกใหม่ได้ 1 ครั้งต่อเฟรม
pub fn new_frame() { BUDGET.with(|b| b.set(1)); }
pub fn clear_cache() { CACHE.with(|c| c.borrow_mut().clear()); }

fn hit(w: &World, f: Find, tx: i32, ty: i32) -> bool {
    match f {
        Find::Obj(l) => l.contains(&w.obj_at(tx, ty)),
        Find::Tree => { let o = w.obj_at(tx, ty); is_tree(o) && o != T_SAPLING }
        Find::Tile(t) => w.tile_at(tx, ty) == t,
        Find::Water => w.tile_at(tx, ty) < 2,
        _ => false,
    }
}
/// สแกนเป็นวงขยายออกจากผู้เล่น หยุดเมื่อเจอและวงถัดไปไกลกว่าแน่ๆ
fn scan(w: &World, f: Find, ox: i32, oy: i32) -> Option<(i32, i32)> {
    let mut best: Option<(i32, (i32, i32))> = None;
    for r in 0..=SCAN_R {
        if let Some((d2, _)) = best { if r * r > d2 { break; } }
        for dy in -r..=r {
            let step = if dy.abs() == r { 1 } else { (2 * r).max(1) };
            let mut dx = -r;
            while dx <= r {
                if hit(w, f, ox + dx, oy + dy) {
                    let d2 = dx * dx + dy * dy;
                    if best.map_or(true, |(b, _)| d2 < b) { best = Some((d2, (ox + dx, oy + dy))); }
                }
                dx += step;
            }
        }
    }
    best.map(|b| b.1)
}
/// ช่อง (จริงในโลก) ของเป้าหมายที่ใกล้ที่สุด · None = ไม่มีใกล้ๆ / ไม่มีจุดตายตัว
pub fn nearest(g: &Game, f: Find) -> Option<(i32, i32)> {
    let (px, py) = (tfloor(g.p.x), tfloor(g.p.y));
    match f {
        Find::No => None,
        Find::Mob(k) => g.en.iter().filter(|e| !e.dead && match k { 1 => e.k == EK::Slime, 2 => e.k == EK::Skel, 3 => e.k == EK::Bat, 4 => e.night, _ => true })
            .map(|e| (tfloor(e.x), tfloor(e.y))).min_by_key(|&(x, y)| (x - px).pow(2) + (y - py).pow(2)),
        Find::Animal => g.animals.iter().filter(|a| !a.dead && AN_STAT[a.k as usize].4 > 0)
            .map(|a| (tfloor(a.x), tfloor(a.y))).min_by_key(|&(x, y)| (x - px).pow(2) + (y - py).pow(2)),
        _ => {
            let cached = CACHE.with(|c| c.borrow().get(&f).copied());
            if let Some((o, res)) = cached {
                let moved = (o.0 - px).abs().max((o.1 - py).abs());
                let still = res.map_or(true, |t| hit(&g.w, f, t.0, t.1));
                // ยังใช้ได้ถ้าเดินไปไม่ไกลและเป้าหมายยังอยู่ · ไม่งั้นสแกนใหม่ (ถ้าเฟรมนี้ยังมีโควตา)
                if moved <= 5 && still { return res; }
                if BUDGET.with(|b| b.get()) <= 0 { return if still { res } else { None }; }
            }
            BUDGET.with(|b| b.set(b.get() - 1));
            let res = scan(&g.w, f, px, py);
            CACHE.with(|c| c.borrow_mut().insert(f, ((px, py), res)));
            res
        }
    }
}
/// ตำแหน่งสัมพัทธ์จากผู้เล่น (dx, dy เป็นช่อง)
pub fn rel(g: &Game, f: Find) -> Option<(i32, i32)> {
    let (px, py) = (tfloor(g.p.x), tfloor(g.p.y));
    nearest(g, f).map(|(x, y)| (x - px, y - py))
}
/// "ขวา 12 ช่อง" / "อยู่ตรงนี้เลย"
pub fn dir_text(dx: i32, dy: i32) -> String {
    let d = ((dx * dx + dy * dy) as f32).sqrt().round() as i32;
    if d <= 1 { return "อยู่ตรงนี้เลย".into(); }
    let (ax, ay) = (dx.abs(), dy.abs());
    let n = if ax * 2 < ay { if dy < 0 { "บน" } else { "ล่าง" } }
        else if ay * 2 < ax { if dx > 0 { "ขวา" } else { "ซ้าย" } }
        else { match (dx > 0, dy < 0) { (true, true) => "ขวาบน", (true, false) => "ขวาล่าง", (false, true) => "ซ้ายบน", _ => "ซ้ายล่าง" } };
    format!("{} {} ช่อง", n, d)
}
/// ข้อความระยะของแถว: มีจุดใกล้ๆ → ทิศ+ระยะ · ไม่มี → บอกให้สำรวจ · ไม่มีจุดตายตัว → ว่าง
pub fn where_text(g: &Game, f: Find) -> String {
    if f == Find::No { return String::new(); }
    match rel(g, f) {
        Some((dx, dy)) => dir_text(dx, dy),
        None => match f { Find::Mob(_) | Find::Animal => "ยังไม่เจอใกล้ๆ".into(), _ => "ยังไม่เจอใกล้ๆ ลองเดินสำรวจ".into() },
    }
}

// ======================= วาด =======================
fn fit(sp: &Sprites, r: Rect, x: f32, y: f32, sz: f32, c: Color) {
    let k = sz / r.w.max(r.h);
    let (w, h) = (r.w * k, r.h * k);
    draw_texture_ex(&sp.tex, x + (sz - w) / 2., y + (sz - h) / 2., c, DrawTextureParams { source: Some(r), dest_size: Some(vec2(w, h)), ..Default::default() });
}
pub fn draw_pic(sp: &Sprites, p: Pic, x: f32, y: f32, sz: f32) {
    match p {
        Pic::Obj(o) => fit(sp, sp.obj[o as usize], x, y, sz, WHITE),
        Pic::Icon(i) => draw_texture_ex(&sp.tex, x, y, WHITE, DrawTextureParams { source: Some(sp.icons[i]), dest_size: Some(vec2(sz, sz)), ..Default::default() }),
        Pic::Tile(t) => fit(sp, sp.tiles[t as usize][0], x, y, sz, WHITE),
        Pic::Merchant => fit(sp, sp.ex.merchant[0], x, y, sz, WHITE),
        Pic::Slime => fit(sp, sp.slime[0], x, y, sz, WHITE),
        Pic::Skel => fit(sp, sp.skel, x, y, sz, WHITE),
        Pic::Bat => fit(sp, sp.bat[0], x, y, sz, WHITE),
        Pic::Golem => fit(sp, sp.ex.golem[0], x, y, sz, WHITE),
        Pic::Animal(k) => fit(sp, sp.animals[k as usize * 2], x, y, sz, WHITE),
        Pic::Rod => mcui::pix_icon_rod(x, y, sz / 12.),
        Pic::Pick => mcui::pix_icon_pick(x, y, sz / 12.),
    }
}
/// ลูกศรชี้ทิศ: ก้านยาว + หัวสามเหลี่ยม (ไม่พึ่งฟอนต์) — มองแล้วรู้ทันทีว่าชี้ไปทางไหน
pub fn draw_arrow(cx: f32, cy: f32, r: f32, dx: i32, dy: i32, col: Color) {
    let l = ((dx * dx + dy * dy) as f32).sqrt();
    if l < 1.5 { draw_circle(cx, cy, r * 0.45, col); return; }
    let (ux, uy) = (dx as f32 / l, dy as f32 / l);
    let (nx, ny) = (-uy, ux);
    let tail = vec2(cx - ux * r, cy - uy * r);
    let neck = vec2(cx + ux * r * 0.1, cy + uy * r * 0.1);
    draw_line(tail.x, tail.y, neck.x, neck.y, (r * 0.38).max(2.), col);
    let tip = vec2(cx + ux * r * 1.15, cy + uy * r * 1.15);
    let a = vec2(neck.x + nx * r * 0.62, neck.y + ny * r * 0.62);
    let b = vec2(neck.x - nx * r * 0.62, neck.y - ny * r * 0.62);
    draw_triangle(tip, a, b, col);
}
/// ตัดข้อความให้พอดีความกว้าง
pub fn fit_text(font: &Font, t: &str, size: f32, maxw: f32) -> String {
    if measure_text(t, Some(font), size as u16, 1.).width <= maxw { return t.to_string(); }
    let cs: Vec<char> = t.chars().collect();
    let mut n = cs.len();
    while n > 1 {
        n -= 1;
        let c: String = cs[..n].iter().collect::<String>() + "..";
        if measure_text(&c, Some(font), size as u16, 1.).width <= maxw { return c; }
    }
    "..".into()
}
fn vis(y: f32, h: f32, c: Rect) -> bool { y + h > c.y && y < c.y + c.h }

pub const ROW_H: f32 = 28.;
pub const HEAD_H: f32 = 20.;
/// แถวคู่มือของ 1 ชิ้น: หัว (ชื่อของ + ขาดเท่าไร) + แถวที่มา (สูงสุด max_src) — คืนความสูง
pub fn draw_howto(g: &Game, sp: &Sprites, font: &Font, c: Rect, x: f32, y: f32, w: f32, u: f32, it: usize, lack: Option<i32>, max_src: usize) -> f32 {
    let srcs = item_howto(g, it);
    let n = srcs.len().min(max_src);
    let h = HEAD_H * u + n as f32 * ROW_H * u;
    if !vis(y, h, c) { return h; }
    draw_rectangle(x, y + 1., w, h - 2., Color::new(0.14, 0.15, 0.22, 0.9));
    draw_rectangle(x, y + 1., 3. * u, h - 2., hx(0xffb060));
    draw_texture_ex(&sp.tex, x + 8. * u, y + 2. * u, WHITE, DrawTextureParams { source: Some(sp.icons[it]), dest_size: Some(vec2(16. * u, 16. * u)), ..Default::default() });
    let head = match lack { Some(k) => format!("ขาด{} {} — หาได้จาก", INAME[it], k), None => format!("{} — หาได้จาก", INAME[it]) };
    text(font, &head, x + 28. * u, y + 14. * u, 12. * u, hx(0xffb060));
    if srcs.is_empty() { text(font, "(ของชิ้นนี้ได้จากการเล่นทั่วไป)", x + 8. * u, y + HEAD_H * u + 14. * u, 11. * u, hx(0x9a9aaa)); }
    for (k, sr) in srcs.iter().take(n).enumerate() {
        let ry = y + HEAD_H * u + k as f32 * ROW_H * u;
        mcui::slot(x + 8. * u, ry + 2. * u, 24. * u, false, false);
        draw_pic(sp, sr.pic, x + 10. * u, ry + 4. * u, 20. * u);
        let wt = where_text(g, sr.find);
        let wd = if wt.is_empty() { 0. } else { measure_text(&wt, Some(font), (11. * u) as u16, 1.).width + 26. * u };
        let how = fit_text(font, &sr.how, 11. * u, w - 44. * u - wd);
        text(font, &how, x + 38. * u, ry + 18. * u, 11. * u, hx(0xdde0ee));
        if !wt.is_empty() {
            let found = rel(g, sr.find);
            let col = if found.is_some() { hx(0x9ad8ff) } else { hx(0x8a8a9a) };
            if let Some((dx, dy)) = found { draw_arrow(x + w - wd + 10. * u, ry + 14. * u, 7. * u, dx, dy, col); }
            text(font, &wt, x + w - wd + 22. * u, ry + 18. * u, 11. * u, col);
        }
    }
    h
}
/// ขาดของหลายอย่าง: โชว์ทีละชิ้น (สูงสุด 3 ชิ้น) — คืนความสูงรวม (รวมช่องว่างท้าย)
pub fn draw_lack(g: &Game, sp: &Sprites, font: &Font, c: Rect, y: f32, u: f32, lack: &[(usize, i32)]) -> f32 {
    let mut yy = y;
    for &(it, n) in lack.iter().take(3) {
        yy += draw_howto(g, sp, font, c, c.x + 6. * u, yy, c.w - 12. * u, u, it, Some(n), 2) + 4. * u;
    }
    yy - y
}
/// ของที่ยังขาดจากรายการวัสดุ (ชิ้นที่ในกระเป๋ามีน้อยกว่าที่ต้องใช้) → (ชิ้น, ขาดอีกเท่าไร)
pub fn lack_of(g: &Game, need: &[(usize, i32)], extra: Option<usize>) -> Vec<(usize, i32)> {
    let mut v: Vec<(usize, i32)> = need.iter().filter(|&&(it, n)| g.p.inv[it] < n).map(|&(it, n)| (it, n - g.p.inv[it])).collect();
    if let Some(e) = extra { if g.p.inv[e] < 1 { v.push((e, 1)); } }
    v
}
/// แถวเควสต์พ่อค้า (ใต้ชื่อเควสต์): ไปหาได้ที่ไหน
pub fn draw_quest_howto(g: &Game, sp: &Sprites, font: &Font, c: Rect, x: f32, y: f32, w: f32, u: f32, kind: u8) -> f32 {
    let srcs = quest_howto(kind);
    let h = HEAD_H * u + srcs.len() as f32 * ROW_H * u;
    if !vis(y, h, c) { return h; }
    draw_rectangle(x, y + 1., w, h - 2., Color::new(0.14, 0.15, 0.22, 0.9));
    draw_rectangle(x, y + 1., 3. * u, h - 2., hx(0x6ad0ff));
    text(font, "ไม่รู้จะไปหาที่ไหน? ลองดูตรงนี้", x + 10. * u, y + 14. * u, 12. * u, hx(0x9ad8ff));
    for (k, sr) in srcs.iter().enumerate() {
        let ry = y + HEAD_H * u + k as f32 * ROW_H * u;
        mcui::slot(x + 8. * u, ry + 2. * u, 24. * u, false, false);
        draw_pic(sp, sr.pic, x + 10. * u, ry + 4. * u, 20. * u);
        let wt = where_text(g, sr.find);
        let wd = if wt.is_empty() { 0. } else { measure_text(&wt, Some(font), (11. * u) as u16, 1.).width + 26. * u };
        text(font, &fit_text(font, &sr.how, 11. * u, w - 44. * u - wd), x + 38. * u, ry + 18. * u, 11. * u, hx(0xdde0ee));
        if !wt.is_empty() {
            let found = rel(g, sr.find);
            let col = if found.is_some() { hx(0x9ad8ff) } else { hx(0x8a8a9a) };
            if let Some((dx, dy)) = found { draw_arrow(x + w - wd + 10. * u, ry + 14. * u, 7. * u, dx, dy, col); }
            text(font, &wt, x + w - wd + 22. * u, ry + 18. * u, 11. * u, col);
        }
    }
    h
}

#[cfg(test)]
mod tests_v32 {
    use super::*;

    #[test]
    fn every_item_but_trivial_has_a_source() {
        let g = Game::new_world_seed(901, Sfx::default());
        for it in 0..NI {
            let v = item_howto(&g, it);
            assert!(!v.is_empty(), "{} ต้องมีที่มา", INAME[it]);
            for sr in &v { assert!(!sr.how.is_empty() && !sr.short.is_empty()); }
        }
    }
    #[test]
    fn dishes_and_crafts_use_real_costs() {
        let g = Game::new_world_seed(902, Sfx::default());
        let stew = item_howto(&g, DISH_STEW);
        assert!(stew[0].how.contains("เนื้อสัตว์ 2") && stew[0].how.contains("สมุนไพร 1"), "{}", stew[0].how);
        let fire = item_howto(&g, FIRE);
        assert!(fire[0].how.contains("ไม้ 3") && fire[0].how.contains("หิน 2"), "{}", fire[0].how);
        let arrow = item_howto(&g, ARROW);
        assert!(arrow.iter().any(|x| x.how.contains("8 เหรียญ")), "ราคาลูกธนูจากร้านค้า");
        let gem = item_howto(&g, GEM);
        assert!(gem.iter().any(|x| x.how.contains("45 เหรียญ")), "อัญมณีที่พ่อค้าหมู่บ้านขาย");
    }
    #[test]
    fn tool_hints_follow_inventory() {
        let mut g = Game::new_world_seed(903, Sfx::default());
        g.p.pick = false; g.p.rod = false;
        assert!(item_howto(&g, STONE).iter().any(|x| x.how.contains("ยังไม่มี")));
        assert!(item_howto(&g, FISH)[0].how.contains("ยังไม่มี"));
        g.p.pick = true; g.p.rod = true;
        assert!(!item_howto(&g, STONE).iter().any(|x| x.how.contains("ยังไม่มี")));
        assert!(!item_howto(&g, FISH)[0].how.contains("ยังไม่มี"));
    }
    #[test]
    fn direction_text() {
        assert_eq!(dir_text(12, 1), "ขวา 12 ช่อง");
        assert_eq!(dir_text(-9, 0), "ซ้าย 9 ช่อง");
        assert_eq!(dir_text(0, -7), "บน 7 ช่อง");
        assert_eq!(dir_text(0, 20), "ล่าง 20 ช่อง");
        assert_eq!(dir_text(6, -6), "ขวาบน 8 ช่อง");
        assert_eq!(dir_text(-5, 5), "ซ้ายล่าง 7 ช่อง");
        assert_eq!(dir_text(1, 0), "อยู่ตรงนี้เลย");
    }
    #[test]
    fn finds_nearest_placed_object_and_tree() {
        let mut g = Game::new_world_seed(904, Sfx::default());
        let (px, py) = (tfloor(g.p.x), tfloor(g.p.y));
        clear_cache(); new_frame();
        // ปักก้อนหินสองก้อน (ไกล/ใกล้) — ต้องเลือกใกล้
        g.w.placed.insert((px + 15, py + 2), 4);
        g.w.placed.insert((px - 6, py), 4);
        let r = rel(&g, Find::Obj(&[4])).expect("เจอก้อนหิน");
        assert!(r.0.abs().max(r.1.abs()) <= 7 || g.w.obj_at(px + r.0, py + r.1) == 4);
        assert_eq!(g.w.obj_at(px + r.0, py + r.1), 4);
        let d = r.0 * r.0 + r.1 * r.1;
        assert!(d <= 36, "ต้องเจอก้อนที่ใกล้กว่า (หรือใกล้เท่ากัน) ไม่ใช่ก้อนไกล: {:?}", r);
    }
    #[test]
    fn scan_budget_and_cache() {
        let g = Game::new_world_seed(905, Sfx::default());
        clear_cache(); new_frame();
        let a = nearest(&g, Find::Tile(3));
        // เฟรมเดียวกัน โควตาหมดแล้ว แต่ผลแคชใช้ต่อได้
        let b = nearest(&g, Find::Tile(3));
        assert_eq!(a, b);
        // ชนิดอื่นในเฟรมเดียวกันที่ไม่มีแคช → ยังสแกนไม่ได้ในเฟรมนี้ก็ต้องไม่ค้าง/พัง
        let _ = nearest(&g, Find::Tile(5));
        new_frame();
        let _ = nearest(&g, Find::Tile(5));
    }
    #[test]
    fn mobs_and_animals_found_from_live_lists() {
        let mut g = Game::new_world_seed(906, Sfx::default());
        g.en.clear(); g.animals.clear();
        assert!(nearest(&g, Find::Mob(0)).is_none() && nearest(&g, Find::Animal).is_none());
        let (x, y) = (g.p.x + 80., g.p.y);
        g.make_enemy(EK::Slime, x, y, 1., 0, false);
        assert_eq!(rel(&g, Find::Mob(1)), Some((5, 0)));
        assert!(nearest(&g, Find::Mob(2)).is_none(), "ไม่มีโครงกระดูก");
        let d = g.new_animal(A_DEER, g.p.x - 48., g.p.y + 16.);
        g.animals.push(d);
        assert_eq!(rel(&g, Find::Animal), Some((-3, 1)));
    }
    #[test]
    fn quest_kinds_all_have_howto() {
        for k in 0..8u8 { assert!(!quest_howto(k).is_empty(), "เควสต์ชนิด {}", k); }
    }
    #[test]
    fn lack_lists_only_what_is_missing() {
        let mut g = Game::new_world_seed(907, Sfx::default());
        g.p.inv[WOOD] = 5; g.p.inv[STONE] = 0; g.p.inv[BP_ARM] = 0;
        let l = lack_of(&g, &[(WOOD, 4), (STONE, 6)], Some(BP_ARM));
        assert_eq!(l, vec![(STONE, 6), (BP_ARM, 1)]);
        g.p.inv[STONE] = 2;
        assert_eq!(lack_of(&g, &[(WOOD, 4), (STONE, 6)], None), vec![(STONE, 4)]);
    }
}
