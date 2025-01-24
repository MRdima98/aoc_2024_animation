/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    ["./**/*.{html,js,rs}"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  }
  theme: {
    extend: {
    },
  },
  plugins: [],
};
