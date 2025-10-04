# Product Requirements Document (PRD) - MediaForge (MVP)

## 1. Elevator Pitch
MediaForge is a subscription-based SaaS platform that delivers professional-grade image and video editing directly in the browser. Built for creators, small businesses, and hobbyists, it offers quick, intuitive tools like file conversion, background removal, and color grading without the steep learning curve or heavy cost of desktop software. With a responsive, Adobe-lite interface and Rust-powered backend for speed and security, MediaForge makes media editing accessible, fast, and affordable.

## 2. Who is this app for
- **Content Creators & Influencers**: YouTubers, TikTokers, Instagram users editing thumbnails, short clips, or color grading videos.  
- **Small Businesses & Marketers**: E-commerce owners preparing product photos, social managers making promotional media.  
- **Hobbyists & Educators**: Casual users making memes, family videos, or school/teaching visuals.  

## 3. Functional Requirements (MVP Scope)
- **File Conversion**
  - Image: JPG, PNG, WEBP, GIF, HEIC
  - Options: resize, compress, batch process
  - Video: MP4, MOV, AVI
  - Options: trim, change resolution, adjust frame rates, merge clips
- **Background Removal**
  - Images: algorithmic background removal, option to replace with custom background
  - Videos: basic frame-by-frame subject separation (short clips, up to 30s)
- **Color Grading**
  - Presets (e.g., Vintage, Cinematic, Bright & Poppy)
  - Manual adjustments: hue, saturation, brightness, contrast
  - LUT (Look-Up Table) import support
- **Account & Subscription**
  - Basic (free/low cost): limited daily use, watermarked outputs, access to core converters
  - Pro ($9.99/month): unlimited use, no watermark, priority processing, advanced tools
  - Enterprise (later phase): team accounts, API, collaboration (not in MVP)
- **Security**
  - Encrypted uploads, temporary file storage with auto-deletion
  - Compliance: GDPR/CCPA friendly practices

## 4. User Stories
- **As a content creator**, I want to convert my iPhone photos to JPG and resize them for Instagram so I can post quickly.  
- **As a YouTuber**, I want to remove the background from a clip to make a clean thumbnail.  
- **As a small business owner**, I want to color grade product photos with presets so they look consistent on my online store.  
- **As a hobbyist**, I want to merge a few short videos into one clip to share with friends.  
- **As a teacher**, I want to compress videos so I can easily share them with students online.  

## 5. User Interface
- **Layout**
  - Clean, Adobe-lite style dashboard
  - Drag-and-drop upload area for images and videos
  - Left-side navigation panel for tool categories (Convert, Remove Background, Color Grade)
  - Real-time preview window with side-by-side before/after
  - Top bar with account access, subscription status, and export options
- **Design**
  - Minimalist, responsive layout for desktop and mobile browsers
  - Light and dark mode support
  - One-click actions for presets; sliders for manual fine-tuning
- **Export**
  - Download to device in selected format
  - Option to save to user account (Pro tier)

---
