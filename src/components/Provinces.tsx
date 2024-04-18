import { Coords } from "./Map"
import { State } from "./States"
import { GeoJSON } from 'react-leaflet'
import { FeatureCollection, Feature, Geometry } from 'geojson'

type ProvincesProps = {
  state: State
  provinceCoords: { [key: string]: Coords }
  renderBreaker: number
}

export default function Provinces({ state, provinceCoords, renderBreaker }: ProvincesProps) {
  const provinceStyle = (feature?: Feature<Geometry, { name: string }>) => {
    return {
      fillColor: feature ? feature.properties.name.replace('x', '#') : 'transparent',
      fillOpacity: 0.5,
      color: 'black',
      weight: 2
    }
  }

  const provinceData: FeatureCollection<Geometry, { name: string }> = {
    type: "FeatureCollection",
    features: state.provinces.map((province) => {
      return {
        type: "Feature",
        properties: { name: province },
        geometry: {
          type: "Polygon",
          coordinates: provinceCoords[province]
        }
      }
    })
  }
  return <GeoJSON data={provinceData} key={state.name + renderBreaker} style={provinceStyle} />
}
