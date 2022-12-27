export const randIntRange = (min: number, max: number) => {
  return Math.floor(Math.random() * (max - min + 1)) + min
}

function sfc32 (a: number, b: number, c: number, d: number) {
  return function () {
    a >>>= 0
    b >>>= 0
    c >>>= 0
    d >>>= 0
    let t = (a + b) | 0
    a = b ^ (b >>> 9)
    b = (c + (c << 3)) | 0
    c = (c << 21) | (c >>> 11)
    d = (d + 1) | 0
    t = (t + d) | 0
    c = (c + t) | 0
    return (t >>> 0) / 4294967296
  }
}
const seed = 1337 ^ 0xdeadbeef
export const generateRandFunction = () => sfc32(0x9e3779b9, 0x243f6a88, 0xb7e15162, seed)

export const walletTruncated = (walletAddress: string) => {
  return `${walletAddress.slice(0, 9)}...${walletAddress.slice(walletAddress.length - 4, walletAddress.length)}`
}
