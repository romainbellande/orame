/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/**/*.{rs,html,css}',
    './index.html'
  ],
  theme: {
    extend: {
      transitionProperty: {
        'max-height': 'max-height',
      }
    },
  },
  variant: {},
  plugins: [require('@tailwindcss/forms')],
}
