/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx,html,vue}",
    "./public/**/*.html"
  ],
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
