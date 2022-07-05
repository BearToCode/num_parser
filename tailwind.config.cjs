/** @type {import('tailwindcss').Config} */

const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        'primary': {
          50: '#e9e9e9',
          100: '#d4d3d3',
          150: '#bebdbd',
          200: '#a8a7a7',
          250: '#939291',
          300: '#7d7c7a',
          350: '#676664',
          400: '#51504e',
          450: '#3c3a38',
          500: '#262422',
          550: '#22201f',
          600: '#22201f',
          650: '#1e1d1b',
          700: '#1b1918',
          750: '#171614',
          800: '#131211',
          850: '#0f0e0e',
          900: '#0b0b0a',
          950: '#080707'
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
        'sans': ['JetBrainsMono', ...defaultTheme.fontFamily.sans]
      }
    },
  },
  plugins: [],
}
