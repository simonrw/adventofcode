import pytest
from day4 import analyse_part1, analyse_part2, is_anagram


@pytest.mark.parametrize('input,expected', [
    ([['aa', 'bb', 'cc', 'dd', 'ee']], 1),
    ([['aa', 'bb', 'cc', 'dd', 'aa']], 0),
    ([['aa', 'bb', 'cc', 'dd', 'aaa']], 1),
])
def test_part_one(input, expected):
    assert analyse_part1(input) == expected


@pytest.mark.parametrize('input,expected', [
    ([['abcde', 'fghij']], 1),
    ([['abcde', 'xyz', 'ecdab']], 0),
])
def test_part_two(input, expected):
    assert analyse_part2(input) == expected


@pytest.mark.parametrize('input,expected', [
    (('abc', 'cba'), True),
    (('abc', 'cbd'), False),
])
def test_is_anagram(input, expected):
    assert is_anagram(*input) == expected
