import pytest
from day4 import analyse_part1


@pytest.mark.parametrize('input,expected', [
    ([['aa', 'bb', 'cc', 'dd', 'ee']], 1),
    ([['aa', 'bb', 'cc', 'dd', 'aa']], 0),
    ([['aa', 'bb', 'cc', 'dd', 'aaa']], 1),
])
def test_part_one(input, expected):
    assert analyse_part1(input) == expected
