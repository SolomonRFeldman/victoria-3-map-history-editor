type ProductionMethod = {
  name: string
}

export type ProductionMethodGroup = {
  name: string,
  production_methods: ProductionMethod[]
}

type ChooseProductionMethodProps = {
  pmgs: ProductionMethodGroup[]
  stateBuildingPms: string[] | null
  onPmChange: (pm: string[]) => void
}


export default function ChooseProductionMethod({ pmgs, stateBuildingPms, onPmChange }: ChooseProductionMethodProps) {
  const handleChoosePm = (newPm: string, oldPm?: string) => {
    console.log(newPm, oldPm)
    const newPms = stateBuildingPms?.map((pm) => pm === oldPm ? newPm : pm) || []
    onPmChange(newPms)
  }


  return(
    <div className="card bg-base-100 shadow-xl">
      <div className="card-body">
        <h3 className="card-title text-sm">Production Methods</h3>
        {pmgs.map(pmg => {
          const value = stateBuildingPms?.find((pmName) => pmg.production_methods.map(pm => pm.name).includes(pmName))
          
          return (
            <select value={value} className="select select-bordered select-xs" onChange={(event) => handleChoosePm(event.target.value, value)}>
              {pmg.production_methods.map(pm => {
                return (
                  <option selected={stateBuildingPms?.includes(pm.name)} value={pm.name}>{pm.name}</option>
                )
              })}
            </select>
          )
        })}
      </div>
    </div>
  )
}
