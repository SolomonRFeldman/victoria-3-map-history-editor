import { Pop, State } from "../States"

type StateInfoProps = {
  selectedState: State,
  onStateChange: (state: State) => void
}

export default function StateInfo({ selectedState, onStateChange }: StateInfoProps) {
  // Will come in use once building info is added
  // const [tabSelection, setTabSelection] = useState('population')
  // const handleTabSelection = (tab: string) => setTabSelection(tab)
  // const isSelected = (tab: string) => tab === tabSelection ? 'bg-purple-400' : ''
  const handlePopulationChange = (pop: Pop, size: number) => {
    const newPop = {...pop, size}
    const newPops = selectedState.pops.map((p) => p === pop ? newPop : p)
    onStateChange({...selectedState, pops: newPops})
  }

  const handleRemovePop = (pop: Pop) => {
    const newPops = selectedState.pops.filter((p) => p !== pop)
    onStateChange({...selectedState, pops: newPops})
  }

  return(
    <div>
      <h2 className="card-title text-base">State: {selectedState.name}</h2>
      {/* <div role="tablist" className="tabs tabs-boxed">
        <div role="tab" className={`tab ${isSelected('population')}`} onClick={() => handleTabSelection('population')}>Population</div>
        <div role="tab" className={`tab ${isSelected('buildings')}`} onClick={() => handleTabSelection('buildings')}>Buildings</div>
      </div> */}
      <table className="table">
        <thead>
          <tr>
            <th>Culture</th>
            <th>Religion</th>
            <th>Size</th>
            <th>Population Type</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
        {selectedState?.pops.sort((pop1, pop2) => pop2.size - pop1.size).map((pop) => {
          return (
            <tr key={pop.culture + pop.religion + pop.pop_type}>
              <td>{pop.culture}</td>
              <td>{pop.religion}</td>
              <td><input type="text" placeholder="0" className="input input-sm" value={pop.size} onChange={(e) => handlePopulationChange(pop, parseInt(e.target.value))} /></td>
              <td>{pop.pop_type}</td>
              <td><button className="btn btn-square btn-xs btn-error" onClick={() => handleRemovePop(pop)}>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M5 12h14" />
                </svg>
              </button></td>
            </tr>
          )
        })}
        </tbody>
      </table>
    </div>
  )
}
