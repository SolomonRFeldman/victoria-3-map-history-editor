import { State } from "../States"

type StateInfoProps = {
  selectedState: State
}

export default function StateInfo({ selectedState }: StateInfoProps) {
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
      <table className="table">
        <thead>
          <tr>
            <th>Culture</th>
            <th>Religion</th>
            <th>Size</th>
            <th>Population Type</th>
          </tr>
        </thead>
        <tbody>
        {selectedState?.pops.sort((pop1, pop2) => pop2.size - pop1.size).map((pop) => {
          return (
            <tr>
              <td>{pop.culture}</td>
              <td>{pop.religion}</td>
              <td>{pop.size}</td>
              <td>{pop.pop_type}</td>
            </tr>
          )
        })}
        </tbody>
      </table>
    </div>
  )
}
