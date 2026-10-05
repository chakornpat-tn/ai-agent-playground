import type { Meta, StoryObj } from '@storybook/vue3-vite'

import ProductRating from './ProductRating.vue'

const meta = {
  title: 'Components/ProductRating',
  component: ProductRating,
  tags: ['autodocs'],
  parameters: {
    docs: {
      description: {
        component: 'Compact five-star product rating with an optional review count.',
      },
    },
  },
  argTypes: {
    rating: { control: { type: 'range', min: 0, max: 5, step: 0.5 } },
    reviews: { control: { type: 'number', min: 0, step: 1 } },
  },
  args: {
    rating: 4.5,
    reviews: 128,
  },
} satisfies Meta<typeof ProductRating>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = {}

export const RatingStates: Story = {
  render: () => ({
    components: { ProductRating },
    template: `
      <div style="display: grid; gap: 16px; font-family: sans-serif;">
        <ProductRating :rating="5" :reviews="256" />
        <ProductRating :rating="3.5" :reviews="42" />
        <ProductRating :rating="0" />
      </div>
    `,
  }),
}
