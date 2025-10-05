import React from 'react'

export default function Preview({file}){
  const [url, setUrl] = React.useState(null)
  React.useEffect(()=>{
    if(!file) return setUrl(null)
    const u = URL.createObjectURL(file)
    setUrl(u)
    return ()=> URL.revokeObjectURL(u)
  }, [file])

  if(!file) return <div className="preview">No file selected</div>
  const isImage = file.type.startsWith('image/')
  return (
    <div className="preview">
      <div>Name: {file.name} ({Math.round(file.size/1024)} KB)</div>
      {isImage ? <img src={url} alt={file.name} style={{maxWidth:320, marginTop:8}} /> : <div>Video file selected (preview not available)</div>}
    </div>
  )
}
