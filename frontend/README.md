# MediaForge Frontend - Complete Installation Guide

## 📋 Overview
This guide provides step-by-step instructions to implement the new MediaForge frontend with a professional Adobe-lite interface, free-tier-first approach, and clean architecture.

## 🎯 What's Included
- **Adobe-lite Interface**: Left sidebar navigation, professional workspace
- **Free-Tier First**: 3 free uploads, no login required initially
- **Smart Upgrade Flow**: Context-aware prompts when limits are reached
- **Tool Panels**: Convert, Remove Background, Color Grading
- **Real-time Preview**: Professional file preview with metadata
- **Job Management**: Track all processed files

## 📁 File Structure
```
frontend/
├── src/
│   ├── components/
│   │   ├── Sidebar.jsx              ← Left navigation panel
│   │   ├── Header.jsx               ← Top bar with user status
│   │   ├── Workspace.jsx            ← Main work area container
│   │   ├── UploadZone.jsx           ← Drag & drop file upload
│   │   ├── Preview.jsx              ← File preview with metadata
│   │   ├── ToolPanel.jsx            ← Contextual editing tools
│   │   ├── JobsList.jsx             ← Recent jobs display
│   │   ├── Modal.jsx                ← Base modal component
│   │   ├── AuthModal.jsx            ← Login/Register modal
│   │   └── FreeLimitModal.jsx       ← Upgrade prompt modal
│   ├── App.jsx                      ← Main application
│   ├── main.jsx                     ← Entry point (keep existing)
│   └── index.css                    ← Global styles with Tailwind
├── tailwind.config.js               ← Tailwind configuration
├── postcss.config.js                ← PostCSS config (auto-generated)
└── package.json                     ← Dependencies (update if needed)
```

## 🚀 Step-by-Step Installation

### Step 1: Install Dependencies

```bash
cd frontend

# Install Tailwind CSS and dependencies
npm install -D tailwindcss@latest postcss@latest autoprefixer@latest

# Initialize Tailwind (creates tailwind.config.js and postcss.config.js)
npx tailwindcss init -p

# Optional: Install Inter font for better typography
npm install @fontsource/inter
```

### Step 2: Replace/Create Component Files

Create the `src/components/` directory if it doesn't exist:
```bash
mkdir -p src/components
```

Copy each component from the artifacts above:

1. **src/App.jsx** - Copy from artifact "App.jsx - Main Application"
2. **src/components/Sidebar.jsx** - Copy from artifact "Sidebar.jsx - Tool Navigation"
3. **src/components/Header.jsx** - Copy from artifact "Header.jsx - Top Navigation Bar"
4. **src/components/Workspace.jsx** - Copy from artifact "Workspace.jsx - Main Work Area"
5. **src/components/UploadZone.jsx** - Copy from artifact "UploadZone.jsx - File Upload Area"
6. **src/components/Preview.jsx** - Copy from artifact "Preview.jsx - File Preview Display"
7. **src/components/ToolPanel.jsx** - Copy from artifact "ToolPanel.jsx - Editing Tools Controls"
8. **src/components/JobsList.jsx** - Copy from artifact "JobsList.jsx - Recent Jobs Display"
9. **src/components/Modal.jsx** - Copy from artifact "Modal.jsx - Base Modal Component"
10. **src/components/AuthModal.jsx** - Copy from artifact "AuthModal.jsx - Authentication Modal"
11. **src/components/FreeLimitModal.jsx** - Copy from artifact "FreeLimitModal.jsx - Free Limit Notification"

### Step 3: Update Configuration Files

**Replace `src/index.css`:**
```bash
# Copy from artifact "index.css - Global Styles"
```

**Replace `tailwind.config.js`:**
```bash
# Copy from artifact "tailwind.config.js - Tailwind Configuration"
```

### Step 4: Optional - Add Inter Font

If you want to use the Inter font (recommended):

```jsx
// Add to src/main.jsx at the top
import '@fontsource/inter';
```

Or add via CDN in `index.html`:
```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
```

### Step 5: Clean Up Old Files

Remove files that are no longer needed:
```bash
rm src/App.css
rm src/styles.css
rm src/reportWebVitals.js  # if exists
```

### Step 6: Update Vite Config (if needed)

Ensure your `vite.config.js` has the proxy configured:

```javascript
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    port: 3000,
    proxy: {
      '/api': {
        target: 'http://localhost:8080',  // Your backend URL
        changeOrigin: true,
      }
    }
  }
})
```

### Step 7: Start Development Server

```bash
npm run dev
```

Visit `http://localhost:3000` to see your new MediaForge dashboard!

## ✨ Features Overview

### 1. Free Tier Experience
- ✅ Immediate access without login
- ✅ 3 free uploads per day
- ✅ Visual counter showing remaining uploads
- ✅ Warning at 1 upload remaining
- ✅ Blocking modal when limit reached

