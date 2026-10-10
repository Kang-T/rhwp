"""SFNT cmap/hmtx에서 검증한 기술 기호 범위를 추출한다. 폰트 bytes는 저장하지 않는다."""

import argparse
import hashlib
import json
from pathlib import Path

from fontTools.ttLib import TTFont


def extract(path, family):
    raw = path.read_bytes()
    with TTFont(path) as font:
        families = {
            name.toUnicode()
            for name in font["name"].names
            if name.nameID in (1, 16)
        }
        if family not in families:
            raise ValueError("요청한 face가 SFNT family 이름에 없습니다")
        if font["head"].macStyle & 3:
            raise ValueError("이 추출기는 regular 프로그램만 지원합니다")
        cmap = font.getBestCmap()
        values = [
            font["hmtx"][cmap[codepoint]][0] if codepoint in cmap else 0
            for codepoint in range(0x2300, 0x2400)
        ]
        if not all(0 <= width <= 65535 for width in values):
            raise ValueError("advance가 u16 범위를 벗어납니다")
        return {
            "family": family,
            "fontSha256": hashlib.sha256(raw).hexdigest(),
            "em": font["head"].unitsPerEm,
            "start": 0x2300,
            "end": 0x23FF,
            "widths": values,
        }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--font", type=Path, required=True)
    parser.add_argument("--family", required=True)
    parser.add_argument("--rust-output", type=Path, required=True)
    parser.add_argument("--evidence-output", type=Path, required=True)
    args = parser.parse_args()
    if any(c in args.family for c in '\\"\n\r'):
        raise ValueError("잘못된 family 이름")
    protected = {"font_metrics_data.rs", "font_metrics_generated.rs", "font_metrics_overlays.rs"}
    if args.rust_output.resolve().name in protected:
        raise ValueError("기존 메트릭 영역을 덮어쓸 수 없습니다")
    record = extract(args.font, args.family)
    rows = [", ".join(map(str, record["widths"][i:i + 16])) for i in range(0, 256, 16)]
    source = (
        "// scripts/extract_verified_symbol_metrics.py의 SFNT cmap/hmtx 직접 추출.\n"
        f"// face: {record['family']}; SHA-256: {record['fontSha256']}; em: {record['em']}.\n"
        "// U+2300..U+23FF 전체 범위. 0은 실제 cmap 결측이며 기존 폭을 덮어쓰지 않는다.\n"
        "static VERIFIED_TECHNICAL_SYMBOL_WIDTHS: [u16; 256] = [\n"
        + "".join(f"    {row},\n" for row in rows)
        + "];\n\n"
        + "fn verified_technical_symbol_width(metric: &FontMetric, ch: char) -> Option<u16> {\n"
        + f'    if metric.name != "{record["family"]}" || metric.em_size != {record["em"]}\n'
        + "        || metric.bold || metric.italic || !(0x2300..=0x23FF).contains(&(ch as u32)) {\n"
        + "        return None;\n    }\n"
        + "    let width = VERIFIED_TECHNICAL_SYMBOL_WIDTHS[(ch as u32 - 0x2300) as usize];\n"
        + "    (width > 0).then_some(width)\n}\n"
    )
    args.rust_output.write_text(source, encoding="utf8")
    args.evidence_output.write_text(json.dumps(record, ensure_ascii=False, indent=2) + "\n", encoding="utf8")


if __name__ == "__main__":
    main()
