#!/usr/bin/env python3
"""สร้างเพลง BGM สเตอริโอ (2-3 นาที/เพลง) + SFX ลง assets/audio (deterministic, ใช้แค่ numpy)

  bgm_0 = กลางวัน/สำรวจ   bgm_1 = ค่าย/ฐาน   bgm_2 = บอส
  bgm_3 = กลางคืน        bgm_4 = ถ้ำ/ดันเจี้ยน   bgm_5 = เมนูหลัก
  sfx_<ชื่อ> = เอฟเฟกต์ตามฟิลด์ใน Sfx (audio.rs)

ใช้:  python3 tools/gen_audio.py [--out assets/audio] [--secs 160] [--sr 22050]
"""
import argparse, os, sys, wave
import numpy as np

TRACKS = {
    # idx: (ชื่อ, bpm, root midi, scale, progression (องศาของคอร์ด, minor?), seed)
    0: ("day",   104, 67, "maj", [(0, 0), (5, 1), (3, 0), (4, 0)], 11),
    1: ("camp",   84, 65, "maj", [(0, 0), (3, 0), (4, 0), (5, 1)], 22),
    2: ("boss",  150, 50, "min", [(0, 1), (5, 0), (3, 0), (6, 0)], 33),
    3: ("night",  62, 57, "min", [(0, 1), (5, 0), (3, 0), (4, 1)], 44),
    4: ("cave",   50, 50, "phr", [(0, 1), (1, 0), (0, 1), (6, 0)], 55),
    5: ("menu",   72, 60, "maj", [(0, 0), (4, 0), (5, 1), (3, 0)], 66),
}
SCALES = {"maj": [0, 2, 4, 5, 7, 9, 11], "min": [0, 2, 3, 5, 7, 8, 10], "phr": [0, 1, 3, 5, 7, 8, 10]}


def mhz(m):
    return 440.0 * 2.0 ** ((np.asarray(m, dtype=np.float64) - 69.0) / 12.0)


