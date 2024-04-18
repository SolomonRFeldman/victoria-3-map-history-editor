import { Coords } from "./Map"
import { GeoJSON } from 'react-leaflet'
import { FeatureCollection, Feature, Geometry } from 'geojson'
import { LeafletEventHandlerFnMap } from "leaflet"
import { Country } from "./Countries"


export type State = {
  name: string,
  color: string
  provinces: string[]
}

type StatesProps = {
  country: Country
  stateCoords: { [key: string]: Coords }
  renderBreaker: number
  eventHandlers: LeafletEventHandlerFnMap
}

export default function States({ country, stateCoords, renderBreaker, eventHandlers }: StatesProps) {
  const stateStyle = (feature?: Feature<Geometry, State>) => {
    return {
      fillColor: feature ? feature.properties.provinces[0].replace('x', '#') : 'transparent',
      fillOpacity: 0.5,
      color: 'purple',
      weight: 2
    }
  }

  const stateData: FeatureCollection<Geometry, State> = {
    type: "FeatureCollection",
    features: country.states.map((state) => {
      return {
        type: "Feature",
        properties: state,
        geometry: {
          type: "Polygon",
          coordinates: stateCoords[`${country.name}:${state.name}`]
        }
      }
    })
  }

  return <GeoJSON data={stateData} key={country.name + renderBreaker} style={stateStyle} eventHandlers={eventHandlers} />
}
