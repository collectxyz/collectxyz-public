export interface XyzResponse {
  extension: {
    arrival: number
    coordinates: {
      x: number
      y: number
      z: number
    }
    prev_coordinates: {
      x: number
      y: number
      z: number
    } | null
  }
  description: string
  image: string
  name: string
  approvals: unknown
  owner: string
}
