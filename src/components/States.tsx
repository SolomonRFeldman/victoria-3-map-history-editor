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
  selectedState: State | null
  renderBreaker: number
  eventHandlers: LeafletEventHandlerFnMap
}

export default function States({ country, stateCoords, selectedState, renderBreaker, eventHandlers }: StatesProps) {
  const stateStyle = (feature?: Feature<Geometry, State>) => {
    return {
      fillColor: feature ? feature.properties.provinces[0].replace('x', '#') : 'transparent',
      fillOpacity: 0.5,
      color: 'purple',
      weight: 2
    }
  }

  const states = country.states.filter((state) => state.name != selectedState?.name)

  const stateData: FeatureCollection<Geometry, State> = {
    type: "FeatureCollection",
    features: states.map((state) => {
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

  return <GeoJSON data={stateData} key={country.name + selectedState?.name + renderBreaker} style={stateStyle} eventHandlers={eventHandlers} />
}
