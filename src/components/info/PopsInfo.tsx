import { useEffect, useRef, useState } from "react"
import { Pop } from "../States"

type PopsInfoProps = {
  pops: Pop[]
  onPopsChange: (pops: Pop[]) => void
}

const presentString = (value: string) => value === '' ? null : value

const PlusButton = ({ onClick }: { onClick: () => void }) => {
  return (
    <button className="btn btn-square btn-xs btn-success" onClick={onClick}>
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
        <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
      </svg>
    </button>
  )
}

const MinusButton = ({ onClick }: { onClick: () => void }) => {
  return (
    <button className="btn btn-square btn-xs btn-error" onClick={onClick}>
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
        <path strokeLinecap="round" strokeLinejoin="round" d="M5 12h14" />
      </svg>
    </button>
  )
}

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
      <td><PlusButton onClick={handleCreatePop} /></td>
      <td><input ref={inputRef} type="text" placeholder="Culture" className="input input-sm w-24 -ml-3" value={culture} onChange={(e) => setCulture(e.target.value)} /></td>
      <td><input type="text" placeholder="Religion" className="input input-sm w-24 -ml-3" value={religion} onChange={(e) => setReligion(e.target.value)} /></td>
      <td><input type="text" placeholder="Size" className="input input-sm w-24 -ml-3" value={size} onChange={(e) => setSize(parseInt(e.target.value) || 0)} /></td>
      <td><input type="text" placeholder="Population Type" className="input input-sm w-24 -ml-3" value={popType} onChange={(e) => setPopType(e.target.value)} /></td>
      <td><MinusButton onClick={onCancel} /></td>
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
    <table className="table">
      <thead>
        <tr>
          <th>{ !isCreatingPop && <PlusButton onClick={() => setIsCreatingPop(true)} /> }</th>
          <th>Culture</th>
          <th>Religion</th>
          <th>Size</th>
          <th>Population Type</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        {isCreatingPop && <CreatePopForm onCreatePop={handleAddPop} onCancel={() => setIsCreatingPop(false)} />}
        {pops.sort((pop1, pop2) => pop2.size - pop1.size).map((pop) => {
          return (
            <tr key={pop.culture + pop.religion + pop.pop_type}>
              <td></td>
              <td>{pop.culture}</td>
              <td>{pop.religion}</td>
              <td><input type="text" placeholder="0" className="input input-sm w-24 -ml-3" value={pop.size} onChange={(e) => handlePopulationChange(pop, parseInt(e.target.value) || 0)} /></td>
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
  )
}
