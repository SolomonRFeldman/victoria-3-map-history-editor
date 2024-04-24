import { useEffect, useState } from "react"
import { State } from "../States"
import PopsInfo from "./PopsInfo"
import StateBuildingsInfo from "./StateBuildingsInfo"

type TabSelection = 'population' | 'buildings'

type StateInfoProps = {
  selectedState: State,
  onStateChange: (state: State) => void
}

export default function StateInfo({ selectedState, onStateChange }: StateInfoProps) {
  const [tabSelection, setTabSelection] = useState<TabSelection>('population')
  const handleTabSelection = (tab: TabSelection) => setTabSelection(tab)
  const isSelected = (tab: string) => tab === tabSelection ? 'bg-purple-400' : ''

  useEffect(() => {
    const handleTabPress = (event: KeyboardEvent) => {
      if (event.key === 'Tab') { 
        event.preventDefault()
        setTabSelection((tabSelection) => tabSelection === 'population' ? 'buildings' : 'population') 
      }
    }
    window.addEventListener('keydown', handleTabPress)
    return () => window.removeEventListener('keydown', handleTabPress)
  }, [])

  return(
    <div>
      <h2 className="card-title text-base justify-end">State: {selectedState.name}</h2>
      <div className="kbd float-left">e</div>
      <div role="tablist" className="tabs tabs-boxed float-right tooltip tooltip-left" data-tip='Tab'>
        <div role="tab" className={`tab ${isSelected('population')}`} onClick={() => handleTabSelection('population')}>Population</div>
        <div role="tab" className={`tab ${isSelected('buildings')}`} onClick={() => handleTabSelection('buildings')}>Buildings</div>
      </div>
      { tabSelection === 'population' && <PopsInfo key={selectedState.name} pops={selectedState.pops} onPopsChange={(pops) => onStateChange({...selectedState, pops})} /> }
      { tabSelection === 'buildings' && <StateBuildingsInfo buildings={selectedState.state_buildings} onBuildingsChange={(state_buildings) => onStateChange({...selectedState, state_buildings})} /> }
    </div>
  )
}