### 2. User Interface
- ✅ Left sidebar with tool categories
- ✅ Main workspace with drag & drop
- ✅ Real-time file preview
- ✅ Contextual tool panels
- ✅ Professional color scheme
- ✅ Smooth animations and transitions

### 3. Tools (UI Ready, Backend Integration Needed)
- 🔄 **Convert**: Format, quality, resize options
- ✂️ **Remove Background**: One-click removal, color replacement
- 🎨 **Color Grade**: Presets and manual adjustments

### 4. Job Management
- ✅ Track all uploaded files
- ✅ Status indicators
- ✅ Watermark badges for free users
- ✅ Download buttons
- ✅ Timestamps

## 🎨 Customization Guide

### Change Colors

Edit `tailwind.config.js`:
```javascript
colors: {
  primary: {
    DEFAULT: '#YOUR_COLOR',
    // ... other shades
  },
}
```

### Change Free Upload Limit

In `src/App.jsx`:
```javascript
const [freeUploadsRemaining, setFreeUploadsRemaining] = useState(5); // Change from 3
```

### Change Pricing

In `src/components/AuthModal.jsx` and `src/components/FreeLimitModal.jsx`:
```javascript
$9.99/month → $14.99/month
```

### Add/Remove Tools

In `src/components/Sidebar.jsx`:
```javascript
const tools = [
  {
    id: 'your-tool',
    name: 'Your Tool',
    icon: '🔧',
    description: 'Tool description'
  },
  // ... other tools
];
```

Then add corresponding case in `src/components/ToolPanel.jsx`.

## 🔌 Backend Integration

### API Endpoints Required

Your backend should implement:

```
POST /api/upload
  - Accepts: multipart/form-data with file
  - Returns: { asset_id: string }

POST /api/auth/login
  - Accepts: { email, password }
  - Returns: { user: {...}, token: string }

POST /api/auth/register
  - Accepts: { email, password }
  - Returns: { user: {...}, token: string }

GET /api/jobs
  - Returns: [{ id, filename, status, ... }]

GET /api/download/:jobId
  - Returns: processed file
```

### Adding Real Authentication

Replace mock auth in `src/components/AuthModal.jsx`:

```javascript
const handleSubmit = async (e) => {
  e.preventDefault();
  setIsLoading(true);

  try {
    const response = await fetch('/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email, password })
    });
    
    const data = await response.json();
    
    if (response.ok) {
      onAuth(data.user);
      // Store token in memory or secure cookie
    } else {
      alert(data.message || 'Authentication failed');
    }
  } catch (error) {
    alert('Network error');
  } finally {
    setIsLoading(false);
  }
};
```

## 🐛 Troubleshooting

### Tailwind Classes Not Working
```bash
# Restart dev server
npm run dev

# Check tailwind.config.js content paths
content: ["./index.html", "./src/**/*.{js,jsx}"]

# Verify index.css has @tailwind directives
```

### Components Not Found
```bash
# Check file paths and exports
# Ensure all components use `export default`
```

### API Calls Failing
```bash
# Check vite.config.js proxy settings
# Verify backend is running
# Check browser console for CORS errors
```

### Upload Not Working
```bash
# Verify FormData is being sent correctly
# Check backend accepts multipart/form-data
# Ensure file size limits are appropriate
```

## 📱 Responsive Design

The interface is fully responsive:
- **Desktop**: Full sidebar + workspace layout
- **Tablet**: Collapsible sidebar, stacked layout
- **Mobile**: Bottom navigation, vertical stacking

To test:
```bash
# Open browser dev tools
# Toggle device toolbar
# Test at various breakpoints
```

## 🔐 Security Considerations

1. **Never store sensitive data in localStorage** (already avoided)
2. **Use HTTP-only cookies for tokens**
3. **Implement CSRF protection**
4. **Validate file types server-side**
5. **Limit file sizes appropriately**
6. **Rate limit API endpoints**

## 📊 Next Steps

1. **Backend Integration**: Connect real API endpoints
2. **Stripe Integration**: Add payment processing
3. **File Processing**: Implement actual conversion, bg removal, color grading
4. **User Persistence**: Store user state and preferences
5. **Analytics**: Track usage and conversions
6. **Email Notifications**: Upgrade reminders, newsletters
7. **Dark Mode**: Toggle between light/dark themes
8. **Keyboard Shortcuts**: Add power user features

## 💡 Tips

- Start with the upload flow and get that working end-to-end
- Add real processing tools one at a time
- Test the upgrade flow thoroughly - it's your revenue driver
- Monitor free tier usage to optimize conversion rates
- Collect user feedback early and iterate

## 🆘 Getting Help

If you encounter issues:
1. Check browser console for errors
2. Verify all files are in correct locations
3. Ensure dependencies are installed
4. Check that backend API is accessible
5. Review network tab for failed requests

## 📝 License

This is part of the MediaForge MVP project. Adjust as needed for your use case.

---

**Created with ❤️ for MediaForge MVP**