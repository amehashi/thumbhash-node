import { join } from 'path'
import { cwd } from 'process'

import { loadImage, createCanvas } from '@napi-rs/canvas'
import b from 'benny'

import { rgbaToThumbHash as rgbaToThumbHashNode } from '../index.js'

async function loadImageAndConvertToRgba() {
  const maxSize = 100
  const imgPath = join(cwd(), './benchmark/fixture/img.png')
  const image = await loadImage(imgPath)
  const width = image.width
  const height = image.height

  const scale = Math.min(maxSize / width, maxSize / height)
  const resizedWidth = Math.round(width * scale)
  const resizedHeight = Math.round(height * scale)

  const canvas = createCanvas(resizedWidth, resizedHeight)
  const ctx = canvas.getContext('2d')
  ctx.drawImage(image, 0, 0, canvas.width, canvas.height)

  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height)
  const rgba = new Uint8Array(imageData.data.buffer)

  return {
    height: imageData.height,
    width: imageData.width,
    rgba,
  }
}

async function run() {
  const { height, width, rgba } = await loadImageAndConvertToRgba()

  await b.suite(
    'to thumbhash',

    b.add('thumbhash', async () => {
      const rgbaToThumbHash = await import('thumbhash').then((mod) => mod.rgbaToThumbHash)
      rgbaToThumbHash(width, height, rgba)
    }),

    b.add('thumbhash-node', () => {
      rgbaToThumbHashNode(width, height, rgba)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
