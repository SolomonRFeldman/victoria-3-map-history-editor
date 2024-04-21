import { useState } from "react"
import { State } from "../States"
import PopsInfo from "./PopsInfo"
import BuildingsInfo from "./BuildingsInfo"

type StateInfoProps = {
  selectedState: State,
  onStateChange: (state: State) => void
}
export default function StateInfo({ selectedState, onStateChange }: StateInfoProps) {
  const [tabSelection, setTabSelection] = useState('population')
  const handleTabSelection = (tab: string) => setTabSelection(tab)
  const isSelected = (tab: string) => tab === tabSelection ? 'bg-purple-400' : ''

  return(
    <div>
      <h2 className="card-title text-base justify-end">State: {selectedState.name}</h2>
      <div role="tablist" className="tabs tabs-boxed float-right">
        <div role="tab" className={`tab ${isSelected('population')}`} onClick={() => handleTabSelection('population')}>Population</div>
        <div role="tab" className={`tab ${isSelected('buildings')}`} onClick={() => handleTabSelection('buildings')}>Buildings</div>
      </div>
      { tabSelection === 'population' && <PopsInfo pops={selectedState.pops} onPopsChange={(pops) => onStateChange({...selectedState, pops})} /> }
      { tabSelection === 'buildings' && <BuildingsInfo buildings={selectedState.buildings} /> }
    </div>
  )
}
