import { MapContainer } from 'react-leaflet'
import { CRS, LatLngBoundsExpression, LeafletMouseEvent } from 'leaflet'
import './Map.css'
import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import Countries, { Country } from './Countries'
import States, { State } from './States'
import Provinces from './Provinces'
import Background from './Background'
import { exists, readTextFile } from '@tauri-apps/api/fs';
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

const getStateCoords = async () => {
  const cacheDir = await appCacheDir()
  const path = `${cacheDir}/states.json`
  const fileExists = await exists(path)
  
  if (!fileExists) { return {} }

  const fileContents = await readTextFile(`${cacheDir}/states.json`);
  return JSON.parse(fileContents) as StateCoords
}

const getCountries = async () => {
  const cacheDir = await appCacheDir()
  const path = `${cacheDir}/countries.json`
  const fileExists = await exists(path)
  
  if (!fileExists) { return [] }

  const fileContents = await readTextFile(`${cacheDir}/countries.json`);
  return JSON.parse(fileContents) as Country[]
}

export default function Map() {
  const [countries, setCountries] = useState<Country[]>([])
  const [stateCoords, setStateCoords] = useState<StateCoords>({})
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

    const unlistenToStateData = listen('load-state-coords', () => {
      getStateCoords().then((stateCoords) => {
        console.log(stateCoords)
        setStateCoords(stateCoords)
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
    getStateCoords().then((stateCoords) => setStateCoords(stateCoords))
    getCountries().then((countries) => {
      setCountries(countries)
      forceRerender()
    })

    return () => {
      unlistenToProvinceCoords.then((unlisten) => unlisten())
      unlistenToStateData.then((unlisten) => unlisten())
      unlistenToCountryData.then((unlisten) => unlisten())
    }
  }, [setStateCoords])

  const handleControlClickCountry = async (event: LeafletMouseEvent) => {
    if (selectedCountry && selectedState) {
      if (selectedProvince) {
        const toCountry = event.sourceTarget.feature.properties as Country
        const { to_country: responseToCountry, from_country: responseFromCountry, to_state_coords: responseToStateCoords, from_state_coords: responseFromStateCoords } = await invoke<TransferProvinceResponse>("transfer_province", { 
          state: selectedState.name,
          province: selectedProvince,
          fromCountry: selectedCountry,
          toCountry,
          fromCoords: stateCoords[`${selectedCountry.name}:${selectedState.name}`],
          toCoords: stateCoords[`${toCountry.name}:${selectedState.name}`] || [],
          provinceCoords: provinceCoords[selectedProvince]
        })

        handleTransferResponse({ toCountry: responseToCountry, fromCountry: responseFromCountry, toStateCoords: responseToStateCoords, fromStateCoords: responseFromStateCoords, selectedState })
        setSelectedState((state) => state?.name === selectedState.name ? responseFromCountry.states.find((state) => state.name === selectedState.name) || null : state)
        setSelectedProvince((province) => province === selectedProvince ? null : province)
      } else {
        const toCountry = event.sourceTarget.feature.properties as Country
        const { to_country: responseToCountry, from_country: responseFromCountry, state_coords: responseStateCoords } = await invoke<TransferStateResponse>("transfer_state", { 
          state: selectedState.name,
          fromCountry: selectedCountry,
          toCountry,
          fromCoords: stateCoords[`${selectedCountry.name}:${selectedState.name}`],
          toCoords: stateCoords[`${toCountry.name}:${selectedState.name}`] || [] 
        })

        handleTransferResponse({ toCountry: responseToCountry, fromCountry: responseFromCountry, toStateCoords: responseStateCoords, fromStateCoords: [], selectedState })
        setSelectedState((state) => state?.name === selectedState.name ? null : state)
      }
      forceRerender()
    }
  }

  const handleTransferResponse = ({ toCountry, fromCountry, toStateCoords, fromStateCoords, selectedState }: { toCountry: Country, fromCountry: Country, toStateCoords: Coords, fromStateCoords: Coords, selectedState: State }) => {
    const stateCoordsCopy = { ...stateCoords, [`${toCountry.name}:${selectedState.name}`]: toStateCoords }
    fromStateCoords.length > 0 ? stateCoordsCopy[`${fromCountry.name}:${selectedState.name}`] = fromStateCoords : delete stateCoordsCopy[`${fromCountry.name}:${selectedState.name}`]
    setStateCoords(stateCoordsCopy)

    const fromCountryIndex = countries.findIndex((country) => country.name === fromCountry.name)
    const toCountryIndex = countries.findIndex((country) => country.name === toCountry.name)

    if (toCountryIndex === -1) { countries.push(toCountry) }
    countries[toCountryIndex] = toCountry
    fromCountry.states.length > 0 ? countries[fromCountryIndex] = fromCountry : countries.splice(fromCountryIndex, 1)

    setSelectedCountry((country) => country?.name === fromCountry.name ? fromCountry : country)

    setCountries([...countries])
    forceRerender()
  }

  const handleClickCountry = (event: LeafletMouseEvent) => {
    if (event.originalEvent.ctrlKey || event.originalEvent.metaKey) { return handleControlClickCountry(event) }

    const country = event.sourceTarget.feature.properties as Country
    setSelectedProvince(null)
    setSelectedState(null)
    setSelectedCountry(countries.find((c) => c.name === country.name) || null)
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

  useEffect(() => {
    const timer = setTimeout(() => {
      if (countries.length > 0 && Object.keys(stateCoords).length > 0) {
        console.log("Caching state");
        invoke<TransferProvinceResponse>("cache_state", { countries, states: stateCoords });
      }
    }, 3000);
    return () => clearTimeout(timer);
  }, [countries, stateCoords])

  const handleCountryChange = (country: Country) => {
    setCountries(countries.map((c) => c.name === country.name ? country : c))
    setSelectedCountry(country)
    setSelectedState((state) => state?.name === selectedState?.name ? country.states.find((s) => s.name === selectedState?.name) || null : state)
  }

  const handleCreateCountry = async (countryDefinition: CountryDefinition) => {
    if (selectedCountry && selectedState) {
      if (selectedProvince) {
        const { to_country, from_country, to_state_coords, from_state_coords } = await invoke<TransferProvinceResponse>("create_country_from_province", {
          countryDefinition,
          fromCountry: selectedCountry,
          state: selectedState.name,
          province: selectedProvince,
          stateCoords: stateCoords[`${selectedCountry.name}:${selectedState.name}`],
          provinceCoords: provinceCoords[selectedProvince]
        })
        handleTransferResponse({ toCountry: to_country, fromCountry: from_country, toStateCoords: to_state_coords, fromStateCoords: from_state_coords, selectedState })
        setSelectedState((state) => state?.name === selectedState.name ? from_country.states.find((state) => state.name === selectedState.name) || null : state)
        setSelectedProvince((province) => province === selectedProvince ? null : province)
      } else {
        const { to_country: responseToCountry, from_country: responseFromCountry, state_coords: responseStateCoords } = await invoke<TransferStateResponse>("create_country", { 
          countryDefinition,
          fromCountry: selectedCountry,
          state: selectedState.name,
          coords: stateCoords[`${selectedCountry.name}:${selectedState.name}`]
        })
        handleTransferResponse({ toCountry: responseToCountry, fromCountry: responseFromCountry, toStateCoords: responseStateCoords, fromStateCoords: [], selectedState })
        setSelectedState((state) => state?.name === selectedState.name ? null : state)
      }
    }
  }

  return (
    <div>
      <MapContainer center={[0, 0]} minZoom={-2} maxZoom={2} doubleClickZoom={false} crs={CRS.Simple} bounds={bounds}>
        <Background bounds={bounds} />
        <Countries countries={countries} renderBreaker={renderBreaker} eventHandlers={{ click: handleClickCountry }} />
        { selectedCountry && <States country={selectedCountry} stateCoords={stateCoords} renderBreaker={renderBreaker} eventHandlers={{ click: handleClickState }} selectedState={selectedState} /> }
        { selectedState && <Provinces state={selectedState} provinceCoords={provinceCoords} renderBreaker={renderBreaker} eventHandlers={{ click: handleClickProvince }} selectedProvince={selectedProvince} /> }
      </MapContainer>
      { selectedCountry && <SelectionInfo selectedCountry={selectedCountry} selectedState={selectedState} selectedProvince={selectedProvince} onCountryChange={handleCountryChange} /> }
      { selectedState && <CreateCountry createdCountries={countries} onCreateCountry={handleCreateCountry} /> }
    </div>
  ) 
}
