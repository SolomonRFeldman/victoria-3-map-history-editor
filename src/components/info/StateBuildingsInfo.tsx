import { StateBuilding } from "../States"
import StateBuildingInfo from "./StateBuildingInfo"
import AddStateBuilding from "./AddStateBuilding"

type BuildingsInfoProps = {
  buildings: StateBuilding[]
  onBuildingsChange: (building: StateBuilding[]) => void
}

export default function StateBuildingsInfo({ buildings, onBuildingsChange }: BuildingsInfoProps) {
  const handleBuildingChange = (building: StateBuilding) => {
    if (building.level === 0) {
      const newBuildings = buildings.filter((b) => b.name !== building.name)
      onBuildingsChange(newBuildings)
    } else {
      const newBuildings = buildings.map((b) => b.name === building.name ? building : b)
      onBuildingsChange(newBuildings)
    }
  }

  const handleAddStateBuilding = (building: StateBuilding) => {
    const newBuildings = [...buildings, building]
    onBuildingsChange(newBuildings)
  }
  
  return(
    <table className="table table-xs">
      <thead>
        <tr>
          <th>PMs</th>
          <th>Building</th>
          <th>Production Methods</th>
          <th>Level</th>
        </tr>
      </thead>
      <tbody>
        {buildings.sort((building1, building2) => (building2.level || 0) - (building1.level || 0)).map((building) => <StateBuildingInfo key={building.name} stateBuilding={building} onBuildingChange={handleBuildingChange} />)}
        <AddStateBuilding stateBuildings={buildings} onAddStateBuilding={handleAddStateBuilding} />
      </tbody>
    </table>
  )
}
