/** @type {import('tailwindcss').Config} */
module.exports = {
    safelist: [
        {
            pattern: /^from-(orange|blue|yellow|sky|cyan|ink|purple|red|emerald|teal|indigo|amber|rust|green|lime|pink|rose|violet|gray|slate|zinc|neutral|stone|fuchsia)(-\d{2,3})?$/,
            variants: ['hover', 'focus'],
        },
        {
            pattern: /^to-(orange|blue|yellow|sky|cyan|ink|purple|red|emerald|teal|indigo|amber|rust|green|lime|pink|rose|violet|gray|slate|zinc|neutral|stone)(-\d{2,3})?$/,
            variants: ['hover', 'focus'],
        },
        {
            pattern: /^via-(orange|blue|yellow|sky|cyan|ink|purple|red|emerald|teal|indigo|amber|rust|green|lime|pink|rose|violet|gray|slate|zinc|neutral|stone)(-\d{2,3})?$/,
            variants: ['hover', 'focus'],
        },
    ],
    content: [
        "./src/**/*.rs",
        "./index.html",
    ],
    theme: {
        extend: {
            fontFamily: {
                sans: ['Inter', 'system-ui', 'sans-serif'],
                mono: ['Manrope', 'monospace'],
            },
            colors: {
                rust: {
                    50: '#fff5ed',
                    100: '#ffe8d4',
                    200: '#ffcca8',
                    300: '#ffa771',
                    400: '#ff7a38',
                    500: '#ff5a10',
                    600: '#e83f06',
                    700: '#c02e08',
                    800: '#98260f',
                    900: '#7a2210',
                },
                ink: {
                    50: '#f6f7f9',
                    100: '#ecedf1',
                    200: '#d4d7df',
                    300: '#aeb3c0',
                    400: '#818899',
                    500: '#616879',
                    600: '#4d5362',
                    700: '#404551',
                    800: '#1a1d26',
                    900: '#12141b',
                    950: '#0a0b10',
                }
            },
            animation: {
                'float': 'float 6s ease-in-out infinite',
                'fade-up': 'fadeUp 0.8s ease-out both',
                'ping': 'ping 1s cubic-bezier(0, 0, 0.2, 1) infinite',
            },
            keyframes: {
                float: {
                    '0%, 100%': {transform: 'translateY(0px)'},
                    '50%': {transform: 'translateY(-12px)'},
                },
                fadeUp: {
                    '0%': {opacity: '0', transform: 'translateY(24px)'},
                    '100%': {opacity: '1', transform: 'translateY(0)'},
                },
            }
        }
    },
    plugins: [],
    darkMode: 'class',
};