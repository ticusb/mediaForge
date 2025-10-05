import React, {useState} from 'react'
import UploadZone from './components/UploadZone.jsx'
import Preview from './components/Preview.jsx'

export default function App(){
  const [file, setFile] = useState(null)
  const [jobs, setJobs] = useState([])

  async function handleUpload(){
    if(!file) return alert('Select a file first')
    const fd = new FormData()
    fd.append('file', file, file.name)

    try{
      const res = await fetch('/api/upload', { method: 'POST', body: fd })
      if(!res.ok) throw new Error('Upload failed')
      const json = await res.json()
      alert('Uploaded: ' + json.asset_id)
      setJobs(j => [{id: json.asset_id, status: 'uploaded', filename: file.name}, ...j])
    }catch(e){
      alert('Upload error: ' + e.message)
    }
  }

  return (
    <div style={{padding:16}}>
      <h1>MediaForge Dashboard</h1>
      <UploadZone onFile={setFile} />
      <Preview file={file} />
      <div style={{marginTop:12}}>
        <button onClick={handleUpload}>Upload</button>
      </div>
      <div style={{marginTop:16}}>
        <h3>Recent jobs</h3>
        {jobs.length === 0 ? <div>No jobs yet</div> : (
          <ul>
            {jobs.map(j => <li key={j.id}>{j.filename} — {j.status}</li>)}
          </ul>
        )}
      </div>
    </div>
  )
}
