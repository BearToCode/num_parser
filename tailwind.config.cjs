/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        'primary': {
          100: '#36393F',
          200: '#21262d',
          300: '#161b22',
          400: '#0D1117'
        }
      }
    },
  },
  plugins: [],
}
