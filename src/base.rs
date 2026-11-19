//! ระบบสร้างฐาน: กำแพงไม้/หิน · ประตู · กล่องเก็บของ · การโต้ตอบ

use super::*;

pub struct Piece {
    pub name: &'static str,
    pub desc: &'static str,
    pub obj: u8,
    pub icon: usize,
    pub cost: &'static [(usize, i32)],
}
pub const PIECES: [Piece; 13] = [
    Piece { name: "กำแพงไม้", desc: "กั้นทางได้ ตีพังได้ (ได้ไม้คืน)", obj: 10, icon: IC_WWALL, cost: &[(WOOD, 2)] },
    Piece { name: "กำแพงหิน", desc: "แข็งแรงกว่ากำแพงไม้", obj: 11, icon: IC_SWALL, cost: &[(STONE, 3)] },
    Piece { name: "ประตู", desc: "กด Q หรือปุ่ม \"ใช้\" เพื่อเปิด-ปิด", obj: 12, icon: IC_DOOR, cost: &[(WOOD, 4)] },
    Piece { name: "กล่องเก็บของ", desc: "เปิดแล้วย้ายไอเทมเข้า-ออกได้ บันทึกลงเซฟ", obj: 14, icon: IC_BOX, cost: &[(WOOD, 8), (STONE, 2)] },
    Piece { name: "ทั่งตีบวก", desc: "ตีบวกอาวุธ/เกราะ +1..+5 ด้วยแร่และชิ้นส่วนบอส", obj: 17, icon: IC_ANVIL, cost: &[(STONE, 12), (WOOD, 4)] },
    Piece { name: "คบเพลิง", desc: "ให้แสงสว่างรอบฐานตอนกลางคืน", obj: 18, icon: IC_TORCH, cost: &[(WOOD, 1), (STONE, 1)] },
    Piece { name: "หม้อทำอาหาร", desc: "ทำอาหาร/ปรุงยา จากเนื้อ เห็ด สมุนไพร (ได้บัฟชั่วคราว)", obj: 26, icon: IC_POT, cost: &[(STONE, 6), (WOOD, 4)] },
    Piece { name: "ไถดิน", desc: "แปลงทุ่งหญ้าเป็นดินปลูก (ไม่เสียของ · วางซ้ำ = คืนหญ้า)", obj: OBJ_TILL, icon: IC_HOE, cost: &[] },
    Piece { name: "เตียงนอน", desc: "นอนตอนกลางคืนเพื่อข้ามไปเช้า · ฟื้นเลือดเต็ม · เป็นจุดเกิด", obj: 33, icon: IC_BED, cost: &[(WOOD, 12), (STONE, 2)] },
    Piece { name: "โต๊ะคราฟต์ขั้นสูง", desc: "สร้างอาวุธ/เกราะขั้น 2 (ต้องมีของขั้น 1 ก่อน)", obj: 34, icon: IC_ATABLE, cost: &[(STONE, 20), (WOOD, 10), (GEM, 4)] },
    Piece { name: "ปราสาท", desc: "ศูนย์กลางอาณาจักร (ต้องมี + กำแพงเมือง 12 ชิ้น + ชาวบ้าน >10 = เก็บภาษีได้)", obj: 37, icon: IC_ATABLE, cost: &[(STONE, 80), (WOOD, 40), (GEM, 10)] },
    Piece { name: "บ่อน้ำ", desc: "น้ำดื่มสะอาด (ดื่มแล้ว +45) · ชาวบ้านในรัศมี 20 ช่องมีน้ำใช้ ไม่ต้องอยู่ริมน้ำ", obj: 39, icon: IC_POT, cost: &[(STONE, 10), (WOOD, 4)] },
    Piece { name: "กำแพงเมือง", desc: "กำแพงหินสูง ทนทานกว่ากำแพงหิน (นับรวมเป็นกำแพงเมือง)", obj: 38, icon: IC_SWALL, cost: &[(STONE, 8)] },
];

