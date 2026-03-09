import { Cell, Universe } from "wasm-game-of-life"
import { memory } from "wasm-game-of-life/wgol_bg.wasm"

const CELL_SIZE = 5 // px
const GRID_COLOR = "#CCCCCC"
const DEAD_COLOR = "#FFFFFF"
const ALIVE_COLOR = "#000000"

const universe = Universe.new()
const width = universe.width()
const height = universe.height()

const canvas = document.getElementById("wgol-canvas")
canvas.height = (CELL_SIZE + 1) * height + 1
canvas.width = (CELL_SIZE + 1) * width + 1

const ctx = canvas.getContext("2d")
const getIndex = (row, column) => row * width + column

const drawGrid = () => {
  ctx.beginPath()
  ctx.strokeStyle = GRID_COLOR

  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0)
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1)
  }

  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1)
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1)
  }
  ctx.stroke()
}

const drawCells = () => {
  const cellsPtr = universe.cells()
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height)
  ctx.beginPath()

  // draw all live cells in one pass and all dead cells in another
  // to minimize calls to `CanvasRenderingContext2D.fillStyle`
  // profiling has shown this setter to be a performance bottleneck

  ctx.fillStyle = ALIVE_COLOR
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col)
      if (cells[idx] !== Cell.Alive) continue

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE,
      )
    }
  }

  ctx.fillStyle = DEAD_COLOR
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col)
      if (cells[idx] !== Cell.Dead) continue

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE,
      )
    }
  }
  ctx.stroke()
}

const fps = {
  fps: document.getElementById("fps"),
  frames: [],
  lastFrameTimeStamp: performance.now(),

  render() {
    const now = performance.now()
    const delta = now - this.lastFrameTimeStamp // milisecs
    this.lastFrameTimeStamp = now
    const fps = (1 / delta) * 1000
    this.frames.push(fps)
    if (this.frames.length > 100) this.frames.shift()

    let min = Infinity
    let max = -Infinity
    let sum = 0

    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i]
      min = Math.min(this.frames[i], min)
      max = Math.max(this.frames[i], max)
    }

    const mean = sum / this.frames.length
    this.fps.textContent = `\
frames per second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`
  },
}

const renderLoop = () => {
  fps.render()
  universe.tick()
  drawGrid()
  drawCells()
  requestAnimationFrame(renderLoop)
}

drawGrid()
drawCells()
requestAnimationFrame(renderLoop)
