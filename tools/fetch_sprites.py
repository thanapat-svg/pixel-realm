#!/usr/bin/env python3
"""ดาวน์โหลดแพ็กภาพ CC0 -> ตัดช่องตาม tools/sprite_map.json -> assets/sprites/<out>.png
ล้มเหลวแบบเงียบ (exit 0) เสมอ เพื่อไม่ให้ build APK พัง. สร้างภาพ preview พร้อมเลขช่องที่ build/sprite_preview/

ใช้:  python3 tools/fetch_sprites.py [--out assets/sprites] [--preview build/sprite_preview]
env:  SPRITE_PACK_URL=<url png ของสเปรดชีตเพิ่มเติม, ตั้งชื่อ sheet = "custom">
"""
import argparse, json, os, sys, urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
try:
    from PIL import Image, ImageDraw
except Exception:
    print("[sprites] ไม่มี Pillow -> ข้ามขั้นตอนภาพ (pip install pillow)")
    sys.exit(0)


def fetch(url):
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "pixel-realm-assets"})
        with urllib.request.urlopen(req, timeout=40) as r:
            b = r.read()
    except Exception as e:
        print(f"[sprites]   x {url} ({type(e).__name__}: {e})")
        return None
    if not b.startswith(b"\x89PNG"):
        why = "Git-LFS pointer" if b.startswith(b"version https://git-lfs") else "ไม่ใช่ PNG"
        print(f"[sprites]   x {url} ({why}, {len(b)} B)")
        return None
    return b


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default="assets/sprites")
    ap.add_argument("--preview", default="build/sprite_preview")
    a = ap.parse_args()
    srcs = json.load(open(os.path.join(HERE, "sprite_sources.json")))["sheets"]
    if os.environ.get("SPRITE_PACK_URL"):
        srcs["custom"] = {"cell": int(os.environ.get("SPRITE_CELL", "16")), "gap": 0, "urls": [os.environ["SPRITE_PACK_URL"]]}
    maps = json.load(open(os.path.join(HERE, "sprite_map.json")))["entries"]
    os.makedirs(a.out, exist_ok=True)
    os.makedirs(a.preview, exist_ok=True)
    sheets = {}
    for name, s in srcs.items():
        for u in s["urls"]:
            b = fetch(u)
            if b:
                p = os.path.join(a.preview, f"{name}_raw.png")
                open(p, "wb").write(b)
                im = Image.open(p).convert("RGBA")
                # สเปรดชีต 1-bit ใช้สีพื้นทึบ -> ทำให้โปร่งใส (colorkey ด้วยสีที่พบมากที่สุด)
                if s.get("colorkey", True) and im.getchannel("A").getextrema()[0] == 255:
                    bgc = max(im.getcolors(1 << 20))[1]
                    px = im.load()
                    for yy in range(im.height):
                        for xx in range(im.width):
                            if px[xx, yy] == bgc:
                                px[xx, yy] = (0, 0, 0, 0)
                sheets[name] = (im, s["cell"], s.get("gap", 0))
                print(f"[sprites] ok {name}: {im.size} จาก {u}")
                # preview: ขยาย 3x + เส้นกริดเลขคอลัมน์/แถวทุก 5 ช่อง
                k = 3
                pv = im.resize((im.width * k, im.height * k), Image.NEAREST).convert("RGBA")
                d = ImageDraw.Draw(pv)
                step = (s["cell"] + s.get("gap", 0))
                for c in range(0, im.width // step + 1, 5):
                    d.line([(c * step * k, 0), (c * step * k, pv.height)], fill=(255, 0, 0, 160))
                    d.text((c * step * k + 2, 2), str(c), fill=(255, 255, 0, 255))
                for r in range(0, im.height // step + 1, 5):
                    d.line([(0, r * step * k), (pv.width, r * step * k)], fill=(255, 0, 0, 160))
                    d.text((2, r * step * k + 2), str(r), fill=(255, 255, 0, 255))
                pv.save(os.path.join(a.preview, f"{name}_grid.png"))
                break
        else:
            print(f"[sprites] ไม่ได้ภาพ {name} (ทุก URL ล้มเหลว) -> ข้าม")
    n = 0
    for e in maps:
        if not e.get("enabled") or e["sheet"] not in sheets:
            continue
        im, cell, gap = sheets[e["sheet"]]
        c, r = e["cell"]
        step = cell + gap
        box = (c * step, r * step, c * step + cell, r * step + cell)
        if box[2] > im.width or box[3] > im.height:
            print(f"[sprites] ช่อง {e['cell']} เกินขอบ {e['sheet']} -> ข้าม {e['out']}")
            continue
        im.crop(box).save(os.path.join(a.out, e["out"] + ".png"))
        n += 1
    print(f"[sprites] เขียน {n} ไฟล์ลง {a.out}")


if __name__ == "__main__":
    try:
        main()
    except Exception as ex:
        print(f"[sprites] ข้อผิดพลาด {ex!r} -> ข้าม (ไม่ล้ม build)")
    sys.exit(0)
