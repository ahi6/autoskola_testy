/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      spacing: {
        '96': '24rem',
      },
      maxHeight: {
        '96': '24rem',
      },
    },
  },
  safelist: [
    'space-y-4',
    'space-y-6',
    'justify-start',
    'justify-end',
    'max-w-[80%]',
    'bg-gray-200',
    'bg-blue-500',
    'border-gray-200',
    'rounded-lg',
    'mb-3',
    'overflow-y-auto'
  ],
  plugins: [],
};
