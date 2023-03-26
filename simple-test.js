const { join } = require('path')
const { cwd } = require('process')

const { loadImage, createCanvas } = require('@napi-rs/canvas')

const { rgbaToThumbHash } = require('./index.js')

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

const run = async () => {
  const { height, width, rgba } = await loadImageAndConvertToRgba()

  rgbaToThumbHash(width, height, rgba)
}

run()
