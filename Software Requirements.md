# Software Requirements Specification – MediaForge (MVP)

## System Design
- **Frontend (Web App)**  
  - React + Tailwind CSS for responsive UI.  
  - WebAssembly modules (optional) for lightweight client-side processing (e.g., previews).  
  - Drag-and-drop upload, contextual editing controls, live preview.  

- **Backend (Core Services)**  
  - Rust for file processing (conversion, background removal, color grading).  
  - REST/GraphQL API layer exposed to frontend.  
  - Temporary file storage with auto-deletion.  

- **Storage & Security**  
  - Encrypted S3-compatible object storage for uploads.  
  - Ephemeral storage lifecycle (auto-delete after session or export).  

---

## Architecture Pattern
- **Microservices + API Gateway**  
  - Separate services for *conversion*, *background removal*, and *color grading*.  
  - API Gateway routes requests to appropriate service.  

- **Frontend Pattern**  
  - SPA (Single Page Application) in React.  

- **Scalable Deployment**  
  - Containerized services via Docker/Kubernetes.  

---

## State Management
- **Client-side**  
  - React Context + Hooks for local state (tool selection, file upload, editing parameters).  
  - Redux Toolkit if state complexity grows (e.g., multiple files, batch editing).  

- **Server-side**  
  - Stateless APIs (session token passed in requests).  
  - Session/user data stored in Redis or Postgres.  

---

## Data Flow
1. User uploads file → frontend sends file to backend (encrypted HTTPS).  
2. Backend service processes (conversion, background removal, or grading).  
3. Processed file stored in temp storage.  
4. Frontend polls or subscribes (WebSocket/long-polling) for job completion.  
5. User previews results and downloads/export.  

---

## Technical Stack
- **Frontend**: React, Tailwind CSS, Vite/Next.js.  
- **Backend**: Rust (Actix-web or Axum), optional Spring Boot for billing/subscription service.  
- **Database**: PostgreSQL (accounts, subscription status, metadata).  
- **Caching**: Redis (sessions, job queue).  
- **Storage**: S3-compatible object store (MinIO, AWS S3, DigitalOcean Spaces).  
- **Deployment**: Docker + Kubernetes, CI/CD via GitHub Actions.  

---

## Authentication Process
- **JWT-based authentication** for web sessions.  
- **OAuth2.0** support (Google, Apple, GitHub login).  
- Tokens stored in secure HTTP-only cookies.  
- Free vs. Pro access enforced via role claims in JWT.  

---

## Route Design
- **Frontend Routes (React SPA)**  
  - `/login` → authentication.  
  - `/dashboard` → main workspace.  
  - `/convert` → file conversion tools.  
  - `/remove-bg` → background removal tools.  
  - `/color-grade` → grading tools.  
  - `/export` → export/download options.  

- **API Routes (REST/GraphQL)**  
  - `POST /api/upload` → upload media.  
  - `POST /api/convert` → process file conversion.  
  - `POST /api/remove-bg` → process background removal.  
  - `POST /api/color-grade` → apply presets or manual adjustments.  
  - `GET /api/status/:jobId` → fetch job status.  
  - `GET /api/download/:jobId` → download final file.  

---

## API Design
- **Request/Response (JSON-based)**  
- File uploads via `multipart/form-data`.  
- Long-running tasks (video processing) handled asynchronously with job IDs.  
- WebSocket channel for progress updates.  

**Example Request**  
```http
POST /api/convert
Content-Type: multipart/form-data
Body: { file, options: { format: "jpg", resize: "1080x1080" } }
Response: { jobId: "12345", status: "processing" }
