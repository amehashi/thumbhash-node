# `thumbhash-node`

![https://github.com/amehashi/thumbhash-node/actions](https://github.com/amehashi/thumbhash-node/workflows/CI/badge.svg)

[`thumbhash`](https://github.com/evanw/thumbhash) binding for Node.js.



## Install this package

```
yarn add thumbhash-node
pnpm add thumbhash-node
npm install thumbhash-node
```

## Support matrix

### Operating Systems

|                  | node14 | node16 | node18 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      |
| Android arm64    | ✓      | ✓      | ✓      |
| Android armv7    | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      |

## API

```ts
/** Encodes an RGBA image to a ThumbHash. RGB should not be premultiplied by A. */
export function rgbaToThumbHash(w: number, h: number, rgba: Uint8Array): Uint8Array
/**
 * Extracts the approximate aspect ratio of the original image.
 * An error will be returned if the input is too short.
 */
export function thumbHashToApproximateAspectRatio(hash: Uint8Array): number
export interface Rgba {
  r: number
  g: number
  b: number
  a: number
}
/**
 * Extracts the average color from a ThumbHash.
 * Returns the RGBA values where each value ranges from 0 to 1.
 * RGB is not be premultiplied by A.
 * An error will be returned if the input is too short.
 */
export function thumbHashToAverageRGBA(hash: Uint8Array): Rgba
export interface Image {
  width: number
  height: number
  rgba: Uint8Array
}
/**
 * Decodes a ThumbHash to an RGBA image.
 * RGB is not be premultiplied by A.
 * Returns the width, height, and pixels of the rendered placeholder image.
 * An error will be returned if the input is too short.
 */
export function thumbHashToRGBA(hash: Uint8Array): Image

```