/// สิ่งที่ Game ขอให้ UI เปิด
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Req {
    Chest((i32, i32)),
    Shop,
    Forge,
    Cook,
    Adv,
    Npc(u32),
}

impl Game {
    pub fn can_till(&self, tx: i32, ty: i32) -> bool {
        let (t, o) = (self.w.tile_at(tx, ty), self.w.obj_at(tx, ty));
        ((t == T_SOIL || t == T_WET) && (o == 0 || (27..=32).contains(&o))) || ((t == 3 || t == 4) && (o == 0 || o == 3))
    }
    pub fn player_in_tile(&self, tx: i32, ty: i32) -> bool {
        let p = &self.p;
        [(p.x - 4., p.y - 4.), (p.x + 4., p.y - 4.), (p.x - 4., p.y), (p.x + 4., p.y)].iter().any(|&(x, y)| tfloor(x) == tx && tfloor(y) == ty)
    }
    pub fn can_place_at(&self, tx: i32, ty: i32) -> bool {
        let o = self.w.obj_at(tx, ty);
        !SOLIDT[self.w.tile_at(tx, ty) as usize] && (o == 0 || o == 3) && !self.player_in_tile(tx, ty)
    }
    pub fn place_piece(&mut self) {
        if self.w.dun.is_some() {
            self.toast("สร้างในดันเจี้ยนไม่ได้");
            return;
        }
        let Some(sel) = self.build_sel else {
            self.toast("เลือกสิ่งปลูกสร้างจากเมนูสร้าง (B) ก่อน");
            return;
        };
        let pc = &PIECES[sel];
        if pc.obj == OBJ_TILL {
            let k = self.build_target();
            if self.till_at(k) {
                self.save();
            }
            return;
        }
        if !can_pay(&self.p, pc.cost) {
            self.toast("วัสดุไม่พอ");
            return;
        }
        let (tx, ty) = self.build_target();
        if !self.can_place_at(tx, ty) {
            self.toast("วางตรงนี้ไม่ได้");
            return;
        }
        for &(it, n) in pc.cost {
            self.p.inv[it] -= n;
        }
        self.w.placed.insert((tx, ty), pc.obj);
        self.ev[3] += 1;
        if pc.obj == 14 {
            self.chests.insert((tx, ty), [0; NI]);
        }
        self.obj_dmg.remove(&(tx, ty));
        play(&self.sfx.door);
        self.burst(tx as f32 * 16. + 8., ty as f32 * 16. + 10., hx(0xc8a070), 8, 40.);
        self.save();
    }

