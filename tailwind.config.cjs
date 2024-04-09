/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {}
  },
  plugins: [
    require("@catppuccin/tailwindcss")({
      defaultFlavour: "macchiato"
    }),
  ]
}
