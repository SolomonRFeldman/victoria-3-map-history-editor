import StateBuildingInfo, { StateBuilding } from "./StateBuildingInfo"
import AddStateBuilding from "./AddStateBuilding"
import { useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api/core"

type BuildingsInfoProps = {
  stateId: number
}

export default function StateBuildingsInfo({ stateId }: BuildingsInfoProps) {
  const [buildings, setBuildings] = useState<StateBuilding[]>([])
  useEffect(() => {
    invoke<StateBuilding[]>("get_buildings", { stateId }).then((buildings) => {
      setBuildings(buildings)
    })
  }, [stateId])

  const handleAddStateBuilding = () => {}
  const handleBuildingChange = () => {}
  
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
