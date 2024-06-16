import struct
import math

# https://en.wikipedia.org/wiki/Double-precision_floating-point_format


def conceal(six_bytes_msg: str):
    if len(six_bytes_msg) != 6:
        raise ValueError("need six bytes")

    # pack to 6 bytes
    msg_bytes = six_bytes_msg.encode("utf-8")
    msg_int = int.from_bytes(msg_bytes, byteorder="big")

    nan_bits = (0x7FF << 52) | msg_int

    # pack into a double-precision float
    val = struct.unpack("d", struct.pack("q", nan_bits))[0]

    return val


def extract(nan_val):
    if not math.isnan(nan_val):
        raise ValueError("Input value is not NaN")
    nan_bits = struct.unpack("q", struct.pack("d", nan_val))[0]
    msg_int: int = nan_bits & ((1 << 52) - 1)
    msg_bytes = msg_int.to_bytes(6, byteorder="big")
    return msg_bytes.decode("utf-8")


if __name__ == "__main__":
    x = conceal("hello!")
    print(x)  # Should print nan
    print(type(x))  # <class 'float'>
    print(x + 5)  # nan
    print(x / 2)  # nan
    print(x)  # nan
    print(extract(x))  # 'hello!'
