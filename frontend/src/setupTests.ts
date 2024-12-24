import "@testing-library/jest-dom";

Object.assign(global, {
  TextEncoder: function () {
    return {
      encode: function (str: string) {
        return new Uint8Array([...str].map((c) => c.charCodeAt(0)));
      },
    };
  },
  TextDecoder: function () {
    return {
      decode: function (arr: Uint8Array) {
        return String.fromCharCode(...arr);
      },
    };
  },
});
