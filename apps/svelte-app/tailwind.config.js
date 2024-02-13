/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      gridTemplateRows: {
        layout: '0fr 1fr 0fr'
      }
    }
  },
  plugins: [require('daisyui'), require('postcss-import')]
};
