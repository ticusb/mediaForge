/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                primary: {
                    DEFAULT: '#3A8DFF',
                    50: '#E8F2FF',
                    100: '#D1E5FF',
                    200: '#A3CBFF',
                    300: '#75B1FF',
                    400: '#4797FF',
                    500: '#3A8DFF',
                    600: '#0A6EFF',
                    700: '#0056D6',
                    800: '#003FA3',
                    900: '#002870',
                },
                success: {
                    DEFAULT: '#34C759',
                    50: '#E8F8ED',
                    100: '#D1F1DB',
                    200: '#A3E3B7',
                    300: '#75D593',
                    400: '#47C76F',
                    500: '#34C759',
                    600: '#29A047',
                    700: '#1F7935',
                    800: '#145223',
                    900: '#0A2B12',
                },
                sidebar: {
                    light: '#F5F5F5',
                    dark: '#2A2A2A',
                },
            },
            fontFamily: {
                sans: ['Inter', 'system-ui', '-apple-system', 'BlinkMacSystemFont',
                    'Segoe UI', 'Roboto', 'Helvetica Neue', 'Arial', 'sans-serif'],
            },
            boxShadow: {
                'soft': '0 2px 8px rgba(0, 0, 0, 0.08)',
                'medium': '0 4px 16px rgba(0, 0, 0, 0.12)',
                'strong': '0 8px 32px rgba(0, 0, 0, 0.16)',
            },
            animation: {
                'fade-in': 'fadeIn 0.3s ease-in-out',
                'slide-up': 'slideUp 0.3s ease-out',
                'scale-in': 'scaleIn 0.2s ease-out',
            },
            keyframes: {
                fadeIn: {
                    '0%': { opacity: '0' },
                    '100%': { opacity: '1' },
                },
                slideUp: {
                    '0%': { transform: 'translateY(20px)', opacity: '0' },
                    '100%': { transform: 'translateY(0)', opacity: '1' },
                },
                scaleIn: {
                    '0%': { transform: 'scale(0.95)', opacity: '0' },
                    '100%': { transform: 'scale(1)', opacity: '1' },
                },
            },
        },
    },
    plugins: [],
}