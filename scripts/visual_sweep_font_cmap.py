"""같은 실행에서 반복 임베딩된 전체 글꼴의 cmap 검증 결과만 재사용한다."""

import hashlib
import io

# 원본 바이트는 저장하지 않는다. 새 바이트는 매번 해시를 계산해 별도로 검사한다.
_UNICODE_COUNTS: dict[str, int] = {}
_CACHE_LIMIT = 128


def embedded_font_unicode_count(data: bytes) -> tuple[str, int]:
    """실제 바이트의 SHA와 검증된 Unicode 매핑 수를 반환한다."""
    from fontTools.ttLib import TTFont

    digest = hashlib.sha256(data).hexdigest()
    count = _UNICODE_COUNTS.get(digest)
    if count is None:
        with TTFont(io.BytesIO(data), fontNumber=0) as font:
            cmap = font.getBestCmap() if "cmap" in font else None
            if not cmap:
                raise ValueError("Unicode cmap이 없거나 비어 있음")
            count = len(cmap)
        if len(_UNICODE_COUNTS) >= _CACHE_LIMIT:
            _UNICODE_COUNTS.pop(next(iter(_UNICODE_COUNTS)))
        _UNICODE_COUNTS[digest] = count
    return digest, count
