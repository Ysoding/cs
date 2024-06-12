def encode(number: int) -> bytes:
    output_bytes = []
    while number > 0:
        n = number & 0x7F
        number >>= 7
        if number > 0:  # add MSB bit
            n |= 0x80
        output_bytes.append(n)
    return bytes(output_bytes)


def decode(bdata: bytes) -> int:
    res = 0
    for x in reversed(bdata):
        res <<= 7
        res = res | (x & 0x7F)

    return res


if __name__ == "__main__":
    for name in [
        "1.uint64",
        "2.uint64",
        "150.uint64",
        "281474976710657.uint64",
        "maxint.uint64",
    ]:
        with open(name, "rb") as f:
            binary_data = f.read()
        num = int.from_bytes(binary_data, byteorder="big")
        print(f"binary_data: {binary_data} number value: {num}")
        print(f"encode: {encode(num)}")
        print(f"decode: {decode(encode(num))}")
        print("-------------------------")
