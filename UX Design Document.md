# MediaForge – User Interface Design Document

## Layout Structure
- **Left Sidebar:**
  - Tool categories: *Convert, Remove Background, Color Grade*.
  - Icons + text labels for clarity.
  - Sidebar collapsible for mobile.

- **Main Workspace:**
  - Central drag-and-drop upload zone (large, prominent).
  - Once file is uploaded, workspace shifts to show real-time editing area.

- **Preview Panel:**
  - Side-by-side before/after preview.
  - Toggle button for single/fullscreen preview.

- **Top Bar:**
  - Account access (avatar icon, dropdown for settings).
  - Subscription status (Free / Pro).
  - Export button (always visible).

---

## Core Components
- **File Upload Zone**: drag-and-drop + “Browse” button.
- **Tool Controls Panel** (right side, contextual):
  - Convert: format dropdown, resolution/size sliders.
  - Background: remove/replace, upload custom background.
  - Color Grade: presets grid, sliders for hue/brightness/contrast.
- **Preview Window:** interactive, updates in real time.
- **Export Panel:** format options + download button.

---

## Interaction Patterns
- **Drag-and-drop first interaction** (core entry point).
- **Contextual controls** appear after upload, matching tool selected.
- **Undo/redo buttons** always visible in editing mode.
- **Presets are one-click actions**; manual sliders allow fine-tuning.
- **Export confirmation modal** with file size and format options.

---

## Visual Design Elements & Color Scheme
- **Style:** Minimalist, Adobe-lite inspired.
- **Colors:**
  - Primary: Cool blue (#3A8DFF) for action buttons.
  - Secondary: Neutral gray for sidebar (#F5F5F5 light / #2A2A2A dark).
  - Accent: Success green (#34C759) for export/download.
- **Light & Dark Modes:** toggle available in account menu.
- **Icons:** simple line-based icons (upload, tools, preview).

---

## Mobile, Web App, Desktop Considerations
- **Mobile:**
  - Sidebar collapses into bottom navigation bar.
  - Preview window stacks above tool controls.
- **Web App (Desktop browsers):**
  - Default full layout (sidebar + workspace + controls).
  - Responsive grid ensures resizing works smoothly.
- **Desktop App (future phase):**
  - Could wrap web version with Electron for offline use.

---

## Typography
- **Primary Font:** Inter or Roboto (clean, accessible sans-serif).
- **Hierarchy:**
  - Tool headings: Bold, 18px
  - Labels & sliders: Regular, 14px
  - Buttons: Medium weight, 16px

---

## Accessibility
- **Keyboard navigation:** all tools accessible via Tab / Enter.
- **Color contrast compliance:** WCAG AA minimum.
- **Alt text prompts:** for uploaded images/videos where possible.
- **Clear error states:** e.g., unsupported file type message.
