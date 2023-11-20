module.exports = {
  content: [
    './html/**/*.{tera}',
  ],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {
      animation: ['group-hover']
    },
  },
  plugins: [],
}
