const defaultTheme = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        'primary': {
          50: '#e8e8e9',
          100: '#d0d1d3',
          150: '#b9bbbd',
          200: '#b9bbbd',
          250: '#a2a4a7',
          300: '#8b8d91',
          350: '#73767a',
          400: '#5c5f64',
          450: '#45494e',
          500: '#2d3238',
          550: '#161b22',
          600: '#14181f',
          650: '#12161b',
          700: '#0f1318',
          750: '#0d1014',
          800: '#0b0e11',
          850: '#090b0e',
          900: '#07080a',
          950: '#020303'
        },
        'secondary': {
          100: '#f4fbfd',
          200: '#ddf4fa',
          300: '#c7ecf6',
          400: '#b0e4f2',
          500: '#99ddef',
          600: '#8ed9ed',
          700: '#72aebe',
          800: '#55828e',
          900: '#39575f'
        }
      },
      fontFamily: {
        // @ts-ignore
        'sans': ['JetBrainsMono', ...defaultTheme.fontFamily.sans]
      }
    },
  },
  plugins: [],
}
