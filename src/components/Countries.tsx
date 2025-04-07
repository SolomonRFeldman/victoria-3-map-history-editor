import { Coords } from "./Map"
import { GeoJSON } from 'react-leaflet'
import { FeatureCollection, Feature, Geometry } from 'geojson'
import { LeafletEventHandlerFnMap } from "leaflet"

export type Country = {
  id: number,
  tag: string,
  color: [number, number, number],
  setup: CountrySetup
  border: Coords,
}

type CountrySetup = {
  base_tech: string | null,
  technologies_researched: string[]
}

type CountriesProps = {
  countries: Country[]
  renderBreaker: number
  eventHandlers: LeafletEventHandlerFnMap
}

export default function Countries({ countries, renderBreaker, eventHandlers }: CountriesProps) {
  const countryStyle = (feature?: Feature<Geometry, { name: string, color: string }>) => {
    const color = feature?.properties.color as [number, number, number] | undefined
    return {
      fillColor: color ? `rgb(${color[0]}, ${color[1]}, ${color[2]})` : 'transparent',
      fillOpacity: 0.5,
      color: 'black',
      weight: 1
    }
  }

  const countryData: FeatureCollection<Geometry, Country> = {
    type: "FeatureCollection",
    features: countries.map((country) => {
      return {
        type: "Feature",
        properties: country,
        geometry: {
          type: "Polygon",
          coordinates: country.border
        }
      }
    })
  }

  return <GeoJSON data={countryData} key={renderBreaker} style={countryStyle} eventHandlers={eventHandlers} />
}