class Synth:
    def __init__(self, sr, secs):
        self.sr = sr
        self.n = int(sr * secs)
        self.L = np.zeros(self.n, np.float32)
        self.R = np.zeros(self.n, np.float32)
        self.rng = np.random.default_rng(0)

    def _put(self, t0, wave_, pan, vol):
        i = int(t0 * self.sr)
        if i >= self.n:
            return
        m = min(len(wave_), self.n - i)
        # pan: -1..1 (equal power)
        a = (pan + 1) * np.pi / 4
        self.L[i:i + m] += wave_[:m] * vol * np.cos(a)
        self.R[i:i + m] += wave_[:m] * vol * np.sin(a)

    def tone(self, t0, dur, hz, kind, vol=0.3, pan=0.0):
        sr = self.sr
        rel = {"pad": 1.2, "bell": 2.5, "pluck": 0.4, "bass": 0.15, "lead": 0.25, "brass": 0.2, "drone": 2.0, "flute": 0.3}.get(kind, 0.3)
        m = int((dur + rel) * sr)
        t = np.arange(m) / sr
        hz = float(hz)
        w = np.zeros(m)

        def harm(hs, amps, det=0.0, vib=0.0):
            out = np.zeros(m)
            f = hz * (1 + det) * (1 + vib * np.sin(2 * np.pi * 5.2 * t) * np.minimum(1, t / 0.4))
            ph = 2 * np.pi * np.cumsum(f) / sr
            for h, a in zip(hs, amps):
                if hz * h < sr * 0.45:
                    out += a * np.sin(h * ph)
            return out

        if kind == "pad":
            for d in (-0.004, 0.0, 0.004):
                w += harm([1, 2, 3, 4], [1, .45, .25, .1], d) / 3
            att, dec = min(1.2, dur * 0.45), 0
            env = np.minimum(1, t / att) * np.where(t < dur, 1, np.exp(-(t - dur) / (rel * 0.35)))
        elif kind == "drone":
            w = harm([1, 2, 3], [1, .5, .2], 0.002) + harm([1], [.6], -0.003)
            env = np.minimum(1, t / 1.5) * np.where(t < dur, 1, np.exp(-(t - dur) / (rel * 0.4)))
        elif kind == "pluck":
            w = harm([1, 2, 3, 4, 5], [1, .5, .3, .15, .08])
            env = np.exp(-t * 5.5) * np.minimum(1, t / 0.004)
        elif kind == "bell":
            w = sum(a * np.sin(2 * np.pi * hz * r * t) * np.exp(-t * dc) for r, a, dc in ((1, 1, 1.6), (2.76, .5, 2.4), (5.4, .25, 3.6), (8.9, .1, 5)))
            env = np.minimum(1, t / 0.003)
        elif kind == "bass":
            w = harm([1, 2, 3, 4, 5, 6], [1, .6, .35, .2, .1, .05])
            env = np.minimum(1, t / 0.01) * np.where(t < dur, np.exp(-t * 1.5), 0) * 1.0
            env = env + 0
        elif kind == "lead":
            w = harm([1, 3, 5, 7], [1, .33, .2, .14], 0.0, 0.004)
            env = np.minimum(1, t / 0.02) * np.where(t < dur, 1, np.exp(-(t - dur) / 0.08))
        elif kind == "flute":
            w = harm([1, 2, 3], [1, .18, .06], 0.0, 0.006) + 0.03 * self.rng.standard_normal(m) * np.minimum(1, t / 0.05)
            env = np.minimum(1, t / 0.07) * np.where(t < dur, 1, np.exp(-(t - dur) / 0.12))
        else:  # brass
            w = harm([1, 2, 3, 4, 5, 6, 7, 8], [1, .8, .6, .5, .35, .25, .15, .1])
            env = np.minimum(1, t / 0.05) * np.where(t < dur, 1, np.exp(-(t - dur) / 0.1))
        self._put(t0, (w * env).astype(np.float32), pan, vol)

    def kick(self, t0, vol=0.55):
        m = int(0.28 * self.sr)
        t = np.arange(m) / self.sr
        f = 45 + 110 * np.exp(-t * 28)
        w = np.sin(2 * np.pi * np.cumsum(f) / self.sr) * np.exp(-t * 11)
        self._put(t0, w.astype(np.float32), 0, vol)

    def snare(self, t0, vol=0.3):
        m = int(0.22 * self.sr)
        t = np.arange(m) / self.sr
        w = (self.rng.standard_normal(m) * 0.7 + np.sin(2 * np.pi * 190 * t) * 0.5) * np.exp(-t * 20)
        self._put(t0, w.astype(np.float32), 0.1, vol)

    def hat(self, t0, vol=0.1, pan=0.3, open_=False):
        m = int((0.18 if open_ else 0.05) * self.sr)
        t = np.arange(m) / self.sr
        nz = np.diff(self.rng.standard_normal(m + 1))
        self._put(t0, (nz * np.exp(-t * (14 if open_ else 70))).astype(np.float32), pan, vol)

    def finish(self, reverb=0.28, delay=None):
        sr, n = self.sr, self.n
        L, R = self.L.astype(np.float64), self.R.astype(np.float64)
        if delay:  # ping-pong: ตั้งเวลาเป็นวินาที, วนรอบ (เพลงวนลูปได้ไร้รอยต่อ)
            d = int(delay * sr)
            for k, g in enumerate((0.32, 0.18, 0.1), 1):
                L += np.roll(R, d * k) * g * (1 if k % 2 else 0.6)
                R += np.roll(L, d * k) * g * (1 if k % 2 == 0 else 0.6)
        # รีเวิร์บสเตอริโอด้วย FFT (วงกลม => ท้ายเพลงไหลกลับไปต้นเพลง ลูปเนียน)
        rl = int(1.8 * sr)
        tt = np.arange(rl) / sr
        out = []
        for ch, seed in ((L, 1), (R, 2)):
            r = np.random.default_rng(seed).standard_normal(rl) * np.exp(-tt * 2.8)
            r = np.convolve(r, np.ones(6) / 6, "same")  # ตัดแหลม
            r /= np.abs(r).sum() * 0.5
            N = 1 << (n - 1).bit_length()
            wet = np.fft.irfft(np.fft.rfft(ch, N) * np.fft.rfft(r, N), N)[:n]
            wet[: rl] += 0  # (ไม่ wrap เต็ม ใช้แค่ ZP — ท้ายเพลงมีเงียบอยู่แล้ว)
            out.append(ch + wet * reverb)
        L, R = out
        # ขยายสเตอริโอเล็กน้อย (mid/side)
        mid, side = (L + R) / 2, (L - R) / 2
        L, R = mid + side * 1.25, mid - side * 1.25
        pk = max(np.abs(L).max(), np.abs(R).max(), 1e-9)
        L, R = np.tanh(L / pk * 1.15) / np.tanh(1.15), np.tanh(R / pk * 1.15) / np.tanh(1.15)
        st = np.stack([L, R], 1) * 0.89
        # TPDF dither
        rng = np.random.default_rng(5)
        st += (rng.random(st.shape) - rng.random(st.shape)) / 32768.0
        return np.clip(st * 32767, -32768, 32767).astype(np.int16)


