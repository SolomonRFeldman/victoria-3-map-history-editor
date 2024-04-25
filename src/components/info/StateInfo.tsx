import { useEffect, useRef, useState } from "react"
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
  const divRef = useRef<HTMLTableElement>(null)

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

  const handleOnKeyDown = (event: React.KeyboardEvent) => {
    if (event.key === 'Tab' || event.key === 'e') {
      event.stopPropagation()
    }
    if (event.key === 'Escape') {
      event.stopPropagation()
      divRef.current?.focus()
      divRef.current?.blur()
    }
  }
  useEffect(() => {
    const handlePopsHotkeys = (event: KeyboardEvent) => {
      if (event.key === 'e') {
        divRef.current?.focus()
      }
    }
    window.addEventListener('keydown', handlePopsHotkeys)
    return () => window.removeEventListener('keydown', handlePopsHotkeys)
  }, [])

  return(
    <div>
      <h2 className="card-title text-base justify-end">State: {selectedState.name}</h2>
      <div className='grid grid-cols-2 justify-items-stretch items-end pb-1'>
        <div className="kbd justify-self-start h-6">e</div>
        <div role="tablist" className="tabs tabs-boxed tooltip tooltip-left justify-self-end" data-tip='Tab'>
          <div role="tab" className={`tab ${isSelected('population')}`} onClick={() => handleTabSelection('population')}>Population</div>
          <div role="tab" className={`tab ${isSelected('buildings')}`} onClick={() => handleTabSelection('buildings')}>Buildings</div>
        </div>
      </div>
      <div className="card block" ref={divRef} tabIndex={0} onKeyDown={handleOnKeyDown}>
        { tabSelection === 'population' && <PopsInfo key={selectedState.name} pops={selectedState.pops} onPopsChange={(pops) => onStateChange({...selectedState, pops})} /> }
        { tabSelection === 'buildings' && <StateBuildingsInfo buildings={selectedState.state_buildings} onBuildingsChange={(state_buildings) => onStateChange({...selectedState, state_buildings})} /> }
      </div>
    </div>
  )
}
