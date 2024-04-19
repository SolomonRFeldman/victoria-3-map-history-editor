import { Coords } from "./Map"
import { State } from "./States"
import { GeoJSON } from 'react-leaflet'
import { FeatureCollection, Feature, Geometry } from 'geojson'
import { LeafletEventHandlerFnMap } from "leaflet"

type ProvincesProps = {
  state: State
  provinceCoords: { [key: string]: Coords }
  selectedProvince: string | null
  renderBreaker: number
  eventHandlers: LeafletEventHandlerFnMap
}

export default function Provinces({ state, provinceCoords, selectedProvince, renderBreaker, eventHandlers }: ProvincesProps) {
  const provinceStyle = (feature?: Feature<Geometry, { name: string }>) => {
    return {
      fillColor: feature ? feature.properties.name.replace('x', '#') : 'transparent',
      fillOpacity: 0.5,
      color: 'black',
      weight: 2
    }
  }

  const selectedProvinceStyle = (feature?: Feature<Geometry, { name: string }>) => {
    return {
      fillColor: feature ? feature.properties.name.replace('x', '#') : 'transparent',
      fillOpacity: 0.5,
      color: 'purple',
      weight: 3,
    }
  }

  const hasAllProvinceData = state.provinces.every((province) => provinceCoords[province] !== undefined)
  const featureProvinces = hasAllProvinceData ? state.provinces.filter((province) => province != selectedProvince) : []

  const provinceData: FeatureCollection<Geometry, { name: string }> = {
    type: "FeatureCollection",
    features: featureProvinces.map((province) => {
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
  const selectedProvinceData: FeatureCollection<Geometry, { name: string }> = {
    type: "FeatureCollection",
    features: selectedProvince ? [{
      type: "Feature",
      properties: { name: selectedProvince },
      geometry: {
        type: "Polygon",
        coordinates: provinceCoords[selectedProvince]
      }
    }] : []
  }

  return (
    <>
      <GeoJSON data={provinceData} key={state.name + selectedProvince + renderBreaker} style={provinceStyle} eventHandlers={eventHandlers} />
      <GeoJSON data={selectedProvinceData} key={state.name + selectedProvince + renderBreaker + 'selected'} style={selectedProvinceStyle} />
    </>
  )
}