    /// วัตถุที่โต้ตอบได้ใกล้ตัวที่สุด (ประตู/กล่อง/พ่อค้า)
    pub fn interact_target(&self) -> Option<((i32, i32), u8)> {
        let (ptx, pty) = (tfloor(self.p.x), tfloor(self.p.y - 3.));
        let mut best: Option<(f32, (i32, i32), u8)> = None;
        for ty in pty - 2..=pty + 2 {
            for tx in ptx - 2..=ptx + 2 {
                let o = self.w.obj_at(tx, ty);
                if !matches!(o, 12 | 13 | 14 | 15 | 17 | 19 | 20 | 24 | 25 | 26 | 33 | 34 | 36 | 39 | 41 | 42) {
                    continue;
                }
                if (o == 41 || o == 42) && !self.fruit_ready((tx, ty)) { continue; }
                let d = (tx as f32 * 16. + 8. - self.p.x).hypot(ty as f32 * 16. + 8. - (self.p.y - 3.));
                if d < 28. && best.map_or(true, |b| d < b.0) {
                    best = Some((d, (tx, ty), o));
                }
            }
        }
        best.map(|b| (b.1, b.2))
    }
    pub fn interact(&mut self) {
        if self.fish.is_some() {
            self.fish_press();
            return;
        }
        if self.npc_interact() {
            return;
        }
        if self.try_farm() {
            return;
        }
        let Some((k, o)) = self.interact_target() else {
            if self.drink_ahead() { return; }
            if !self.start_fish() {
                self.toast("ไม่มีอะไรให้ใช้ตรงนี้");
            }
            return;
        };
        match o {
            12 => {
                self.w.placed.insert(k, 13);
                play(&self.sfx.door);
                self.save();
            }
            13 => {
                if self.player_in_tile(k.0, k.1) {
                    self.toast("ปิดไม่ได้ มีคนยืนอยู่ในประตู");
                } else {
                    self.w.placed.insert(k, 12);
                    play(&self.sfx.door);
                    self.save();
                }
            }
            14 => {
                self.chests.entry(k).or_insert([0; NI]);
                play(&self.sfx.chest);
                self.req = Some(Req::Chest(k));
            }
            17 => {
                play(&self.sfx.click);
                self.req = Some(Req::Forge);
            }
            25 => {
                self.big_map = true;
                self.mini_t = 99.;
                self.toast("แตะสัญลักษณ์ W บนแผนที่เพื่อวาร์ป");
            }
            26 => {
                play(&self.sfx.click);
                self.req = Some(Req::Cook);
            }
            33 => self.sleep(k),
            39 => self.drink_well(),
            41 | 42 => { self.pick_fruit(k, o); }
            34 => {
                play(&self.sfx.click);
                self.req = Some(Req::Adv);
            }
            19 | 36 => self.enter_dungeon(k),
            20 => self.exit_dungeon(),
            24 => self.open_rare_chest(k),
            _ => {
                play(&self.sfx.buy);
                self.shop_camp = Some(k);
                self.req = Some(Req::Shop);
            }
        }
    }
    /// ย้ายไอเทมระหว่างกระเป๋า <-> กล่อง
    pub fn chest_move(&mut self, k: (i32, i32), item: usize, n: i32, into: bool) {
        let c = self.chests.entry(k).or_insert([0; NI]);
        if into {
            let m = n.min(self.p.inv[item]);
            c[item] += m;
            self.p.inv[item] -= m;
        } else {
            let m = n.min(c[item]);
            c[item] -= m;
            self.p.inv[item] += m;
        }
        self.save();
    }
}

/// ข้อความช่วยเหลือเมื่ออยู่ใกล้สิ่งที่โต้ตอบได้
pub fn interact_hint(o: u8, touch: bool) -> String {
    let key = if touch { "ปุ่ม \"ใช้\"" } else { "Q" };
    match o {
        12 => format!("{}: เปิดประตู", key),
        13 => format!("{}: ปิดประตู", key),
        14 => format!("{}: เปิดกล่องเก็บของ", key),
        17 => format!("{}: ใช้ทั่งตีบวก", key),
        25 => format!("{}: เปิดแผนที่วาร์ป", key),
        26 => format!("{}: ทำอาหาร / ปรุงยา", key),
        19 => format!("{}: เข้าดันเจี้ยนลับ (อันตราย!)", key),
        36 => format!("{}: เข้าถ้ำใต้ดิน (มืด · มีแร่ · มอนสเตอร์ดุ!)", key),
        33 => format!("{}: นอน (ข้ามไปเช้า)", key),
        39 => format!("{}: ดื่มน้ำจากบ่อ", key),
        41 => format!("{}: เก็บแอปเปิล (ไม่ต้องโค่นต้น)", key),
        42 => format!("{}: ดื่มน้ำมะพร้าว", key),
        34 => format!("{}: ใช้โต๊ะคราฟต์ขั้นสูง", key),
        20 => format!("{}: ออกจากดันเจี้ยน", key),
        24 => format!("{}: เปิดหีบหายาก", key),
        _ => format!("{}: คุยกับพ่อค้า", key),
    }
}
