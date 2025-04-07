import { MapContainer } from 'react-leaflet'
import { CRS, LatLngBoundsExpression, LeafletMouseEvent } from 'leaflet'
import './Map.css'
import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import Countries, { Country } from './Countries'
import States, { State } from './States'
import Provinces from './Provinces'
import Background from './Background'
import { exists, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';
import { appCacheDir } from '@tauri-apps/api/path'
import SelectionInfo from './info/SelectionInfo'
import CreateCountry, { CountryDefinition } from './CreateCountry'

export type Coords = [number, number][][]

type ProvincesCoords = {
  [key: string]: Coords
}

type StateCoords = {
  [key: string]: Coords
}

type TransferStateResponse = {
  to_country: Country,
  from_country: Country,
  state_coords: Coords
}

type TransferProvinceResponse = {
  to_country: Country,
  from_country: Country,
  to_state_coords: Coords
  from_state_coords: Coords
}

const bounds: LatLngBoundsExpression = [[0, 0], [3616, 8192]]

const getProvinceCoords = async () => {
  const cacheDir = await appCacheDir()
  const path = `${cacheDir}/provinces.json`
  const fileExists = await exists(path)
  
  if (!fileExists) { return {} }

  const fileContents = await readTextFile(`${cacheDir}/provinces.json`);
  return JSON.parse(fileContents) as ProvincesCoords
}

const getCountries = async () => {
  return invoke<Country[]>("get_country", {})
}

export default function Map() {
  const [countries, setCountries] = useState<Country[]>([])
  const [provinceCoords, setProvinceCoords] = useState<ProvincesCoords>({})
  const [selectedCountry, setSelectedCountry] = useState<Country | null>(null)
  const [selectedState, setSelectedState] = useState<State | null>(null)
  const [selectedProvince, setSelectedProvince] = useState<string | null>(null)
  const [renderBreaker, setRenderBreaker] = useState(Date.now())
  const forceRerender = () => setRenderBreaker(Date.now())

  useEffect(() => {
    const unlistenToProvinceCoords = listen<ProvincesCoords>('load-province-coords', () => {
      getProvinceCoords().then((provinceCoords) => {
        console.log(provinceCoords)
        setProvinceCoords(provinceCoords)
      })
    })

    const unlistenToCountryData = listen<Country[]>('load-country-data', () => {
      getCountries().then((countries) => {
        console.log(countries)
        setCountries(countries)
        setSelectedProvince(null)
        setSelectedState(null)
        setSelectedCountry(null)
        forceRerender()
      })
    })

    getProvinceCoords().then((provinceCoords) => setProvinceCoords(provinceCoords))
    getCountries().then((countries) => {
      setCountries(countries)
      forceRerender()
    })

    return () => {
      unlistenToProvinceCoords.then((unlisten) => unlisten())
      unlistenToCountryData.then((unlisten) => unlisten())
    }
  }, [])

  const handleControlClickCountry = async (event: LeafletMouseEvent) => {
    // if (selectedCountry && selectedState) {
    //   if (selectedProvince) {
    //     const toCountry = event.sourceTarget.feature.properties as Country
    //     const { to_country: responseToCountry, from_country: responseFromCountry, to_state_coords: responseToStateCoords, from_state_coords: responseFromStateCoords } = await invoke<TransferProvinceResponse>("transfer_province", { 
    //       state: selectedState.name,
    //       province: selectedProvince,
    //       fromCountry: selectedCountry,
    //       toCountry,
    //       fromCoords: stateCoords[`${selectedCountry.name}:${selectedState.name}`],
    //       toCoords: stateCoords[`${toCountry.name}:${selectedState.name}`] || [],
    //       provinceCoords: provinceCoords[selectedProvince]
    //     })

    //     handleTransferResponse({ toCountry: responseToCountry, fromCountry: responseFromCountry, toStateCoords: responseToStateCoords, fromStateCoords: responseFromStateCoords, selectedState })
    //     setSelectedState((state) => state?.name === selectedState.name ? responseFromCountry.states.find((state) => state.name === selectedState.name) || null : state)
    //     setSelectedProvince((province) => province === selectedProvince ? null : province)
    //   } else {
    //     const toCountry = event.sourceTarget.feature.properties as Country
    //     const { to_country: responseToCountry, from_country: responseFromCountry, state_coords: responseStateCoords } = await invoke<TransferStateResponse>("transfer_state", { 
    //       state: selectedState.name,
    //       fromCountry: selectedCountry,
    //       toCountry,
    //       fromCoords: stateCoords[`${selectedCountry.name}:${selectedState.name}`],
    //       toCoords: stateCoords[`${toCountry.name}:${selectedState.name}`] || [] 
    //     })

    //     handleTransferResponse({ toCountry: responseToCountry, fromCountry: responseFromCountry, toStateCoords: responseStateCoords, fromStateCoords: [], selectedState })
    //     setSelectedState((state) => state?.name === selectedState.name ? null : state)
    //   }
    //   forceRerender()
    // }
  }

  const handleTransferResponse = ({ }: { toCountry: Country, fromCountry: Country, toStateCoords: Coords, fromStateCoords: Coords, selectedState: State }) => {
    // const stateCoordsCopy = { ...stateCoords, [`${toCountry.name}:${selectedState.name}`]: toStateCoords }
    // fromStateCoords.length > 0 ? stateCoordsCopy[`${fromCountry.name}:${selectedState.name}`] = fromStateCoords : delete stateCoordsCopy[`${fromCountry.name}:${selectedState.name}`]
    // setStateCoords(stateCoordsCopy)

    // const fromCountryIndex = countries.findIndex((country) => country.name === fromCountry.name)
    // const toCountryIndex = countries.findIndex((country) => country.name === toCountry.name)

    // if (toCountryIndex === -1) { countries.push(toCountry) }
    // countries[toCountryIndex] = toCountry
    // fromCountry.states.length > 0 ? countries[fromCountryIndex] = fromCountry : countries.splice(fromCountryIndex, 1)

    // setSelectedCountry((country) => country?.name === fromCountry.name ? fromCountry : country)

    // const updatedCountries = fromStateCoords.length > 0 ? [...countries] : handleTransferOwnership({ toCountry, fromCountry, selectedState, countries })
    // setCountries(updatedCountries)
    // forceRerender()
  }

  const handleTransferOwnership = ({}: { toCountry: Country, fromCountry: Country, selectedState: State, countries: Country[] }) => {
    // return countries.map((country) => ({
    //   ...country,
    //   states: country.states.map((state) => ({
    //     ...state,
    //     state_buildings: state.state_buildings.map((building) => ({
    //       ...building,
    //       ownership: building.ownership
    //         ? {
    //             ...building.ownership,
    //             buildings: building.ownership.buildings.map((building) => {
    //               if (
    //                 building.country == `c:${fromCountry.name}` &&
    //                 `s:${building.region}` == selectedState.name
    //               ) {
    //                 return {
    //                   ...building,
    //                   country: `c:${toCountry.name}`,
    //                 };
    //               }
    //               return building;
    //             }),
    //             countries: building.ownership.countries.map((country) => {
    //               if (state.name === selectedState.name) {
    //                 return {
    //                   ...country,
    //                   country: `c:${toCountry.name}`,
    //                 };
    //               }
    //               return country;
    //             }),
    //           }
    //         : null,
    //     })),
    //   })),
    // }))
  }

  const handleClickCountry = (event: LeafletMouseEvent) => {
    if (event.originalEvent.ctrlKey || event.originalEvent.metaKey) { return handleControlClickCountry(event) }

    const country = event.sourceTarget.feature.properties as Country
    setSelectedProvince(null)
    setSelectedState(null)
    setSelectedCountry(countries.find((c) => c.tag === country.tag) || null)
  }

  const handleClickState = (event: LeafletMouseEvent) => {
    const state = event.sourceTarget.feature.properties as State
    setSelectedProvince(null)
    setSelectedState(state)
  }

  const handleClickProvince = (event: LeafletMouseEvent) => {
    const province = event.sourceTarget.feature.properties.name
    setSelectedProvince(province)
  }

  useEffect(() => {
    const handleEscapePress = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        if (selectedProvince) { setSelectedProvince(null) } else if (selectedState) { setSelectedState(null) } else if (selectedCountry) { setSelectedCountry(null) }
      }
    }
    window.addEventListener('keydown', handleEscapePress)
    return () => window.removeEventListener('keydown', handleEscapePress)
  }, [selectedCountry, selectedState, selectedProvince])

  const handleCountryChange = (country: Country) => {
    const countryToSend = { ...country, border: [] }
    const startTime = Date.now()
    invoke<Country>('update_country', { country: countryToSend }).then((newCountry) => {
      console.log(`Time taken to update country: ${Date.now() - startTime}ms`)
      console.log(country)
      setSelectedCountry({ ...newCountry, border: [...country.border] })
    })
  }

  const handleCreateCountry = async (countryDefinition: CountryDefinition) => {
    // if (selectedCountry && selectedState) {
    //   if (selectedProvince) {
    //     const { to_country, from_country, to_state_coords, from_state_coords } = await invoke<TransferProvinceResponse>("create_country_from_province", {
    //       countryDefinition,
    //       fromCountry: selectedCountry,
    //       state: selectedState.name,
    //       province: selectedProvince,
    //       stateCoords: stateCoords[`${selectedCountry.name}:${selectedState.name}`],
    //       provinceCoords: provinceCoords[selectedProvince]
    //     })
    //     handleTransferResponse({ toCountry: to_country, fromCountry: from_country, toStateCoords: to_state_coords, fromStateCoords: from_state_coords, selectedState })
    //     setSelectedState((state) => state?.name === selectedState.name ? from_country.states.find((state) => state.name === selectedState.name) || null : state)
    //     setSelectedProvince((province) => province === selectedProvince ? null : province)
    //   } else {
    //     const { to_country: responseToCountry, from_country: responseFromCountry, state_coords: responseStateCoords } = await invoke<TransferStateResponse>("create_country", { 
    //       countryDefinition,
    //       fromCountry: selectedCountry,
    //       state: selectedState.name,
    //       coords: stateCoords[`${selectedCountry.name}:${selectedState.name}`]
    //     })
    //     handleTransferResponse({ toCountry: responseToCountry, fromCountry: responseFromCountry, toStateCoords: responseStateCoords, fromStateCoords: [], selectedState })
    //     setSelectedState((state) => state?.name === selectedState.name ? null : state)
    //   }
    // }
  }

  const [states, setStates] = useState<State[]>([])

  useEffect(() => {
    setStates([])
    if (selectedCountry) {
      const startTime = Date.now()
      invoke<State[]>('get_states', { countryId: selectedCountry.id }).then((states) => {
        setStates(states)
        console.log(`Time taken to get states: ${Date.now() - startTime}ms`)
        console.log(states)
      })
    }
  }, [selectedCountry])

  return (
    <div>
      <MapContainer center={[0, 0]} minZoom={-2} maxZoom={2} doubleClickZoom={false} crs={CRS.Simple} bounds={bounds}>
        <Background bounds={bounds} />
        <Countries countries={countries} renderBreaker={renderBreaker} eventHandlers={{ click: handleClickCountry }} />
        { selectedCountry && states.length && <States country={selectedCountry} states={states} renderBreaker={renderBreaker} eventHandlers={{ click: handleClickState  }} selectedState={selectedState} /> }
        { selectedState && <Provinces state={selectedState} provinceCoords={provinceCoords} renderBreaker={renderBreaker} eventHandlers={{ click: handleClickProvince }} selectedProvince={selectedProvince} /> }
      </MapContainer>
      { selectedCountry && <SelectionInfo selectedCountry={selectedCountry} selectedState={selectedState} selectedProvince={selectedProvince} onCountryChange={handleCountryChange} /> }
      { selectedState && <CreateCountry createdCountries={countries} onCreateCountry={handleCreateCountry} /> }
    </div>
  ) 
}
