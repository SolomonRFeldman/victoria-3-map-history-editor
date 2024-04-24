import { StateBuilding } from "../States"
import StateBuildingInfo from "./StateBuildingInfo"

type BuildingsInfoProps = {
  buildings: StateBuilding[]
  onBuildingsChange: (building: StateBuilding[]) => void
}

export default function BuildingsInfo({ buildings, onBuildingsChange }: BuildingsInfoProps) {
  const onBuildingChange = (building: StateBuilding) => {
    const newBuildings = buildings.map((b) => b.name === building.name ? building : b)
    onBuildingsChange(newBuildings)
  }
  
  return(
    <table className="table table-xs">
      <thead>
        <tr>
          <th className="max-w-5">PMs</th>
          <th className="max-w-32">Building</th>
          <th className="max-w-24">Production Methods</th>
          <th>Level</th>
        </tr>
      </thead>
      <tbody>
        {buildings.sort((building1, building2) => (building2.level || 0) - (building1.level || 0)).map((building) => <StateBuildingInfo key={building.name} stateBuilding={building} onBuildingChange={onBuildingChange} />)}
      </tbody>
    </table>
  )
}
