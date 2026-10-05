import type { Meta, StoryObj } from '@storybook/vue3-vite'

import AppIcon from './AppIcon.vue'

const iconNames = [
  'apple',
  'arrow-right',
  'bars',
  'basket',
  'bolt',
  'bread',
  'cart',
  'chevron-down',
  'chevron-left',
  'chevron-right',
  'clock',
  'drumstick',
  'envelope',
  'fish',
  'leaf',
  'lock',
  'magnifying-glass',
  'mug',
  'store',
  'user',
  'wine',
  'xmark',
]

const meta = {
  title: 'Components/AppIcon',
  component: AppIcon,
  tags: ['autodocs'],
  parameters: {
    docs: {
      description: {
        component: 'Inline SVG icon component used throughout the grocery shopping interface.',
      },
    },
  },
  argTypes: {
    name: { control: 'select', options: iconNames },
    size: { control: { type: 'range', min: 12, max: 48, step: 2 } },
  },
  args: {
    name: 'basket',
    size: 24,
  },
} satisfies Meta<typeof AppIcon>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = {}

export const IconGallery: Story = {
  render: () => ({
    components: { AppIcon },
    setup: () => ({ iconNames }),
    template: `
      <div style="display: grid; grid-template-columns: repeat(6, minmax(80px, 1fr)); gap: 24px; width: min(720px, 100%);">
        <div v-for="name in iconNames" :key="name" style="display: grid; justify-items: center; gap: 8px; color: #555; font-size: 11px;">
          <AppIcon :name="name" :size="28" />
          <span>{{ name }}</span>
        </div>
      </div>
    `,
  }),
}
