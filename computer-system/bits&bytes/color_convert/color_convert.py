import sys


def css_hex_val_convert_to_rgb(hex_str):
    typ = len(hex_str) - 1
    res = ""
    if typ == 6:
        r = int(hex_str[1:3], base=16)
        g = int(hex_str[3:5], base=16)
        b = int(hex_str[5:7], base=16)
        res = f"rgb({r} {g} {b})"
    elif typ == 8:
        r = int(hex_str[1:3], base=16)
        g = int(hex_str[3:5], base=16)
        b = int(hex_str[5:7], base=16)
        a = int(hex_str[7:9], base=16) / 255
        res = f"rgba({r} {g} {b} / {a:.5f})"
    elif typ == 3:
        r = int(hex_str[1] * 2, base=16)
        g = int(hex_str[2] * 2, base=16)
        b = int(hex_str[3] * 2, base=16)
        res = f"rgb({r} {g} {b})"
    elif typ == 4:
        r = int(hex_str[1] * 2, base=16)
        g = int(hex_str[2] * 2, base=16)
        b = int(hex_str[3] * 2, base=16)
        a = int(hex_str[4] * 2, base=16) / 255
        res = f"rgba({r} {g} {b} / {a:.5f})"
    return res


def test_main():
    test_cases = {
        "#000000": "rgb(0 0 0)",
        "#000001": "rgb(0 0 1)",
        "#ff0000": "rgb(255 0 0)",
        "#ffffff": "rgb(255 255 255)",
        "#0000FFC0": "rgba(0 0 255 / 0.75294)",
        "#00f8": "rgba(0 0 255 / 0.53333)",
        "#123": "rgb(17 34 51)",
        "#fff": "rgb(255 255 255)",
    }
    for val, expected in test_cases.items():
        actual = css_hex_val_convert_to_rgb(val)
        assert actual == expected, f"actual: {actual} != expected: {expected}"
    print("Pass~~~")


if __name__ == "__main__":
    test_main()
