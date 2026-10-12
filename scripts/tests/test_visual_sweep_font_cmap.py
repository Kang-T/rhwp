"""cmap 재사용이 변경·손상된 글꼴을 정상 글꼴로 오판하지 않는지 검사한다."""

import io
import base64
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.ttLib import TTFont

from scripts import visual_sweep_font_cmap as cmap
from scripts import visual_sweep as sweep


def font_bytes(characters: str, remove_cmap: bool = False) -> bytes:
    builder = FontBuilder(1000, isTTF=True)
    glyphs = [".notdef", *[f"g{ord(ch)}" for ch in characters]]
    builder.setupGlyphOrder(glyphs)
    builder.setupCharacterMap({ord(ch): f"g{ord(ch)}" for ch in characters})
    builder.setupGlyf({glyph: TTGlyphPen(None).glyph() for glyph in glyphs})
    builder.setupHorizontalMetrics({glyph: (500, 0) for glyph in glyphs})
    builder.setupHorizontalHeader(ascent=800, descent=-200)
    builder.setupNameTable({"familyName": "CacheContract", "styleName": "Regular"})
    builder.setupOS2(sTypoAscender=800, sTypoDescender=-200, usWinAscent=800, usWinDescent=200)
    builder.setupPost()
    if remove_cmap:
        del builder.font["cmap"]
    output = io.BytesIO()
    builder.save(output)
    return output.getvalue()


class EmbeddedFontCmapCacheTests(unittest.TestCase):
    def setUp(self):
        cmap._UNICODE_COUNTS.clear()

    def tearDown(self):
        cmap._UNICODE_COUNTS.clear()

    def test_identical_font_bytes_are_parsed_once(self):
        data = font_bytes("AB")
        with patch("fontTools.ttLib.TTFont", wraps=TTFont) as parser:
            first = cmap.embedded_font_unicode_count(data)
            self.assertEqual(cmap.embedded_font_unicode_count(bytes(data)), first)
            self.assertEqual(parser.call_count, 1)
        self.assertEqual(first[1], 2)

    def test_changed_and_missing_cmap_bytes_are_not_accepted_from_cache(self):
        first = cmap.embedded_font_unicode_count(font_bytes("A"))
        second = cmap.embedded_font_unicode_count(font_bytes("AB"))
        self.assertNotEqual(first[0], second[0])
        self.assertEqual((first[1], second[1]), (1, 2))
        invalid = font_bytes("A", remove_cmap=True)
        for _ in range(2):
            with self.assertRaisesRegex(ValueError, "Unicode cmap"):
                cmap.embedded_font_unicode_count(invalid)
        with self.assertRaises(Exception):
            cmap.embedded_font_unicode_count(b"not a font")
        self.assertEqual(len(cmap._UNICODE_COUNTS), 2)

    def test_cache_does_not_retain_an_unbounded_set_of_fonts(self):
        with patch.object(cmap, "_CACHE_LIMIT", 2):
            for characters in ["A", "AB", "ABC"]:
                self.assertEqual(cmap.embedded_font_unicode_count(font_bytes(characters))[1], len(characters))
        self.assertEqual(len(cmap._UNICODE_COUNTS), 2)

    def test_svg_payload_keeps_unicode_whitespace_and_strict_base64_validation(self):
        payload = base64.b64encode(font_bytes("AB")).decode("ascii")
        wrapped = "\n\t \u2003\u0085".join(payload[i:i + 40] for i in range(0, len(payload), 40))
        with tempfile.TemporaryDirectory() as directory:
            svg = Path(directory) / "font.svg"
            def inspect(encoded):
                svg.write_text(f'<svg><style>@font-face{{font-family:Fixture;src:url(data:font/ttf;base64,{encoded})}}</style></svg>')
                return sweep.inspect_svg_embedded_fonts(svg)[0]
            valid = inspect(wrapped)
            self.assertEqual(valid['status'], 'passed')
            self.assertEqual(valid['unicode_mapping_count'], 2)
            self.assertEqual(inspect(wrapped + '@')['status'], 'failed')


if __name__ == "__main__":
    unittest.main()
