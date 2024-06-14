def rotate_bmp(input_path, output_path):
    with open(input_path, "rb") as f:
        data = f.read()
        assert data[:2] == b"BM"

        pixels = []
        offset = int.from_bytes(data[10:14], byteorder="little")
        width = int.from_bytes(data[18:22], byteorder="little")
        height = int.from_bytes(data[22:26], byteorder="little")

        # rotate

        for y in range(width):
            for x in range(height):
                ty, tx = x, width - y - 1
                new_idx = ty * width + tx
                # new_idx = (ty + tx * height)
                idx = offset + 3 * new_idx
                pixels.append(data[idx : idx + 3])

    with open(output_path, "wb") as f:
        f.write(data[:offset])
        f.write(b"".join(pixels))


if __name__ == "__main__":
    rotate_bmp("teapot.bmp", "teapot-rotated.bmp")
