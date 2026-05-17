import pytest
from japanese_address_parser_py import Parser, parse


def test_parse_function():
    result = parse("東京都目黒区上目黒2-19-15")
    assert result.address["prefecture"] == "東京都"
    assert result.address["city"] == "目黒区"
    assert result.address["town"] == "上目黒二丁目"
    assert result.address["rest"] == "19-15"
    assert result.error == {}


def test_parser_class():
    parser = Parser()
    result = parser.parse("京都府乙訓郡大山崎町字円明寺小字夏目3")
    assert result.address["prefecture"] == "京都府"
    assert result.address["city"] == "乙訓郡大山崎町"
    assert result.address["town"] == "字円明寺"
    assert result.address["rest"] == "小字夏目3"
    assert result.error == {}
