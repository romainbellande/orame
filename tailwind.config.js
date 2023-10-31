/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/**/*.{rs,html,css}',
    './index.html'
  ],
  theme: {
    extend: {},
  },
  variant: {},
  plugins: [require('@tailwindcss/forms')],
}
