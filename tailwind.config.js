module.exports = {
  content: ['./src/**/*.rs', './index.html'],
  darkMode: 'class',
  theme: {
    extend: {
      screens: {
        short: { raw: '(max-height: 650px)' },
        xshort: { raw: '(max-height: 560px)' },
        xxshort: { raw: '(max-height: 490px)' },
      },
    },
  },
  plugins: [],
}
