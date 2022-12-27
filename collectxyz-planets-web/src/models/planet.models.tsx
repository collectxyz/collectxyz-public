export enum ResourceIdentifier {
  Ice = 'xyzICE',
  Metal = 'xyzMETAL',
  Rock = 'xyzROCK',
  Water = 'xyzWATER',
  Gas = 'xyzGAS',
  Gem = 'xyzGEM',
  Life = 'xyzLIFE',
}

export interface PlanetResponse {
  xyz_nft_id: string
  planet_id: string
  resources: ResourceResponse[]
  discovery_time: number
  discovered_contract_version?: unknown
}

export interface ResourceResponse {
  resource_richness_score: number
  resource_identifier: ResourceIdentifier
}

export const rockColorPalette = [
  'rgb(248, 211, 148)',
  'rgb(248, 211, 148)',
  'rgb(217, 151, 85)',
  'rgb(217, 151, 85)',
  'rgb(177, 112, 60)',
  'rgb(177, 112, 60)',
  'rgb(135, 62, 23)',
]
export const metalColorPalette = [
  'rgb(217, 215, 215)',
  'rgb(217, 215, 215)',
  'rgb(217, 215, 215)',
  'rgb(150, 148, 146)',
  'rgb(150, 148, 146)',
  'rgb(150, 148, 146)',
  'rgb(128, 125, 124)',
]
export const iceColorPalette = [
  'rgb(255,255,255)',
  'rgb(255,255,255)',
  'rgb(155,245,250)',
  'rgb(155,245,250)',
  'rgb(91,187,220)',
  'rgb(91,187,220)',
  'rgb(71,147,210)',
]
export const gasColorPalette = [
  'rgb(255,255,255)',
  'rgb(255,255,255)',
  'rgb(240, 215, 215)',
  'rgb(240, 215, 215)',
  'rgb(240, 180, 180)',
  'rgb(217, 215, 215)',
  'rgb(217, 215, 215)',
]
export const waterColorPalette = [
  'rgb(76,83,190)',
  'rgb(76,83,190)',
  'rgb(76,102,190)',
  'rgb(76,102,190)',
  'rgb(71,147,190)',
  'rgb(71,147,190)',
  'rgb(36,45,120)',
]
export const gemColorPalette = [
  'rgb(239,145,145)',
  'rgb(239,145,145)',
  'rgb(195,50,50)',
  'rgb(195,50,50)',
  'rgb(182, 54, 54)',
  'rgb(182, 54, 54)',
  'rgb(157,47,47)',
]
export const lifeColorPalette = [
  'rgb(188, 220, 140)',
  'rgb(188, 220, 140)',
  'rgb(136,162,103)',
  'rgb(136,162,103)',
  'rgb(124,169,76)',
  'rgb(124,169,76)',
  'rgb(100,131,76)',
]
export class PlanetModel {
  public xyz_nft_id: string;
  public planet_id: string;
  public resources: ResourceResponse[];
  public discovery_time: number;
  public discovered_contract_version?: unknown;

  public constructor (data: PlanetResponse) {
    this.xyz_nft_id = data.xyz_nft_id
    this.planet_id = data.planet_id
    this.resources = data.resources
    this.discovery_time = data.discovery_time
    this.discovered_contract_version = data.discovered_contract_version
  }

  public get gasBlurMultiplier (): number {
    const gas = this.resources.find((resource) => (
      resource.resource_identifier === ResourceIdentifier.Gas
    ))
    return gas === undefined ? 0.6 : gas.resource_richness_score
  }

  public get sortedResources (): ResourceResponse[] {
    const sortedResources = this.resources.sort((a, b) => {
      if (b.resource_identifier === ResourceIdentifier.Life) {
        return 1
      } else if (a.resource_identifier === ResourceIdentifier.Life) {
        return -1
      }
      if (b.resource_identifier === ResourceIdentifier.Gem) {
        return 1
      } else if (a.resource_identifier === ResourceIdentifier.Gem) {
        return -1
      }
      if (b.resource_identifier === ResourceIdentifier.Gas) {
        return 1
      } else if (a.resource_identifier === ResourceIdentifier.Gas) {
        return -1
      }
      if (b.resource_identifier === ResourceIdentifier.Water) {
        return 1
      } else if (a.resource_identifier === ResourceIdentifier.Water) {
        return -1
      }

      if (b.resource_richness_score > a.resource_richness_score) {
        return 1
      } else if (b.resource_richness_score < a.resource_richness_score) {
        return -1
      }

      if (b.resource_identifier === ResourceIdentifier.Ice) {
        return 1
      } else if (a.resource_identifier === ResourceIdentifier.Ice) {
        return -1
      }
      if (b.resource_identifier === ResourceIdentifier.Metal) {
        return 1
      } else if (a.resource_identifier === ResourceIdentifier.Metal) {
        return -1
      }
      if (b.resource_identifier === ResourceIdentifier.Rock) {
        return 1
      } else if (a.resource_identifier === ResourceIdentifier.Rock) {
        return -1
      }
      return 0
    })
    return sortedResources
  }

  public get primaryResource (): ResourceResponse {
    return this.sortedResources[0]
  }

  public get primaryColorPalette (): string[] {
    switch (this.primaryResource.resource_identifier) {
      case ResourceIdentifier.Gem: {
        return gemColorPalette
      }
      case ResourceIdentifier.Life: {
        return lifeColorPalette
      }
      case ResourceIdentifier.Water: {
        return waterColorPalette
      }
      case ResourceIdentifier.Gas: {
        return gasColorPalette
      }
      case ResourceIdentifier.Ice: {
        return iceColorPalette
      }
      case ResourceIdentifier.Metal: {
        return metalColorPalette
      }
      case ResourceIdentifier.Rock: {
        return rockColorPalette
      }
    }
  }
}