def compose(idx, secs, sr):
    name, bpm, root, scale_name, prog, seed = TRACKS[idx]
    rng = np.random.default_rng(seed)
    sc = SCALES[scale_name]
    beat = 60.0 / bpm
    bar = beat * 4
    bars = max(8, int(round(secs / bar / 8)) * 8)  # คูณ 8 บาร์
    s = Synth(sr, bars * bar + 3.0)
    s.rng = rng

    def deg(d, octv=0):  # องศาในสเกล -> midi
        return root + sc[d % 7] + 12 * (d // 7) + 12 * octv

    def chord_notes(c):
        d, _ = prog[c]
        return [deg(d), deg(d + 2), deg(d + 4)]

    # โครงเพลง: (จำนวนบาร์ 8 บาร์/ส่วน) ชั้นเสียงที่เปิด
    n8 = bars // 8
    layout = ["intro"] + ["A", "B", "C", "A", "B"] * 8
    sections = layout[:n8]
    if n8 >= 2:
        sections[-1] = "outro"
    # motif สำหรับ lead ต่อ section (4 บาร์ วนซ้ำ 2 ครั้ง)
    motifs = {}
    for key in ("A", "B", "C"):
        m = []
        pos = 4 + int(rng.integers(0, 3))
        for step in range(16):  # 16 ช่อง (ตัวขาว/ดำ ผสม)
            pos = int(np.clip(pos + rng.choice([-2, -1, -1, 0, 1, 1, 2]), 0, 11))
            m.append(None if rng.random() < 0.28 else pos)
        motifs[key] = m

    heavy = name == "boss"
    sparse = name in ("night", "cave")
    for si, sec in enumerate(sections):
        for b in range(8):
            bi = si * 8 + b
            t0 = bi * bar
            ci = (b // 2) % len(prog)
            cn = chord_notes(ci)
            on = {
                "intro": {"pad"},
                "A": {"pad", "bass", "arp"},
                "B": {"pad", "bass", "arp", "lead", "drum"},
                "C": {"pad", "lead", "bass"},
                "outro": {"pad", "arp"},
            }[sec]
            # แพด: 1 คอร์ดต่อ 2 บาร์ (วางตอนบาร์คู่)
            if "pad" in on and b % 2 == 0:
                for k, nn in enumerate(cn):
                    s.tone(t0, bar * 2, mhz(nn - 12 if idx != 4 else nn - 24), "pad" if idx != 4 else "drone", 0.20 if not heavy else 0.16, -0.5 + k * 0.5)
            # เบส
            if "bass" in on:
                bn = cn[0] - 24
                if heavy:
                    for q in range(8):
                        s.tone(t0 + q * beat / 2, beat / 2 * 0.9, mhz(bn + (7 if q % 4 == 3 else 0)), "bass", 0.30, -0.05)
                elif sparse:
                    s.tone(t0, bar * 0.9, mhz(bn), "bass", 0.28, 0)
                else:
                    for q, off in enumerate((0, 0, 7, 0)):
                        s.tone(t0 + q * beat, beat * 0.9, mhz(bn + off), "bass", 0.26, -0.05)
            # อาร์เปจโจ
            if "arp" in on:
                step = beat / 2 if not sparse else beat
                cnt = int(bar / step)
                for q in range(cnt):
                    nn = cn[(q if q % 8 < 4 else 8 - q % 8) % 3] + 12 + (12 if q % 4 == 3 else 0)
                    kind = "bell" if sparse or idx == 5 else "pluck"
                    s.tone(t0 + q * step, step * 0.9, mhz(nn), kind, 0.12, ((q % 4) - 1.5) * 0.45)
            # ลีด
            if "lead" in on:
                mo = motifs[sec if sec in motifs else "A"]
                sl = bar / 4 * (2 if sparse else 1)
                for q in range(4):
                    d = mo[((b % 4) * 4 + q) % 16]
                    if d is None:
                        continue
                    kind = "brass" if heavy else ("flute" if idx in (1, 5) else "lead")
                    nn = deg(d, 1) if sec != "C" else deg(d + 2, 1)
                    s.tone(t0 + q * sl, sl * 0.85, mhz(nn), kind, 0.15 if not heavy else 0.13, 0.25 if q % 2 else -0.25)
            # กลอง
            if "drum" in on and idx in (0, 1, 2, 5):
                for q in range(4):
                    tq = t0 + q * beat
                    if heavy:
                        s.kick(tq)
                        s.kick(tq + beat * 0.75, 0.32) if q % 2 else None
                        if q in (1, 3):
                            s.snare(tq)
                        s.hat(tq, 0.1, -0.3)
                        s.hat(tq + beat / 2, 0.1, 0.3)
                    else:
                        if q % 2 == 0:
                            s.kick(tq, 0.34)
                        else:
                            s.snare(tq, 0.16)
                        s.hat(tq + beat / 2, 0.07, 0.35 if q % 2 else -0.35)
            elif heavy and "pad" in on:  # บอสต้องมีจังหวะแม้ไม่ใช่ส่วน B
                for q in range(4):
                    s.kick(t0 + q * beat, 0.4)
            # ถ้ำ: เสียงหยดน้ำสุ่ม + ลมหวีด
            if idx == 4:
                for _ in range(2):
                    tt = t0 + float(rng.random()) * bar
                    s.tone(tt, 0.1, mhz(root + 24 + sc[int(rng.integers(0, 7))]), "bell", 0.1, float(rng.random()) * 2 - 1)
            # กลางคืน: เสียงระฆังโน้ตห่างๆ
            if idx == 3 and b % 2 == 1:
                s.tone(t0 + beat, 1, mhz(deg(int(rng.integers(2, 9)), 1)), "bell", 0.1, float(rng.random()) * 2 - 1)
    return s.finish(reverb=0.34 if sparse or idx == 5 else 0.22, delay=beat * 0.75)


# ---------------------------------------------------------------- SFX
def sfx_bank(sr=44100):
    rng = np.random.default_rng(99)

    def tt(d):
        return np.arange(int(d * sr)) / sr

    def noise(d, f=1.0):
        n = rng.standard_normal(int(d * sr))
        if f < 1:  # lowpass หยาบๆ
            k = max(1, int(1 / f))
            n = np.convolve(n, np.ones(k) / k, "same")
        return n

    def sweep(d, f0, f1, wave="sin"):
        t = tt(d)
        f = f0 * (f1 / f0) ** (t / d)
        ph = 2 * np.pi * np.cumsum(f) / sr
        return np.sign(np.sin(ph)) * 0.6 if wave == "sq" else np.sin(ph)

    def env(w, dec, att=0.003):
        t = tt(len(w) / sr)
        return w * np.exp(-t * dec) * np.minimum(1, t / att)

    def seq(notes, nd=0.09, wave="sq", dec=7):
        return np.concatenate([env(sweep(nd, f, f, wave), dec) for f in notes])

    def mix(*ws):
        L = max(len(w) for w in ws)
        o = np.zeros(L)
        for w in ws:
            o[: len(w)] += w
        return o

    B = {}
    B["swing"] = env(noise(0.16, 0.2) * np.linspace(0.2, 1, int(.16 * sr)), 14) * 0.9
    B["hit"] = mix(env(noise(0.12, 0.4), 30), env(sweep(0.12, 220, 80), 25))
    B["chop"] = mix(env(noise(0.1, 0.3), 40), env(sweep(0.14, 160, 70, "sq"), 22) * 0.5)
    B["rock"] = mix(env(noise(0.16, 0.5), 28), env(sweep(0.12, 900, 400, "sq"), 40) * 0.3)
    B["hurt"] = mix(env(sweep(0.3, 330, 90, "sq"), 8) * 0.7, env(noise(0.2, 0.4), 20) * 0.5)
    B["kill"] = mix(env(sweep(0.4, 400, 60, "sq"), 7) * 0.6, env(noise(0.3, 0.3), 12))
    B["gem"] = seq([1319, 1760, 2093], 0.07, "sin", 9)
    B["lvl"] = seq([523, 659, 784, 1047, 1319], 0.1, "sq", 5) * 0.7
    B["craft"] = mix(env(sweep(0.08, 900, 600, "sq"), 30) * 0.5, np.concatenate([np.zeros(int(.09 * sr)), env(sweep(0.16, 1200, 1200), 12) * 0.6]))
    B["eat"] = env(noise(0.22, 0.25) * (np.sin(2 * np.pi * 18 * tt(.22)) > 0), 9) * 0.7
    B["fire"] = env(noise(0.4, 0.12), 6) * 0.8
    B["chest"] = seq([440, 554, 659, 880], 0.1, "sq", 6) * 0.7
    B["shoot"] = env(sweep(0.12, 900, 250, "sq"), 18) * 0.6
    B["boom"] = mix(env(noise(0.7, 0.08), 5), env(sweep(0.6, 90, 28), 5))
    B["buy"] = seq([880, 1175], 0.07, "sin", 10)
    B["door"] = mix(env(sweep(0.18, 120, 70), 14), env(noise(0.1, 0.3), 30) * 0.4)
    B["dash"] = env(noise(0.2, 0.15) * np.linspace(1, 0.1, int(.2 * sr)), 10) * 0.8
    B["crit"] = mix(seq([1100, 1500], 0.06, "sq", 14), env(noise(0.1, 0.5), 30) * 0.5)
    B["pickup"] = env(sweep(0.12, 700, 1400), 12) * 0.8
    B["upgrade"] = seq([392, 494, 587, 784, 988, 1175], 0.09, "sq", 4) * 0.7
    B["roar"] = mix(env(sweep(0.9, 90, 45, "sq"), 2.5) * 0.6, env(noise(0.8, 0.1), 3))
    B["battk"] = mix(env(sweep(0.3, 220, 80, "sq"), 7) * 0.5, env(noise(0.25, 0.3), 12))
    B["quest"] = seq([659, 659, 784, 988], 0.11, "sq", 5) * 0.7
    B["trap"] = mix(env(sweep(0.16, 500, 120, "sq"), 14) * 0.5, env(noise(0.14, 0.5), 25))
    B["fball"] = mix(env(sweep(0.3, 600, 200), 7), env(noise(0.28, 0.15), 8) * 0.6)
    B["deny"] = env(sweep(0.14, 200, 110, "sq"), 15) * 0.6
    B["click"] = env(sweep(0.04, 1000, 900), 60) * 0.7
    return B, sr


def write_wav(path, data, sr):
    with wave.open(path, "wb") as w:
        w.setnchannels(data.shape[1] if data.ndim == 2 else 1)
        w.setsampwidth(2)
        w.setframerate(sr)
        w.writeframes(data.astype("<i2").tobytes())


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default="assets/audio")
    ap.add_argument("--secs", type=float, default=140)
    ap.add_argument("--sr", type=int, default=22050)
    ap.add_argument("--only", default="", help="เช่น 0,3  (ค่าเริ่มต้น: ทุกเพลง)")
    a = ap.parse_args()
    os.makedirs(a.out, exist_ok=True)
    want = [int(x) for x in a.only.split(",") if x] or list(TRACKS)
    total = 0
    for i in want:
        st = compose(i, a.secs, a.sr)
        p = os.path.join(a.out, f"bgm_{i}.wav")
        write_wav(p, st, a.sr)
        total += os.path.getsize(p)
        print(f"bgm_{i} ({TRACKS[i][0]}): {len(st) / a.sr:.0f}s stereo {os.path.getsize(p) / 1e6:.1f} MB", flush=True)
    if not a.only:
        # v15: เอฟเฟกต์/เสียงเดิน/บรรยากาศ สังเคราะห์แบบสมจริง 44.1 kHz (tools/sound_hq.py)
        sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
        import sound_hq
        sound_hq.generate(a.out)
        for f in os.listdir(a.out):
            if f.startswith(("sfx_", "step_", "amb_", "mob_")):
                total += os.path.getsize(os.path.join(a.out, f))
    print(f"รวมเสียง {total / 1e6:.1f} MB")


if __name__ == "__main__":
    sys.exit(main())
