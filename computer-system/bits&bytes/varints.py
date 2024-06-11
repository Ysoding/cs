def encode(num):
    output_bytes = []
    while True:
        has_next = ((num >> 7) & 0x7F) != 0
        if has_next:
            output_bytes.append((num & 0x7F) | 0x80)
        else:
            output_bytes.append(num & 0x7F)
            break
        num >>= 7
    return bytes(output_bytes)


def decode(binary_data):
    # big-edian
    return 0


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
