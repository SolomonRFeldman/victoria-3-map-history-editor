import { useEffect, useRef, useState } from "react"
import { Pop } from "../States"
import { MinusIcon, PlusIcon } from "@heroicons/react/24/outline"

type PopsInfoProps = {
  pops: Pop[]
  onPopsChange: (pops: Pop[]) => void
}

const presentString = (value: string) => value === '' ? null : value

const CreatePopForm = ({ onCreatePop, onCancel }: { onCreatePop: (pop: Pop) => void, onCancel: () => void }) => {
  const [culture, setCulture] = useState('')
  const [religion, setReligion] = useState('')
  const [size, setSize] = useState(1)
  const [popType, setPopType] = useState('')

  const handleCreatePop = () => {
    if(culture !== '' && size > 0) {
      onCreatePop({culture, religion: presentString(religion), size, pop_type: presentString(popType)})
    }
  }
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (inputRef.current) {
      inputRef.current.focus();
    }
  }, []);

  return (
    <tr>
      <td><button className="btn btn-square btn-xs btn-success" onClick={handleCreatePop}><PlusIcon className="w-5 h-5" /></button></td>
      <td className="max-w-20"><input ref={inputRef} type="text" placeholder="Culture" className="input input-xs w-24 -ml-2" value={culture} onChange={(e) => setCulture(e.target.value)} /></td>
      <td className="max-w-20"><input type="text" placeholder="Religion" className="input input-xs w-24 -ml-2" value={religion} onChange={(e) => setReligion(e.target.value)} /></td>
      <td className="max-w-20"><input type="text" placeholder="Size" className="input input-xs w-24 -ml-2" value={size} onChange={(e) => setSize(parseInt(e.target.value) || 0)} /></td>
      <td><input type="text" placeholder="Population Type" className="input input-xs w-28 -ml-2" value={popType} onChange={(e) => setPopType(e.target.value)} /></td>
      <td><button className="btn btn-square btn-xs btn-error" onClick={onCancel}><MinusIcon className="w-5 h-5" /></button></td>
    </tr>
  )
}

export default function PopsInfo({ pops, onPopsChange }: PopsInfoProps) {
  const handlePopulationChange = (pop: Pop, size: number) => {
    const newPop = {...pop, size}
    const newPops = pops.map((p) => p === pop ? newPop : p)
    onPopsChange(newPops)
  }

  const handleRemovePop = (pop: Pop) => {
    const newPops = pops.filter((p) => p !== pop)
    onPopsChange(newPops)
  }

  const handleAddPop = (pop: Pop) => {
    if (!pops.find((p) => p.culture === pop.culture && p.religion === pop.religion && p.pop_type === pop.pop_type)) {
      const newPops = [...pops, pop]
      onPopsChange(newPops)
      setIsCreatingPop(false)
    }
  }

  const [isCreatingPop, setIsCreatingPop] = useState(false)
  return(
    <table className="table table-xs">
      <thead>
        <tr>
          <th className="max-w-6">{ !isCreatingPop && <button className="btn btn-square btn-xs btn-success"  onClick={() => setIsCreatingPop(true)}><PlusIcon className="w-5 h-5" /></button> }</th>
          <th className="max-w-16">Culture</th>
          <th>Religion</th>
          <th className="max-w-16">Size</th>
          <th className="max-w-20">Population Type</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        {isCreatingPop && <CreatePopForm onCreatePop={handleAddPop} onCancel={() => setIsCreatingPop(false)} />}
        {pops.sort((pop1, pop2) => pop2.size - pop1.size).map((pop) => {
          return (
            <tr key={pop.culture + pop.religion + pop.pop_type}>
              <td></td>
              <td className="max-w-16">{pop.culture}</td>
              <td>{pop.religion}</td>
              <td className="max-w-16"><input type="text" placeholder="0" className="input input-xs w-24 -ml-3" value={pop.size} onChange={(e) => handlePopulationChange(pop, parseInt(e.target.value) || 0)} /></td>
              <td>{pop.pop_type}</td>
              <td><button className="btn btn-square btn-xs btn-error" onClick={() => handleRemovePop(pop)}>
                <MinusIcon className="w-5 h-5" />
              </button></td>
            </tr>
          )
        })}
      </tbody>
    </table>
  )
}
