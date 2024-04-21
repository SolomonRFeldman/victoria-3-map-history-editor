import { State } from "../States"
import PopsInfo from "./PopsInfo"

type StateInfoProps = {
  selectedState: State,
  onStateChange: (state: State) => void
}
export default function StateInfo({ selectedState, onStateChange }: StateInfoProps) {
  // Will come in use once building info is added
  // const [tabSelection, setTabSelection] = useState('population')
  // const handleTabSelection = (tab: string) => setTabSelection(tab)
  // const isSelected = (tab: string) => tab === tabSelection ? 'bg-purple-400' : ''

  return(
    <div>
      <h2 className="card-title text-base">State: {selectedState.name}</h2>
      {/* <div role="tablist" className="tabs tabs-boxed">
        <div role="tab" className={`tab ${isSelected('population')}`} onClick={() => handleTabSelection('population')}>Population</div>
        <div role="tab" className={`tab ${isSelected('buildings')}`} onClick={() => handleTabSelection('buildings')}>Buildings</div>
      </div> */}
      <PopsInfo pops={selectedState.pops} onPopsChange={(pops) => onStateChange({...selectedState, pops})} />
    </div>
  )
}
