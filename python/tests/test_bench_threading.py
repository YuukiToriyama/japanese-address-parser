"""Benchmarks for verifying the GIL-release effect on ``Parser.parse``.

``Parser.parse`` releases the GIL via ``py.allow_threads`` so that concurrent
Python threads can overlap HTTP I/O while fetching Geolonia masters. These
benchmarks keep that behavior under regression watch.

Because the benchmarks perform real HTTP requests, they are disabled by
default (see ``[tool.pytest.ini_options]`` in ``pyproject.toml``). Pass
``--benchmark-enable`` to run them explicitly::

    uv run pytest tests/test_bench_threading.py \\
        --benchmark-enable --benchmark-only --benchmark-group-by=group
"""

from __future__ import annotations

from concurrent.futures import ThreadPoolExecutor

import pytest
from japanese_address_parser_py import Parser

from tests.fixtures.addresses import PREFECTURE_CAPITAL_ADDRESSES

# 外部 HTTP の負荷を抑えるため先頭 16 件に絞る
BENCH_ADDRESSES: list[str] = PREFECTURE_CAPITAL_ADDRESSES[:16]


@pytest.fixture(scope="module")
def parser() -> Parser:
    return Parser()


@pytest.fixture(scope="module")
def addresses() -> list[str]:
    return BENCH_ADDRESSES


def _run_serial(parser: Parser, addresses: list[str]) -> None:
    for addr in addresses:
        parser.parse(addr)


def _run_threaded(parser: Parser, addresses: list[str], n_threads: int) -> None:
    with ThreadPoolExecutor(max_workers=n_threads) as executor:
        # list() で全タスクの完了を待つ
        list(executor.map(parser.parse, addresses))


@pytest.mark.benchmark(group="serial_vs_threading")
def test_bench_serial(benchmark, parser: Parser, addresses: list[str]) -> None:
    """Baseline: parse all addresses sequentially on a single thread."""
    benchmark(_run_serial, parser, addresses)


@pytest.mark.benchmark(group="serial_vs_threading")
def test_bench_threaded_8(benchmark, parser: Parser, addresses: list[str]) -> None:
    """Parse all addresses across 8 worker threads."""
    benchmark(_run_threaded, parser, addresses, 8)


@pytest.mark.benchmark(group="thread_scaling")
@pytest.mark.parametrize("n_threads", [1, 2, 4, 8, 16])
def test_bench_scaling(
    benchmark, parser: Parser, addresses: list[str], n_threads: int
) -> None:
    """Sweep the worker-thread count to observe scaling behavior."""
    benchmark(_run_threaded, parser, addresses, n_threads)
