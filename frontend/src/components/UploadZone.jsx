import React from 'react'

export default function UploadZone({onFile}){
  const [drag, setDrag] = React.useState(false)

  function handleDrop(ev){
    ev.preventDefault()
    setDrag(false)
    const files = ev.dataTransfer.files
    if(files && files[0]) onFile(files[0])
  }

  function handleChange(ev){
    const files = ev.target.files
    if(files && files[0]) onFile(files[0])
  }

  return (
    <div className="upload-zone" onDragOver={(e)=>{e.preventDefault(); setDrag(true);}} onDragLeave={()=>setDrag(false)} onDrop={handleDrop}>
      <strong>{drag ? 'Drop file to upload' : 'Drag & drop a file here'}</strong>
      <div style={{marginTop:8}}>
        <input type="file" onChange={handleChange} />
      </div>
    </div>
  )
}
