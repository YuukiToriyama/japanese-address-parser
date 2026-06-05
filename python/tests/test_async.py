import asyncio
import pytest

from japanese_address_parser_py import Parser, parse_async


@pytest.mark.asyncio
async def test_parse_async_function():
    """Standalone parse_async function test."""
    result = await parse_async("東京都目黒区上目黒2-19-15")

    assert result.address["prefecture"] == "東京都"
    assert result.address["city"] == "目黒区"
    assert result.address["town"] == "上目黒二丁目"
    assert result.address["rest"] == "19-15"
    assert result.error == {}


@pytest.mark.asyncio
async def test_parse_async_concurrency():
    """Test if multiple async calls can run concurrently."""
    addresses = [
        "神奈川県横浜市港南区港南四丁目2番10号",
        "神奈川県横浜市南区浦舟町2丁目33番地",
        "神奈川県横浜市緑区寺山町118番地",
    ]
    tasks = [parse_async(addr) for addr in addresses]
    results = await asyncio.gather(*tasks)

    assert len(results) == 3
    assert results[0].address["prefecture"] == "神奈川県"
    assert results[0].address["city"] == "横浜市港南区"
    assert results[1].address["prefecture"] == "神奈川県"
    assert results[1].address["city"] == "横浜市南区"
    assert results[2].address["prefecture"] == "神奈川県"
    assert results[2].address["city"] == "横浜市緑区"


@pytest.mark.asyncio
async def test_parser_parse_async_method():
    """Parser::parse_async method test."""
    parser = Parser()
    result = await parser.parse_async("東京都江戸川区中央一丁目4番1号")

    assert result.address["prefecture"] == "東京都"
    assert result.address["city"] == "江戸川区"
    assert result.address["town"] == "中央一丁目"
    assert result.address["rest"] == "4番1号"
    assert result.error == {}


@pytest.mark.asyncio
async def test_parser_parse_async_concurrency():
    """Parser::parse_async concurrency test."""
    parser = Parser()
    addresses = [
        "神奈川県横浜市都筑区茅ケ崎中央32番1号",
        "神奈川県横浜市神奈川区広台太田町3番地8",
        "神奈川県横浜市鶴見区鶴見中央三丁目20番1号",
    ]
    tasks = [parser.parse_async(addr) for addr in addresses]
    results = await asyncio.gather(*tasks)

    assert len(results) == 3
    assert results[0].address["prefecture"] == "神奈川県"
    assert results[0].address["city"] == "横浜市都筑区"
    assert results[1].address["prefecture"] == "神奈川県"
    assert results[1].address["city"] == "横浜市神奈川区"
    assert results[2].address["prefecture"] == "神奈川県"
    assert results[2].address["city"] == "横浜市鶴見区"
