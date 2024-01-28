/** @type {import('tailwindcss').Config} */
// eslint-disable-next-line no-undef
module.exports = {
  content: ['./src/**/*.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      gridTemplateRows: {
        layout: '0fr 1fr 0fr',
      },
    },
  },
  // eslint-disable-next-line no-undef, global-require
  plugins: [require('daisyui'), require('postcss-import')],
};
