#define PY_SSIZE_T_CLEAN
#include <Python.h>

static PyObject *cvarint_encode(PyObject *self, PyObject *args) {
  unsigned long long n = 0;

  if (!PyArg_ParseTuple(args, "K", &n)) {
    return NULL;
  }

  char part;
  char out[10];
  int i = 0;

  while (n > 0) {
    part = n & 0x7F;
    n >>= 7;
    if (n > 0) {
      part |= 0x80;
    }

    out[i++] = part;
  }

  return PyBytes_FromStringAndSize(out, i);
}

static PyObject *cvarint_decode(PyObject *self, PyObject *args) {
  Py_buffer varints;

  if (!PyArg_ParseTuple(args, "y*", &varints)) {
    return NULL;
  }

  unsigned long long n = 0;
  char *data = varints.buf;
  Py_ssize_t length = varints.len;

  for (int i = length - 1; i >= 0; i--) {
    n <<= 7;
    n |= (data[i] & 0x7F);
  }

  return PyLong_FromUnsignedLongLong(n);
}

static PyMethodDef CVarintMethods[] = {
    {"encode", cvarint_encode, METH_VARARGS, "Encode an integer as varint."},
    {"decode", cvarint_decode, METH_VARARGS,
     "Decode varint bytes to an integer."},
    {NULL, NULL, 0, NULL}};

static struct PyModuleDef cvarintmodule = {
    PyModuleDef_HEAD_INIT, "cvarint",
    "A C implementation of protobuf varint encoding", -1, CVarintMethods};

PyMODINIT_FUNC PyInit_cvarint(void) { return PyModule_Create(&cvarintmodule); }