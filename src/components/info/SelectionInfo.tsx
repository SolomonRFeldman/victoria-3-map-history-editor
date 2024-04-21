import { Country } from "../Countries"
import { State } from "../States"
import { useEffect, useRef } from "react"
import { DomEvent } from "leaflet"
import StateInfo from "./StateInfo"

type SelectionInfoProps = {
  selectedCountry: Country
  selectedState: State | null
  selectedProvince: string | null
}

export default function SelectionInfo({ selectedCountry, selectedState, selectedProvince }: SelectionInfoProps) {
  const infoRef = useRef(null);
  useEffect(() => {
    if (infoRef.current) {
      DomEvent.disableClickPropagation(infoRef.current);
    }
  }, []);


  return (
    <div ref={infoRef} className='leaflet-top leaflet-right card card-compact bg-base-100 m-4'>
      <div className="card-body leaflet-control">
        <h1 className="card-title">Country: {selectedCountry.name}</h1>
        { selectedState && <StateInfo selectedState={selectedState} /> }
        { selectedProvince && <h3 className="card-title text-sm">Province: {selectedProvince}</h3> }
      </div>
    </div>
  )
}
